# Pocket Runtime Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a native `pyxel-pocket` binary that can reproduce the public Pyxel Python API through PocketPy without changing the existing CPython/PyO3, Python CLI, or Pyodide paths.

**Architecture:** Add a new `crates/pyxel-pocket` workspace member. It vendors PocketPy C source, generates a small Rust FFI layer, registers a native PocketPy `pyxel` module backed by `pyxel-core`, and exposes a `pyxel-pocket app.py` command. The branch stays separate from the existing runtime paths.

**Current acceptance target:** This is not validated by calling a small subset of functions. The meaningful gate is public API parity against `python/pyxel/__init__.pyi`: module constants/variables/functions, public classes, and public class methods must be present on the PocketPy `pyxel` module. The old `pocketpy` branch should be used as the wrapper coverage baseline, while editor modules, pyxapp execution, and Python standard-library compatibility shims remain out of this native API parity slice unless they are required by the Pyxel API itself.

**Tech Stack:** Rust 2021, Cargo workspace, `cc`, `bindgen`, PocketPy C API, existing `pyxel-core`.

---

## File Structure

- Modify `crates/Cargo.toml`
  - Add `pyxel-pocket` to the workspace members.
- Create `crates/pyxel-pocket/Cargo.toml`
  - Define the library and `pyxel-pocket` binary.
  - Depend on `pyxel-core`.
  - Use `cc` and `bindgen` as build dependencies.
- Create `crates/pyxel-pocket/build.rs`
  - Compile vendored `pocketpy.c`.
  - Generate bindings from vendored `pocketpy.h`.
- Create `crates/pyxel-pocket/vendor/pocketpy/VERSION`
  - Record the vendored upstream version.
- Create `crates/pyxel-pocket/vendor/pocketpy/pocketpy.c`
  - Vendored PocketPy source.
- Create `crates/pyxel-pocket/vendor/pocketpy/pocketpy.h`
  - Vendored PocketPy header.
- Create `crates/pyxel-pocket/src/ffi.rs`
  - Include generated bindings.
- Create `crates/pyxel-pocket/src/value.rs`
  - Convert PocketPy stack values to Rust primitives and return Rust values to PocketPy.
- Create `crates/pyxel-pocket/src/runtime.rs`
  - Initialize/finalize PocketPy and execute source strings.
- Create `crates/pyxel-pocket/src/module.rs`
  - Create the native `pyxel` module and register wrapper groups.
- Create `crates/pyxel-pocket/src/wrappers/system.rs`
  - Bind `init`, `run`, `quit`, `flip`, and `show`.
- Create `crates/pyxel-pocket/src/wrappers/graphics.rs`
  - Bind `cls`, `pset`, `line`, `rect`, `rectb`, and `text`.
- Create `crates/pyxel-pocket/src/wrappers/input.rs`
  - Bind `btn`, `btnp`, and `btnr`.
- Create `crates/pyxel-pocket/src/wrappers/variables.rs`
  - Bind MVP constants and synchronize `width`, `height`, `frame_count`, `mouse_x`, and `mouse_y`.
- Create `crates/pyxel-pocket/tests/api_parity.rs`
  - Parse `python/pyxel/__init__.pyi` for the public API surface.
  - Probe the PocketPy `pyxel` module for every expected module path and class method path.
  - Report missing paths by default; fail when `PYXEL_POCKET_REQUIRE_API_PARITY=1` is set.

The MVP wrapper list above is only a bootstrap sequence. It is not the completion target.
- Create `crates/pyxel-pocket/src/wrappers/mod.rs`
  - Re-export wrapper modules.
- Create `crates/pyxel-pocket/src/lib.rs`
  - Public runtime API used by the binary and tests.
- Create `crates/pyxel-pocket/src/main.rs`
  - Implement `pyxel-pocket app.py`.
- Create `crates/pyxel-pocket/tests/runtime_smoke.rs`
  - Smoke-test source execution and command behavior.

## Task 1: Workspace and PocketPy Source

