mod common;
use common::*;
use grain_id::*;

#[test]
fn nil_conversion() {
    assert_conversion(<GrainId>::NIL);
}

#[test]
fn max_conversion() {
    assert_conversion(<GrainId>::MAX);
}

#[test]
fn boundary_value() {
    let max_value = GrainId::MAX.to_u64();
    let _ = <GrainId>::try_from(max_value).unwrap();
    let _ = <GrainId>::try_from(max_value + 1).unwrap_err();
}

#[test]
fn partial_ord() {
    assert!(<GrainId>::NIL < <GrainId>::MAX);
}

#[test]
fn sort() {
    let mut vec = vec![<GrainId>::MAX, <GrainId>::NIL];
    vec.sort();
    assert_eq!(vec[0], <GrainId>::NIL);
    assert_eq!(vec[1], <GrainId>::MAX);
}
