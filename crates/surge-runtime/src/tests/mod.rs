#![cfg(test)]

use crate::{LuaModuleLoader, lua::ModuleChunk, start_runtime};

struct MockLoader;

impl LuaModuleLoader for MockLoader {
    fn entrypoint(&self) -> ModuleChunk<'_> {
        "local a = require(\"a\"); print(a); return { loaded = a }".into()
    }

    fn load(&self, module_name: &str) -> Option<ModuleChunk<'_>> {
        match module_name {
            "a" => Some("return \"A\"".into()),
            "b" => Some("return \"B\"".into()),
            _ => None,
        }
    }
}

#[test]
fn simple() {
    let ml = MockLoader;
    let res = start_runtime(ml);

    res.unwrap();
}
