mod chunk;
mod loader;

use mlua::{Function, Lua, Table, Value};
use std::sync::Arc;

pub use chunk::ModuleChunk;
pub use loader::LuaModuleLoader;

fn modify_loaders<L: LuaModuleLoader>(lua: &Lua, loader: Arc<L>) -> anyhow::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let old_loaders: Table = package.get("loaders")?;
    let preload: Function = old_loaders.get(1)?;

    let searcher_loader = Arc::clone(&loader);
    let custom_searcher = lua.create_function(move |lua, name: String| {
        let chunk = match searcher_loader.load(&name) {
            Some(chunk) => chunk,
            None => {
                let msg = format!("\n\tno module '{}' found in custom loader", name);
                return Ok((Value::String(lua.create_string(&msg)?), Value::Nil));
            }
        };

        let func = match chunk {
            ModuleChunk::Source(source) => {
                lua.load(source.as_ref()).set_name(&name).into_function()?
            }
            ModuleChunk::Bytecode(bytecode) => {
                lua.load(&*bytecode).set_name(&name).into_function()?
            }
        };

        Ok((
            Value::Function(func),
            Value::String(lua.create_string(&name)?),
        ))
    })?;

    let loaders = lua.create_table()?;
    loaders.set(1, preload)?;
    loaders.set(2, custom_searcher)?;
    package.set("loaders", loaders)?;

    Ok(())
}

pub fn setup_lua_vm<L: LuaModuleLoader>(loader: Arc<L>) -> anyhow::Result<Lua> {
    let lua = Lua::new();
    modify_loaders(&lua, loader)?;

    Ok(lua)
}
