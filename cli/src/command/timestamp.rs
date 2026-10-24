use chrono::{DateTime, Local};
use clap::Args;
use grain_id::GrainId;

use crate::option::{BaseTimeOption, BaseTimeOptionArgs};

#[derive(Args, Debug)]
/// Generate time-based grain-id
pub struct TimestampArgs {
    #[command(flatten)]
    base: BaseTimeOptionArgs,
    timestamp: Option<DateTime<Local>>,
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
