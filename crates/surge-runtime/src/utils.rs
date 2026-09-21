use mlua::Lua;

use crate::lua::ModuleChunk;

pub fn into_lua_function(
    lua: &Lua,
    chunk: ModuleChunk,
    chunk_name: &str,
) -> anyhow::Result<mlua::Function> {
    let func = match chunk {
        ModuleChunk::Source(src) => lua
            .load(src.as_ref())
            .set_name(chunk_name)
            .into_function()?,
        ModuleChunk::Bytecode(bc) => lua.load(&*bc).set_name(chunk_name).into_function()?,
    };

    Ok(func)
}
