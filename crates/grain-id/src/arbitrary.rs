use ::arbitrary::{Arbitrary, Result, Unstructured};

use crate::{GrainId, GrainIdPrefix};

impl<'a> Arbitrary<'a> for GrainId {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Self::from_u64_lossy(u64::arbitrary(u)?))
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        u64::size_hint(depth)
    }
}

impl<'a> Arbitrary<'a> for GrainIdPrefix {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let id = GrainId::arbitrary(u)?;
        let len = u.int_in_range(0..=GrainId::LEN as u8)?;
        Ok(Self::of(id, len).expect("len is at most 7"))
    }
}
