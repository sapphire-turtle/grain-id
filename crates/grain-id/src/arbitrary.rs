use super::*;
use ::arbitrary::{Arbitrary, Result, Unstructured};
impl<'a> Arbitrary<'a> for GrainId {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Self::from_u64_lossy(u64::arbitrary(u)?))
    }
}
