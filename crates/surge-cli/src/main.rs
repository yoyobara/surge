use mlua::{Error as LuaError, Lua, Result as LuaResult};

fn factorial(lua_ctx: &Lua, n: usize) -> LuaResult<usize> {
    dbg!(n);
    if n < 1 {
        return Err(LuaError::RuntimeError("AYO".to_string()));
    }

    if n == 1 {
        Ok(1)
    } else {
        Ok(n * factorial(lua_ctx, n - 1)?)
    }
}

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let func = lua.create_function(factorial)?;
    lua.globals().set("factorial", func)?;

    let file_content = std::fs::read_to_string("./example.lua")?;
    lua.load(file_content).exec()?;

    Ok(())
}
