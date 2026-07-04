use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let pocketpy_dir = Path::new(&manifest_dir).join("vendor/pocketpy");
    let pocketpy_c = pocketpy_dir.join("pocketpy.c");
    let pocketpy_h = pocketpy_dir.join("pocketpy.h");

    println!("cargo:rerun-if-changed={}", pocketpy_c.display());
    println!("cargo:rerun-if-changed={}", pocketpy_h.display());
    println!(
        "cargo:rerun-if-changed={}",
        pocketpy_dir.join("VERSION").display()
    );

    cc::Build::new()
        .file(&pocketpy_c)
        .include(&pocketpy_dir)
        .std("c11")
        .define("NDEBUG", None)
        .warnings(false)
        .compile("pocketpy");

    let mut builder = bindgen::Builder::default()
        .header(pocketpy_h.to_string_lossy().into_owned())
        .allowlist_function("py_.*")
        .allowlist_type("py_.*")
        .allowlist_var("py_.*|tp_.*|PY_.*")
        .use_core()
        .generate_comments(false)
        .layout_tests(false);

    // PocketPy leaves PK_API empty on Emscripten, so parse the header as the host
    // target while compiling the C implementation for the real Cargo target.
    if env::var("TARGET").unwrap().contains("emscripten") {
        builder = builder.clang_arg(format!("--target={}", env::var("HOST").unwrap()));
    }

    let bindings = builder
        .generate()
        .expect("failed to generate PocketPy bindings");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("failed to write PocketPy bindings");
}
