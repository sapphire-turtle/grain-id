use clap::Args;
use grain_id::GrainId;

/// Decode grain-id string to integer.
#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Grain id String
    ///
    /// Examples: "123" "456-789" "abc-def-ghj" "kmn-pqr-stv-wxy"
    grain_id: GrainId,
}

impl DecodeArgs {
    pub fn run(self) {
        println!("{}", u64::from(self.grain_id))
    }
}
