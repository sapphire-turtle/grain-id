use clap::Args;
use grain_id::GrainId;

#[derive(Args, Debug)]
/// Generate random grain-id
pub struct RandomCommandArgs;

impl RandomCommandArgs {
    pub fn run(self) {
        println!("{}", rand::random::<GrainId>())
    }
}
