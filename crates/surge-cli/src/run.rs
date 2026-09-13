use std::time::Duration;

use surge_runtime::start_runtime;

use crate::config::RunArgs;

pub fn handle_run(_args: &RunArgs) -> anyhow::Result<()> {
    start_runtime(
        "return {run = 5, config = '22'}".to_string(),
        1,
        Duration::from_secs(10),
    )
}
