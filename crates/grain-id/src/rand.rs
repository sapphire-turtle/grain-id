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
    /// Every value, including [`NIL`](Self::NIL) and [`MAX`](Self::MAX), is equally likely.
    /// Uses [`rand::rng()`](::rand::rng); to use another generator, sample
    /// [`StandardUniform`] with it instead (e.g. `rng.random::<GrainId>()`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// let id = GrainId::random();
    /// assert!(id <= GrainId::MAX);
    /// ```
    #[cfg(feature = "std")]
    pub fn random() -> Self {
        <GrainId>::from_u64_lossy(::rand::random())
    }
}
