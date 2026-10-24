use core::{fmt::Display, str::FromStr};

use crate::{
    Error,
    alphabet::{BASE, u5_to_char_lossy},
};

/// Grain id struct
///
/// # Examples
#[cfg_attr(feature = "default", doc = "```rust")]
#[cfg_attr(not(feature = "default"), doc = "```ignore")]
/// # use grain_id::*;
/// # fn main() -> Result<(), Error> {
/// // Generate random value.
/// let random = GrainId::random();
///
/// assert_ne!(random, GrainId::NIL);
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
    /// The size of the integer type in bits.
    ///
    /// This is not equal actually stored size.
    pub const BITS: u32 = 35;

    /// The capacity value of the grain id
    #[deprecated(since = "0.8.1")]
    pub const CAPACITY: u64 = (BASE as u64).pow(7);

    const MAX_VALUE_PLUS_ONE: u64 = (BASE as u64).pow(7);
    const MAX_VALUE: u64 = Self::MAX_VALUE_PLUS_ONE - 1;

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
    /// assert_eq!(GrainId::MAX, GrainId::try_from(GrainId::CAPACITY - 1)?);
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
    pub fn is_nil(&self) -> bool {
        self == &Self::NIL
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
    pub fn is_max(&self) -> bool {
        self == &Self::MAX
    }

    /// "Converts an unsigned integer to [`GrainId`] by truncating bits that exceed the valid range.")]
    ///
    /// This is a lossy conversion that masks the input value to fit within the ID's bit limit.
    /// If you need to detect out-of-range values, use [`TryFrom`] instead.
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
    /// // values exceeding 35 bits are truncated (MSB(s) dropped
    /// let oversized_int = valid_int + GrainId::CAPACITY;
    /// let overflowed_id = GrainId::from_u64_lossy(oversized_int);
    /// assert_ne!(u64::from(overflowed_id), oversized_int);
    /// // Only lower 35 bits retained
    /// assert_eq!(u64::from(overflowed_id), valid_int)
    /// ```
    pub const fn from_u64_lossy(int: u64) -> Self {
        Self::from_u64_unchecked(int & Self::MAX_VALUE)
    }

    /// Attempts to convert a [`u64`]  to [`GrainId`].
    ///
    ///
    /// Return error if the value is larger than [`GrainId::MAX`].
    /// If you don't need to detect out-of-range values, use [`from_uint_lossy`](Self::from_uint_lossy).
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
    pub const fn from_u64(value: u64) -> Result<Self, crate::Error> {
        if value < Self::MAX_VALUE_PLUS_ONE {
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

    fn from_chars(value: [char; 7]) -> Result<Self, Error> {
        Ok(Self::from_u64_unchecked(u5s_to_u35_unchecked(
            chars_to_u5s(value)?,
        )))
    }

    pub(crate) fn to_chars(self) -> [char; 7] {
        [
            u5_to_char_lossy((self.0 >> 30) as u8),
            u5_to_char_lossy((self.0 >> 25) as u8),
            u5_to_char_lossy((self.0 >> 20) as u8),
            u5_to_char_lossy((self.0 >> 15) as u8),
            u5_to_char_lossy((self.0 >> 10) as u8),
            u5_to_char_lossy((self.0 >> 5) as u8),
            u5_to_char_lossy((self.0) as u8),
        ]
    }

    /// Creates a `GrainId` from a [`Duration`](std::time::Duration) based on decisecond precision.
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
    pub fn from_duration(duration: core::time::Duration) -> Self {
        let decisecs = duration.as_millis() / 100;
        Self::from_u64_lossy(decisecs as u64)
    }

    #[deprecated(since = "0.15.0", note = "Use `to_u64().to_be_bytes()` instead.")]
    #[doc = crate::macros::doc_to_bytes!("big endian")]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let bytes = GrainId::from_u64(0x123456789)?.to_be_bytes();
    /// assert_eq!(bytes, [0, 0, 0, 0x01, 0x23, 0x45, 0x67, 0x89]);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    #[deprecated(since = "0.15.0", note = "Use `to_u64().to_le_bytes()` instead.")]
    #[doc = crate::macros::doc_to_bytes!("little endian")]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let bytes = GrainId::from_u64(0x123456789)?.to_le_bytes();
    /// assert_eq!(bytes, [0x89, 0x67, 0x45, 0x23, 0x01, 0, 0, 0]);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    #[deprecated(
        since = "0.15.0",
        note = "Use `GrainId::from_u64(u64::from_be_bytes(bytes))?` instead."
    )]
    #[doc = crate::macros::doc_from_bytes!("big endian", Self::from_be_bytes_lossy)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let value = [0, 0, 0, 0x01, 0x23, 0x45, 0x67, 0x89];
    /// assert_eq!(
    ///     GrainId::from_be_bytes(value)?,
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Oversized value returns error
    /// let oversized_value = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    /// assert!(GrainId::from_be_bytes(oversized_value).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_be_bytes(bytes: [u8; 8]) -> Result<Self, Error> {
        Self::from_u64(u64::from_be_bytes(bytes))
    }

    #[deprecated(
        since = "0.15.0",
        note = "Use `GrainId::from_u64(u64::from_le_bytes(bytes))?` instead."
    )]
    #[doc = crate::macros::doc_from_bytes!("little endian", Self::from_le_bytes_lossy)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let value = [0x89, 0x67, 0x45, 0x23, 0x01, 0, 0, 0];
    /// assert_eq!(
    ///     GrainId::from_le_bytes(value)?,
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Oversized value returns error
    /// let oversized_value = [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];
    /// assert!(GrainId::from_le_bytes(oversized_value).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_le_bytes(bytes: [u8; 8]) -> Result<Self, Error> {
        Self::from_u64(u64::from_le_bytes(bytes))
    }

    #[deprecated(
        since = "0.15.0",
        note = "Use `GrainId::from_u64_lossy(u64::from_be_bytes(bytes))` instead."
    )]
    #[doc = crate::macros::doc_from_bytes_lossy!("big endian")]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    ///
    /// let value = [0, 0, 0, 0x01, 0x23, 0x45, 0x67, 0x89];
    /// assert_eq!(
    ///     GrainId::from_be_bytes_lossy(value),
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Oversized value will be truncated.
    /// let oversized_value = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    /// assert_eq!(
    ///     GrainId::from_be_bytes_lossy(oversized_value),
    ///     GrainId::from_u64(0x789abcdef)?
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_be_bytes_lossy(bytes: [u8; 8]) -> Self {
        Self::from_u64_lossy(u64::from_be_bytes(bytes))
    }
    #[deprecated(
        since = "0.15.0",
        note = "Use `GrainId::from_u64_lossy(u64::from_le_bytes(bytes))` instead."
    )]
    #[doc = crate::macros::doc_from_bytes_lossy!("little endian")]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let value = [0x89, 0x67, 0x45, 0x23, 0x01, 0, 0, 0];
    /// assert_eq!(
    ///     GrainId::from_le_bytes_lossy(value),
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Oversized value will be truncated.
    /// let oversized_value = [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];
    /// assert_eq!(
    ///     GrainId::from_le_bytes_lossy(oversized_value),
    ///     GrainId::from_u64(0x789abcdef)?
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_le_bytes_lossy(bytes: [u8; 8]) -> Self {
        Self::from_u64_lossy(u64::from_le_bytes(bytes))
    }

    #[deprecated(since = "0.15.0", note = "Use `to_byte_suffix()` instead.")]
    #[doc = crate::macros::doc_to_bytes!("big endian", compact)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let bytes = GrainId::from_u64(0x123456789)?.to_be_bytes_compact();
    /// assert_eq!(bytes, [0x01, 0x23, 0x45, 0x67, 0x89]);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_be_bytes_compact(self) -> [u8; 5] {
        let bytes = self.0.to_be_bytes();
        [bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
    }

    #[deprecated(
        since = "0.15.0",
        note = "Use `to_u64().to_le_bytes()` for an 8-byte little-endian representation."
    )]
    #[doc = crate::macros::doc_to_bytes!("little endian", compact)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let bytes = GrainId::from_u64(0x123456789)?.to_le_bytes_compact();
    /// assert_eq!(bytes, [0x89, 0x67, 0x45, 0x23, 0x01]);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_le_bytes_compact(self) -> [u8; 5] {
        let bytes = self.0.to_le_bytes();
        [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]]
    }

    #[deprecated(
        since = "0.15.0",
        note = "Use `from_byte_suffix()` for lossy conversion, or `GrainId::from_u64(u64::from_be_bytes([0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]]))?` for checked conversion."
    )]
    #[doc = crate::macros::doc_from_bytes!("big endian", compact)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let value = [0x01, 0x23, 0x45, 0x67, 0x89];
    /// assert_eq!(
    ///     GrainId::from_be_bytes_compact(value)?,
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Overflowed value returns error.
    /// let oversized_value = [0x12, 0x34, 0x56, 0x78, 0x90];
    /// assert!(GrainId::from_be_bytes_compact(oversized_value).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_be_bytes_compact(bytes: [u8; 5]) -> Result<Self, Error> {
        Self::from_u64(u64::from_be_bytes([
            0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        ]))
    }

    #[deprecated(
        since = "0.15.0",
        note = "Use `GrainId::from_u64(u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0]))?` instead."
    )]
    #[doc = crate::macros::doc_from_bytes!("little endian", compact)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let value = [0x89, 0x67, 0x45, 0x23, 0x01];
    /// assert_eq!(
    ///     GrainId::from_le_bytes_compact(value)?,
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Oversized value returns error.
    /// let oversized_value = [0x90, 0x78, 0x56, 0x34, 0x12];
    /// assert!(GrainId::from_le_bytes_compact(oversized_value).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_le_bytes_compact(bytes: [u8; 5]) -> Result<Self, Error> {
        Self::from_u64(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0,
        ]))
    }

    #[deprecated(since = "0.15.0", note = "Use `from_byte_suffix()` instead.")]
    #[doc = crate::macros::doc_from_bytes_lossy!("big endian", compact)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let value = [0x01, 0x23, 0x45, 0x67, 0x89];
    /// assert_eq!(
    ///     GrainId::from_be_bytes_compact_lossy(value),
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Oversized value will be truncated.
    /// let oversized_value = [0x12, 0x34, 0x56, 0x78, 0x90];
    /// assert_eq!(
    ///     GrainId::from_be_bytes_compact_lossy(oversized_value),
    ///     GrainId::from_u64(0x234567890)?
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_be_bytes_compact_lossy(bytes: [u8; 5]) -> Self {
        Self::from_u64_lossy(u64::from_be_bytes([
            0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        ]))
    }

    #[deprecated(
        since = "0.15.0",
        note = "Use `GrainId::from_u64_lossy(u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0]))` instead."
    )]
    #[doc = crate::macros::doc_from_bytes_lossy!("little endian", compact)]
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// # fn main() -> Result<(), Error> {
    /// let value = [0x89, 0x67, 0x45, 0x23, 0x01];
    /// assert_eq!(
    ///     GrainId::from_le_bytes_compact_lossy(value),
    ///     GrainId::from_u64(0x123456789)?
    /// );
    ///
    /// // Oversized value will be truncated
    /// let oversized_value = [0x90, 0x78, 0x56, 0x34, 0x12];
    /// assert_eq!(
    ///     GrainId::from_le_bytes_compact_lossy(oversized_value),
    ///     GrainId::from_u64(0x234567890)?
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_le_bytes_compact_lossy(bytes: [u8; 5]) -> Self {
        Self::from_u64_lossy(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0,
        ]))
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
        GrainId::from_u64_lossy(self.to_u64().wrapping_sub(rhs.to_u64()))
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
        let value = (bytes[0] as u64) << 27
            | (bytes[1] as u64) << 19
            | (bytes[2] as u64) << 11
            | (bytes[3] as u64) << 3
            | (bytes[4] as u64) >> 5;
        Self::from_u64_lossy(value)
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
        let value = ((bytes[0] & 0x07) as u64) << 32
            | (bytes[1] as u64) << 24
            | (bytes[2] as u64) << 16
            | (bytes[3] as u64) << 8
            | (bytes[4] as u64);
        Self::from_u64_unchecked(value)
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
        let v = self.0;
        [
            (v >> 27) as u8,
            (v >> 19) as u8,
            (v >> 11) as u8,
            (v >> 3) as u8,
            ((v & 0x07) << 5) as u8,
        ]
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
        let v = self.0;
        [
            (v >> 32) as u8,
            (v >> 24) as u8,
            (v >> 16) as u8,
            (v >> 8) as u8,
            v as u8,
        ]
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
        GrainId::from_u64_lossy(self.to_u64() + 1)
    }
}

