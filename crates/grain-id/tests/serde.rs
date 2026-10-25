//! Test for serde feature
#![cfg(feature = "serde")]

use grain_id::{GrainId, GrainIdPrefix};

use serde_test::{Compact, Configure, Readable, Token, assert_ser_tokens, assert_tokens};
use serde_test::{assert_de_tokens, assert_de_tokens_error};

fn assert_tokens_readable(value: &Readable<GrainId>, tokens: &[Token]) {
    assert_ser_tokens(value, tokens);
    assert_de_tokens(value, tokens);
}
fn assert_tokens_compact(value: &Compact<GrainId>, tokens: &[Token]) {
    assert_tokens(value, tokens);
}

#[test]
fn nil_readable() {
    assert_tokens_readable(&GrainId::NIL.readable(), &[Token::Str("0000000")])
}

#[test]
fn max_readable() {
    assert_tokens_readable(&GrainId::MAX.readable(), &[Token::Str("zzzzzzz")]);
}

#[test]
fn nil_compact() {
    assert_tokens_compact(&GrainId::NIL.compact(), &[Token::U64(0)])
}

#[test]
fn max_compact() {
    assert_tokens_compact(&GrainId::MAX.compact(), &[Token::U64(0x7FFFFFFFF)]);
}

#[test]
fn prefix_readable() {
    let prefix: GrainIdPrefix = "a1".parse().unwrap();
    assert_ser_tokens(&prefix.readable(), &[Token::Str("a1")]);
    assert_de_tokens(&prefix.readable(), &[Token::Str("a1")]);
}

#[test]
fn prefix_empty_readable() {
    let prefix: GrainIdPrefix = "".parse().unwrap();
    assert_ser_tokens(&prefix.readable(), &[Token::Str("")]);
    assert_de_tokens(&prefix.readable(), &[Token::Str("")]);
}

#[test]
fn prefix_compact() {
    let prefix: GrainIdPrefix = "a1".parse().unwrap();
    let min_value = u64::from(prefix.min());
    assert_tokens(
        &prefix.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::U64(min_value),
            Token::U8(2),
            Token::TupleEnd,
        ],
    );
}

#[test]
fn readable_accepts_borrowed_and_owned_strings() {
    let id: GrainId = "012abcd".parse().unwrap();
    assert_de_tokens(&id.readable(), &[Token::BorrowedStr("012abcd")]);
    assert_de_tokens(&id.readable(), &[Token::String("012abcd")]);
}

#[test]
fn compact_rejects_out_of_range() {
    assert_de_tokens_error::<Compact<GrainId>>(
        &[Token::U64(1 << 35)],
        "value out of range: 0x800000000",
    );
}

#[test]
fn prefix_compact_rejects_stray_bits() {
    // "a1" with a bit set below the second character.
    let prefix: GrainIdPrefix = "a1".parse().unwrap();
    let value = u64::from(prefix.min()) | 1;
    assert_de_tokens_error::<Compact<GrainIdPrefix>>(
        &[
            Token::Tuple { len: 2 },
            Token::U64(value),
            Token::U8(2),
            Token::TupleEnd,
        ],
        &format!("value out of range: {value:#x}"),
    );
}
