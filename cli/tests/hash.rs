use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use grain_id::GrainId;

fn bin() -> PathBuf {
    PathBuf::from(std::env!("CARGO_BIN_EXE_grain-id"))
}

fn run_hash(subcmd: &str, args: &[&str]) -> String {
    let output = Command::new(bin())
        .arg(subcmd)
        .args(args)
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}

fn assert_hash<D: digest::Digest>(subcmd: &str, input: &[u8], arg: &str) {
    let expected = GrainId::from_digest::<D>(input);
    let actual: GrainId = run_hash(subcmd, &[arg]).parse().unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn md5_string() {
    assert_hash::<md5::Md5>("md5", b"hello", "--string=hello");
}

#[test]
fn sha1_string() {
    assert_hash::<sha1::Sha1>("sha1", b"hello", "--string=hello");
}

#[test]
fn sha256_string() {
    assert_hash::<sha2::Sha256>("sha256", b"hello", "--string=hello");
}

#[test]
fn sha256_file() {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    let content = b"grain-id hash test";
    tmp.write_all(content).unwrap();
    tmp.flush().unwrap();

    let expected = GrainId::from_digest::<sha2::Sha256>(content);
    let actual: GrainId = run_hash("sha256", &[tmp.path().to_str().unwrap()])
        .parse()
        .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn sha256_stdin() {
    let content = b"stdin test";
    let expected = GrainId::from_digest::<sha2::Sha256>(content);

    let mut child = Command::new(bin())
        .arg("sha256")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(content).unwrap();
    let output = child.wait_with_output().unwrap().stdout;

    let actual: GrainId = String::from_utf8(output).unwrap().trim().parse().unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn different_algorithms_differ() {
    let md5_id: GrainId = run_hash("md5", &["--string=test"]).parse().unwrap();
    let sha1_id: GrainId = run_hash("sha1", &["--string=test"]).parse().unwrap();
    let sha256_id: GrainId = run_hash("sha256", &["--string=test"]).parse().unwrap();
    assert_ne!(md5_id, sha1_id);
    assert_ne!(sha1_id, sha256_id);
    assert_ne!(md5_id, sha256_id);
}
