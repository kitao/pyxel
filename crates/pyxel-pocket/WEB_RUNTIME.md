# Pocket Web Runtime Design

## Status

This document records the first Web-facing design for the PocketPy runtime on
the `pocket` branch. It is a design note only. It does not change the existing
Pyodide-backed Pyxel Web runtime.

## Goal

Add a separate PocketPy Web runtime that can prove Pyxel scripts run in the
browser through the `pyxel-pocket` implementation, without destabilizing the
current Pyodide path.

The first proof is a browser smoke test that loads a PocketPy-built WASM
artifact, executes a small Pyxel script, and renders to the existing canvas
surface.

## Non-Goals

- Do not replace `wasm/pyxel.js`.
- Do not add runtime switching to `launchPyxel()` in the first step.
- Do not change `app2html` output in the first step.
- Do not support Pyxel Editor, Pyxel Code Maker, Pyxel Web Launcher, or package
  imports in the first step.
- Do not treat Python language compatibility as complete because a browser smoke
  test passes.

## Architecture

Use a parallel Web runtime rather than extending the existing Pyodide runtime.

The initial artifact shape is:

- `wasm/pyxel-pocket.js`: browser bootstrap for the PocketPy runtime.
- `wasm/pyxel-pocket.wasm`: the Emscripten output containing `pyxel-pocket`,
  PocketPy, and the Pyxel core runtime.

The public browser entry point should be separate from `launchPyxel()`, for
example:

```js
launchPyxelPocket({
  command: "run",
  script: "...",
});
```

This keeps the existing `launchPyxel()` contract Pyodide-only until the PocketPy
runtime proves its canvas, input, audio, filesystem, reset, and error paths.

## JavaScript Sharing

Do not make JavaScript sharing a prerequisite for the first PocketPy Web proof.
The first `pyxel-pocket.js` may duplicate small browser-shell pieces from
`wasm/pyxel.js` when that keeps the PocketPy smoke test isolated and easier to
debug.

The PocketPy launcher must not import or depend on `wasm/pyxel.js`, because that
file carries Pyodide-specific loading, filesystem, and command-execution
behavior. Early duplication is acceptable only for runtime-neutral browser
behavior such as screen setup, the startup prompt, error overlays, keyboard
normalization, and virtual gamepad handling.

After the PocketPy browser smoke test works, duplicated browser-shell behavior
can be extracted into a small shared helper if all of these are true:

- the helper has no Pyodide or PocketPy dependency;
- both launchers can use it without changing their public entry points;
- extracting it reduces real maintenance risk rather than making the first
  runtime proof harder to reason about.

Runtime-specific work remains separate even after any shared helper exists:
Pyodide loading, wheel installation, Pyodide filesystem mirroring, Python command
execution, PocketPy WASM loading, PocketPy virtual files, and reset behavior.

## Rust Runtime Boundary

The native `pyxel-pocket` runner currently owns host-specific behavior:

- reading scripts from the host filesystem;
- changing the process current directory;
- extracting `.pyxapp` files into temporary directories;
- restarting the process for `pyxel.reset()`.

The Web runtime should not reuse that host layer directly. Instead, split the
runner into:

- a platform-neutral execution core that accepts source text, a display name,
  and virtual files;
- a native host adapter that keeps the current filesystem and restart behavior;
- a Web host adapter that supplies in-memory files, browser reset behavior, and
  JavaScript-facing error reporting.

This avoids making browser support a collection of native-runner exceptions.

## Browser Data Flow

The first Web path is intentionally small:

1. `pyxel-pocket.js` creates or reuses the Pyxel screen elements.
2. It waits for the user gesture required by browser audio policy.
3. It initializes the PocketPy WASM module and binds it to the canvas.
4. It passes an inline script to the PocketPy execution core.
5. The script imports `pyxel`, calls the public API, and renders one visible
   frame or runs the normal Pyxel loop.
6. Runtime errors are shown in the same page rather than replacing it.

`command: "play"` and `.pyxapp` base64 injection come after this path is proven.

## Build Integration

Add PocketPy Web build integration as separate targets. The existing
`build-wasm`, `lint-wasm`, and Pyodide wheel flow should keep their current
meaning.

Candidate targets:

- `build-pocket-wasm`: build the `pyxel-pocket` Web artifact.
- `lint-pocket-wasm`: check the PocketPy Web build without producing release
  output.

The first build target may be narrow and experimental, but it should not rely on
manual command history as its only specification.

## Testing

The first acceptance gate is deliberately not "Python compatibility".

Required first checks:

- `cargo check` or equivalent for the PocketPy Emscripten target;
- a browser smoke test that loads `pyxel-pocket.js` and renders a visible Pyxel
  frame;
- a failure-path smoke test that surfaces a Python/PocketPy error on the page;
- `git diff --check`.

After the smoke test is stable, add representative screenshot parity checks for
small examples that exercise drawing, input, resource loading, and audio setup.

## Open Risks

- The current Pyodide runtime owns several browser helpers that are not
  Pyodide-specific in concept, such as screen creation, startup prompt, virtual
  gamepad, and error overlays. Reusing them may require a small shared JS helper,
  but that refactor should wait until the independent PocketPy browser smoke
  test works.
- Emscripten main-loop ownership must be checked carefully so PocketPy, SDL2,
  and Pyxel reset behavior do not fight each other.
- `.pyxapp` support needs an in-memory extraction model before it can be called
  compatible with native `pyxel-pocket play`.

## Decision

Proceed with the separate PocketPy Web runtime. Do not wire it into the current
Pyodide launcher until the independent artifact has passed the first browser
smoke tests.
