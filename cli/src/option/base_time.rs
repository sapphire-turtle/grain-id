use std::time::SystemTime;

use clap::Args;

/// Base timestamp options
///
/// Specify what timestamp will be used to calculate time-based GrainId.
#[derive(Args, Debug)]
#[command(next_help_heading = "Base time options", about = None, long_about = None)]
#[group(multiple = false, required = true)]
pub struct BaseTimeOptionArgs {
    /// Use UNIX_EPOCH as base time.
    #[arg(short, long)]
    pub unix: bool,

    /// Use the given RFC 3339 timestamp (e.g. `2025-01-01T00:00:00+09:00`) as base time.
    #[arg(short, long, value_parser = parse_timestamp)]
    pub base: Option<SystemTime>,
}

/// Enum parsed from BaseTimeOptionArgs.
pub enum BaseTimeOption {
    Unix,
    Base(SystemTime),
}

impl From<BaseTimeOptionArgs> for BaseTimeOption {
    fn from(value: BaseTimeOptionArgs) -> Self {
        match (value.unix, value.base) {
            (true, None) => Self::Unix,
            (false, Some(x)) => Self::Base(x),
            _ => unreachable!(),
        }
    }
}

/// Parses an RFC 3339 timestamp string into a [`SystemTime`] via [`jiff::Timestamp`].
pub fn parse_timestamp(s: &str) -> Result<SystemTime, jiff::Error> {
    SystemTime::try_from(s.parse::<jiff::Timestamp>()?)
}
