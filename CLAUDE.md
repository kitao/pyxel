# Pyxel

Retro game engine for Python — Rust core (SDL2, OpenGL, blip_buf) with PyO3 bindings.

## Structure

```
crates/pyxel-core/       Rust engine
crates/pyxel-binding/    PyO3 Python bindings
python/                  Python package
```

## Build

```
source .venv/bin/activate
make            # build
make lint       # native lint (clippy + ruff)
make lint-wasm  # WASM lint
```

Always use `make` — never run `cargo check` or `cargo clippy` directly.
Run both `make lint` and `make lint-wasm` after any code change and fix all warnings.
See `Makefile` for prerequisites, WASM setup, and all available targets.

## Coding Conventions

**Style:**

- Follow each language's idioms — natural, concise code
- Comments in concise English
- Use blank lines between logical sections, not after every line

**Rust patterns:**

- Raw pointers (`*mut T`) are intentional for the singleton pattern — performance-first
- Prefer `Box::into_raw` over `transmute`
- Prefer `AtomicU32` over `LazyLock<Mutex<T>>` for simple values
- `std::sync::Once` must be fully-qualified in macros for hygiene; import in regular code
- `platform::lock_audio()` / `unlock_audio()` for SDL2 audio sync

## CHANGELOG

Maintained in `CHANGELOG.md` at the repo root.

- `## x.y.z` header with `- ` bullet points
- Concise English, one item per line, start with a verb (Added, Fixed, Removed, etc.)
- Flat list only — no nested sub-items
- Under 60 characters; 80 max for complex entries
- Newest entries first within each section
