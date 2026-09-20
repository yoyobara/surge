use std::sync::Arc;

use mlua::Lua;

use crate::lua::{setup_lua_vm, LuaModuleLoader};

pub struct Vu {
    lua: Lua,
    code: String,
}

impl Vu {
    pub fn new(loader: Arc<impl LuaModuleLoader>) -> anyhow::Result<Self> {
        let lua = setup_lua_vm(Arc::clone(&loader))?;
        let code = loader.entrypoint().to_string();

        Ok(Self { code, lua })
    }

    pub async fn initialize(&self) -> anyhow::Result<()> {
        let _test: mlua::Value = self.lua.load(&self.code).eval()?;
        Ok(())
    }

    pub async fn _mainloop(&self) -> anyhow::Result<()> {
        println!("mainlooping");
        Ok(())
    }
}
