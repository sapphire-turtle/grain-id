//! Test for serde feature
#![cfg(feature = "serde")]

use grain_id::{GrainId, GrainIdPrefix};

use serde_test::assert_de_tokens;
use serde_test::{Compact, Configure, Readable, Token, assert_ser_tokens, assert_tokens};

fn assert_tokens_readable<'de>(value: &Readable<GrainId>, tokens: &'de [Token]) {
    assert_ser_tokens(value, tokens);
    #[cfg(feature = "std")]
    assert_de_tokens(value, tokens);
}
fn assert_tokens_compact<'de>(value: &Compact<GrainId>, tokens: &'de [Token]) {
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
    assert_ser_tokens(&prefix.clone().readable(), &[Token::Str("a1")]);
    assert_de_tokens(&prefix.readable(), &[Token::Str("a1")]);
}

#[test]
fn prefix_empty_readable() {
    let prefix: GrainIdPrefix = "".parse().unwrap();
    assert_ser_tokens(&prefix.clone().readable(), &[Token::Str("")]);
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
