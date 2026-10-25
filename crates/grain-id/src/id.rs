use core::{fmt::Display, str::FromStr};

use crate::{Error, alphabet};

/// Grain id struct
///
/// # Examples
#[cfg_attr(all(feature = "rand", feature = "std"), doc = "```rust")]
#[cfg_attr(not(all(feature = "rand", feature = "std")), doc = "```ignore")]
/// # use grain_id::*;
/// # fn main() -> Result<(), Error> {
/// // Generate random value.
/// let random = GrainId::random();
///
/// // Parse from string.
/// let from_str: GrainId = "012abcd".parse()?;
///
/// // Parse from integer.
/// let from_int: GrainId = 35990925u64.try_into()?;
///
/// assert_eq!(from_str, from_int);
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GrainId(u64);

impl GrainId {
    /// The number of significant bits of a [`GrainId`].
    ///
    /// The value is stored in a [`u64`], but only the lower 35 bits are used.
    pub const BITS: u32 = 35;

    /// The number of characters of the string representation.
    pub(crate) const LEN: usize = 7;

    const MAX_VALUE: u64 = (1 << Self::BITS) - 1;

    const fn from_u64_unchecked(value: u64) -> Self {
        Self(value)
    }

    /// The smallest value that can be represented by [`GrainId`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!(GrainId::NIL, "0000000".parse::<GrainId>()?);
    /// assert_eq!(GrainId::NIL, GrainId::try_from(0)?);
    /// # Ok(())
    /// # }
    /// ```
    pub const NIL: Self = Self::from_u64_unchecked(0);

    /// The largest value that can be represented by [`GrainId`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!(GrainId::MAX, "zzzzzzz".parse::<GrainId>()?);
    /// assert_eq!(GrainId::MAX, GrainId::try_from((1 << GrainId::BITS) - 1)?);
    /// # Ok(())
    /// # }
    /// ```
    pub const MAX: Self = Self::from_u64_unchecked(Self::MAX_VALUE);

    /// Test if the [`GrainId`] is nil.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert!("0000000".parse::<GrainId>()?.is_nil());
    /// # Ok(())
    /// # }
    /// ```
    pub const fn is_nil(&self) -> bool {
        self.0 == Self::NIL.0
    }

    /// Test if the [`GrainId`] is max.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert!("zzzzzzz".parse::<GrainId>()?.is_max());
    /// # Ok(())
    /// # }
    /// ```
    pub const fn is_max(&self) -> bool {
        self.0 == Self::MAX.0
    }

    /// Converts an unsigned integer to [`GrainId`] by truncating bits that exceed the valid range.
    ///
    /// This is a lossy conversion that keeps only the lower [`BITS`](Self::BITS) bits
    /// (i.e. the value modulo 2<sup>35</sup>).
    /// If you need to detect out-of-range values, use [`from_u64`](Self::from_u64) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// // Values within range are preserved
    /// let valid_int = 123;
    /// let id = GrainId::from_u64_lossy(valid_int);
    /// assert_eq!(u64::from(id), valid_int);
    ///
    /// // Values exceeding 35 bits are truncated (MSBs dropped)
    /// let oversized_int = valid_int + (1 << GrainId::BITS);
    /// let overflowed_id = GrainId::from_u64_lossy(oversized_int);
    /// assert_eq!(u64::from(overflowed_id), valid_int)
    /// ```
    pub const fn from_u64_lossy(int: u64) -> Self {
        Self::from_u64_unchecked(int & Self::MAX_VALUE)
    }

