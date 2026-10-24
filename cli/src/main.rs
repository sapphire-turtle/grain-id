mod command;
mod option;

use clap::{Parser, Subcommand};

use crate::command::{
    DecodeArgs, EncodeArgs, Md5Args, RandomCommandArgs, Sha1Args, Sha256Args, TimestampArgs,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about, infer_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    command: CliSubcommand,
}

impl Cli {
    pub fn run(self) {
        self.command.run()
    }
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    Decode(DecodeArgs),
    Encode(EncodeArgs),
    Md5(Md5Args),
    Sha1(Sha1Args),
    Sha256(Sha256Args),
    Timestamp(TimestampArgs),
    Random(RandomCommandArgs),
}

impl CliSubcommand {
    pub fn run(self) {
        match self {
            CliSubcommand::Decode(args) => args.run(),
            CliSubcommand::Encode(args) => args.run(),
            CliSubcommand::Md5(args) => args.run(),
            CliSubcommand::Sha1(args) => args.run(),
            CliSubcommand::Sha256(args) => args.run(),
            CliSubcommand::Timestamp(args) => args.run(),
            CliSubcommand::Random(args) => args.run(),
        }
    }
}

fn main() {
    let args = Cli::parse();
    args.run();
}
