use core::fmt::Display;
use core::str::FromStr;
use std::sync::OnceLock;

use crate::{
    Error, GrainId,
    alphabet::{char_to_u5, u5_to_char_lossy},
};

/// A prefix for filtering or querying [`GrainId`] values by their string representation.
///
/// When grain-ids are stored as integers (e.g. in SQLite or protobuf), prefix-based
/// filtering is not straightforward. `GrainIdPrefix` solves this by computing the
/// inclusive numeric range [`min`](Self::min)..=[`max`](Self::max) that corresponds
/// to all IDs starting with the given prefix.
///
/// # Examples
///
/// ```rust
/// # use grain_id::{GrainId, GrainIdPrefix, Error};
/// # fn main() -> Result<(), Error> {
/// let prefix: GrainIdPrefix = "a1".parse()?;
///
/// assert_eq!(prefix.min(), "a100000".parse::<GrainId>()?);
/// assert_eq!(prefix.max(), "a1zzzzz".parse::<GrainId>()?);
/// assert!(prefix.contains("a1b2c3d".parse()?));
/// assert!(!prefix.contains("a200000".parse()?));
/// # Ok(())
/// # }
/// ```
///
/// # rusqlite usage
///
/// ```rust,ignore
/// let prefix: GrainIdPrefix = "a1".parse()?;
/// // WHERE id BETWEEN ?1 AND ?2
/// stmt.execute(params![
///     prefix.min(),
///     prefix.max(),
/// ])?;
/// ```
#[derive(Debug)]
pub struct GrainIdPrefix {
    /// The prefix value stored in the high bits.
    ///
    /// For prefix `"a1"` (`len=2`), bits 34-30 hold `'a'=10` and bits 29-25
    /// hold `'1'=1`, with remaining lower bits set to zero.
    value: u64,
    /// Number of prefix characters (0-7).
    len: u8,
    min: OnceLock<GrainId>,
    max: OnceLock<GrainId>,
}

impl GrainIdPrefix {
    pub(crate) fn new(value: u64, len: u8) -> Self {
        Self {
            value,
            len,
            min: OnceLock::new(),
            max: OnceLock::new(),
        }
    }

    pub(crate) fn value(&self) -> u64 {
        self.value
    }

    /// Returns the number of characters in this prefix.
    pub fn len(&self) -> u8 {
        self.len
    }

    /// Returns `true` if this prefix has zero characters (matches all IDs).
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the smallest [`GrainId`] that matches this prefix.
    pub fn min(&self) -> GrainId {
        *self.min.get_or_init(|| GrainId::from_u64_lossy(self.value))
    }

    /// Returns the largest [`GrainId`] that matches this prefix.
    pub fn max(&self) -> GrainId {
        *self.max.get_or_init(|| {
            let shift = 5 * (7 - self.len as u32);
            let mask = if shift == 0 { 0 } else { (1u64 << shift) - 1 };
            GrainId::from_u64_lossy(self.value | mask)
        })
    }

    /// Returns `true` if `id` starts with this prefix.
    pub fn contains(&self, id: GrainId) -> bool {
        self.min() <= id && id <= self.max()
    }

    /// Returns the inclusive range `(min, max)` of matching [`GrainId`] values.
    pub fn range(&self) -> (GrainId, GrainId) {
        (self.min(), self.max())
    }
}

impl Clone for GrainIdPrefix {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            len: self.len,
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl PartialEq for GrainIdPrefix {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.len == other.len
    }
}

impl Eq for GrainIdPrefix {}

impl core::hash::Hash for GrainIdPrefix {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.len.hash(state);
    }
}

impl FromStr for GrainIdPrefix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len > 7 {
            return Err(Error::InvalidLength(len));
        }
        let mut value: u64 = 0;
        for (index, c) in s.chars().enumerate() {
            let u5 = char_to_u5(c).ok_or(Error::InvalidCharacter {
                character: c,
                index,
            })?;
            let shift = 5 * (6 - index as u32);
            value |= (u5 as u64) << shift;
        }
        Ok(Self {
            value,
            len: len as u8,
            min: OnceLock::new(),
            max: OnceLock::new(),
        })
    }
}

impl Display for GrainIdPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..self.len as u32 {
            let shift = 5 * (6 - i);
            let u5 = (self.value >> shift) as u8;
            write!(f, "{}", u5_to_char_lossy(u5))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_a1() {
        let prefix: GrainIdPrefix = "a1".parse().unwrap();
        assert_eq!(prefix.min(), "a100000".parse::<GrainId>().unwrap());
        assert_eq!(prefix.max(), "a1zzzzz".parse::<GrainId>().unwrap());
    }

    #[test]
    fn test_prefix_empty() {
        let prefix: GrainIdPrefix = "".parse().unwrap();
        assert_eq!(prefix.min(), GrainId::NIL);
        assert_eq!(prefix.max(), GrainId::MAX);
        assert!(prefix.is_empty());
    }

    #[test]
    fn test_prefix_full() {
        let prefix: GrainIdPrefix = "zzzzzzz".parse().unwrap();
        assert_eq!(prefix.min(), GrainId::MAX);
        assert_eq!(prefix.max(), GrainId::MAX);
    }

    #[test]
    fn test_contains() {
        let prefix: GrainIdPrefix = "a1".parse().unwrap();
        assert!(prefix.contains("a100000".parse().unwrap()));
        assert!(prefix.contains("a1b2c3d".parse().unwrap()));
        assert!(prefix.contains("a1zzzzz".parse().unwrap()));
        assert!(!prefix.contains("a0zzzzz".parse().unwrap()));
        assert!(!prefix.contains("a200000".parse().unwrap()));
    }

    #[test]
    fn test_display() {
        let prefix: GrainIdPrefix = "a1b".parse().unwrap();
        assert_eq!(prefix.to_string(), "a1b");
    }

    #[test]
    fn test_display_empty() {
        let prefix: GrainIdPrefix = "".parse().unwrap();
        assert_eq!(prefix.to_string(), "");
    }

    #[test]
    fn test_len() {
        let prefix: GrainIdPrefix = "a1".parse().unwrap();
        assert_eq!(prefix.len(), 2);
    }

    #[test]
    fn test_range() {
        let prefix: GrainIdPrefix = "a1".parse().unwrap();
        let (min, max) = prefix.range();
        assert_eq!(min, prefix.min());
        assert_eq!(max, prefix.max());
    }

    #[test]
    fn test_invalid_length() {
        let result = "a1b2c3de".parse::<GrainIdPrefix>();
        assert!(matches!(result, Err(Error::InvalidLength(8))));
    }

    #[test]
    fn test_invalid_character() {
        let result = "a!".parse::<GrainIdPrefix>();
        assert!(matches!(
            result,
            Err(Error::InvalidCharacter {
                character: '!',
                index: 1
            })
        ));
    }
}
