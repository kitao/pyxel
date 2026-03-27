# Code Beautification Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make all source code structurally clear, performant, and idiomatically beautiful per language conventions.

**Architecture:** Three-layer approach: Rust core → Rust bindings → Python → Web. Each layer is verified with `make lint` + `make lint-wasm` before proceeding to the next.

**Tech Stack:** Rust (pyxel-core, pyxel-binding), Python (editor, CLI, examples), JavaScript/HTML (web tools, WASM loader)

**Spec:** `docs/superpowers/specs/2026-03-24-code-beautification-design.md`

**Constraints:**
- Python public API (`pyxel.*`) signatures and behavior are unchanged
- Intentional `unsafe`/raw pointer singleton patterns are preserved
- Build/lint via `make`, never direct `cargo` commands
- User performs all commits

---

## Layer 1: Rust Core (`crates/pyxel-core/src/`)

### Task 1: Graphics — canvas.rs, image.rs, graphics.rs

**Files:**
- Modify: `crates/pyxel-core/src/canvas.rs` (1404 lines)
- Modify: `crates/pyxel-core/src/image.rs` (937 lines)
- Modify: `crates/pyxel-core/src/graphics.rs` (613 lines)

**Key improvements:**
- `canvas.rs`: Extract fast-path vs dithered-path comment clarity; reduce draw_line() coordinate duplication
- `image.rs`: Extract palette mapping helper (repeated 5+ times); reduce duplication between `draw_image()` and `draw_image_with_transform()`; rename `color_dist()` vars from `dx/dy/dz` to `dr/dg/db`
- `graphics.rs`: Method ordering (constructors → public → private); expression cleanup

- [ ] **Step 1:** Read all three files and identify every concrete change
- [ ] **Step 2:** Apply changes to `canvas.rs` — extract helpers, simplify draw methods, add performance comments
- [ ] **Step 3:** Apply changes to `image.rs` — extract `palette_ref()` helper, reduce draw_image duplication, improve naming
- [ ] **Step 4:** Apply changes to `graphics.rs` — reorder methods, clean up expressions
- [ ] **Step 5:** Run `make lint` and fix all warnings
- [ ] **Step 6:** Run `make lint-wasm` and fix all warnings

### Task 2: Graphics support — font.rs, tilemap.rs, tmx_parser.rs

**Files:**
- Modify: `crates/pyxel-core/src/font.rs` (307 lines)
- Modify: `crates/pyxel-core/src/tilemap.rs` (496 lines)
- Modify: `crates/pyxel-core/src/tmx_parser.rs`

**Key improvements:**
- `font.rs`: Extract BDF parsing from `new()` (84 lines) into dedicated function; reduce duplication between Bdf/Fontdue draw paths
- `tilemap.rs`: Reduce duplication between `collide_resolve_x()` and `collide_resolve_y()` (axis-swapped near-identical logic)

- [ ] **Step 1:** Read all three files and identify concrete changes
- [ ] **Step 2:** Apply changes to `font.rs` — extract BDF parser, unify draw paths
- [ ] **Step 3:** Apply changes to `tilemap.rs` — unify collision resolution axis logic
- [ ] **Step 4:** Apply changes to `tmx_parser.rs` — expression cleanup
- [ ] **Step 5:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 3: Audio — sound.rs, channel.rs, voice.rs, tone.rs, music.rs, audio.rs

**Files:**
- Modify: `crates/pyxel-core/src/sound.rs` (543 lines)
- Modify: `crates/pyxel-core/src/channel.rs` (632 lines)
- Modify: `crates/pyxel-core/src/voice.rs` (628 lines)
- Modify: `crates/pyxel-core/src/tone.rs` (145 lines)
- Modify: `crates/pyxel-core/src/music.rs`
- Modify: `crates/pyxel-core/src/audio.rs`

**Key improvements:**
- `sound.rs`: Split `to_commands()` (157 lines) into semantic sections (tempo, envelope, vibrato, notes)
- `tone.rs`: Eliminate unnecessary `clone()` in `waveform()`
- All files: Method ordering, iterator chain usage, unnecessary allocation removal

