//! Deserializes arbitrary bytes with a human-readable (JSON) and a binary
//! (postcard) format.
//!
//! Deserialization must never panic, and accepted values must serialize
//! back to something that deserializes to the same value.
#![no_main]

use grain_id::{GrainId, GrainIdPrefix};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(id) = serde_json::from_slice::<GrainId>(data) {
        let json = serde_json::to_vec(&id).unwrap();
        assert_eq!(serde_json::from_slice::<GrainId>(&json).unwrap(), id);
    }
    if let Ok(prefix) = serde_json::from_slice::<GrainIdPrefix>(data) {
        let json = serde_json::to_vec(&prefix).unwrap();
        assert_eq!(
            serde_json::from_slice::<GrainIdPrefix>(&json).unwrap(),
            prefix
        );
    }
    if let Ok(id) = postcard::from_bytes::<GrainId>(data) {
        let bin = postcard::to_allocvec(&id).unwrap();
        assert_eq!(postcard::from_bytes::<GrainId>(&bin).unwrap(), id);
    }
    if let Ok(prefix) = postcard::from_bytes::<GrainIdPrefix>(data) {
        let bin = postcard::to_allocvec(&prefix).unwrap();
        let decoded = postcard::from_bytes::<GrainIdPrefix>(&bin).unwrap();
        assert_eq!(decoded, prefix);
        // The binary form must describe the same prefix as the text form.
        assert_eq!(
            decoded.to_string().parse::<GrainIdPrefix>().unwrap(),
            prefix
        );
    }
});
