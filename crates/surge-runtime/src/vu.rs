use mlua::Lua;

pub struct Vu {
    lua: Lua,
    code: String,
}

impl Vu {
    pub fn new(code: String, lua_instance: Lua) -> Self {
        Self {
            code,
            lua: lua_instance,
        }
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
