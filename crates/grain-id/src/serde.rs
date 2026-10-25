use core::fmt;

use ::serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

use crate::{GrainId, GrainIdPrefix, alphabet};

/// Serializes as a 7-character string in human-readable formats and as a [`u64`] otherwise.
impl Serialize for GrainId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(alphabet::as_str(&self.encode()))
        } else {
            serializer.serialize_u64(self.to_u64())
        }
    }
}

impl<'de> Deserialize<'de> for GrainId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct GrainIdVisitor;

        impl Visitor<'_> for GrainIdVisitor {
            type Value = GrainId;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a 7-character grain-id string or an integer below 2^35")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<GrainId, E> {
                v.parse().map_err(E::custom)
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<GrainId, E> {
                GrainId::from_u64(v).map_err(E::custom)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(GrainIdVisitor)
        } else {
            deserializer.deserialize_u64(GrainIdVisitor)
        }
    }
}

/// Serializes as a string of up to 7 characters in human-readable formats and as a
/// `(value, len)` tuple otherwise.
impl Serialize for GrainIdPrefix {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            let (buf, len) = self.encode();
            serializer.serialize_str(alphabet::as_str(&buf[..len]))
        } else {
            (self.value(), self.len()).serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for GrainIdPrefix {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct PrefixVisitor;

            impl Visitor<'_> for PrefixVisitor {
                type Value = GrainIdPrefix;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("a grain-id prefix string of at most 7 characters")
                }

                fn visit_str<E: de::Error>(self, v: &str) -> Result<GrainIdPrefix, E> {
                    v.parse().map_err(E::custom)
                }
            }

            deserializer.deserialize_str(PrefixVisitor)
        } else {
            let (value, len) = <(u64, u8)>::deserialize(deserializer)?;
            GrainIdPrefix::from_parts(value, len).map_err(de::Error::custom)
        }
    }
}
