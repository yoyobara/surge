#![cfg(test)]

use crate::{LuaModuleLoader, start_runtime};

struct MockLoader;

impl LuaModuleLoader for MockLoader {
    fn load(&self, module_name: &str) -> Option<&str> {
        match module_name {
            "a" => Some("return \"A\""),
            "b" => Some("return \"B\""),
            _ => None,
        }
    }
}

#[test]
fn simple() {
    let ml = MockLoader {};
    let res = start_runtime("local z = require(\"m\"); print(z);".to_owned(), ml);

    res.unwrap()
}
