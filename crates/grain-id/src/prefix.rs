use core::fmt::Display;
use core::ops::RangeInclusive;
use core::str::FromStr;

use crate::{Error, GrainId, alphabet};

/// A prefix for filtering or querying [`GrainId`] values by their string representation.
///
/// When grain-ids are stored as integers (e.g. in a database), prefix-based
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
/// # SQL usage
///
/// ```rust,ignore
/// let prefix: GrainIdPrefix = "a1".parse()?;
/// // WHERE id BETWEEN ?1 AND ?2
/// stmt.execute(params![
///     u64::from(prefix.min()) as i64,
///     u64::from(prefix.max()) as i64,
/// ])?;
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrainIdPrefix {
    /// The prefix characters packed into the high bits of a 35-bit value.
    ///
    /// For prefix `"a1"` (`len=2`), bits 34-30 hold `'a'=10` and bits 29-25
    /// hold `'1'=1`. All lower bits are always zero.
    value: u64,
    /// Number of prefix characters (0-7).
    len: u8,
}

impl GrainIdPrefix {
    /// Builds a prefix from its packed representation, validating that `len` is at
    /// most 7 and that `value` has no bits set outside the first `len` characters.
    pub(crate) const fn from_parts(value: u64, len: u8) -> Result<Self, Error> {
        if len as usize > GrainId::LEN {
            return Err(Error::InvalidLength(len as usize));
        }
        if value & !Self::mask(len) != 0 {
            return Err(Error::ValueOutOfRange(value));
        }
        Ok(Self { value, len })
    }

    /// Returns the prefix consisting of the first `len` characters of `id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grain_id::{GrainId, GrainIdPrefix, Error};
    /// # fn main() -> Result<(), Error> {
    /// let id: GrainId = "a1b2c3d".parse()?;
    /// assert_eq!(GrainIdPrefix::of(id, 3)?, "a1b".parse()?);
    /// assert!(GrainIdPrefix::of(id, 8).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub const fn of(id: GrainId, len: u8) -> Result<Self, Error> {
        if len as usize > GrainId::LEN {
            return Err(Error::InvalidLength(len as usize));
        }
        Ok(Self {
            value: id.to_u64() & Self::mask(len),
            len,
        })
    }

    /// The bits covered by the first `len` characters.
    const fn mask(len: u8) -> u64 {
        let free_bits = 5 * (GrainId::LEN as u32).saturating_sub(len as u32);
        GrainId::MAX.to_u64() >> free_bits << free_bits
    }

    pub(crate) const fn value(&self) -> u64 {
        self.value
    }

    /// Returns the number of characters in this prefix.
    pub const fn len(&self) -> u8 {
        self.len
    }

    /// Returns `true` if this prefix has zero characters (matches all IDs).
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the smallest [`GrainId`] that matches this prefix.
    pub const fn min(&self) -> GrainId {
        GrainId::from_u64_lossy(self.value)
    }

    /// Returns the largest [`GrainId`] that matches this prefix.
    pub const fn max(&self) -> GrainId {
        GrainId::from_u64_lossy(self.value | !Self::mask(self.len))
    }

    /// Returns `true` if `id` starts with this prefix.
    pub const fn contains(&self, id: GrainId) -> bool {
        id.to_u64() & Self::mask(self.len) == self.value
    }

    /// Returns the inclusive range of matching [`GrainId`] values.
    pub const fn range(&self) -> RangeInclusive<GrainId> {
        RangeInclusive::new(self.min(), self.max())
    }

    /// Returns a buffer whose first [`len`](Self::len) bytes are the ASCII
    /// string representation, together with that length.
    pub(crate) const fn encode(&self) -> ([u8; GrainId::LEN], usize) {
        (alphabet::encode(self.value), self.len as usize)
    }
}

impl FromStr for GrainIdPrefix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, len) = alphabet::decode(s)?;
        Ok(Self { value, len })
    }
}

impl Display for GrainIdPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (buf, len) = self.encode();
        f.pad(alphabet::as_str(&buf[..len]))
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
        let range = prefix.range();
        assert_eq!(*range.start(), prefix.min());
        assert_eq!(*range.end(), prefix.max());
    }

    #[test]
    fn test_of() {
        let id: GrainId = "a1b2c3d".parse().unwrap();
        for len in 0..=7 {
            let prefix = GrainIdPrefix::of(id, len).unwrap();
            assert_eq!(prefix.to_string(), &"a1b2c3d"[..len as usize]);
            assert!(prefix.contains(id));
        }
    }

    #[test]
    fn test_from_parts_rejects_stray_bits() {
        let prefix: GrainIdPrefix = "a1".parse().unwrap();
        assert_eq!(GrainIdPrefix::from_parts(prefix.value(), 2), Ok(prefix));
        assert!(GrainIdPrefix::from_parts(prefix.value() | 1, 2).is_err());
        assert!(GrainIdPrefix::from_parts(1 << 35, 7).is_err());
        assert!(GrainIdPrefix::from_parts(0, 8).is_err());
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
