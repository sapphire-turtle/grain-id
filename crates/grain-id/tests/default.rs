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
    let mut vec = [<GrainId>::MAX, <GrainId>::NIL];
    vec.sort();
    assert_eq!(vec[0], <GrainId>::NIL);
    assert_eq!(vec[1], <GrainId>::MAX);
}

#[test]
fn parse_rejects_multibyte_input_without_panicking() {
    // 7 bytes but 5 characters
    assert_eq!("ab€cd".parse::<GrainId>(), Err(Error::InvalidLength(5)));
    // 7 characters but more than 7 bytes
    assert!(matches!(
        "abcdeféh".parse::<GrainId>(),
        Err(Error::InvalidLength(8))
    ));
    assert!(matches!(
        "abcdeé0".parse::<GrainId>(),
        Err(Error::InvalidCharacter {
            character: 'é',
            index: 5
        })
    ));
}

#[test]
fn display_respects_padding() {
    assert_eq!(format!("[{:>9}]", GrainId::NIL), "[  0000000]");
}
