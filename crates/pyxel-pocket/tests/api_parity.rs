use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

fn module_level_name(line: &str, prefix: &str) -> Option<String> {
    let rest = line.strip_prefix(prefix)?;
    let name = rest
        .split(|c: char| c == '(' || c == ':' || c.is_whitespace())
        .next()?;
    if name.is_empty() {
        None
    } else {
        Some(name.to_owned())
    }
}

fn annotated_name(line: &str) -> Option<String> {
    let (name, _) = line.split_once(':')?;
    if name.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
        && name
            .chars()
            .next()
            .is_some_and(|c| c == '_' || c.is_ascii_alphabetic())
    {
        Some(name.to_owned())
    } else {
        None
    }
}

fn expected_api_paths() -> BTreeSet<String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let stub_path = manifest_dir.join("../../python/pyxel/__init__.pyi");
    let stub = fs::read_to_string(&stub_path)
        .unwrap_or_else(|err| panic!("failed to read '{}': {err}", stub_path.display()));

    let mut paths = BTreeSet::new();
    let mut current_class: Option<String> = None;

    for line in stub.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('@') {
            continue;
        }

        if line.starts_with("class ") {
            current_class = module_level_name(line, "class ");
            if let Some(name) = &current_class {
                paths.insert(name.clone());
            }
            continue;
        }

        if line.starts_with("def ") {
            current_class = None;
            if let Some(name) = module_level_name(line, "def ") {
                paths.insert(name);
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("    def ") {
            if let Some(class_name) = &current_class {
                if let Some(name) = module_level_name(rest, "") {
                    paths.insert(format!("{class_name}.{name}"));
                }
            }
            continue;
        }

        if !line.starts_with(char::is_whitespace) {
            if let Some(name) = annotated_name(line) {
                current_class = None;
                paths.insert(name);
            }
        }
    }

    paths
}

fn missing_runtime_paths(expected_paths: &BTreeSet<String>) -> Vec<String> {
    let script =
        std::env::temp_dir().join(format!("pyxel-pocket-api-parity-{}.py", std::process::id()));

    let mut source = String::from("import pyxel\n");
    for path in expected_paths {
        source.push_str("try:\n    pyxel.");
        source.push_str(path);
        source.push_str("\nexcept:\n    print('MISSING:");
        source.push_str(path);
        source.push_str("')\n");
    }
    fs::write(&script, source).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();
    let _ = fs::remove_file(&script);

    assert!(
        output.status.success(),
        "api parity probe failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("MISSING:").map(str::to_owned))
        .collect()
}

#[test]
fn pyxel_public_api_parity_report() {
    let expected_paths = expected_api_paths();
    assert!(
        expected_paths.len() > 400,
        "stub parser found only {} API paths",
        expected_paths.len()
    );
    for required_path in [
        "Font.text_width",
        "Image.blt",
        "Tilemap.pset",
        "Channel.play",
        "Sound.set_notes",
        "Music.save",
    ] {
        assert!(
            expected_paths.contains(required_path),
            "stub parser missed class API path {required_path}"
        );
    }

    let missing_paths = missing_runtime_paths(&expected_paths);
    if missing_paths.is_empty() {
        return;
    }

    let preview = missing_paths.to_vec().join("\n  ");
    let message = format!(
        "PocketPy pyxel API parity is incomplete: {} expected paths, {} missing paths.\n  {}",
        expected_paths.len(),
        missing_paths.len(),
        preview
    );

    if std::env::var_os("PYXEL_POCKET_REQUIRE_API_PARITY").is_some() {
        panic!("{message}");
    }

    eprintln!("{message}");
    eprintln!("Set PYXEL_POCKET_REQUIRE_API_PARITY=1 to make this report a failing gate.");
}
