use mlua::Lua;
use std::sync::Arc;

use crate::{
    bindings,
    modules::{self, LuaModuleLoader},
};

pub fn setup_lua_vm<L: LuaModuleLoader>(loader: Arc<L>) -> anyhow::Result<Lua> {
    let lua = Lua::new();
    modules::install_searcher(&lua, loader)?;
    bindings::install(&lua)?;

    Ok(lua)
}