- [ ] **Step 1:** Read all six files and identify concrete changes
- [ ] **Step 2:** Apply changes to `sound.rs` — split `to_commands()`, clean expressions
- [ ] **Step 3:** Apply changes to `channel.rs` and `voice.rs` — method ordering, expression cleanup
- [ ] **Step 4:** Apply changes to `tone.rs`, `music.rs`, `audio.rs` — remove unnecessary clones, clean expressions
- [ ] **Step 5:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 4: Audio parsers — mml_parser.rs, old_mml_parser.rs, bgm_generator.rs, pcm_decoder.rs, mml_command.rs

**Files:**
- Modify: `crates/pyxel-core/src/mml_parser.rs` (941 lines)
- Modify: `crates/pyxel-core/src/old_mml_parser.rs`
- Modify: `crates/pyxel-core/src/bgm_generator.rs` (1809 lines)
- Modify: `crates/pyxel-core/src/pcm_decoder.rs`
- Modify: `crates/pyxel-core/src/mml_command.rs`

**Key improvements:**
- Expression cleanup, iterator usage, method ordering
- `bgm_generator.rs` is the largest file — focus on readability without restructuring generated music logic

- [ ] **Step 1:** Read all five files and identify concrete changes
- [ ] **Step 2:** Apply changes to `mml_parser.rs` — expression cleanup, method ordering
- [ ] **Step 3:** Apply changes to `old_mml_parser.rs` — same patterns
- [ ] **Step 4:** Apply changes to `bgm_generator.rs` — readability improvements, naming
- [ ] **Step 5:** Apply changes to `pcm_decoder.rs` and `mml_command.rs`
- [ ] **Step 6:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 5: System & resources — system.rs, pyxel.rs, resource.rs, resource_data.rs, old_resource_data.rs, settings.rs, screencast.rs

**Files:**
- Modify: `crates/pyxel-core/src/system.rs` (522 lines)
- Modify: `crates/pyxel-core/src/pyxel.rs`
- Modify: `crates/pyxel-core/src/resource.rs`
- Modify: `crates/pyxel-core/src/resource_data.rs`
- Modify: `crates/pyxel-core/src/old_resource_data.rs`
- Modify: `crates/pyxel-core/src/settings.rs`
- Modify: `crates/pyxel-core/src/screencast.rs`

**Key improvements:**
- `resource_data.rs`: Remove redundant fully-qualified paths like `<[Color]>::to_vec()`; unify clone patterns
- All files: Method ordering, expression cleanup

- [ ] **Step 1:** Read all seven files and identify concrete changes
- [ ] **Step 2:** Apply changes to `system.rs` and `pyxel.rs`
- [ ] **Step 3:** Apply changes to `resource.rs`, `resource_data.rs`, `old_resource_data.rs`
- [ ] **Step 4:** Apply changes to `settings.rs` and `screencast.rs`
- [ ] **Step 5:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 6: Input, math, utilities — input.rs, math.rs, utils.rs, rect_area.rs, profiler.rs, window_watcher.rs, lib.rs

**Files:**
- Modify: `crates/pyxel-core/src/input.rs` (216 lines)
- Modify: `crates/pyxel-core/src/math.rs` (193 lines)
- Modify: `crates/pyxel-core/src/utils.rs` (244 lines)
- Modify: `crates/pyxel-core/src/rect_area.rs`
- Modify: `crates/pyxel-core/src/profiler.rs`
- Modify: `crates/pyxel-core/src/window_watcher.rs`
- Modify: `crates/pyxel-core/src/lib.rs`

**Key improvements:**
- `input.rs`: Simplify `is_button_pressed()` with match + guard; extract frame-check helper
- `math.rs`: Define `DEG_TO_RAD`/`RAD_TO_DEG` constants
- `utils.rs`: Merge `simplify_string()` into single filter+map chain; use `format!()` in `add_file_extension()`

