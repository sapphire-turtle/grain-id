use clap::Args;
use grain_id::GrainId;

/// Encode integer to grain-id string.
#[derive(Args, Debug)]
pub struct EncodeArgs {
    value: u64,
}

impl EncodeArgs {
    pub fn run(self) {
        println!("{}", GrainId::from_u64_lossy(self.value))
    }
}
