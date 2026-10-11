use mlua::{Lua, Table, Value};
use surge_common::parse_duration_value;

pub fn module(lua: &Lua) -> anyhow::Result<Table> {
    let module = lua.create_table()?;

    // `sleep(duration)`
    let sleep = lua.create_async_function(|_, duration: Value| {
        let duration = parse_duration_value(duration);
        async move {
            tokio::time::sleep(duration?).await;
            Ok(())
        }
    })?;
    module.set("sleep", sleep)?;

    Ok(module)
}
