# Lua bindings surge needs to supply

Scripts are already loaded through `LuaModuleLoader` / `setup_lua_vm` (surge-runtime)
and every VU runs `run()` in a loop via `call_async`. This doc proposes what
`lua_bindings` should register into that `Lua` instance, grouped by how badly
the framework needs it, plus implementation notes specific to this codebase.

## How bindings should be wired in

`package.loaders` already reserves slot 1 (preload) and slot 2 (the custom
file searcher, see `surge-runtime/src/lua/mod.rs`). Native modules should be
installed into `package.preload["<name>"]` so scripts do `local http =
require("http")` exactly like a user module, but resolve to Rust before the
file searcher ever runs. Suggest `lua_bindings` exposes one entry point:

```rust
pub fn install(lua: &mlua::Lua) -> anyhow::Result<()>
```

called from `setup_lua_vm` right after `modify_loaders`. Each module
(`http`, `metrics`, `json`, ...) gets its own `install_<module>(lua)` internally,
called by `install`.

**Async is mandatory for anything that blocks.** `Vu::mainloop` drives `run()`
with `call_async`, so the whole VU is a single task on the tokio runtime. Any
binding that does I/O (`http.*`, `sleep`) must be built with
`lua.create_async_function`, not `create_function` — a blocking call there
stalls every VU sharing that worker thread, not just the caller.

---

## Priority 1 — needed before this is usable as an HTTP load tester

### `http`
```lua
local http = require("http")

local res = http.get(url, { headers = {...}, timeout = "30s" })
local res = http.post(url, body, { headers = {...} })
-- also: put, patch, delete, head, options
local res = http.request(method, url, body, params)
```
- `Response`: `status`, `status_text`, `body` (string), `headers` (table),
  `timings = { duration_ms }`, plus `res:json()` to decode the body.
- On connection failure, mirror k6 rather than raising a Lua error: return a
  `Response` with `status = 0` and an `error` field. A single failed request
  shouldn't kill the VU's iteration loop.
- Implementation: `reqwest::Client` (rustls, not openssl — avoids build
  friction). Build **one client and reuse it** across requests (per VU or
  shared across all VUs) for connection pooling; `reqwest::Client` is cheap
  to clone.

### `check`
```lua
check(res, {
  ["status is 200"] = function(r) return r.status == 200 end,
})
```
Minimal version can just count pass/fail and print; this is the hook later
metrics/reporting will attach to, so worth stubbing even before the metrics
module exists.

---

## Priority 2 — core VU/test lifecycle

### `sleep`
`sleep(seconds)` — async, backed by `tokio::time::sleep`. Without this every
script either busy-loops or can't pace requests at all.

### VU/iteration context
Something like a `surge` module: `surge.vu_id`, `surge.iteration`,
`surge.env(name)`. Needed for data partitioning ("VU 3 uses row 3 of the
CSV") and for reading CLI/environment parameters into a script.

### `json`
`json.encode(value)` / `json.decode(str)` — needed to build request bodies
and parse arbitrary responses. `res:json()` above can just be
`json.decode(res.body)` under the hood. `mlua`'s `serde` feature
(`LuaSerdeExt`) makes this a thin wrapper over `serde_json::Value`.

---

## Priority 3 — realistic script authoring

### `config` extensions
Today `config = { vus, duration }`. Consider also:
- `config.stages = { { duration = "30s", target = 50 }, ... }` for ramping.
- `config.thresholds` for pass/fail criteria on metrics.
- `setup()` / `teardown()` alongside `run` in the entrypoint table, called
  once per test run rather than per iteration.

### `group(name, fn)`
Tags a block of checks/requests for later reporting, mirrors k6's `group`.

### `metrics`
Custom measurement types beyond built-in HTTP timings:
```lua
local Trend = require("metrics").Trend
local page_load = Trend.new("page_load")
page_load.add(res.timings.duration_ms)
```
(`Counter`, `Gauge`, `Rate`, `Trend`.)

### shared test data
Each `Vu::new` spins up its own `Lua` VM, so a naive `require("mydata")`
re-parses the file per VU. Worth a `data` module that loads once (e.g. from
the loader or a process-wide cache) and hands VUs a read-only view — think
k6's `SharedArray`.

---

## Priority 4 — nice-to-have parity with k6

- `crypto` — md5/sha256/hmac, for signed requests.
- `uuidv4()`, `randomIntBetween`, `randomItem` — test data generation.
- `ws` — websocket support, if surge wants to load-test more than plain HTTP.

---

## Dependency notes for `lua_bindings/Cargo.toml`

Currently only `tokio` + `anyhow`. Will also need:
- `mlua` (same feature set as `surge-runtime`: `anyhow`, `luajit`,
  `vendored`, `send`, `async`, plus `serde` for `LuaSerdeExt`)
- `reqwest` (`rustls-tls`, `json` features; no default-tls/openssl)
- `serde_json` (already in the lockfile transitively)
