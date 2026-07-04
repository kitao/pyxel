# Pocket Runtime Design

## Summary

Add a native-first PocketPy runtime for Pyxel as an independent Rust crate and
binary. The runtime shares `pyxel-core` with the existing CPython/PyO3 binding,
but it does not replace the current Python package, PyO3 extension, Pyodide
loader, or web runtime.

The first implementation target is a separate native command that can run a
small Pyxel script through PocketPy:

```console
pyxel-pocket app.py
```

The design keeps a future web PocketPy runtime possible, but the initial scope
is native. Runtime selection is an entrypoint choice, not a hot swap inside an
already-running Pyxel process.

## Background

Pyxel currently has a clean split between `pyxel-core` and the CPython binding:

- `crates/pyxel-core` owns the engine, platform integration, resources, input,
  graphics, audio, and the frame loop.
- `crates/pyxel-binding` exposes the public Python API through PyO3.
- `python/pyxel` provides the Python package, editor, CLI, examples, and type
  stubs.
- `wasm/pyxel.js` loads Pyodide, installs the Pyxel wheel, and dispatches web
  commands through the existing Python package.

The previous `pocketpy` branch proved that PocketPy can be wired into Pyxel, but
it also showed the failure mode to avoid. That branch pursued broad CPython
compatibility: a complete parallel wrapper surface, many PocketPy source
patches, standard-library shims, editor module embedding, `.pyxapp` support,
and a replacement web runtime. The result was useful research, but too complex
for a refined, maintainable Pyxel feature.

## Goals

- Add a PocketPy runtime without destabilizing existing Pyxel users.
- Keep the existing CPython/PyO3 and Pyodide paths unchanged.
- Make the first milestone small enough to review rigorously.
- Share the engine through `pyxel-core` instead of duplicating engine behavior.
- Keep PocketPy integration explicit, vendored, and reproducible.
- Avoid patching PocketPy for MVP behavior.
- Establish a runtime boundary that can later support a web PocketPy loader.

## Non-Goals

- Do not replace the `pyxel` Python package.
- Do not change `pyxel run`, `pyxel play`, `pyxel edit`, or `pyxel app2html`.
- Do not make PocketPy fully CPython-compatible.
- Do not run Pyxel Editor on PocketPy in the MVP.
- Do not support complete `.pyxapp` compatibility in the MVP.
- Do not add broad `os`, `sys`, `pathlib`, `importlib`, or `zipfile` shims.
- Do not patch PocketPy unless a later review finds a small, well-justified
  blocker.
- Do not hot-swap between CPython and PocketPy inside one running process.

## User-Facing Model

The MVP exposes PocketPy as a separate native command:

```console
pyxel-pocket path/to/app.py
```

The command initializes PocketPy, registers a `pyxel` module backed by
`pyxel-core`, changes the current directory to the script directory, sets
`__file__`, executes the script, and exits with a non-zero status on runtime
errors.

A future CLI integration may add an explicit runtime option:

```console
pyxel run --runtime pocket app.py
```

That option is not part of the MVP because it would couple the existing Python
package CLI to the experimental runtime. Keeping the first command separate
protects the stable user-facing API while the PocketPy surface is still being
validated.

## Runtime Selection

Runtime selection happens before the script starts:

- Native MVP: `pyxel-pocket app.py`.
- Possible native follow-up: `pyxel run --runtime pocket app.py`.
- Possible web follow-up: `launchPyxel({ runtime: "pocket", ... })`.

Runtime selection does not happen after `pyxel.init()` or during the frame loop.
CPython/PyO3 and PocketPy own different VMs, object models, exception types,
module registries, callback handles, and lifetime rules. Abstracting those
differences at run time would create a large compatibility layer and repeat the
complexity of the previous branch.

## Crate Layout

Add a new workspace member:

