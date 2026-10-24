#![cfg(feature = "digest")]

mod common;
use common::*;
use grain_id::*;

// SHA-256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
// First 5 bytes: [0x2c, 0xf2, 0x4d, 0xba, 0x5f]
// value = 0x2c<<27 | 0xf2<<19 | 0x4d<<11 | 0xba<<3 | 0x5f>>5 = 6032616914
const SHA256_HELLO_EXPECTED: u64 = 6032616914;

#[test]
fn from_digest_known_value() {
    use sha2::Sha256;
    let id = GrainId::from_digest::<Sha256>(b"hello");
    assert_eq!(u64::from(id), SHA256_HELLO_EXPECTED);
}

#[test]
fn from_digest_deterministic() {
    use sha2::Sha256;
    let id1 = GrainId::from_digest::<Sha256>(b"grain-id");
    let id2 = GrainId::from_digest::<Sha256>(b"grain-id");
    assert_eq!(id1, id2);
}

#[test]
fn from_digest_distinct_inputs_differ() {
    use sha2::Sha256;
    let id1 = GrainId::from_digest::<Sha256>(b"hello");
    let id2 = GrainId::from_digest::<Sha256>(b"world");
    assert_ne!(id1, id2);
}

#[test]
fn from_digest_empty_input() {
    use sha2::Sha256;
    let id = GrainId::from_digest::<Sha256>(b"");
    assert!(id <= GrainId::MAX);
    assert_conversion(id);
}

#[test]
fn from_digest_sha1() {
    use sha1::Sha1;
    let id = GrainId::from_digest::<Sha1>(b"hello");
    assert!(id <= GrainId::MAX);
    assert_conversion(id);
}

#[test]
fn from_digest_valid_range() {
    use sha2::Sha256;
    let id = GrainId::from_digest::<Sha256>(b"hello");
    assert!(id <= GrainId::MAX);
    assert_conversion(id);
}

#[test]
fn from_xof_deterministic() {
    use shake::Shake128;
    let id1 = GrainId::from_xof::<Shake128>(b"grain-id");
    let id2 = GrainId::from_xof::<Shake128>(b"grain-id");
    assert_eq!(id1, id2);
}

#[test]
fn from_xof_distinct_inputs_differ() {
    use shake::Shake256;
    let id1 = GrainId::from_xof::<Shake256>(b"hello");
    let id2 = GrainId::from_xof::<Shake256>(b"world");
    assert_ne!(id1, id2);
}

#[test]
fn from_xof_valid_range() {
    use shake::Shake128;
    let id = GrainId::from_xof::<Shake128>(b"hello");
    assert!(id <= GrainId::MAX);
    assert_conversion(id);
}

#[test]
fn from_xof_empty_input() {
    use shake::Shake256;
    let id = GrainId::from_xof::<Shake256>(b"");
    assert!(id <= GrainId::MAX);
    assert_conversion(id);
}
