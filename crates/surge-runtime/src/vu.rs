use mlua::Lua;

use crate::utils::pretty_table;

pub struct Vu {
    lua: Lua,
    code: String,
}

impl Vu {
    pub fn new(code: String) -> Self {
        Self {
            lua: Lua::new(),
            code,
        }
    }

    pub async fn initialize(&self) -> anyhow::Result<()> {
        let test: mlua::Table = self.lua.load(&self.code).eval()?;
        println!("{}", pretty_table(test, 4).unwrap());
        Ok(())
    }

    pub async fn mainloop(&self) -> anyhow::Result<()> {
        println!("mainlooping");
        Ok(())
    }
}
