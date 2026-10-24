use chrono::{DateTime, Local};
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

    #[arg(short, long)]
    pub base: Option<DateTime<Local>>,
}

/// Enum parsed from BaseTimeOptionArgs.
pub enum BaseTimeOption {
    Unix,
    Base(DateTime<Local>),
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
