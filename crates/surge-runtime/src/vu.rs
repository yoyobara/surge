use std::sync::Arc;

use mlua::Lua;
use surge_lua::{LuaModuleLoader, setup_lua_vm};

pub struct Vu {
    _lua: Lua,
    run_function: mlua::Function,
}

impl Vu {
    pub fn new<L: LuaModuleLoader>(loader: Arc<L>) -> anyhow::Result<Self> {
        let lua = setup_lua_vm(Arc::clone(&loader))?;

        let entrypoint_table: mlua::Table = loader
            .entrypoint()
            .into_function(&lua, "entrypoint")?
            .call(())?;

        let run_function: mlua::Function = entrypoint_table.get("run")?;

        Ok(Self {
            _lua: lua,
            run_function,
        })
    }

    pub async fn mainloop(self) -> anyhow::Result<()> {
        loop {
            self.run_function.call_async::<()>(()).await?;
        }
    }
}
