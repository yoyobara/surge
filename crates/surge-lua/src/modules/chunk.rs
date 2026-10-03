use std::{borrow::Cow, sync::Arc};

use mlua::{Function, Lua};

#[derive(Clone, Debug)]
pub enum ModuleChunk<'a> {
    Source(Cow<'a, str>),
    Bytecode(Arc<[u8]>),
}

impl<'a> ModuleChunk<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            ModuleChunk::Source(s) => s.as_bytes(),
            ModuleChunk::Bytecode(b) => b.as_ref(),
        }
    }

    pub fn into_function(self, lua: &Lua, name: &str) -> mlua::Result<Function> {
        match self {
            ModuleChunk::Source(src) => lua.load(src.as_ref()).set_name(name).into_function(),
            ModuleChunk::Bytecode(bc) => lua.load(&*bc).set_name(name).into_function(),
        }
    }
}

impl<'a> From<&'a str> for ModuleChunk<'a> {
    fn from(s: &'a str) -> Self {
        ModuleChunk::Source(Cow::Borrowed(s))
    }
}

impl<'a> From<String> for ModuleChunk<'a> {
    fn from(s: String) -> Self {
        ModuleChunk::Source(Cow::Owned(s))
    }
}

impl<'a> From<Arc<[u8]>> for ModuleChunk<'a> {
    fn from(b: Arc<[u8]>) -> Self {
        ModuleChunk::Bytecode(b)
    }
}

impl<'a> From<Vec<u8>> for ModuleChunk<'a> {
    fn from(b: Vec<u8>) -> Self {
        ModuleChunk::Bytecode(Arc::from(b.into_boxed_slice()))
    }
}

impl<'a> From<&'a [u8]> for ModuleChunk<'a> {
    fn from(b: &'a [u8]) -> Self {
        ModuleChunk::Bytecode(Arc::from(b))
    }
}
