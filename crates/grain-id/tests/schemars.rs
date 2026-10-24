#![cfg(all(feature = "schemars", feature = "serde"))]

use grain_id::{GrainId, GrainIdPrefix};
use schemars::{JsonSchema, SchemaGenerator, generate::SchemaSettings};

fn validate_jsonschema(id: GrainId) {
    let schema = serde_json::Value::from(GrainId::json_schema(&mut SchemaGenerator::new(
        SchemaSettings::openapi3(),
    )));
    let instance = serde_json::to_value(id).unwrap();

    jsonschema::validate(&schema, &instance).unwrap();
}

#[test]
fn validate_nil() {
    validate_jsonschema(GrainId::NIL);
}

#[test]
fn validate_max() {
    validate_jsonschema(GrainId::MAX);
}

#[test]
fn validate_random() {
    for _ in 0..16 {
        validate_jsonschema(GrainId::random());
    }
}

fn validate_prefix_jsonschema(prefix: GrainIdPrefix) {
    let schema = serde_json::Value::from(GrainIdPrefix::json_schema(&mut SchemaGenerator::new(
        SchemaSettings::openapi3(),
    )));
    let instance = serde_json::to_value(prefix).unwrap();
    jsonschema::validate(&schema, &instance).unwrap();
}

#[test]
fn validate_prefix_empty() {
    validate_prefix_jsonschema("".parse().unwrap());
}

#[test]
fn validate_prefix_partial() {
    validate_prefix_jsonschema("a1".parse().unwrap());
}

#[test]
fn validate_prefix_full() {
    validate_prefix_jsonschema("zzzzzzz".parse().unwrap());
}
