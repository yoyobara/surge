use std::sync::Arc;

use mlua::Lua;

use crate::{
    lua::{LuaModuleLoader, setup_lua_vm},
    utils::into_lua_function,
};

pub struct Vu {
    _lua: Lua,
    run_function: mlua::Function,
}

impl Vu {
    pub fn new<L: LuaModuleLoader>(loader: Arc<L>) -> anyhow::Result<Self> {
        let lua = setup_lua_vm(Arc::clone(&loader))?;

        let entrypoint = loader.entrypoint();
        let entrypoint_table: mlua::Table =
            into_lua_function(&lua, entrypoint, "entrypoint")?.call(())?;

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
