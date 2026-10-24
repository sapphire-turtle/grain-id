use grain_id::GrainId;
use std::{path::PathBuf, process::Command};

#[test]
fn now_unix() {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_grain-id"));
    let output = String::from_utf8_lossy(
        &Command::new(path)
            .arg("timestamp")
            .arg("--unix")
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .to_owned();
    let _ = output.parse::<GrainId>().unwrap();
}
