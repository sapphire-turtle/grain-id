use std::time::SystemTime;

use clap::Args;
use grain_id::GrainId;

use crate::option::{BaseTimeOption, BaseTimeOptionArgs, parse_timestamp};

#[derive(Args, Debug)]
/// Generate time-based grain-id
pub struct TimestampArgs {
    #[command(flatten)]
    base: BaseTimeOptionArgs,
    /// RFC 3339 timestamp (e.g. `2025-11-28T12:34:56+09:00`). Defaults to now.
    #[arg(value_parser = parse_timestamp)]
    timestamp: Option<SystemTime>,
}

impl TimestampArgs {
    pub fn run(self) {
        println!(
            "{}",
            match (BaseTimeOption::from(self.base), self.timestamp) {
                (BaseTimeOption::Unix, None) => GrainId::now_unix(),
                (BaseTimeOption::Unix, Some(x)) => GrainId::from_timestamp_unix(x),
                (BaseTimeOption::Base(x), None) => GrainId::now_since(x),
                (BaseTimeOption::Base(x), Some(y)) => GrainId::from_timestamp_since(y, x),
            }
        );
    }
}
