# Pyxel Coding Policy

## Principles

- **Determinism.** A rule produces the same verdict on the same input regardless of who applies it or when. Subjective taste is a last resort.
- **Self-applicability.** Every rule applies to every in-scope file (see Verification > Scope), including this document.
- **Performance first.** On hot paths, performance overrides language conventions and idiomatic style.
- **Cross-file consistency.** A local improvement that breaks whole-codebase consistency is a regression.
- **Read naturally.** Code reads concisely to a fluent reader of its language; the language's idioms are preferred over invented forms.

## Standards

### Source Code

#### Performance

- Code on the hot paths is shaped around minimum cost. The hot paths are:
  - per-pixel blit and primitive draws (line, circle, rect);
  - per-sample voice synthesis;
  - per-frame voice update for MML and BGM;
  - the PyO3 FFI boundary (argument marshaling and return paths);
  - SIMD or multi-threaded sections.

- On hot paths, idiomatic patterns that hide cost are not used.
  - e.g., per-frame heap allocations (`Vec::new`, `format!`, `Box::new` in inner loops); avoidable copies or type conversions; bounds checks in tight loops; missed SIMD, loop-unrolling, or inlining opportunities.

- Outside hot paths, code stays idiomatic and readable. Micro-optimization is reserved for the listed hot paths.
  - e.g., `for x in xs { f(x) }` in a config loader is preferred over a hand-unrolled alternative.

#### Naming

