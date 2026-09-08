local shared = require("support.shared_module");

local config = {
    vus = 5,
    duration = '5s'
}

local function run()
    print("TEST")
    shared.shared_method();
end

return {
    run,
    config
}