**Files:**
- Modify: `crates/Cargo.toml`
- Create: `crates/pyxel-pocket/Cargo.toml`
- Create: `crates/pyxel-pocket/build.rs`
- Create: `crates/pyxel-pocket/vendor/pocketpy/VERSION`
- Create: `crates/pyxel-pocket/vendor/pocketpy/pocketpy.c`
- Create: `crates/pyxel-pocket/vendor/pocketpy/pocketpy.h`
- Create: `crates/pyxel-pocket/src/ffi.rs`
- Create: `crates/pyxel-pocket/src/lib.rs`

- [ ] **Step 1: Vendor PocketPy release assets**

Fetch the checked-in vendored source from the official PocketPy release assets. Use the current release chosen for this branch, recorded in `VERSION`.

Run:

```bash
mkdir -p crates/pyxel-pocket/vendor/pocketpy
curl -L -o crates/pyxel-pocket/vendor/pocketpy/pocketpy.c \
  https://github.com/pocketpy/pocketpy/releases/download/v2.0.6/pocketpy.c
curl -L -o crates/pyxel-pocket/vendor/pocketpy/pocketpy.h \
  https://github.com/pocketpy/pocketpy/releases/download/v2.0.6/pocketpy.h
printf '2.0.6\n' > crates/pyxel-pocket/vendor/pocketpy/VERSION
```

Expected: the two vendored source files and `VERSION` exist. If either `curl` command fails, stop and verify the release asset names before continuing.

- [ ] **Step 2: Add workspace member**

Update `crates/Cargo.toml`:

```toml
[workspace]
members = ["pyxel-binding", "pyxel-core", "pyxel-pocket"]
resolver = "2"

[workspace.package]
version = "2.9.6"
authors = ["Takashi Kitao <takashi.kitao@gmail.com>"]
edition = "2021"
license = "MIT"

[profile.release]
codegen-units = 1
lto = "thin"
```

- [ ] **Step 3: Add crate manifest**

Create `crates/pyxel-pocket/Cargo.toml`:

```toml
[package]
name = "pyxel-pocket"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[lib]
name = "pyxel_pocket"

[[bin]]
name = "pyxel-pocket"
path = "src/main.rs"

[dependencies]
pyxel-core = { path = "../pyxel-core" }

[build-dependencies]
bindgen = "0.72"
cc = "1.2"

[features]
sdl2_dynamic = ["pyxel-core/sdl2_dynamic"]
sdl2_static = ["pyxel-core/sdl2_static"]
```

- [ ] **Step 4: Add build script**

Create `crates/pyxel-pocket/build.rs`:

```rust
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

    let bindings = bindgen::Builder::default()
        .header(pocketpy_h.to_string_lossy())
        .allowlist_function("py_.*")
        .allowlist_type("py_.*")
        .allowlist_var("py_.*|tp_.*|PY_.*")
        .use_core()
        .generate_comments(false)
        .layout_tests(false)
        .generate()
        .expect("failed to generate PocketPy bindings");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("failed to write PocketPy bindings");
}
```

- [ ] **Step 5: Add minimal FFI and library shell**

Create `crates/pyxel-pocket/src/ffi.rs`:

```rust
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

Create `crates/pyxel-pocket/src/lib.rs`:

```rust
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod ffi;

pub fn version_marker() -> &'static str {
    include_str!("../vendor/pocketpy/VERSION").trim()
}
```

- [ ] **Step 6: Run crate check**

Run:

```bash
cd crates
cargo check -p pyxel-pocket --features sdl2_static
```

Expected: PASS. If bindgen cannot find libclang, stop and report the environment blocker.

- [ ] **Step 7: Commit**

```bash
git add crates/Cargo.toml crates/pyxel-pocket
git commit -m "Add pyxel-pocket crate scaffold"
```

## Task 2: PocketPy Runtime Shell

**Files:**
- Create: `crates/pyxel-pocket/src/runtime.rs`
- Modify: `crates/pyxel-pocket/src/lib.rs`
- Create: `crates/pyxel-pocket/tests/runtime_smoke.rs`

- [ ] **Step 1: Write failing source execution test**

Create `crates/pyxel-pocket/tests/runtime_smoke.rs`:

```rust
#[test]
fn exec_source_accepts_simple_python() {
    pyxel_pocket::Runtime::new()
        .exec_source("x = 1 + 2", "<test>")
        .unwrap();
}
```

- [ ] **Step 2: Run test to verify RED**

Run:

```bash
cd crates
cargo test -p pyxel-pocket exec_source_accepts_simple_python --features sdl2_static
```

Expected: FAIL because `pyxel_pocket::Runtime` does not exist.

- [ ] **Step 3: Add runtime implementation**

Create `crates/pyxel-pocket/src/runtime.rs`:

```rust
use std::ffi::CString;

