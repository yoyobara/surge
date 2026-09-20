mod lua;
mod tests;
mod utils;
mod vu;

pub use lua::LuaModuleLoader;

use std::sync::Arc;

use crate::vu::Vu;

async fn main(loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let loader = Arc::new(loader);
    let vu = Vu::new(loader)?;

    vu.initialize().await
}

pub fn start_runtime(loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let tokio_runtime = tokio::runtime::Runtime::new()?;
    tokio_runtime.block_on(main(loader))
}