    /// Attempts to convert a [`u64`] to [`GrainId`].
    ///
    /// Returns an error if the value is larger than [`GrainId::MAX`].
    /// If you don't need to detect out-of-range values, use [`from_u64_lossy`](Self::from_u64_lossy).
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!(GrainId::from_u64(0x7FFFFFFFF)?, GrainId::MAX);
    /// assert!(GrainId::from_u64(0x800000000).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub const fn from_u64(value: u64) -> Result<Self, Error> {
        if value <= Self::MAX_VALUE {
            Ok(Self::from_u64_unchecked(value))
        } else {
            Err(Error::ValueOutOfRange(value))
        }
    }

    /// Returns a reference of internal 64bit integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let id = GrainId::from_u64(0x123456789)?;
    /// assert_eq!(id.as_u64(), &0x123456789);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn as_u64(&self) -> &u64 {
        &self.0
    }

    /// Returns an internal 64bit integer value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let id = GrainId::from_u64(0x123456789)?;
    /// assert_eq!(id.to_u64(), 0x123456789);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_u64(self) -> u64 {
        self.0
    }

    /// Returns the string representation as ASCII bytes.
    pub(crate) const fn encode(self) -> [u8; Self::LEN] {
        alphabet::encode(self.0)
    }

    /// Creates a `GrainId` from a [`Duration`](core::time::Duration) based on decisecond precision.
    ///
    /// This method converts the given duration into deciseconds (0.1 second units)
    /// and generates a `GrainId` from it. The resulting 35-bit identifier can
    /// represent approximately 109 years worth of deciseconds.
    ///
    /// # Timestamp Format
    ///
    /// The entire 35 bits are used to represent the timestamp in deciseconds,
    /// allowing for chronological sorting while maintaining a compact 7-character
    /// BASE32 representation.
    ///
    /// # Overflow Behavior
    ///
    /// If the duration exceeds the maximum representable value (2^35 deciseconds,
    /// approximately 109 years), the timestamp will wrap around. This is generally
    /// not a concern for personal applications within a human lifespan.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// use std::time::Duration;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let duration = Duration::new(5, 730_023_852);
    /// let id = GrainId::from_duration(duration);
    /// assert_eq!(id, GrainId::from_u64(57)?);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn from_duration(duration: core::time::Duration) -> Self {
        let decisecs = duration.as_millis() / 100;
        // 2^35 divides 2^64, so truncating to u64 first does not change the result.
        Self::from_u64_lossy(decisecs as u64)
    }

    /// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the boundary of the type.
    ///
    /// # Examples
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!(
    ///     GrainId::from_u64(100)?.wrapping_sub(GrainId::from_u64(100)?),
    ///     GrainId::NIL
    /// );
    /// assert_eq!(
    ///     GrainId::from_u64(100)?.wrapping_sub(GrainId::from_u64(101)?),
    ///     GrainId::MAX
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        GrainId::from_u64_lossy(self.0.wrapping_sub(rhs.0))
    }

    /// Increments the value by 1, wrapping around to [`GrainId::NIL`] on overflow.
    ///
    /// # Examples
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!(GrainId::from_u64(100)?.increment(), GrainId::from_u64(101)?);
    /// assert_eq!(GrainId::MAX.increment(), GrainId::NIL);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn increment(self) -> Self {
        GrainId::from_u64_lossy(self.0 + 1)
    }

    /// Creates a `GrainId` from the leading (most significant) 35 bits of a 5-byte array.
    ///
    /// This is useful for generating a compact identifier from a pre-computed hash,
    /// public key, or any arbitrary byte sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// // Extract leading 35 bits from a byte array
    /// let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    /// let id = GrainId::from_byte_prefix(&bytes);
    /// assert_eq!(id, GrainId::MAX);
    ///
    /// // For longer data, slice the first 5 bytes
    /// let hash: [u8; 32] = [0; 32];
    /// let id = GrainId::from_byte_prefix(hash[..5].try_into().unwrap());
    /// assert_eq!(id, GrainId::NIL);
    /// ```
    pub const fn from_byte_prefix(bytes: &[u8; 5]) -> Self {
        let [a, b, c, d, e] = *bytes;
        Self::from_u64_unchecked(u64::from_be_bytes([0, 0, 0, a, b, c, d, e]) >> 5)
    }

    /// Creates a `GrainId` from the trailing (least significant) 35 bits of a 5-byte array.
    ///
    /// This is the counterpart to [`from_byte_prefix`](Self::from_byte_prefix).
    /// The upper 5 bits of the first byte are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// // Extract trailing 35 bits from a byte array
    /// let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    /// let id = GrainId::from_byte_suffix(&bytes);
    /// assert_eq!(id, GrainId::MAX);
    ///
    /// // Upper 5 bits of the first byte are ignored
    /// let bytes = [0xF8, 0x00, 0x00, 0x00, 0x00];
    /// let id = GrainId::from_byte_suffix(&bytes);
    /// assert_eq!(id, GrainId::NIL);
    /// ```
    pub const fn from_byte_suffix(bytes: &[u8; 5]) -> Self {
        let [a, b, c, d, e] = *bytes;
        Self::from_u64_lossy(u64::from_be_bytes([0, 0, 0, a, b, c, d, e]))
    }

    /// Returns a 5-byte array with the `GrainId` value stored in the leading (most significant) 35 bits.
    ///
    /// This is the counterpart to [`from_byte_prefix`](Self::from_byte_prefix).
    /// The lower 5 bits of the last byte are always zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let bytes = GrainId::from_u64(0x123456789)?.to_byte_prefix();
    /// assert_eq!(bytes, [0x24, 0x68, 0xAC, 0xF1, 0x20]);
    ///
    /// // Round-trip
    /// let id = GrainId::from_u64(0x123456789)?;
    /// assert_eq!(GrainId::from_byte_prefix(&id.to_byte_prefix()), id);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_byte_prefix(self) -> [u8; 5] {
        let [_, _, _, a, b, c, d, e] = (self.0 << 5).to_be_bytes();
        [a, b, c, d, e]
    }

    /// Returns a 5-byte array with the `GrainId` value stored in the trailing (least significant) 35 bits.
    ///
    /// This is the counterpart to [`from_byte_suffix`](Self::from_byte_suffix).
    /// The upper 5 bits of the first byte are always zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let bytes = GrainId::from_u64(0x123456789)?.to_byte_suffix();
    /// assert_eq!(bytes, [0x01, 0x23, 0x45, 0x67, 0x89]);
    ///
    /// // Round-trip
    /// let id = GrainId::from_u64(0x123456789)?;
    /// assert_eq!(GrainId::from_byte_suffix(&id.to_byte_suffix()), id);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_byte_suffix(self) -> [u8; 5] {
        let [_, _, _, a, b, c, d, e] = self.0.to_be_bytes();
        [a, b, c, d, e]
    }
}

