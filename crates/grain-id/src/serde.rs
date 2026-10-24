use super::*;
#[cfg(feature = "std")]
use crate::GrainIdPrefix;
use ::serde::{Deserialize, Serialize, de::Error};

impl Serialize for GrainId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        if serializer.is_human_readable() {
            let chars = self.to_chars();
            let mut buf = [0; 7];
            for i in 0..7 {
                buf[i] = u8::try_from(chars[i]).unwrap();
            }
            let s = unsafe { str::from_utf8_unchecked(&buf) };
            serializer.serialize_str(s)
        } else {
            serializer.serialize_u64(self.to_u64())
        }
    }
}

impl<'de> Deserialize<'de> for GrainId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            #[cfg(feature = "std")]
            {
                <String as Deserialize>::deserialize(deserializer)?
                    .parse::<GrainId>()
                    .map_err(D::Error::custom)
            }
            #[cfg(not(feature = "std"))]
            {
                (<&str as Deserialize>::deserialize(deserializer)?)
                    .parse::<GrainId>()
                    .map_err(D::Error::custom)
            }
        } else {
            let i = u64::deserialize(deserializer)?;
            GrainId::from_u64(i).map_err(D::Error::custom)
        }
    }
}

#[cfg(feature = "std")]
impl Serialize for GrainIdPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            use ::serde::ser::SerializeTuple;
            let mut tup = serializer.serialize_tuple(2)?;
            tup.serialize_element(&self.value())?;
            tup.serialize_element(&self.len())?;
            tup.end()
        }
    }
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for GrainIdPrefix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            <String as Deserialize>::deserialize(deserializer)?
                .parse::<GrainIdPrefix>()
                .map_err(D::Error::custom)
        } else {
            let (value, len) = <(u64, u8) as Deserialize>::deserialize(deserializer)?;
            if len > 7 {
                return Err(D::Error::custom(crate::Error::InvalidLength(len as usize)));
            }
            Ok(GrainIdPrefix::new(value, len))
        }
    }
}
