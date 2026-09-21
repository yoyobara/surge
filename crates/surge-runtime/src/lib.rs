mod lua;
mod tests;
mod utils;
mod vu;

use std::{sync::Arc, time::Duration};

use tokio::time::sleep;

use crate::{lua::LuaModuleLoader, vu::Vu};

async fn main(loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let loader = Arc::new(loader);
    let vu = Vu::new(loader)?;

    let jh = tokio::spawn(vu.mainloop());
    sleep(Duration::from_secs(10)).await;
    jh.abort();

    Ok(())
}

pub fn start_runtime(loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let tokio_runtime = tokio::runtime::Runtime::new()?;
    tokio_runtime.block_on(main(loader))
}