impl AsRef<u64> for GrainId {
    fn as_ref(&self) -> &u64 {
        self.as_u64()
    }
}

impl TryFrom<u64> for GrainId {
    type Error = Error;
    /// Attempts to convert a [`u64`] to [`GrainId`].
    ///
    /// See [`GrainId::from_u64`] for details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// assert!(GrainId::try_from(GrainId::MAX.to_u64()).is_ok());
    /// assert!(GrainId::try_from(GrainId::MAX.to_u64() + 1).is_err());
    /// ```
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::from_u64(value)
    }
}

impl From<GrainId> for u64 {
    fn from(value: GrainId) -> Self {
        value.0
    }
}

impl Display for GrainId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(alphabet::as_str(&self.encode()))
    }
}

impl FromStr for GrainId {
    type Err = Error;

    /// Parses a 7-character grain-id string.
    ///
    /// Decoding is case-insensitive and accepts the aliases described in
    /// [`alphabet`](crate::alphabet).
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!("012ABCD".parse::<GrainId>()?, "o12abcd".parse::<GrainId>()?);
    /// assert_eq!("0123".parse::<GrainId>(), Err(Error::InvalidLength(4)));
    /// // Non-ASCII input is rejected, never panics.
    /// assert!("ab€cd".parse::<GrainId>().is_err());
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len != Self::LEN {
            return Err(Error::InvalidLength(len));
        }
        let (value, _) = alphabet::decode(s)?;
        Ok(Self::from_u64_unchecked(value))
    }
}