use crate::ffi;

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        unsafe {
            ffi::py_initialize();
        }
        Self
    }

    pub fn exec_source(&self, source: &str, filename: &str) -> Result<(), String> {
        let source = CString::new(source).map_err(|_| "source contains NUL byte".to_owned())?;
        let filename =
            CString::new(filename).map_err(|_| "filename contains NUL byte".to_owned())?;
        let ok = unsafe {
            ffi::py_exec(
                source.as_ptr(),
                filename.as_ptr(),
                ffi::py_CompileMode_EXEC_MODE,
                std::ptr::null_mut(),
            )
        };
        if ok {
            Ok(())
        } else {
            unsafe {
                ffi::py_printexc();
            }
            Err(format!("PocketPy failed to execute {filename:?}"))
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            ffi::py_finalize();
        }
    }
}
```

Modify `crates/pyxel-pocket/src/lib.rs`:

```rust
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod ffi;
mod runtime;

pub use runtime::Runtime;

pub fn version_marker() -> &'static str {
    include_str!("../vendor/pocketpy/VERSION").trim()
}
```

- [ ] **Step 4: Run test to verify GREEN**

Run:

```bash
cd crates
cargo test -p pyxel-pocket exec_source_accepts_simple_python --features sdl2_static
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/pyxel-pocket/src crates/pyxel-pocket/tests
git commit -m "Add PocketPy runtime shell"
```

## Task 3: Native `pyxel` Module with `init`

**Files:**
- Create: `crates/pyxel-pocket/src/module.rs`
- Create: `crates/pyxel-pocket/src/value.rs`
- Create: `crates/pyxel-pocket/src/wrappers/mod.rs`
- Create: `crates/pyxel-pocket/src/wrappers/system.rs`
- Create: `crates/pyxel-pocket/src/wrappers/variables.rs`
- Modify: `crates/pyxel-pocket/src/runtime.rs`
- Modify: `crates/pyxel-pocket/src/lib.rs`
- Modify: `crates/pyxel-pocket/tests/runtime_smoke.rs`

- [ ] **Step 1: Write failing `import pyxel` constant exposure test**

Append to `crates/pyxel-pocket/tests/runtime_smoke.rs`:

```rust
#[test]
fn pyxel_module_exposes_constants_without_initializing_core() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
import pyxel
assert pyxel.KEY_Q >= 0
assert pyxel.COLOR_WHITE >= 0
",
            "<test>",
        )
        .unwrap();
}
```

- [ ] **Step 2: Run test to verify RED**

Run:

```bash
cd crates
cargo test -p pyxel-pocket pyxel_module_exposes_constants_without_initializing_core --features sdl2_static
```

Expected: FAIL because the native `pyxel` module is not registered.

- [ ] **Step 3: Add value helpers**

Create `crates/pyxel-pocket/src/value.rs`:

```rust
use std::ffi::{CStr, CString};

use crate::ffi;

pub unsafe fn arg(argv: ffi::py_StackRef, index: usize) -> ffi::py_Ref {
    argv.add(index)
}

pub unsafe fn is_none(value: ffi::py_Ref) -> bool {
    ffi::py_istype(value, ffi::py_PredefinedType_tp_NoneType as ffi::py_Type)
}

pub unsafe fn int_arg(argv: ffi::py_StackRef, index: usize) -> i64 {
    ffi::py_toint(arg(argv, index))
}

pub unsafe fn opt_int_arg(argv: ffi::py_StackRef, index: usize) -> Option<i64> {
    let value = arg(argv, index);
    if is_none(value) {
        None
    } else {
        Some(ffi::py_toint(value))
    }
}

pub unsafe fn opt_bool_arg(argv: ffi::py_StackRef, index: usize) -> Option<bool> {
    let value = arg(argv, index);
    if is_none(value) {
        None
    } else {
        Some(ffi::py_tobool(value))
    }
}

