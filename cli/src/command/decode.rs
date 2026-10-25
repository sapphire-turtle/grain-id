use clap::Args;
use grain_id::GrainId;

/// Decode grain-id string to integer.
#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Grain id string (7 characters, case-insensitive)
    ///
    /// Examples: "0000000" "012abcd" "zzzzzzz"
    grain_id: GrainId,
}

impl DecodeArgs {
    pub fn run(self) {
        println!("{}", u64::from(self.grain_id))
    }
}
