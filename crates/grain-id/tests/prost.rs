#![cfg(feature = "prost")]

use grain_id::{GrainId, GrainIdPrefix, GrainIdPrefixProto, GrainIdProto};

#[test]
fn nil() {
    let nil = GrainIdProto { value: 0 };
    assert_eq!(
        <grain_id::GrainId>::NIL,
        <grain_id::GrainId>::try_from(nil).unwrap()
    );
}

#[test]
fn max() {
    let max = GrainIdProto {
        value: <u64>::from(<grain_id::GrainId>::MAX),
    };
    assert_eq!(
        <grain_id::GrainId>::MAX,
        <grain_id::GrainId>::try_from(max).unwrap()
    );
}

#[test]
#[should_panic]
fn oversized() {
    let oversized = GrainIdProto {
        value: <u64>::from(<grain_id::GrainId>::MAX) + 1,
    };
    let _ = <grain_id::GrainId>::try_from(oversized).unwrap();
}

#[test]
fn prefix_roundtrip() {
    let prefix: GrainIdPrefix = "a1".parse().unwrap();
    let proto = GrainIdPrefixProto::from(prefix.clone());
    let recovered = GrainIdPrefix::try_from(proto).unwrap();
    assert_eq!(prefix.min(), recovered.min());
    assert_eq!(prefix.max(), recovered.max());
    assert_eq!(prefix.len(), recovered.len());
}

#[test]
fn prefix_empty_roundtrip() {
    let prefix: GrainIdPrefix = "".parse().unwrap();
    let proto = GrainIdPrefixProto::from(prefix);
    let recovered = GrainIdPrefix::try_from(proto).unwrap();
    assert_eq!(recovered.min(), GrainId::NIL);
    assert_eq!(recovered.max(), GrainId::MAX);
}

#[test]
fn prefix_from_proto_lossy_clamps_len() {
    let proto = GrainIdPrefixProto { value: 0, len: 100 };
    let prefix = GrainIdPrefix::from_proto_lossy(proto);
    assert_eq!(prefix.len(), 7);
}

#[test]
fn prefix_try_from_invalid_len() {
    let proto = GrainIdPrefixProto { value: 0, len: 8 };
    assert!(GrainIdPrefix::try_from(proto).is_err());
}

#[test]
fn prefix_stray_bits_are_masked() {
    // value with low bits set that belong beyond a len=2 prefix
    let proto = GrainIdPrefixProto {
        value: "a1zzzzz".parse::<GrainId>().unwrap().into(),
        len: 2,
    };
    let prefix = GrainIdPrefix::try_from(proto).unwrap();
    assert_eq!(prefix.min(), "a100000".parse::<GrainId>().unwrap());
    assert_eq!(prefix.max(), "a1zzzzz".parse::<GrainId>().unwrap());
}
