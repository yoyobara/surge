use crate::{LuaModuleLoader, lua::ModuleChunk, main};

struct MockLoader;

const ENTRYPOINT_CODE: &str = "local config = {
    vus = 5,
    duration = '5s'
}

local function run()
    print(\"TEST\")
end

return {
    run = run,
    config = config
}";

impl LuaModuleLoader for MockLoader {
    fn entrypoint(&self) -> ModuleChunk<'_> {
        ENTRYPOINT_CODE.into()
    }

    fn load(&self, _module_name: &str) -> Option<ModuleChunk<'_>> {
        None
    }
}

#[tokio::test]
async fn simple() -> anyhow::Result<()> {
    let ml = MockLoader;
    main(ml).await?;

    Ok(())
}
