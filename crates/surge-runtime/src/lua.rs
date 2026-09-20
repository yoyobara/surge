use mlua::{Function, Lua, Table, Value};

pub trait LuaModuleLoader: Send + 'static {
    fn load(&self, module_name: &str) -> Option<&str>;
}

fn modify_loaders(lua: &Lua, loader: impl LuaModuleLoader) -> anyhow::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let old_loaders: Table = package.get("loaders")?;
    let preload: Function = old_loaders.get(1)?;

    let custom_searcher = lua.create_function(move |lua, name: String| {
        let module_source = match loader.load(&name) {
            Some(source) => source,
            None => {
                return Ok((Value::String(lua.create_string("yo what")?), Value::Nil));
            }
        };

        let func = lua.load(module_source).set_name(&name).into_function()?;

        Ok((
            Value::Function(func),
            Value::String(lua.create_string(&name)?),
        ))
    })?;

    let loaders = lua.create_table()?;
    loaders.set(1, preload)?;
    loaders.set(2, custom_searcher)?;
    package.set("loaders", loaders)?;

    Ok(())
}

pub fn setup_lua_vm(loader: impl LuaModuleLoader) -> anyhow::Result<Lua> {
    let lua = Lua::new();
    modify_loaders(&lua, loader)?;

    Ok(lua)
}
