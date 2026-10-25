use schemars::{JsonSchema, json_schema};

use crate::GrainId;
use crate::GrainIdPrefix;

impl JsonSchema for GrainId {
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        "GrainId".into()
    }
    fn schema_id() -> alloc::borrow::Cow<'static, str> {
        "grain_id::GrainId".into()
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "GrainId",
            "description": "Base32 encoded 7 digit ID",
            "type": "string",
            "minLength" : 7,
            "maxLength" : 7,
            "pattern": "^[a-zA-Z0-9]{7}$",
            "example": "0a1b2c3"
        })
    }
    fn inline_schema() -> bool {
        true
    }
}

impl JsonSchema for GrainIdPrefix {
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        "GrainIdPrefix".into()
    }
    fn schema_id() -> alloc::borrow::Cow<'static, str> {
        "grain_id::GrainIdPrefix".into()
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "GrainIdPrefix",
            "description": "Base32 encoded prefix of a GrainId (0-7 characters)",
            "type": "string",
            "minLength": 0,
            "maxLength": 7,
            "pattern": "^[a-zA-Z0-9]{0,7}$",
            "example": "a1"
        })
    }
    fn inline_schema() -> bool {
        true
    }
}
