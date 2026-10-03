use std::time::Duration;

use anyhow::{anyhow, bail};

/// Parses durations like `"500ms"`, `"30s"`, `"1.5m"`, `"1h"`.
/// A bare number is treated as seconds.
pub fn parse_duration(s: &str) -> anyhow::Result<Duration> {
    let s = s.trim();
    let split = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(s.len());
    let (number, unit) = s.split_at(split);

    let value: f64 = number
        .parse()
        .map_err(|_| anyhow!("invalid duration: {s:?}"))?;
    let secs = match unit {
        "ms" => value / 1000.0,
        "s" | "" => value,
        "m" => value * 60.0,
        "h" => value * 3600.0,
        _ => bail!("invalid duration: {s:?}"),
    };

    Ok(Duration::from_secs_f64(secs))
}
