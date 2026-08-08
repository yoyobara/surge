mod config;

use clap::Parser;
use config::Config;
use mlua::{Lua, Result as LuaResult};

fn main() -> LuaResult<()> {
    let _config = Config::parse();
    let _lua = Lua::new();

    Ok(())
}
