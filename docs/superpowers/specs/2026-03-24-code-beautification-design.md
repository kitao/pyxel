# Code Beautification Design

## Goal

Make all source code in the Pyxel repository structurally clear and performant, following each language's idiomatic style.

## Scope

- All source files: Rust (~17,800 lines), Python (~7,300 lines), JavaScript (~1,000 lines), HTML (53 files)
- Includes in-progress changes on the `develop` branch
- Python public API signatures and behavior are unchanged
- Intentional `unsafe`/raw pointer patterns are preserved
- Auto-generated files and binaries are excluded

## Universal Principles

1. **Structure communicates intent** — Functions and modules are sized and named so their purpose is obvious. Definition order reads naturally: high-level before detail, public before private.
2. **Performance-aware choices** — Avoid unnecessary allocations and copies. Choose optimal expressions on hot paths without sacrificing readability. Comment performance-motivated decisions.
3. **No redundancy** — Use the language's concise idioms. Remove unnecessary intermediate variables, fully-qualified paths, and wrappers.
4. **Consistency** — Same pattern, same expression. No variation without reason. Language conventions take priority over project-wide uniformity.
5. **Comments explain "why"** — Code expresses "what." Section-divider comments are kept when they serve as headings.

## Rust Guidelines

### Structure
- Method order in `impl`: constructors → public methods → private methods
- Split long functions at semantic boundaries; function call overhead is negligible
- `use` statements follow `group_imports = "StdExternalCrate"` (std → external → crate)

### Expression
- Prefer iterator chains where natural; fall back to `for` when chains exceed 3 stages and become hard to read
- Use `if let` / `let else` / `match` by context
- Exhaustive pattern matching to make intent explicit

### Performance
- Eliminate gratuitous `.clone()`, `.to_string()`, `.collect()`
- Prefer `&str` over `String` and `&[T]` over `Vec<T>` where ownership is unnecessary
- Preserve existing singleton-pattern raw pointers and intentional `unsafe`

### Naming
- Do not repeat the type name in a variable when the type context is clear
- Bool-returning functions start with `is_` / `has_` / `can_`

## Python Guidelines

### Structure
- Class body order: `__init__` → public methods → private methods (`_` prefix)
- Top-level constants grouped after imports
- Editor files use section comments to separate UI construction from logic

### Expression
- List comprehensions and generator expressions where natural; no nested comprehensions
- Consistent `f-string` usage
- Truthy/falsy checks for Pythonic conditions; explicit `is None` / `is not None`

### Performance
- Avoid object creation inside loops when unnecessary
- Avoid redundant list copies
- Readability over micro-optimization (Python layer is UI/CLI)

### Naming
- PEP 8: `snake_case` functions, `PascalCase` classes, `UPPER_SNAKE` constants
- Public API (`pyxel.*`) signatures and names are never changed

## Web (JavaScript / HTML) Guidelines

### JavaScript
- `const` / `let` only; no `var`
- Arrow functions and template literals
- Cache repeated DOM element accesses in local variables
- Short functions; top-to-bottom readable flow

### HTML
- Consistent 2-space indentation
- Semantic tags where meaningful
- Consolidate excessive inline styles and scripts

## Process

1. Write and agree on this design document
2. Beautify Rust: `crates/pyxel-core/src/` → `crates/pyxel-binding/src/`
3. Beautify Python: `python/pyxel/` (editor, CLI, examples)
4. Beautify Web: `web/` → `wasm/`
5. Verify with `make lint` + `make lint-wasm` after each layer

### Work Units
- One file or a semantically related group of files per unit
- Each change is reviewed by the user before committing
- The user performs all commits

### Out of Scope
- Python public API changes
- Existing `unsafe` / raw pointer singleton patterns
- Auto-generated files and binaries