fn chars_to_u5s(value: [char; 7]) -> Result<[u8; 7], Error> {
    let mut result = [0; 7];
    for i in 0..7 {
        result[i] = crate::alphabet::char_to_u5(value[i]).ok_or(Error::InvalidCharacter {
            character: value[i],
            index: i,
        })?;
    }
    Ok(result)
}

fn u5s_to_u35_unchecked(value: [u8; 7]) -> u64 {
    ((value[0] as u64) << 30)
        | ((value[1] as u64) << 25)
        | ((value[2] as u64) << 20)
        | ((value[3] as u64) << 15)
        | ((value[4] as u64) << 10)
        | ((value[5] as u64) << 5)
        | (value[6] as u64)
}

impl AsRef<u64> for GrainId {
    fn as_ref(&self) -> &u64 {
        self.as_u64()
    }
}

impl TryFrom<u64> for GrainId {
    type Error = Error;
    /// Attempts to convert a [`u64`]  to [`GrainId`].
    ///
    /// # Error
    /// Return error if the value is larger than [`GrainId::MAX`].
    /// If you don't need to detect out-of-range values, use [`from_u64_lossy`](GrainId::from_u64_lossy).
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// assert!(GrainId::try_from(GrainId::CAPACITY - 1).is_ok());
    /// assert!(GrainId::try_from(GrainId::CAPACITY).is_err());
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
        let chars = self.to_chars();
        write!(
            f,
            "{}{}{}{}{}{}{}",
            chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6]
        )
    }
}

impl FromStr for GrainId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len == 7 {
            let mut chars = s.chars();
            Self::from_chars([
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ])
        } else {
            Err(Error::InvalidLength(len))
        }
    }
}
