#![cfg(feature = "rand")]
mod common;
use common::*;
use grain_id::*;

#[test]
fn random() {
    for _ in 0..10 {
        let id: GrainId = rand::random();
        assert!(id <= GrainId::MAX);
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
        use rand::RngExt as _;
        let s: String = (0..7)
            .map(|_| rng.sample(rand::distr::Alphanumeric) as char)
            .collect();
        let id = s.parse::<GrainId>().unwrap();
        assert_conversion(id);
    }
}

#[test]
fn oversized_random_int() {
    for _ in 0..10 {
        let value: u64 = rand::random_range((<GrainId>::MAX.to_u64() + 1)..<u64>::MAX);
        let _ = <GrainId>::try_from(value).unwrap_err();
    }
}
