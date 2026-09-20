#![cfg(test)]

use crate::{LuaModuleLoader, start_runtime};

struct MockLoader;

impl LuaModuleLoader for MockLoader {
    fn entrypoint(&self) -> &str {
        "local a = require(\"a\"); print(a); return { loaded = a }"
    }

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
    let ml = MockLoader;
    let res = start_runtime(ml);

    res.unwrap();
}

struct MissingDependencyLoader;

impl LuaModuleLoader for MissingDependencyLoader {
    fn entrypoint(&self) -> &str {
        "local missing = require(\"nonexistent\")"
    }

    fn load(&self, _module_name: &str) -> Option<&str> {
        None
    }
}

#[test]
fn missing_dependency() {
    let ml = MissingDependencyLoader;
    let res = start_runtime(ml);

    assert!(res.is_err());
}