- [ ] **Step 1:** Read all seven files and identify concrete changes
- [ ] **Step 2:** Apply changes to `input.rs` — simplify button state logic
- [ ] **Step 3:** Apply changes to `math.rs` — constants, expression cleanup
- [ ] **Step 4:** Apply changes to `utils.rs` — chain simplification
- [ ] **Step 5:** Apply changes to remaining files
- [ ] **Step 6:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 7: Platform layer — platform/*.rs

**Files:**
- Modify: `crates/pyxel-core/src/platform/facade.rs`
- Modify: `crates/pyxel-core/src/platform/sdl2/platform_sdl2.rs`
- Modify: `crates/pyxel-core/src/platform/sdl2/poll_events.rs`

- [ ] **Step 1:** Read all platform files and identify concrete changes
- [ ] **Step 2:** Apply expression and structure improvements
- [ ] **Step 3:** Run `make lint` and `make lint-wasm`, fix all warnings

---

## Layer 2: Rust Bindings (`crates/pyxel-binding/src/`)

### Task 8: Binding utilities and macros — utils.rs, lib.rs, pyxel_singleton.rs, constant_wrapper.rs, variable_wrapper.rs

**Files:**
- Modify: `crates/pyxel-binding/src/utils.rs`
- Modify: `crates/pyxel-binding/src/lib.rs`
- Modify: `crates/pyxel-binding/src/pyxel_singleton.rs`
- Modify: `crates/pyxel-binding/src/constant_wrapper.rs`
- Modify: `crates/pyxel-binding/src/variable_wrapper.rs`

**Key improvements:**
- `utils.rs`: Remove redundant fully-qualified `pyo3::` paths where prelude covers them; reduce duplication in `__iter__`/`__reversed__`; use `vec.repeat(n)` instead of `cycle().take()`
- `lib.rs`: Group module declarations by function (graphics, audio, input)
- `constant_wrapper.rs`: Consider loop-based constant registration

- [ ] **Step 1:** Read all five files and identify concrete changes
- [ ] **Step 2:** Apply changes — clean macros, simplify expressions, improve module grouping
- [ ] **Step 3:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 9: Wrapper files — all *_wrapper.rs

**Files:**
- Modify: `crates/pyxel-binding/src/audio_wrapper.rs`
- Modify: `crates/pyxel-binding/src/channel_wrapper.rs`
- Modify: `crates/pyxel-binding/src/font_wrapper.rs`
- Modify: `crates/pyxel-binding/src/graphics_wrapper.rs`
- Modify: `crates/pyxel-binding/src/image_wrapper.rs`
- Modify: `crates/pyxel-binding/src/input_wrapper.rs`
- Modify: `crates/pyxel-binding/src/math_wrapper.rs`
- Modify: `crates/pyxel-binding/src/music_wrapper.rs`
- Modify: `crates/pyxel-binding/src/resource_wrapper.rs`
- Modify: `crates/pyxel-binding/src/sound_wrapper.rs`
- Modify: `crates/pyxel-binding/src/system_wrapper.rs`
- Modify: `crates/pyxel-binding/src/tilemap_wrapper.rs`
- Modify: `crates/pyxel-binding/src/tone_wrapper.rs`

**Key improvements:**
- Add `inner_ref()`/`inner_mut()` helper methods to reduce repetitive `unsafe { &*self.inner }` blocks
- Unify method ordering: constructors → getters/setters → functional methods
- Fix visibility inconsistency (`wrap()` is `pub` in most files but private in `font_wrapper.rs`)
- Reduce duplicated index validation in `graphics_wrapper.rs`
- Unify type dispatch in `math_wrapper.rs` (`clamp`/`sgn` share identical i64/f64 branching)

- [ ] **Step 1:** Read all wrapper files and identify concrete changes
- [ ] **Step 2:** Add `inner_ref()`/`inner_mut()` helpers where applicable
- [ ] **Step 3:** Apply method reordering and expression cleanup across all wrappers
- [ ] **Step 4:** Fix visibility, reduce validation duplication, unify type dispatch
- [ ] **Step 5:** Run `make lint` and `make lint-wasm`, fix all warnings

---

## Layer 3: Python (`python/pyxel/`)

### Task 10: Editor widgets — `python/pyxel/editor/widgets/`

**Files:**
- Modify: `python/pyxel/editor/widgets/widget.py` (286 lines)
- Modify: `python/pyxel/editor/widgets/scroll_bar.py`
- Modify: `python/pyxel/editor/widgets/button.py`
- Modify: `python/pyxel/editor/widgets/color_picker.py`
- Modify: `python/pyxel/editor/widgets/number_picker.py`
- Modify: `python/pyxel/editor/widgets/radio_button.py`
- Modify: `python/pyxel/editor/widgets/image_button.py`
- Modify: `python/pyxel/editor/widgets/image_toggle_button.py`
- Modify: `python/pyxel/editor/widgets/text_button.py`
- Modify: `python/pyxel/editor/widgets/toggle_button.py`
- Modify: `python/pyxel/editor/widgets/settings.py`
- Modify: `python/pyxel/editor/widgets/widget_var.py`
- Modify: `python/pyxel/editor/widgets/__init__.py`

**Key improvements:**
- `widget.py`: Use `<` for range checks (half-open intervals); extract init helpers
- `scroll_bar.py`: Split `__on_draw()` vertical/horizontal into separate methods
- `button.py`: Remove pointless `return None`
- `radio_button.py`: Unify duplicated mouse_down/mouse_drag handlers
- `number_picker.py`: Extract delta calculation helper

- [ ] **Step 1:** Read all widget files
- [ ] **Step 2:** Apply changes to `widget.py` and `scroll_bar.py`
- [ ] **Step 3:** Apply changes to remaining widget files
- [ ] **Step 4:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 11: Editor core — canvas_panel.py, editor_base.py, field_cursor.py, extensions.py

**Files:**
- Modify: `python/pyxel/editor/canvas_panel.py` (556 lines)
- Modify: `python/pyxel/editor/editor_base.py`
- Modify: `python/pyxel/editor/field_cursor.py` (316 lines)
- Modify: `python/pyxel/editor/extensions.py`

**Key improvements:**
- `canvas_panel.py`: Extract scroll bar init from `__init__`; define constants for magic values (255, 255); reduce nesting
- `editor_base.py`: Move constant dicts to class level; simplify shortcut conditions
- `field_cursor.py`: Extract clamp helper for repeated `max(min(...), 0)` pattern
- `extensions.py`: Use loop for repeated `pyxel.X.method = func` assignments

- [ ] **Step 1:** Read all four files
- [ ] **Step 2:** Apply changes to `canvas_panel.py` — extract inits, define constants, reduce nesting
- [ ] **Step 3:** Apply changes to `editor_base.py`, `field_cursor.py`, `extensions.py`
- [ ] **Step 4:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 12: Editor panels — image_editor.py, tilemap_editor.py, sound_editor.py, music_editor.py, and related

**Files:**
- Modify: `python/pyxel/editor/image_editor.py`
- Modify: `python/pyxel/editor/image_viewer.py`
- Modify: `python/pyxel/editor/tilemap_editor.py`
- Modify: `python/pyxel/editor/tilemap_viewer.py`
- Modify: `python/pyxel/editor/sound_editor.py`
- Modify: `python/pyxel/editor/sound_field.py`
- Modify: `python/pyxel/editor/sound_selector.py`
- Modify: `python/pyxel/editor/music_editor.py`
- Modify: `python/pyxel/editor/music_field.py`
- Modify: `python/pyxel/editor/piano_keyboard.py`
- Modify: `python/pyxel/editor/piano_roll.py`
- Modify: `python/pyxel/editor/octave_bar.py`
- Modify: `python/pyxel/editor/app.py`
- Modify: `python/pyxel/editor/settings.py`
- Modify: `python/pyxel/editor/__init__.py`

**Key improvements:**
- `sound_field.py`: Table-driven key dispatch instead of deep if-elif chains
- `music_editor.py`: Unify `__on_undo`/`__on_redo` into shared helper
- `piano_keyboard.py`: Table-driven Y-coordinate ranges for note mapping
- `tilemap_editor.py`: Unify undo/redo handlers
- `app.py`: Extract color initialization from `__init__`

- [ ] **Step 1:** Read all editor panel files
- [ ] **Step 2:** Apply changes to sound-related files (sound_editor, sound_field, music_editor, music_field, piano_keyboard, piano_roll, octave_bar)
- [ ] **Step 3:** Apply changes to image/tilemap-related files (image_editor, image_viewer, tilemap_editor, tilemap_viewer)
- [ ] **Step 4:** Apply changes to app.py, settings.py, __init__.py
- [ ] **Step 5:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 13: CLI and utilities — cli.py, utils.py, __init__.py, __main__.py

**Files:**
- Modify: `python/pyxel/cli.py` (444 lines)
- Modify: `python/pyxel/utils.py`
- Modify: `python/pyxel/__init__.py`
- Modify: `python/pyxel/__main__.py`

**Key improvements:**
- `cli.py`: Extract `_exit_with_error()` helper; use list comprehensions for glob+isfile patterns; simplify extension handling logic
- `utils.py`: Reduce nesting in ImportFrom handling; define constants for magic strings ("system"/"local")

- [ ] **Step 1:** Read all four files
- [ ] **Step 2:** Apply changes to `cli.py` — error helpers, comprehensions, expression cleanup
- [ ] **Step 3:** Apply changes to `utils.py` — reduce nesting, define constants
- [ ] **Step 4:** Apply changes to `__init__.py` and `__main__.py` if needed
- [ ] **Step 5:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 14: Example scripts — `python/pyxel/examples/`

**Files:**
- Modify: `python/pyxel/examples/09_shooter.py` (382 lines)
- Modify: All other example files (01-08, 10-19, 99)

**Key improvements:**
- Examples serve as documentation — prioritize readability over all else
- Apply consistent formatting, naming, and structure
- `09_shooter.py` is the largest — focus on expression clarity

- [ ] **Step 1:** Read all example files
- [ ] **Step 2:** Apply Pythonic expression improvements across all examples
- [ ] **Step 3:** Run `make lint` and `make lint-wasm`, fix all warnings

---

## Layer 4: Web & WASM

### Task 15: JavaScript — pyxel.js, shared.js, import_hook.py

**Files:**
- Modify: `wasm/pyxel.js` (847 lines)
- Modify: `web/shared.js` (69 lines)
- Modify: `wasm/import_hook.py`

**Key improvements:**
- `pyxel.js`: Replace `let` with `const` where no reassignment; unify function style (arrow vs function keyword); reduce DOM element creation verbosity; modernize XHR to fetch where feasible
- `shared.js`: Consistent function naming; simplify element creation
- `import_hook.py`: Python style cleanup

- [ ] **Step 1:** Read all three files
- [ ] **Step 2:** Apply changes to `pyxel.js` — const/let, function style, DOM patterns
- [ ] **Step 3:** Apply changes to `shared.js` — naming, expression cleanup
- [ ] **Step 4:** Apply changes to `import_hook.py`
- [ ] **Step 5:** Run `make lint` and `make lint-wasm`, fix all warnings

### Task 16: HTML files — web/ and wasm/ directories

**Files:**
- Modify: `web/launcher/index.html`, `web/launcher/url-builder.html`
- Modify: `web/code-maker/index.html`, `web/code-maker/manual.html`, `web/code-maker/pyxel-screen.html`, `web/code-maker/pyxel-editor.html`
- Modify: `web/api-reference/index.html`
- Modify: `web/showcase/index.html`
- Modify: `web/user-guide/index.html`
- Modify: `web/editor-manual/index.html`
- Modify: `web/web-usage/index.html`
- Modify: `web/mml-studio/index.html`, `web/mml-studio/manual.html`, `web/mml-studio/mml-commands.html`, `web/mml-studio/pyxel-screen.html`
- Modify: WASM redirect HTML files (6 files)

**Key improvements:**
- Consistent 2-space indentation
- Move repeated inline styles to CSS classes where beneficial
- Replace inline event handlers (`onclick=`) with `addEventListener` where practical
- Semantic HTML improvements where meaningful

Note: Showcase app/example HTML files (`web/showcase/apps/`, `web/showcase/examples/`, `web/showcase/tools/`) are likely generated/templated — verify before modifying.

- [ ] **Step 1:** Read all HTML files, identify which are hand-written vs generated
- [ ] **Step 2:** Apply changes to hand-written HTML files — indentation, inline styles, event handlers
- [ ] **Step 3:** Apply changes to WASM redirect files if needed
- [ ] **Step 4:** Run `make lint` and `make lint-wasm`, fix all warnings

---

## Final Verification

### Task 17: Full build and lint verification

- [ ] **Step 1:** Run `make lint` — verify zero warnings
- [ ] **Step 2:** Run `make lint-wasm` — verify zero warnings
- [ ] **Step 3:** Run `make build` — verify successful build
- [ ] **Step 4:** Report results to user for final review
