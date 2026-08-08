use std::sync::Arc;

use mlua::Lua;

use crate::config::Config;

pub struct Vu {
    app_config: Arc<Config>,
    lua: Lua,
}

impl Vu {
    pub fn new(app_config: Arc<Config>) -> Self {
        Self {
            app_config,
            lua: Lua::new(),
        }
    }

    pub fn initialize(&self) {}

    pub async fn run(&self) {}

    pub async fn mainloop(&self) {
        loop {
            self.run().await;
        }
    }
}
