mod config;
mod vu;

use std::sync::Arc;

use clap::Parser;
use config::Config;
use mlua::Result as LuaResult;

use crate::vu::Vu;

#[tokio::main]
async fn main() -> LuaResult<()> {
    let config = Arc::new(Config::parse());

    let my_vu = Vu::new(config.clone());

    my_vu.initialize();
    my_vu.mainloop().await;

    Ok(())
}