- Mechanical naming rules (the language's standard case conventions and lint-enforced patterns) apply first.
  - e.g., case convention and prefix/suffix patterns are checked before asking "does this read well?".

- File and directory names follow the same base-name, separator, and role-suffix pattern as their sibling group. Public URLs, generated paths, and author-titled assets keep their established spelling unless every mirrored reference is renamed together.
  - e.g., `*_wrapper.rs` files keep the wrapper suffix, `web/*/index.html` keeps the route directory as the page identity, and `laser-jetman.html` keeps its author-chosen hyphenation.

- A symbol referenced from more than one file uses the same base name at every site (function and type names, CSS classes, HTML IDs, i18n keys, public API entries). Suffixed variants of the base name are allowed when each variant is exposed as a separate public entry. When sibling files disagree, the cross-referenced file's name wins; between a binding and the `.pyi`, the `.pyi` wins.
  - e.g., a `pyxel-core` function `gen_bgm` keeps the same base name in `crates/pyxel-binding/src/*_wrapper.rs` and `python/pyxel/__init__.pyi`; if it is split for separate exposure, the split uses suffixes (`gen_bgm_mml`, `gen_bgm_json`) rather than a renaming.

- Names that signal confusion or rewrite leftovers are rewritten.
  - e.g., the same concept named differently in sibling files (`titleBlock` in one file, `titleDiv` in another for the same UI element) — anti-pattern; an asymmetric verb pair (`saveForGist` alongside `loadFromGist` and `loadFromUrl`) — anti-pattern; stutter or type prefix (`Canvas.drawCanvas()`, `strFoo`) — anti-pattern.

- A language's idiomatic abbreviations are kept as-is.
  - e.g., Python and Rust use `i` for a loop counter and `e` for an exception variable; JavaScript uses `e` for an event and `el` for a DOM element.

- A locally reasonable name with no peer to harmonize with is left as-is. The rename rule above applies when peers exist; with no peer, taste alone is not grounds for renaming.
  - e.g., a self-contained file using `titleDiv` stays as it is when no comparable sibling exists; the same name in a file with sibling files using `titleBlock` is renamed to match.

#### Ordering

- Definitions are ordered top-down: high-level structures and public types come before the free functions that consume them.
  - e.g., a Rust file places `pub struct Foo { ... }` and its impls before any free function consuming `Foo`.

- Where the language requires forward declarations, they precede their use, overriding top-down ordering at the local level.

- Configuration files follow each format's idiomatic grouping; within each group, entries are sorted alphabetically unless the format itself prescribes another order.
  - e.g., `Cargo.toml` orders `[package]` → `[lib]` → `[dependencies]` → `[build-dependencies]` → `[features]` → `[profile.release]` (the established Cargo convention).

#### Comments

- Every comment is in English.

- A comment exists only when it adds intent the code cannot show. Required cases are mechanical or non-obvious operations (bit-twiddling, format-specific encoding) and non-local invariants.
  - e.g., `i += 1  # increment i` — anti-pattern (stated by the code); `i += 1  # wrap at frame boundary` — typical (states intent).

- A block of 30 or more statement lines is preceded by a one-line comment naming the block's role.
  - e.g., a 40-line `match` with many arms gains a one-line header naming the dispatch.

- A file with multiple groups of functions or methods places a one-line separator comment before each group, using the language's idiomatic single-line comment form (no decorative dashes or banners).
  - e.g., Python `# Event handlers`, Rust `// Constructors`, JavaScript `// HTML helpers`.

- A label-style comment does not end with a period; a sentence comment uses normal punctuation.
  - e.g., `// Constructors` (label); `// The camera caches the buffer between frames.` (sentence).

- No documentation comments (Rust `///`/`//!`, Python docstrings, JSDoc `/** */`) anywhere except `python/pyxel/__init__.pyi`. The `.pyi` docstrings are regenerated by `scripts/generate_pyi_docstrings` and are not hand-edited.

- Domain conventions are uniform across all sites that follow them.
  - e.g., the editor widget convention uses `# Variables:` and `# Events:` blocks (`python/pyxel/editor/widgets/widget.py` and every widget file).

- Every comment stands alone out of context. No self-referential gloss, no tautological phrasing.
  - e.g., `the Pyxel API (the API of Pyxel)` — anti-pattern (gloss restates the term); `// explanations to aid understanding` — anti-pattern (tautology).

#### Formatting

- Surface formatting (indentation, line wrapping, quoting) is delegated to `make format` for the file types it covers; hand-written `.md` is formatted by hand; everything else keeps its existing formatting.
  - e.g., a Rust match arm is not hand-aligned; a `Cargo.toml` table is not hand-reformatted.

- Exactly one blank line separates meaningful chunks unless `make format` prescribes otherwise. Runs of blank lines and blank lines inside a chunk are not used.
  - e.g., one blank line between class methods; no double blank between imports; no blank line between a function signature and its first statement.

#### Consistency

- Each file belongs to a sibling group: same directory, same naming pattern, or shared role. Consistency is judged within the group, not against the rest of the codebase.
  - Sibling groups in this repo include: `crates/pyxel-binding/src/*_wrapper.rs`; `python/pyxel/editor/widgets/*.py`; `python/pyxel/editor/*_editor.py`; HTML pages under `web/*/index.html`; language JSON files under `web/**/*.json`.

- A sibling group may be an *exception group*: a deliberate deviation from the language's default conventions for an interface or other self-contained reason. Within an exception group, the group's internal style, its cross-file naming choices toward the mirrored interface, and the framework-level binding conventions it relies on govern.
  - Exception groups in this repo include:
    - `crates/pyxel-binding/src/*_wrapper.rs`: mirrors the Python API rather than Rust conventions (snake_case names, Python-style argument ordering, and Pyxel-historical short names like `blt`/`cls`/`pset` rather than the Rust-idiomatic counterparts in `pyxel-core`) and adopts the PyO3 binding conventions (`#[new]` for `__init__`, `#[getter]`/`#[setter]` for Python attributes);
    - SDL2 call sites: C-style names;
    - samples in `python/pyxel/examples/`: direct control flow, example-local names, and one-blank-line chunks may stay when production-style decomposition or abstraction would make the sample harder to follow.

- Parallel mirrors — shapes deliberately repeated across sibling files for API symmetry or data-structure parallelism — are preserved as-is.
  - e.g., binding wrappers mirror the Python API one-to-one; image and tilemap drawing primitives mirror each other; the `languages` array is independently loaded by each i18n JSON.

- The `.pyi` API stub records each parameter's effective default — the value the implementation resolves to — while its binding may take `None` as a sentinel and resolve it internally. The `.pyi` default and the binding-signature default may therefore differ; that divergence is intentional, not an inconsistency.
  - e.g., the `.pyi` writes `init(title="Pyxel", fps=30, ...)` while the binding takes `Option` sentinels and resolves them; `None` stays in the `.pyi` only where `None` is itself the default behavior (`display_scale` auto, `colkey` / `font` none).

### Testing

Tests cover the product in four layers: Rust unit tests for platform-independent pure logic; Python API tests for the public interface surface; screenshot regression over the bundled examples, apps, and editor; and a manual pass on running samples for look, sound, and feel. Test code itself is in scope for every Source Code rule.

- A behavior is unit-tested when its breakage would not surface in the screenshot regression or the manual pass. These cases qualify:
  - numeric boundaries and degenerate inputs (zero, empty, maximum, negative);
  - rarely-taken branches (special syntax, edge inputs);
  - determinism contracts whose silent change alters existing users' assets;
  - compatibility surfaces (deprecated aliases keep working and warn);
  - save/load and serialization roundtrips;
  - error paths (exception type and message).
  - e.g., the BGM generator's seed-determinism snapshot — typical (a silent change rewrites existing users' music).