```text
crates/
  pyxel-core/
  pyxel-binding/
  pyxel-pocket/
    Cargo.toml
    build.rs
    vendor/
      pocketpy/
        pocketpy.c
        pocketpy.h
        VERSION
    src/
      ffi.rs
      lib.rs
      main.rs
      module.rs
      runtime.rs
      value.rs
      wrappers/
        system.rs
        graphics.rs
        input.rs
        variables.rs
```

`pyxel-pocket` depends on `pyxel-core` and the small build dependencies needed
to compile and bind PocketPy. It does not depend on `pyxel-binding` or PyO3.

The initial wrapper set should stay intentionally narrow. More wrappers are
added only when a concrete MVP example needs them.

## PocketPy Source Management

PocketPy source is vendored under `crates/pyxel-pocket/vendor/pocketpy`.
Builds must not download source code. Network access during normal build,
lint, and test creates reproducibility problems and makes CI failures harder to
diagnose.

The vendored directory records the upstream version in `VERSION`. A later
update script may refresh `pocketpy.c` and `pocketpy.h` from an audited upstream
release, but ordinary builds consume only checked-in files.

No PocketPy patches are part of the MVP. If a patch becomes unavoidable, it
must be isolated under `crates/pyxel-pocket/patches`, documented with the
upstream issue or reason, and covered by a focused test.

## Module Registration

`pyxel-pocket` registers a native PocketPy module named `pyxel`.

The registration layer owns:

- PocketPy VM initialization and finalization.
- Creation of the `pyxel` module.
- Binding Rust functions to PocketPy callables.
- Conversion between PocketPy values and Rust primitive types.
- Conversion of Rust/Pyxel errors into PocketPy exceptions.
- Synchronization of read-only dynamic module variables.

The wrappers call `pyxel-core` directly. They should not call through the PyO3
binding or mimic PyO3 internals.

## MVP API Surface

The first API surface is enough for small scripts and a smoke-test game:

- System:
  - `init(width, height, title=None, fps=None, quit_key=None,
    display_scale=None, capture_scale=None, capture_sec=None, headless=None)`
  - `run(update, draw)`
  - `quit()`
  - `flip()`
  - `show()`
- Graphics:
  - `cls(col)`
  - `pset(x, y, col)`
  - `line(x1, y1, x2, y2, col)`
  - `rect(x, y, w, h, col)`
  - `rectb(x, y, w, h, col)`
  - `text(x, y, s, col)`
- Input:
  - `btn(key)`
  - `btnp(key, hold=None, repeat=None)`
  - `btnr(key)`
- Variables and constants:
  - `width`
  - `height`
  - `frame_count`
  - `mouse_x`
  - `mouse_y`
  - key constants needed by the smoke tests
  - color constants if already mirrored in the existing binding constants

The MVP does not expose image, tilemap, sound, music, channel, tone, font,
resource, screenshot, screencast, or editor APIs. Those surfaces are large
enough to deserve separate review once the runtime boundary is proven.

## Callback Handling

`pyxel.run(update, draw)` stores PocketPy references to the two callback
functions for the lifetime of the frame loop. On each update and draw:

1. Synchronize dynamic module variables such as `frame_count`, `mouse_x`, and
   `mouse_y`.
2. Call the stored PocketPy function with no arguments.
3. If PocketPy reports an exception, print it through PocketPy's exception
   printer, request Pyxel shutdown, and exit the process with a non-zero status.

The wrapper must keep callback ownership simple. It should not support bound
method edge cases or weak-reference semantics beyond what PocketPy already
handles.

## Error Handling

Argument errors should become PocketPy exceptions instead of Rust panics.
Internal invariants may still assert in Rust when they indicate a Pyxel bug,
but user input errors from script calls should return clear Python-side error
messages.

The first implementation should prefer a small conversion helper layer over
ad-hoc checks in every wrapper. The helper layer should cover:

- required integer, float, boolean, and string arguments;
- optional integer, boolean, and string arguments;
- returning `None`, integers, booleans, floats, and strings;
- raising `TypeError`, `ValueError`, and generic exceptions.

## File and Import Behavior

The MVP only runs a single script file. The command:

