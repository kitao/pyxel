#[path = "../build.rs"]
mod build_script;

use std::io::Cursor;

#[test]
fn accepts_matching_sha256() {
    let data = Cursor::new(b"abc");
    let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    assert_eq!(build_script::verify_sha256(data, expected), Ok(()));
}

#[test]
fn rejects_mismatched_sha256() {
    let data = Cursor::new(b"abc");
    let expected = "0000000000000000000000000000000000000000000000000000000000000000";
    let actual = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    let error = build_script::verify_sha256(data, expected).unwrap_err();
    assert_eq!(
        error,
        format!("SHA-256 mismatch: expected {expected}, actual {actual}")
    );
}
