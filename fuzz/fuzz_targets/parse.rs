//! Parses arbitrary strings as `GrainId` / `GrainIdPrefix`.
//!
//! Parsing must never panic, and anything that parses must survive a
//! format -> parse round trip unchanged.
#![no_main]

use grain_id::{GrainId, GrainIdPrefix};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    if let Ok(id) = s.parse::<GrainId>() {
        let formatted = id.to_string();
        assert_eq!(formatted.len(), 7);
        assert_eq!(formatted.parse::<GrainId>(), Ok(id));
        // Decoding is case-insensitive and resolves aliases, so the
        // canonical form only differs from the input by those.
        assert_eq!(s.chars().count(), 7);
    }

    if let Ok(prefix) = s.parse::<GrainIdPrefix>() {
        let formatted = prefix.to_string();
        assert_eq!(formatted.len(), prefix.len() as usize);
        assert_eq!(formatted.parse::<GrainIdPrefix>(), Ok(prefix));
        assert!(prefix.min() <= prefix.max());
        assert!(prefix.contains(prefix.min()));
        assert!(prefix.contains(prefix.max()));
        assert!(prefix.min().to_string().starts_with(&formatted));
        assert!(prefix.max().to_string().starts_with(&formatted));
    }
});
