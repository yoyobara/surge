mod http;

use mlua::{Lua, Table};

type ModuleBuilder = fn(&Lua) -> anyhow::Result<Table>;

/// Native modules, by the name scripts `require` them with.
const NATIVE_MODULES: &[(&str, ModuleBuilder)] = &[("http", http::module)];

/// Registers every native module into `package.preload` so scripts can
/// `require` them like any other module.
pub fn install(lua: &Lua) -> anyhow::Result<()> {
    for (name, build) in NATIVE_MODULES {
        register_preload(lua, name, build(lua)?)?;
    }

    Ok(())
}

fn register_preload(lua: &Lua, name: &str, module: Table) -> anyhow::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let preload: Table = package.get("preload")?;

    let loader = lua.create_function(move |_, ()| Ok(module.clone()))?;
    preload.set(name, loader)?;

    Ok(())
}
