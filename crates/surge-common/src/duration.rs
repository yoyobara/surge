use std::time::Duration;

use anyhow::{anyhow, bail};
use mlua::Value;

/// Parses durations like `"500ms"`, `"30s"`, `"1.5m"`, `"1h"`.
/// A bare number is treated as seconds.
pub fn parse_duration(s: &str) -> anyhow::Result<Duration> {
    let s = s.trim();
    let split = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(s.len());
    let (number, unit) = s.split_at(split);

    let value: f64 = number
        .parse()
        .map_err(|_| anyhow!("invalid duration: {s:?}"))?;
    let secs = match unit {
        "ms" => value / 1000.0,
        "s" | "" => value,
        "m" => value * 60.0,
        "h" => value * 3600.0,
        _ => bail!("invalid duration: {s:?}"),
    };

    Ok(Duration::from_secs_f64(secs))
}

/// Converts a bare number of milliseconds into a `Duration`.
pub fn duration_from_millis(ms: f64) -> anyhow::Result<Duration> {
    if !ms.is_finite() || ms < 0.0 {
        bail!("invalid duration: {ms}ms");
    }

    Ok(Duration::from_secs_f64(ms / 1000.0))
}

/// Accepts a Lua duration string (`"500ms"`, `"1.5s"`) or a bare number of
/// milliseconds, like k6.
pub fn parse_duration_value(value: Value) -> mlua::Result<Duration> {
    match value {
        Value::String(s) => Ok(parse_duration(&s.to_str()?)?),
        Value::Integer(ms) => Ok(duration_from_millis(ms as f64)?),
        Value::Number(ms) => Ok(duration_from_millis(ms)?),
        other => Err(mlua::Error::runtime(format!("invalid duration: {other:?}"))),
    }
}