pub unsafe fn opt_str_arg(argv: ffi::py_StackRef, index: usize) -> Option<String> {
    let value = arg(argv, index);
    if is_none(value) {
        None
    } else {
        let sv = ffi::py_tosv(value);
        let bytes = std::slice::from_raw_parts(sv.data.cast::<u8>(), sv.size as usize);
        Some(String::from_utf8_lossy(bytes).into_owned())
    }
}

pub unsafe fn return_none() {
    ffi::py_newnone(ffi::py_retval());
}

pub unsafe fn set_module_int(module: ffi::py_GlobalRef, name: &CStr, value: i64) {
    let temp = ffi::py_pushtmp();
    ffi::py_newint(temp, value);
    ffi::py_setattr(module, ffi::py_name(name.as_ptr()), temp);
    ffi::py_pop();
}

pub unsafe fn set_const_int(module: ffi::py_GlobalRef, name: &str, value: i64) {
    let name = CString::new(name).unwrap();
    ffi::py_newint(ffi::py_emplacedict(module, ffi::py_name(name.as_ptr())), value);
}
```

- [ ] **Step 4: Add system and variable wrappers**

Create `crates/pyxel-pocket/src/wrappers/mod.rs`:

```rust
pub mod system;
pub mod variables;
```

Create `crates/pyxel-pocket/src/wrappers/system.rs`:

```rust
use crate::ffi;
use crate::value;

unsafe extern "C" fn pyxel_init(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let width = value::int_arg(argv, 0) as u32;
    let height = value::int_arg(argv, 1) as u32;
    let title = value::opt_str_arg(argv, 2);
    let fps = value::opt_int_arg(argv, 3).map(|v| v as u32);
    let quit_key = value::opt_int_arg(argv, 4).map(|v| v as pyxel::Key);
    let display_scale = value::opt_int_arg(argv, 5).map(|v| v as u32);
    let capture_scale = value::opt_int_arg(argv, 6).map(|v| v as u32);
    let capture_sec = value::opt_int_arg(argv, 7).map(|v| v as u32);
    let headless = value::opt_bool_arg(argv, 8);

    pyxel::init(
        width,
        height,
        title.as_deref(),
        fps,
        quit_key,
        display_scale,
        capture_scale,
        capture_sec,
        headless,
    );
    value::return_none();
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(
        module,
        c"init(width, height, title=None, fps=None, quit_key=None, display_scale=None, capture_scale=None, capture_sec=None, headless=None)".as_ptr(),
        Some(pyxel_init),
    );
}
```

Create `crates/pyxel-pocket/src/wrappers/variables.rs`:

```rust
use crate::ffi;
use crate::value;

pub unsafe fn add_constants(module: ffi::py_GlobalRef) {
    value::set_const_int(module, "KEY_NONE", pyxel::KEY_NONE as i64);
    value::set_const_int(module, "KEY_Q", pyxel::KEY_Q as i64);
    value::set_const_int(module, "COLOR_BLACK", pyxel::COLOR_BLACK as i64);
    value::set_const_int(module, "COLOR_WHITE", pyxel::COLOR_WHITE as i64);
    value::set_const_int(module, "COLOR_RED", pyxel::COLOR_RED as i64);
}

pub unsafe fn sync(module: ffi::py_GlobalRef) {
    value::set_module_int(module, c"width", *pyxel::width() as i64);
    value::set_module_int(module, c"height", *pyxel::height() as i64);
    value::set_module_int(module, c"frame_count", *pyxel::frame_count() as i64);
    value::set_module_int(module, c"mouse_x", *pyxel::mouse_x() as i64);
    value::set_module_int(module, c"mouse_y", *pyxel::mouse_y() as i64);
}
```

- [ ] **Step 5: Register module during runtime initialization**

Create `crates/pyxel-pocket/src/module.rs`:

```rust
use crate::ffi;
use crate::wrappers;

pub fn register() {
    unsafe {
        let module = ffi::py_newmodule(c"pyxel".as_ptr());
        wrappers::variables::add_constants(module);
        wrappers::system::add_functions(module);
        wrappers::variables::sync(module);
    }
}
```

Modify `crates/pyxel-pocket/src/runtime.rs`:

```rust
use std::ffi::CString;

