use crate::{Error, GrainId, GrainIdProto};
#[cfg(feature = "std")]
use crate::{GrainIdPrefix, GrainIdPrefixProto};

impl From<GrainId> for GrainIdProto {
    fn from(value: GrainId) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl TryFrom<GrainIdProto> for GrainId {
    type Error = Error;
    fn try_from(value: GrainIdProto) -> Result<Self, Self::Error> {
        Self::try_from(value.value)
    }
}

impl GrainId {
    /// "Converts a [`crate::proto::GrainId`] to [`GrainId`] by truncating bits that exceed the valid range.")]
    ///
    /// This is a lossy conversion that masks the input value to fit within the ID's bit limit.
    /// If you need to detect out-of-range values, use [`TryFrom`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grain_id::*;
    /// // Values within range are preserved
    /// let valid_proto = GrainIdProto { value: 123 };
    /// let id = GrainId::from_proto_lossy(valid_proto);
    /// assert_eq!(<GrainIdProto>::from(id), valid_proto);
    ///
    /// // values exceeding 35 bits are truncated (MSB(s) dropped
    ///
    /// let oversized_proto = GrainIdProto {
    ///     value: valid_proto.value + GrainId::CAPACITY,
    /// };
    /// let overflowed_id = GrainId::from_proto_lossy(oversized_proto);
    /// assert_ne!(GrainIdProto::from(overflowed_id), oversized_proto);
    /// // Only lower 35 bits retained
    /// assert_eq!(GrainIdProto::from(overflowed_id), valid_proto);
    /// ```
    pub fn from_proto_lossy(value: GrainIdProto) -> Self {
        Self::from_u64_lossy(value.value)
    }
}

#[cfg(feature = "std")]
impl From<GrainIdPrefix> for GrainIdPrefixProto {
    fn from(prefix: GrainIdPrefix) -> Self {
        Self {
            value: prefix.value(),
            len: prefix.len() as u32,
        }
    }
}

#[cfg(feature = "std")]
impl TryFrom<GrainIdPrefixProto> for GrainIdPrefix {
    type Error = Error;

    fn try_from(proto: GrainIdPrefixProto) -> Result<Self, Self::Error> {
        if proto.len > 7 {
            return Err(Error::InvalidLength(proto.len as usize));
        }
        let len = proto.len as u8;
        // Mask off any stray bits below the prefix range.
        let value = if len == 0 {
            0
        } else {
            let shift = 5 * (7 - len as u32);
            (proto.value >> shift) << shift
        };
        Ok(GrainIdPrefix::new(value, len))
    }
}

#[cfg(feature = "std")]
impl GrainIdPrefix {
    /// Converts a [`GrainIdPrefixProto`] to [`GrainIdPrefix`], clamping `len` to 7
    /// and masking any stray low bits in `value`.
    pub fn from_proto_lossy(proto: GrainIdPrefixProto) -> Self {
        let len = (proto.len as u8).min(7);
        let value = if len == 0 {
            0
        } else {
            let shift = 5 * (7 - len as u32);
            (proto.value >> shift) << shift
        };
        Self::new(value, len)
    }
}
