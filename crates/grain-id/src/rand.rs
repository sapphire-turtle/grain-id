use super::*;
use ::rand::{
    Rng, RngExt as _,
    distr::{Distribution, StandardUniform},
};

impl Distribution<GrainId> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GrainId {
        <GrainId>::from_u64_lossy(rng.random())
    }
}
impl GrainId {
    /// Generate a new random [`GrainId`].
    ///
    /// This method generate a random ID.
    /// The generated ID is guaranteed to not be the [`NIL`](Self::NIL) value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// let id = GrainId::random();
    /// assert_ne!(id, GrainId::NIL);
    /// ```
    pub fn random() -> Self {
        <GrainId>::from_u64_lossy(::rand::random())
    }
}
