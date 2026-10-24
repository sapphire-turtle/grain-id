use std::path::PathBuf;
use std::process::Command;

use grain_id::GrainId;

fn assert_encode(id: GrainId) {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_grain-id"));
    let output = Command::new(path)
        .arg("encode")
        .arg(format!("{}", id.as_u64()))
        .output()
        .unwrap()
        .stdout;
    assert_eq!(output, format!("{}\n", &id.to_string()).into_bytes());
}

#[test]
fn nil() {
    assert_encode(GrainId::NIL);
}

#[test]
fn max() {
    assert_encode(GrainId::MAX);
}

#[test]
fn random() {
    assert_encode(GrainId::random());
}
