use crate::lua::chunk::ModuleChunk;

pub trait LuaModuleLoader: Send + Sync + 'static {
    fn entrypoint(&self) -> ModuleChunk<'_>;
    fn load(&self, module_name: &str) -> Option<ModuleChunk<'_>>;
}
