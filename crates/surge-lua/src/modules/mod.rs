mod chunk;

use std::sync::Arc;

use mlua::{Function, Lua, Table, Value};

pub use chunk::ModuleChunk;

pub trait LuaModuleLoader: Send + Sync + 'static {
    fn entrypoint(&self) -> ModuleChunk<'_>;
    fn load(&self, module_name: &str) -> Option<ModuleChunk<'_>>;
}

/// Replaces `package.loaders` with the preload searcher followed by one that
/// resolves modules through `loader`.
pub(crate) fn install_searcher<L: LuaModuleLoader>(
    lua: &Lua,
    loader: Arc<L>,
) -> anyhow::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let old_loaders: Table = package.get("loaders")?;
    let preload: Function = old_loaders.get(1)?;

    let custom_searcher = lua.create_function(move |lua, name: String| {
        let Some(chunk) = loader.load(&name) else {
            let msg = format!("\n\tno module '{}' found in custom loader", name);
            return Ok((Value::String(lua.create_string(&msg)?), Value::Nil));
        };

        let func = chunk.into_function(lua, &name)?;
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