use crate::{ffi, module};

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        unsafe {
            ffi::py_initialize();
        }
        module::register();
        Self
    }

    pub fn exec_source(&self, source: &str, filename: &str) -> Result<(), String> {
        let source = CString::new(source).map_err(|_| "source contains NUL byte".to_owned())?;
        let filename =
            CString::new(filename).map_err(|_| "filename contains NUL byte".to_owned())?;
        let ok = unsafe {
            ffi::py_exec(
                source.as_ptr(),
                filename.as_ptr(),
                ffi::py_CompileMode_EXEC_MODE,
                std::ptr::null_mut(),
            )
        };
        if ok {
            Ok(())
        } else {
            unsafe {
                ffi::py_printexc();
            }
            Err(format!("PocketPy failed to execute {filename:?}"))
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            ffi::py_finalize();
        }
    }
}
```

Modify `crates/pyxel-pocket/src/lib.rs`:

```rust
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

mod ffi;
mod module;
mod runtime;
mod value;
mod wrappers;

pub use runtime::Runtime;

pub fn version_marker() -> &'static str {
    include_str!("../vendor/pocketpy/VERSION").trim()
}
```

- [ ] **Step 6: Run test to verify GREEN**

Run:

```bash
cd crates
cargo test -p pyxel-pocket pyxel_module_exposes_constants_without_initializing_core --features sdl2_static
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add crates/pyxel-pocket
git commit -m "Register minimal PocketPy pyxel module"
```

## Task 4: Binary Entrypoint

**Files:**
- Create: `crates/pyxel-pocket/src/main.rs`
- Modify: `crates/pyxel-pocket/tests/runtime_smoke.rs`

- [ ] **Step 1: Write failing command test**

Append to `crates/pyxel-pocket/tests/runtime_smoke.rs`:

```rust
use std::fs;
use std::process::Command;

