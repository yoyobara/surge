use mlua::Lua;

pub trait LuaModuleLoader {
    fn load(&self, module_name: &str) -> &str;
}

pub fn setup_lua_vm(loader: impl LuaModuleLoader) -> Lua {
    let lua = Lua::new();

    lua
}
