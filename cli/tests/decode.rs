use std::path::PathBuf;
use std::process::Command;

use grain_id::GrainId;

fn assert_decode(id: GrainId) {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_grain-id"));
    let output = Command::new(path)
        .arg("decode")
        .arg(&id.to_string())
        .output()
        .unwrap()
        .stdout;
    assert_eq!(output, format!("{}\n", id.as_u64()).into_bytes());
}

#[test]
fn nil() {
    assert_decode(GrainId::NIL);
}

#[test]
fn max() {
    assert_decode(GrainId::MAX);
}

#[test]
fn random() {
    assert_decode(GrainId::random());
}
