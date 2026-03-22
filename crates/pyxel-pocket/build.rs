use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const POCKETPY_VERSION: &str = "2.1.8";

fn download(url: &str, dest: &str) {
    let status = Command::new("curl")
        .args(["-Lo", dest, url])
        .status()
        .expect("Failed to execute curl");
    assert!(status.success(), "Failed to download {url}");
}

/// Apply a single patch file to the source.
/// Patch format: lines before `@@@ FIND` are comments,
/// lines between `@@@ FIND` and `@@@ REPLACE` are the search pattern,
/// lines after `@@@ REPLACE` are the replacement.
fn apply_patch(src: &mut String, patch_path: &str) {
    let patch = fs::read_to_string(patch_path)
        .unwrap_or_else(|e| panic!("Failed to read patch {patch_path}: {e}"));

    let find_marker = "@@@ FIND\n";
    let replace_marker = "@@@ REPLACE\n";

    let find_start = patch
        .find(find_marker)
        .unwrap_or_else(|| panic!("{patch_path}: missing @@@ FIND"));
    let replace_start = patch
        .find(replace_marker)
        .unwrap_or_else(|| panic!("{patch_path}: missing @@@ REPLACE"));

    let find = &patch[find_start + find_marker.len()..replace_start];
    let replace = &patch[replace_start + replace_marker.len()..];

    let find = find.trim_end_matches('\n');
    let replace = replace.trim_end_matches('\n');

    let new_src = src.replacen(find, replace, 1);
    assert!(
        new_src != *src,
        "{patch_path}: pattern not found in source — patch may be outdated"
    );
    *src = new_src;
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let pocketpy_dir = format!("{out_dir}/pocketpy-{POCKETPY_VERSION}");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let patches_dir = format!("{manifest_dir}/patches");

    // Download amalgamated files
    if !Path::new(&pocketpy_dir).exists() {
        fs::create_dir_all(&pocketpy_dir).unwrap();
        let base_url =
            format!("https://github.com/pocketpy/pocketpy/releases/download/v{POCKETPY_VERSION}");
        download(
            &format!("{base_url}/pocketpy.c"),
            &format!("{pocketpy_dir}/pocketpy.c"),
        );
        download(
            &format!("{base_url}/pocketpy.h"),
            &format!("{pocketpy_dir}/pocketpy.h"),
        );

        // Apply patches in order
        let pocketpy_c = format!("{pocketpy_dir}/pocketpy.c");
        let mut src = fs::read_to_string(&pocketpy_c).unwrap();

        let mut patches: Vec<_> = fs::read_dir(&patches_dir)
            .unwrap_or_else(|e| panic!("Failed to read patches dir: {e}"))
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "patch"))
            .collect();
        patches.sort();

        for patch in &patches {
            let name = patch.file_name().unwrap().to_string_lossy();
            apply_patch(&mut src, &patch.to_string_lossy());
            eprintln!("Applied patch: {name}");
        }

        fs::write(&pocketpy_c, src).unwrap();
    }

    // Rebuild when patches change
    println!("cargo:rerun-if-changed=patches");

    // Build static library
    cc::Build::new()
        .file(format!("{pocketpy_dir}/pocketpy.c"))
        .include(&pocketpy_dir)
        .std("c11")
        .define("NDEBUG", None)
        .warnings(false)
        .compile("pocketpy");

    // Generate Rust FFI bindings
    // PocketPy's C API is target-independent, so when cross-compiling for
    // Emscripten we override bindgen's target to use the host's system headers.
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let mut builder = bindgen::Builder::default()
        .header(format!("{pocketpy_dir}/pocketpy.h"))
        .allowlist_function("py_.*")
        .allowlist_type("py_.*")
        .allowlist_var("py_.*|tp_.*|PY_.*")
        .allowlist_function("KeyError|StopIteration|TypeError")
        .use_core()
        .generate_comments(false)
        .layout_tests(target == host);
    if target != host {
        builder = builder.clang_arg(format!("--target={host}"));
    }
    let bindings = builder
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .unwrap();
}