- canonicalizes the script path;
- changes the process current directory to the script's parent directory;
- sets `__file__` in the script global scope;
- executes the script source.

General package imports, `.pyxapp` archive extraction, and compatibility shims
are outside the MVP. If PocketPy can import sibling `.py` files without extra
Pyxel code, that behavior can be left intact, but Pyxel should not add a custom
import system in the first milestone.

## Web Follow-Up Shape

The web follow-up should be designed after the native runtime is usable. The
target shape is a second runtime path, not a replacement for Pyodide:

```javascript
launchPyxel({ runtime: "pocket", command: "run", name: "app.py" });
```

The loader may share screen creation, virtual gamepad, file-drop UI, and error
overlay code with the current web runtime. It should load a separate
`pyxel_pocket.js` and `pyxel_pocket.wasm` artifact, then dispatch commands
through exported PocketPy runtime functions.

The web follow-up should not be implemented until the native wrapper boundary,
error path, and smoke tests are stable.

## Build Integration

MVP build integration is narrow:

- Add `pyxel-pocket` to the Cargo workspace.
- Add a targeted build command or documented cargo command for the native
  binary.
- Keep `make build`, `make test`, `make lint`, `make build-wasm`, and
  `make lint-wasm` behavior unchanged unless the user explicitly chooses to
  promote PocketPy into those gates.

A later promotion can add:

- `make build-pocket`
- `make test-pocket`
- `make lint-pocket`
- `make build-pocket-wasm`

Those targets should stay separate from the stable product gates until the
runtime is no longer experimental.

## Testing

The MVP needs focused tests at three levels:

- Rust unit tests for conversion helpers and wrapper argument validation.
- Native command smoke tests that run tiny PocketPy scripts in headless mode.
- A manual or scripted smoke sample that opens a window and exercises the frame
  loop, drawing, and input.

The first automated smoke scripts should be small and owned by
`crates/pyxel-pocket`, not copied from the full `python/pyxel/examples` set.
The full examples are CPython examples and should not be treated as a PocketPy
compatibility contract.

Suggested initial smoke scripts:

- initialize headless, draw a pixel, and quit;
- initialize headless, run one frame, read `frame_count`, and quit;
- call a wrapper with invalid arguments and assert the command exits non-zero.

## Documentation

The initial documentation should stay close to the experimental runtime:

- crate-level README or module comment for `pyxel-pocket`;
- a short development note under the design or plan docs;
- no user-guide or README promotion until the runtime supports a meaningful
  subset and its limitations are stable.

The public Pyxel documentation should not imply CPython compatibility for
PocketPy. Any later user-facing docs must call it an alternate runtime with a
limited API surface.

## Risks

- API duplication can drift from `pyxel-binding`.
  - Mitigation: keep the initial surface narrow and compare wrapper names
    against the existing `.pyi` before adding each API group.
- PocketPy behavior may differ from CPython in visible ways.
  - Mitigation: define examples as PocketPy-compatible scripts, not CPython
    compatibility tests.
- Web support may pressure the native design into loader-specific shortcuts.
  - Mitigation: keep web out of the MVP and expose a small runtime API from
    native first.
- Vendoring PocketPy may add maintenance work.
  - Mitigation: record the version, avoid local patches, and update through an
    explicit script later.

## Milestones

1. Add `pyxel-pocket` crate with vendored PocketPy source, FFI generation, and
   a command that can execute a Python source string.
2. Register a minimal `pyxel` module and run a headless script using
   `pyxel.init`.
3. Add the system, graphics, input, variable, and constant MVP wrappers.
4. Add native smoke tests for successful execution and error handling.
5. Evaluate whether the wrapper boundary is clean enough to plan the web
   follow-up.

## Open Decisions

- The binary name is `pyxel-pocket` for the MVP. A later product decision can
  rename it or hide it behind `pyxel run --runtime pocket`.
- PocketPy remains experimental until explicitly promoted. Promotion requires a
  separate decision about docs, CI gates, release notes, and support policy.
- Web runtime support is a follow-up milestone, not part of the native-first
  MVP.