#[test]
fn binary_runs_script_file() {
    let script = std::env::temp_dir().join("pyxel-pocket-smoke.py");
    fs::write(
        &script,
        "import pyxel\npyxel.init(8, 8, headless=True)\n",
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .status()
        .unwrap();

    assert!(status.success());
    let _ = fs::remove_file(script);
}
```

- [ ] **Step 2: Run test to verify RED**

Run:

```bash
cd crates
cargo test -p pyxel-pocket binary_runs_script_file --features sdl2_static
```

Expected: FAIL because `src/main.rs` does not exist.

- [ ] **Step 3: Implement binary**

Create `crates/pyxel-pocket/src/main.rs`:

```rust
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn print_usage() {
    eprintln!("Usage: pyxel-pocket SCRIPT.py");
}

fn run_script(path: &Path) -> Result<(), String> {
    let path = fs::canonicalize(path)
        .map_err(|err| format!("cannot resolve '{}': {err}", path.display()))?;
    let source = fs::read_to_string(&path)
        .map_err(|err| format!("cannot read '{}': {err}", path.display()))?;

    if let Some(parent) = path.parent() {
        env::set_current_dir(parent)
            .map_err(|err| format!("cannot enter '{}': {err}", parent.display()))?;
    }

    let runtime = pyxel_pocket::Runtime::new();
    let file_path = path.to_string_lossy().replace('\\', "\\\\").replace('\'', "\\'");
    runtime.exec_source(&format!("__file__ = '{file_path}'"), "<setup>")?;
    runtime.exec_source(
        &source,
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<script>"),
    )
}

fn main() {
    let mut args = env::args_os();
    let _program = args.next();
    let Some(script) = args.next() else {
        print_usage();
        process::exit(1);
    };
    if args.next().is_some() {
        print_usage();
        process::exit(1);
    }

    if let Err(err) = run_script(Path::new(&script)) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
```

- [ ] **Step 4: Run test to verify GREEN**

Run:

```bash
cd crates
cargo test -p pyxel-pocket binary_runs_script_file --features sdl2_static
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/pyxel-pocket
git commit -m "Add pyxel-pocket script runner"
```

## Task 5: MVP Wrappers and Smoke Tests

**Files:**
- Create: `crates/pyxel-pocket/src/wrappers/graphics.rs`
- Create: `crates/pyxel-pocket/src/wrappers/input.rs`
- Modify: `crates/pyxel-pocket/src/wrappers/mod.rs`
- Modify: `crates/pyxel-pocket/src/wrappers/system.rs`
- Modify: `crates/pyxel-pocket/src/module.rs`
- Modify: `crates/pyxel-pocket/src/value.rs`
- Modify: `crates/pyxel-pocket/tests/runtime_smoke.rs`

- [ ] **Step 1: Write failing MVP API script test**

Append to `crates/pyxel-pocket/tests/runtime_smoke.rs`:

```rust
#[test]
fn mvp_api_script_runs_headless() {
    let script_path = std::env::temp_dir().join("pyxel-pocket-mvp.py");
    fs::write(
        &script_path,
        "\
import pyxel
pyxel.init(16, 16, headless=True)
pyxel.cls(pyxel.COLOR_BLACK)
pyxel.pset(1, 2, pyxel.COLOR_WHITE)
pyxel.line(0, 0, 3, 3, pyxel.COLOR_RED)
pyxel.rect(1, 1, 4, 4, pyxel.COLOR_WHITE)
pyxel.rectb(0, 0, 8, 8, pyxel.COLOR_RED)
pyxel.text(0, 0, 'ok', pyxel.COLOR_WHITE)
assert pyxel.width == 16
assert pyxel.height == 16
assert pyxel.btn(pyxel.KEY_Q) == False
assert pyxel.btnp(pyxel.KEY_Q) == False
assert pyxel.btnr(pyxel.KEY_Q) == False
",
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script_path)
        .status()
        .unwrap();

    assert!(status.success());
}
```

- [ ] **Step 2: Run test to verify RED**

Run:

```bash
cd crates
cargo test -p pyxel-pocket mvp_api_script_runs_headless --features sdl2_static
```

Expected: FAIL because the graphics and input wrappers are missing.

- [ ] **Step 3: Add float and bool return helpers**

Extend `crates/pyxel-pocket/src/value.rs`:

```rust
pub unsafe fn float_arg(argv: ffi::py_StackRef, index: usize) -> f32 {
    let value = arg(argv, index);
    let mut out = 0.0;
    if ffi::py_castfloat(value, &mut out) {
        out as f32
    } else {
        ffi::py_tofloat(value) as f32
    }
}

pub unsafe fn return_bool(value: bool) {
    ffi::py_newbool(ffi::py_retval(), value);
}
```

- [ ] **Step 4: Add graphics wrappers**

Create `crates/pyxel-pocket/src/wrappers/graphics.rs`:

```rust
use crate::ffi;
use crate::value;

unsafe extern "C" fn cls(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().clear(value::int_arg(argv, 0) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn pset(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_pixel(
        value::float_arg(argv, 0),
        value::float_arg(argv, 1),
        value::int_arg(argv, 2) as pyxel::Color,
    );
    value::return_none();
    true
}

unsafe extern "C" fn line(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_line(
        value::float_arg(argv, 0),
        value::float_arg(argv, 1),
        value::float_arg(argv, 2),
        value::float_arg(argv, 3),
        value::int_arg(argv, 4) as pyxel::Color,
    );
    value::return_none();
    true
}

unsafe extern "C" fn rect(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_rect(
        value::float_arg(argv, 0),
        value::float_arg(argv, 1),
        value::float_arg(argv, 2),
        value::float_arg(argv, 3),
        value::int_arg(argv, 4) as pyxel::Color,
    );
    value::return_none();
    true
}

unsafe extern "C" fn rectb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_rect_border(
        value::float_arg(argv, 0),
        value::float_arg(argv, 1),
        value::float_arg(argv, 2),
        value::float_arg(argv, 3),
        value::int_arg(argv, 4) as pyxel::Color,
    );
    value::return_none();
    true
}

unsafe extern "C" fn text(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let text = value::opt_str_arg(argv, 2).unwrap_or_default();
    pyxel::pyxel().draw_text(
        value::float_arg(argv, 0),
        value::float_arg(argv, 1),
        &text,
        value::int_arg(argv, 3) as pyxel::Color,
        None,
    );
    value::return_none();
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(module, c"cls(col)".as_ptr(), Some(cls));
    ffi::py_bind(module, c"pset(x, y, col)".as_ptr(), Some(pset));
    ffi::py_bind(module, c"line(x1, y1, x2, y2, col)".as_ptr(), Some(line));
    ffi::py_bind(module, c"rect(x, y, w, h, col)".as_ptr(), Some(rect));
    ffi::py_bind(module, c"rectb(x, y, w, h, col)".as_ptr(), Some(rectb));
    ffi::py_bind(module, c"text(x, y, s, col)".as_ptr(), Some(text));
}
```

- [ ] **Step 5: Add input wrappers**

Create `crates/pyxel-pocket/src/wrappers/input.rs`:

```rust
use crate::ffi;
use crate::value;

unsafe extern "C" fn btn(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_bool(pyxel::pyxel().is_button_down(
        value::int_arg(argv, 0) as pyxel::Key,
    ));
    true
}

unsafe extern "C" fn btnp(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_bool(pyxel::pyxel().is_button_pressed(
        value::int_arg(argv, 0) as pyxel::Key,
        value::opt_int_arg(argv, 1).map(|v| v as u32),
        value::opt_int_arg(argv, 2).map(|v| v as u32),
    ));
    true
}

unsafe extern "C" fn btnr(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_bool(pyxel::pyxel().is_button_released(
        value::int_arg(argv, 0) as pyxel::Key,
    ));
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(module, c"btn(key)".as_ptr(), Some(btn));
    ffi::py_bind(module, c"btnp(key, hold=None, repeat=None)".as_ptr(), Some(btnp));
    ffi::py_bind(module, c"btnr(key)".as_ptr(), Some(btnr));
}
```

- [ ] **Step 6: Register wrappers and sync variables after init**

Modify `crates/pyxel-pocket/src/wrappers/mod.rs`:

```rust
pub mod graphics;
pub mod input;
pub mod system;
pub mod variables;
```

Modify `crates/pyxel-pocket/src/module.rs`:

```rust
use crate::ffi;
use crate::wrappers;

pub fn register() {
    unsafe {
        let module = ffi::py_newmodule(c"pyxel".as_ptr());
        wrappers::variables::add_constants(module);
        wrappers::system::add_functions(module);
        wrappers::graphics::add_functions(module);
        wrappers::input::add_functions(module);
        wrappers::variables::sync(module);
    }
}

pub fn sync_variables() {
    unsafe {
        let module = ffi::py_getmodule(c"pyxel".as_ptr());
        wrappers::variables::sync(module);
    }
}
```

Add this line after `pyxel::init(...)` in `crates/pyxel-pocket/src/wrappers/system.rs`:

```rust
crate::module::sync_variables();
```

- [ ] **Step 7: Run test to verify GREEN**

Run:

```bash
cd crates
cargo test -p pyxel-pocket mvp_api_script_runs_headless --features sdl2_static
```

Expected: PASS.

- [ ] **Step 8: Run focused crate checks**

Run:

```bash
cd crates
cargo test -p pyxel-pocket --features sdl2_static
cargo check -p pyxel-core --features sdl2_static
```

Expected: both commands PASS.

- [ ] **Step 9: Commit**

```bash
git add crates/pyxel-pocket
git commit -m "Add PocketPy MVP Pyxel wrappers"
```

## Self-Review

- Spec coverage:
  - Native-only `pyxel-pocket app.py`: Task 4.
  - No CPython/PyO3 changes: no tasks modify `crates/pyxel-binding` or `python/pyxel`.
  - No Web changes: no tasks modify `wasm` or `web`.
  - Vendored PocketPy source: Task 1.
  - Minimal `pyxel` native module: Task 3.
  - System, graphics, input, variables, constants MVP: Tasks 3 and 5.
  - Smoke tests: Tasks 2, 3, 4, and 5.
- Placeholder scan:
  - No placeholder markers or unspecified implementation steps.
  - Each code-changing step includes concrete file content or exact inserted code.
- Type consistency:
  - The runtime type is consistently `Runtime`.
  - The wrapper modules are consistently `system`, `graphics`, `input`, and `variables`.
  - The binary name is consistently `pyxel-pocket`.
- Test isolation:
  - In-process runtime tests do not call `pyxel.init`.
  - `pyxel.init` smoke tests run through the `pyxel-pocket` binary in a separate process, so native `pyxel-core` global state is initialized once per process.