- A behavior whose breakage is plainly visible or audible when running a sample is left to the screenshot regression and the manual pass; its internals gain no unit test.
  - e.g., a golden file of synthesized waveform bytes — anti-pattern (audible breakage, already covered by samples and ear).

- A test verifies what its name and comments claim; a test that cannot fail for the claimed reason is fixed or removed.
  - e.g., a wraparound test whose inputs never wrap — anti-pattern.

- A deterministic outcome is pinned exactly. An assertion accepting several outcomes is reserved for genuine nondeterminism, with the source named in a comment.
  - e.g., `play_pos()` may be `None` right after `play()` (audio-thread timing) — typical; "level is 0.0 or 1.0" for a deterministic envelope — anti-pattern.

- Every test executes in `make test`. A test excluded by a `cfg` gate carries a comment naming the condition and where it does run.

### Documentation

#### Prose

- Documentation prose reads as natural technical writing in its own language, using the target language's standard conventions for compound-noun chains rather than literal translation from another language.
  - e.g., English "package installation guide" — typical; "installation of the package guide" — anti-pattern (translationese).

- Japanese text separates Japanese characters from adjacent alphanumeric tokens with a single half-width space, regardless of which file the text lives in; code spans keep their literal spacing.
  - e.g., 「Web 版 Pyxel」「16 色」「.pyxres ファイル」 — typical; 「Web版」「16色」 — anti-pattern (missing separation).

- Japanese technical loanwords follow the project's adopted spelling rather than a mechanical English-suffix rule. Unlisted terms follow established usage in comparable developer documentation, then stay consistent across sibling documentation.
  - Adopted spellings: 「ブラウザ」「エディタ」「パラメータ」「バッファ」「コンストラクタ」「ユーザー」「サーバー」「コンピュータ」.
  - e.g., 「ブラウザ上で実行」「コードエディタ」「URL パラメータ」「画面録画バッファ」「コンストラクタで初期化」 — typical; mixing 「ブラウザ」 and 「ブラウザー」 for the same concept in sibling pages — anti-pattern.

- Japanese text chooses parenthesis width by content: parentheses containing Japanese characters are full-width and sit flush; parentheses with ASCII-only content are half-width, separated by half-width spaces except against punctuation.
  - e.g., 「イメージバンク（Image クラスのインスタンス）のリスト (0-2)」 — typical; 「リスト（0-2）」 — anti-pattern (full-width around ASCII-only content).

#### Translation

- The maintainer writes in Japanese; Japanese is the source of truth for translation. Translations route through English first, then to every other language.

- Each target language follows its own technical-writing conventions and retains established English loanwords where the target language conventionally uses them.
  - e.g., German, Spanish, Italian, and Portuguese keep loanwords like "Editor" and "Gamepad" in English; French instead uses native forms such as "éditeur" and "manette", keeping only product names like "Pyxel Editor" in English.

- A target-language translation is compared against the English version, not the Japanese source.
  - e.g., a German `"Installation des Pakets Anleitung"` mirrors a Japanese compound-noun chain and is rewritten as `"Paket-Installationsanleitung"`.

#### Proper Nouns

The authoritative Pyxel product names are: Pyxel, Pyxel Editor, Pyxel Showcase, Pyxel Code Maker, Pyxel MML Studio, Pyxel Web Launcher, Pyxel User Examples, and Pyxel Composer. The abbreviations Pyxel Web (the web version), Pyxel MML (the MML variant), and Pyxel API (the public API) may stand in for their full forms.

- Listed product names are not translated and their casing is not altered.
  - e.g., `Pyxel Editor` in every language — never `pyxel editor`, `Pyxel-Editor`, or `ピクセルエディタ`.

- Every other proper noun retains the author's chosen representation, including hyphens, spacing, and casing.
  - e.g., `laser-jetman.html` keeps its hyphen; author-titled examples are not renamed to fit a `Pyxel`-prefixed pattern.

- A descriptive label may stand in for a product name when the surrounding context establishes the reference and the label reads naturally there. Outside such contexts, the product name follows the casing rule above.
  - e.g., a "Related Sites" section that introduces Pyxel Showcase as "the Pyxel community showcase" reads naturally; references to the same product elsewhere still write `Pyxel Showcase`.

### Release Notes

