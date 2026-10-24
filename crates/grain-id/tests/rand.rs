#![cfg(feature = "rand")]
mod common;
use common::*;
use grain_id::*;

#[test]
fn random() {
    for _ in 0..10 {
        let id: GrainId = rand::random();
        assert!(<GrainId>::NIL < id);
        assert!(<GrainId>::MAX >= id);
    }
}

#[test]
fn random_int() {
    for _ in 0..10 {
        let value: u64 = rand::random_range(1..=<GrainId>::MAX.to_u64());
        let id = <GrainId>::try_from(value).unwrap();
        assert_conversion(id);
    }
}

#[cfg(feature = "std")]
#[test]
fn random_str() {
    let mut rng = rand::rng();
    for _ in 0..10 {
        let mut buf = ['0'; 7];
        for i in 0..6 {
            use rand::RngExt as _;

            let c = rng.sample(rand::distr::Alphanumeric) as char;
            buf[i] = c;
        }
        let s: String = buf.into_iter().collect();
        let id = (&s).parse::<GrainId>().unwrap();
        assert_conversion(id);
    }
}

#[test]
#[cfg(feature = "rand")]
fn oversized_random_int() {
    for _ in 0..10 {
        let value: u64 = rand::random_range((<GrainId>::MAX.to_u64() + 1)..<u64>::MAX);
        let _ = <GrainId>::try_from(value).unwrap_err();
    }
}
