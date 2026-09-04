#[path = "../build.rs"]
mod build_script;

use std::io::Cursor;
use std::path::Path;

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

#[test]
fn ci_uses_the_same_sdl2_archive_and_sha256() {
    let workflow_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/build.yml");
    let workflow = std::fs::read_to_string(workflow_path).unwrap();

    assert!(
        workflow.contains(&format!("SDL2_VERSION: \"{}\"", build_script::SDL2_VERSION)),
        "CI must use the SDL2 version pinned by build.rs"
    );
    assert!(
        workflow.contains(&format!("SDL2_SHA256: \"{}\"", build_script::SDL2_SHA256)),
        "CI must use the SDL2 SHA-256 pinned by build.rs"
    );
}