- A `CHANGELOG.md` entry exists when the change carries (a) a concrete user benefit, or (b) a debugger breadcrumb a future maintainer can follow. Changes that match neither are not recorded.
  - User benefits include: feature addition, bug fix, visible behavior change, performance improvement.
  - Breadcrumbs include: dependency update; shipped runtime update; build-toolchain update that affects release artifacts; build-config change (e.g., a new `cfg(...)` gate); feature flag addition; internal runtime change; scoped refactor or cleanup; public API rename; release-process change.

- A breadcrumb names a concrete investigation surface; test-only, policy-only, and ignore-file changes are omitted unless they also change product, build, or release behavior.
  - e.g., `Updated pyo3 crate to version 0.29` remains useful, `Updated dependencies` is too broad, and adding tests or `.gitignore` entries is not a breadcrumb by itself.

- Sub-changes within a single commit are evaluated separately under the rule above.
  - e.g., a commit that fixes a bug and renames a public type produces two entries; a sub-change that is neither a user benefit nor a breadcrumb is omitted.

- Entries describe the change relative to the previous release. A change to code that has not shipped folds into the entry that introduces that code and does not produce its own entry.
  - e.g., a fix to a feature added earlier in the same unreleased version is absorbed by that feature's `Added` entry rather than gaining a `Fixed` entry.

- Each entry's verb, grammar form, and object specificity match prior entries of the same change category.
  - e.g., for an audio fix, the entry mirrors prior `Fixed` entries' tense and object specificity.

- Each entry fits a single line of at most 80 characters; entries typically run around 60. Longer descriptions are split into sub-changes per the rule above.
  - e.g., `Fixed Pyxel Editor color picker cursor shape across palette sizes` (65 chars) fits the typical band; entries needing more detail become two short entries instead of one long line.

- Each entry is verified against the actual code diff, not the commit message. Commit messages may understate or misstate the diff.

- Documentation wording and translation touch-ups bundle into a single summary line.
  - e.g., `Update web titles and docs wording` covers a commit touching many doc strings.

- Every rule in this section is reapplied on every revision. Earlier drafts are not accepted without rechecking.

## Verification

### Scope

- This policy applies to every git-tracked file that `.gitattributes` does not mark as `binary`, including this file.

- Files excluded because they are toolchain output:
  - `*.tmx` (Tiled tilemap editor output)
  - `*.bdf` (font tooling output)
  - `Cargo.lock` and `*-lock.json` (package-manager lockfiles)
  - `web/styles.css` (a Tailwind CSS build artifact)
  - `.md` files whose first line begins with `<!-- This file is generated` (output of `scripts/generate_docs`)

- A file's code-side aspects (structure, syntax, identifiers, non-prose elements) remain in scope even when its prose content is reviewed separately.

### Format, Lint, and Test

- After a code change, `make format` runs before the commit.

- `make lint` (native build) and `make lint-wasm` (WebAssembly build) are warning-free at all times. The two builds use different feature sets and target environments; both pass.
  - Clippy warnings count as failures. Suppression with `#[allow(...)]` requires that the suppression itself be justified.

- After a code change, `make test` passes before completion is claimed. A flaky failure does not waive the rule; the failure is reproduced and the underlying cause fixed.

## Conventions of This File

- A new concern joins an existing section before a new section is added. A new section is warranted only when no existing section fits.
  - e.g., a wording guideline for CHANGELOG entries belongs under `Standards > Release Notes`, not as a top-level section.

- Individual past incidents are not recorded. The lesson folds into the nearest existing rule or its example.
  - e.g., a one-off false-positive finding belongs in a commit message or the contributor's working notes, not as a named bullet here.

- A section with an authoritative enumeration separates the list from the rules. The list appears either in the introductory prose, followed by rule bullets, or as sub-bullets or numbered items under the rule that needs the detail.
  - e.g., `Standards > Documentation > Proper Nouns` lists product names and abbreviations in its intro and uses bullets for casing rules; `Standards > Source Code > Performance` enumerates hot paths as sub-bullets under the rule that introduces them.

- Each rule may be followed by an `e.g.,` sub-bullet that lists typical examples and, when useful, boundary cases or hypothetical anti-patterns.
  - Hypothetical anti-patterns read clearly as anti-patterns and are not asserted to exist in the code.
  - An `e.g.` line illustrates its rule and never substitutes for it; matching the example alone does not satisfy the rule.
  - A language-specific rule names the language in its rule statement.

- After revising any section, the whole file is re-read and balance confirmed. Substantial growth in one part triggers a review of its structurally comparable peers for parallel gaps; minor edits do not. Proportionality is checked by section length and bullet count.
