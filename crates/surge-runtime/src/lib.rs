mod lua;
mod tests;
mod utils;
mod vu;

pub use lua::LuaModuleLoader;

use crate::{lua::setup_lua_vm, vu::Vu};

async fn main(loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let code = loader.entrypoint().to_string();
    let vu = Vu::new(code, setup_lua_vm(loader)?);

    vu.initialize().await
}

pub fn start_runtime(loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let tokio_runtime = tokio::runtime::Runtime::new()?;
    tokio_runtime.block_on(main(loader))
}
