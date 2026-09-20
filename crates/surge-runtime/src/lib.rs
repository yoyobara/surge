mod lua;
mod tests;
mod utils;
mod vu;

pub use lua::LuaModuleLoader;

use crate::{lua::setup_lua_vm, vu::Vu};

async fn main(code: String, loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let vu = Vu::new(code, setup_lua_vm(loader)?);

    vu.initialize().await
}

pub fn start_runtime(code: String, loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let tokio_runtime = tokio::runtime::Runtime::new()?;
    tokio_runtime.block_on(main(code, loader))
}
