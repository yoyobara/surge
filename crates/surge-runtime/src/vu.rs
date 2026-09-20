use std::sync::Arc;

use mlua::Lua;

use crate::lua::{setup_lua_vm, LuaModuleLoader, ModuleChunk};

pub struct Vu<L: LuaModuleLoader> {
    lua: Lua,
    loader: Arc<L>,
}

impl<L: LuaModuleLoader> Vu<L> {
    pub fn new(loader: Arc<L>) -> anyhow::Result<Self> {
        let lua = setup_lua_vm(Arc::clone(&loader))?;

        Ok(Self { lua, loader })
    }

    pub async fn initialize(&self) -> anyhow::Result<()> {
        let entrypoint = self.loader.entrypoint();
        let func = match entrypoint {
            ModuleChunk::Source(src) => {
                self.lua
                    .load(src.as_ref())
                    .set_name("entrypoint")
                    .into_function()?
            }
            ModuleChunk::Bytecode(bc) => {
                self.lua
                    .load(&*bc)
                    .set_name("entrypoint")
                    .into_function()?
            }
        };

        let _test: mlua::Value = func.call(())?;
        Ok(())
    }

    pub async fn _mainloop(&self) -> anyhow::Result<()> {
        println!("mainlooping");
        Ok(())
    }
}
