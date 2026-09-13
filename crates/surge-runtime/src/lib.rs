mod utils;
mod vu;

use crate::vu::Vu;
use std::time::Duration;

async fn spawn_vu(code: String) -> anyhow::Result<()> {
    let vu = Vu::new(code);
    vu.initialize().await?;
    vu.mainloop().await?;

    Ok(())
}

async fn spawn_vus(code: String, vus: u32) -> anyhow::Result<()> {
    for _i in 0..vus {
        let cloned_code = code.clone();
        tokio::spawn(spawn_vu(cloned_code)).await??;
    }

    Ok(())
}

pub fn start_runtime(code: String, vus: u32, _duration: Duration) -> anyhow::Result<()> {
    let tokio_runtime = tokio::runtime::Runtime::new()?;
    tokio_runtime.block_on(spawn_vus(code, vus))
}
