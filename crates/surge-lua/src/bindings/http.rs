use std::time::{Duration, Instant};

use mlua::{Lua, LuaSerdeExt, LuaString, Table, Value};
use reqwest::{Client, Method};
use surge_common::parse_duration;

/// Options accepted as the trailing `params` table of every request function.
#[derive(Default)]
struct Params {
    headers: Vec<(String, String)>,
    timeout: Option<Duration>,
}

impl Params {
    fn parse(table: Option<Table>) -> mlua::Result<Self> {
        let Some(table) = table else {
            return Ok(Self::default());
        };

        let mut headers = Vec::new();
        if let Some(header_table) = table.get::<Option<Table>>("headers")? {
            for pair in header_table.pairs::<String, String>() {
                headers.push(pair?);
            }
        }

        let timeout = match table.get::<Value>("timeout")? {
            Value::Nil => None,
            Value::String(s) => Some(parse_duration(&s.to_str()?)?),
            // Bare numbers are milliseconds, like k6.
            Value::Integer(ms) if ms >= 0 => Some(Duration::from_millis(ms as u64)),
            Value::Number(ms) if ms >= 0.0 => Some(Duration::from_secs_f64(ms / 1000.0)),
            other => {
                return Err(mlua::Error::runtime(format!("invalid timeout: {other:?}")));
            }
        };

        Ok(Self { headers, timeout })
    }
}

struct RawResponse {
    status: u16,
    status_text: &'static str,
    headers: Vec<(String, Vec<u8>)>,
    body: Vec<u8>,
}

#[derive(Clone)]
struct HttpContext {
    client: Client,
    response_mt: Table,
}

impl HttpContext {
    async fn request(
        &self,
        lua: Lua,
        method: Method,
        url: String,
        body: Option<Vec<u8>>,
        params: Params,
    ) -> mlua::Result<Table> {
        let mut builder = self.client.request(method, url);
        for (name, value) in params.headers {
            builder = builder.header(name, value);
        }
        if let Some(timeout) = params.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(body) = body {
            builder = builder.body(body);
        }

        let start = Instant::now();
        let result = Self::send(builder).await;
        let duration = start.elapsed();

        let res = lua.create_table()?;
        res.set_metatable(Some(self.response_mt.clone()))?;
        let timings = lua.create_table()?;
        timings.set("duration_ms", duration.as_secs_f64() * 1000.0)?;
        res.set("timings", timings)?;

        let headers = lua.create_table()?;
        match result {
            Ok(raw) => {
                res.set("status", raw.status)?;
                res.set("status_text", raw.status_text)?;
                res.set("body", lua.create_string(&raw.body)?)?;
                for (name, value) in raw.headers {
                    // Repeated headers are folded into one comma-separated value.
                    let merged = match headers.get::<Option<LuaString>>(name.as_str())? {
                        Some(existing) => [&existing.as_bytes()[..], b", ", &value].concat(),
                        None => value,
                    };
                    headers.set(name, lua.create_string(&merged)?)?;
                }
            }
            Err(err) => {
                // A failed request is reported, not raised, so one bad request
                // doesn't kill the VU's iteration loop.
                res.set("status", 0)?;
                res.set("status_text", "")?;
                res.set("body", "")?;
                res.set("error", format_error(&err))?;
            }
        }
        res.set("headers", headers)?;

        Ok(res)
    }

    async fn send(builder: reqwest::RequestBuilder) -> reqwest::Result<RawResponse> {
        let response = builder.send().await?;
        let status = response.status();
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| (name.to_string(), value.as_bytes().to_vec()))
            .collect();
        let body = response.bytes().await?.to_vec();

        Ok(RawResponse {
            status: status.as_u16(),
            status_text: status.canonical_reason().unwrap_or(""),
            headers,
            body,
        })
    }
}

/// Includes the source chain, since reqwest's top-level message alone
/// (e.g. "error sending request") hides the actual cause.
fn format_error(err: &dyn std::error::Error) -> String {
    let mut message = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}

fn parse_method(method: &str) -> mlua::Result<Method> {
    Method::from_bytes(method.to_ascii_uppercase().as_bytes())
        .map_err(|_| mlua::Error::runtime(format!("invalid HTTP method: {method:?}")))
}

fn body_bytes(body: Option<LuaString>) -> Option<Vec<u8>> {
    body.map(|b| b.as_bytes().to_vec())
}

fn create_response_metatable(lua: &Lua) -> mlua::Result<Table> {
    let methods = lua.create_table()?;
    methods.set(
        "json",
        lua.create_function(|lua, this: Table| {
            let body: LuaString = this.get("body")?;
            let value: serde_json::Value =
                serde_json::from_slice(&body.as_bytes()).map_err(mlua::Error::external)?;
            lua.to_value(&value)
        })?,
    )?;

    let mt = lua.create_table()?;
    mt.set("__index", methods)?;
    Ok(mt)
}

pub fn module(lua: &Lua) -> anyhow::Result<Table> {
    let ctx = HttpContext {
        client: Client::new(),
        response_mt: create_response_metatable(lua)?,
    };
    let module = lua.create_table()?;

    // `fn(url, params)`
    for (name, method) in [("get", Method::GET), ("head", Method::HEAD)] {
        let ctx = ctx.clone();
        let func =
            lua.create_async_function(move |lua, (url, params): (String, Option<Table>)| {
                let ctx = ctx.clone();
                let method = method.clone();
                let params = Params::parse(params);
                async move { ctx.request(lua, method, url, None, params?).await }
            })?;
        module.set(name, func)?;
    }

    // `fn(url, body, params)`
    for (name, method) in [
        ("post", Method::POST),
        ("put", Method::PUT),
        ("patch", Method::PATCH),
        ("delete", Method::DELETE),
        ("options", Method::OPTIONS),
    ] {
        let ctx = ctx.clone();
        let func = lua.create_async_function(
            move |lua, (url, body, params): (String, Option<LuaString>, Option<Table>)| {
                let ctx = ctx.clone();
                let method = method.clone();
                let body = body_bytes(body);
                let params = Params::parse(params);
                async move { ctx.request(lua, method, url, body, params?).await }
            },
        )?;
        module.set(name, func)?;
    }

    // `request(method, url, body, params)`
    let request =
        lua.create_async_function(
            move |lua,
                  (method, url, body, params): (
                String,
                String,
                Option<LuaString>,
                Option<Table>,
            )| {
                let ctx = ctx.clone();
                let method = parse_method(&method);
                let body = body_bytes(body);
                let params = Params::parse(params);
                async move { ctx.request(lua, method?, url, body, params?).await }
            },
        )?;
    module.set("request", request)?;

    Ok(module)
}
