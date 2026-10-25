/// A general error that can occur when working with grain-ids.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An integer is larger than [`GrainId::MAX`](crate::GrainId::MAX), or does not fit the
    /// expected bit layout.
    #[error("value out of range: {0:#x}")]
    ValueOutOfRange(u64),
    /// A string has an unexpected number of characters.
    ///
    /// A [`GrainId`](crate::GrainId) must have exactly 7 characters, and a
    /// `GrainIdPrefix` at most 7.
    #[error("invalid length: found {0} characters")]
    InvalidLength(usize),
    /// A string contains a character that is not in the alphabet.
    #[error("invalid character {character:?} at index {index}")]
    InvalidCharacter { character: char, index: usize },
}
