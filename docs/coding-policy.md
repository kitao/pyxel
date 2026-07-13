# Pyxel Coding Policy

## Principles

- **Determinism.** A rule produces the same verdict on the same frozen input regardless of who applies it or when. Taste alone never determines a verdict.
- **Self-applicability.** Every rule applies to every in-scope surface it governs (see Verification > Scope), including the corresponding parts of this document.
- **Performance first.** Subject to correctness, demonstrated hot-path cost overrides language conventions and idiomatic style.
- **Cross-file consistency.** Explicit policy determines required departures from language idiom; otherwise language idiom determines the correct form, which comparable sites use uniformly.
- **Read naturally.** Code reads concisely to a fluent reader of its language; the language's idioms are preferred over invented forms.

## Standards

### Source Code

#### Performance

- Code on the hot paths eliminates avoidable cost. The hot paths are:
  - per-pixel blit and primitive draws (line, circle, rect);
  - per-pixel 3D rasterization (triangle fill and shading);
  - per-sample voice synthesis;
  - per-frame voice update for MML and BGM;
  - per-frame 3D collision and BVH queries;
  - the PyO3 FFI boundary (argument marshaling and return paths);
  - SIMD or multi-threaded sections.

- On hot paths, idiomatic patterns with a measured or mechanically demonstrated avoidable cost are not used.
  - e.g., per-frame heap allocations (`Vec::new`, `format!`, `Box::new` in inner loops); avoidable copies or type conversions; bounds checks in tight loops; missed SIMD, loop-unrolling, or inlining opportunities.

- Outside hot paths, code stays idiomatic and readable. Micro-optimization is reserved for the listed hot paths.
  - e.g., `for x in xs { f(x) }` in a config loader is preferred over a hand-unrolled alternative.

#### Naming

- Mechanical naming rules (the language's standard case conventions and lint-enforced patterns) apply first.

- File and directory names use a policy-compliant, language-idiomatic base-name, separator, and role-suffix pattern uniformly within their sibling group. Public URLs, generated paths, and author-titled assets keep their established spelling unless every mirrored reference is renamed together.
  - e.g., `*_wrapper.rs` files keep the wrapper suffix, `web/*/index.html` keeps the route directory as the page identity, and `laser-jetman.html` keeps its author-chosen hyphenation.

- A symbol referenced from more than one file uses the same base name at every site (function and type names, CSS classes, HTML IDs, i18n keys, public API entries). Suffixed variants of the base name are allowed when each variant is exposed as a separate public entry. When mirrors disagree, the authoritative public surface wins; between a binding and the `.pyi`, the `.pyi` wins. If no authoritative surface exists, the language-idiomatic name is chosen and every mirrored reference is updated together.
  - e.g., a `pyxel-core` function `gen_bgm` keeps the same base name in `crates/pyxel-binding/src/*_wrapper.rs` and `python/pyxel/__init__.pyi`; if it is split for separate exposure, the split uses suffixes (`gen_bgm_mml`, `gen_bgm_json`) rather than a renaming.

- A name is rewritten when it gives one concept a different base from its peers, breaks a symmetric verb family, repeats its owner or type without distinguishing meaning, or names an obsolete concept.
  - e.g., `titleBlock` and `titleDiv` for the same UI concept — anti-pattern; `Canvas.drawCanvas()` — anti-pattern (stutter).

- A language's idiomatic abbreviations are kept as-is.
  - e.g., Python and Rust use `i` for a loop counter and `e` for an exception variable; JavaScript uses `e` for an event and `el` for a DOM element.

- A locally reasonable name with no peer to harmonize with is left as-is. The rename rule above applies when peers exist; with no peer, taste alone is not grounds for renaming.
  - e.g., a self-contained file using `titleDiv` stays as it is when no comparable sibling exists; the same name in a file with sibling files using `titleBlock` is renamed to match.

#### Ordering

- Definitions are ordered top-down: high-level structures and public types come before the free functions that consume them.
  - e.g., a Rust file places `pub struct Foo { ... }` and its impls before any free function consuming `Foo`.

- Where the language requires forward declarations, they precede their use, overriding top-down ordering at the local level.

- Configuration files follow each format's idiomatic grouping; within each group, entries are sorted alphabetically unless the format itself prescribes another order.
  - e.g., `Cargo.toml` keeps dependencies, build dependencies, features, and release profiles in their conventional tables and sorts entries within each table.

#### Comments

- Every comment is in English.

- A comment exists only when it adds intent that neither the code nor an existing comment shows; a shared rationale lives once, at the site that owns the decision, unless another rule requires the repetition. Required cases are mechanical or non-obvious operations (bit-twiddling, format-specific encoding) and non-local invariants.
  - e.g., `i += 1  # increment i` — anti-pattern; `i += 1  # wrap at frame boundary` — typical; the same workaround explained at its deciding and dependent sites — anti-pattern.

- A comment is as short as its intent allows; comments expressing equally complex intent use comparable granularity.
  - e.g., a five-line header on a short helper where sibling files use one-line headers — anti-pattern (surplus wording); a widget's `# Variables:` and `# Events:` blocks matching the convention across widget files — typical.

- A one-line separator comment identifies a meaningful group only when the language's structure does not make the group equally clear; no size threshold alone requires one. It uses the language's idiomatic single-line comment form, in sentence case, without decorative dashes or banners.
  - e.g., Python `# Event handlers`, Rust `// Constructors`, JavaScript `// HTML helpers`.

- A label-style comment does not end with a period; a comment of two or more sentences punctuates every sentence, including the last. A single-sentence comment's terminal period is optional; either form is left as-is.
  - e.g., `// Constructors` (label); `// Playback has ended` and `// Playback has ended.` — both typical single-sentence forms.

- No documentation comments (Rust `///`/`//!`, Python docstrings, JSDoc `/** */`) anywhere except `python/pyxel/__init__.pyi`. The `.pyi` docstrings are regenerated by `scripts/generate_pyi_docstrings` and are not hand-edited.

- Every comment is understandable at its site without historical or external context. No self-referential gloss, no tautological phrasing.
  - e.g., `the Pyxel API (the API of Pyxel)` — anti-pattern (gloss restates the term); `// explanations to aid understanding` — anti-pattern (tautology).

#### Formatting

- Surface formatting (indentation, line wrapping, quoting) is delegated to `make format` for the file types it covers; hand-written `.md` is formatted by hand; every other file follows the standard conventions of its language or data format without unrelated reformatting.
  - e.g., a Rust match arm is not hand-aligned; a `Cargo.toml` table is not hand-reformatted.

- Exactly one blank line separates meaningful chunks unless `make format` prescribes otherwise. Runs of blank lines and blank lines inside a chunk are not used.
  - e.g., one blank line between class methods; no double blank between imports; no blank line between a function signature and its first statement.

#### Consistency

- Each file participates in every structurally comparable sibling group identified by a common directory, naming pattern, or shared role. Consistency is judged within each group; codebase-wide prevalence does not determine correctness.
  - e.g., sibling groups include `crates/pyxel-binding/src/*_wrapper.rs`; `python/pyxel/editor/widgets/*.py`; `python/pyxel/editor/*_editor.py`; HTML pages under `web/*/index.html`; language JSON files under `web/**/*.json`.

- A sibling group is an *exception group* only where this policy names the group, the convention it departs from, and the reason. The exception applies only to that convention; every other rule remains in force.
  - The exception groups are:
    - `crates/pyxel-binding/src/*_wrapper.rs`: mirrors the Python API rather than following Rust conventions (snake_case names, Python-style argument ordering, and Pyxel-historical short names like `blt`/`cls`/`pset` rather than the Rust-idiomatic counterparts in `pyxel-core`) and adopts the PyO3 binding conventions (`#[new]` for `__init__`, `#[getter]`/`#[setter]` for Python attributes);
    - SDL2 call sites: preserve the external SDL2 API's C-style names so calls remain recognizable against its documentation;
    - samples in `python/pyxel/examples/`: direct control flow and example-local names may stay when production-style decomposition or abstraction would make the sample harder to follow.

- Parallel mirrors — shapes deliberately repeated across sibling files for API symmetry or data-structure parallelism — preserve their shared structure. A correction is applied to every affected mirror rather than preserving a shared defect.
  - e.g., binding wrappers mirror the Python API one-to-one; image and tilemap drawing primitives mirror each other; each i18n JSON file repeats the `languages` array.

- Error and warning messages form codebase-wide families by failure kind rather than per-file groups: a message that mirrors a standard Python error keeps CPython's exact wording, a parameter constraint starts with the parameter as written, and any other message reuses its family's policy-compliant, language-idiomatic shape and casing rather than introducing a new form.
  - e.g., `fps must be greater than 0` and `scale must be greater than 0` — typical (one constraint family across files); a lone `draw: <message>` prefix among sentence-style siblings — anti-pattern.

- The `.pyi` API stub records each parameter's effective default — the value the implementation resolves to — while its binding may take `None` as a sentinel and resolve it internally. The `.pyi` default and the binding-signature default may therefore differ; that divergence is intentional, not an inconsistency.
  - e.g., the `.pyi` writes `init(title="Pyxel", fps=30, ...)` while the binding takes `Option` sentinels and resolves them; `None` stays in the `.pyi` only where `None` is itself the default behavior (`display_scale` auto, `colkey` / `font` none).

### Testing

Tests cover the product in four layers: Rust unit tests for platform-independent pure logic; Python API tests for the public interface surface; reference regression of screenshots from the bundled examples, apps, and editor plus rendered audio; and a manual pass on running samples for look, sound, and feel. Test code itself is in scope for every Source Code rule.

- A behavior is unit-tested when its breakage would not surface in the reference regression or the manual pass. These cases qualify:
  - numeric boundaries and degenerate inputs (zero, empty, maximum, negative);
  - rarely-taken branches (special syntax, edge inputs);
  - determinism contracts whose silent change alters existing users' assets;
  - compatibility surfaces (deprecated aliases keep working and warn);
  - save/load and serialization roundtrips;
  - error paths (exception type and message).
  - e.g., the BGM generator's seed-determinism snapshot — typical (a silent change rewrites existing users' music).

- A behavior already exercised by the reference regression and the manual pass, whose breakage is plainly visible or audible there, is not duplicated by an internal unit test.
  - e.g., a music mixing change is caught by the committed audio renders and the manual pass — typical; a unit test re-asserting the same waveform sample-by-sample — anti-pattern (duplicates the reference regression).

- A test verifies what its name and comments claim; a test that cannot fail for the claimed reason is fixed or removed.
  - e.g., a wraparound test whose inputs never wrap — anti-pattern.

- A deterministic outcome is pinned exactly. An assertion accepting several outcomes is reserved for genuine nondeterminism, with the source named in a comment.
  - e.g., `play_pos()` may be `None` right after `play()` (audio-thread timing) — typical; "level is 0.0 or 1.0" for a deterministic envelope — anti-pattern.

- Every automated test executes in `make test`.

### Documentation

#### Prose

- Documentation prose reads as natural technical writing in its own language, using the target language's standard conventions for compound-noun chains rather than literal translation from another language.
  - e.g., English "package installation guide" — typical; "installation of the package guide" — anti-pattern (translationese).

- Japanese text separates Japanese characters from adjacent alphanumeric tokens with a single half-width space, regardless of which file the text lives in; code spans keep their literal spacing.
  - e.g., 「Web 版 Pyxel」「16 色」「.pyxres ファイル」 — typical; 「Web版」「16色」 — anti-pattern (missing separation).

- Japanese technical loanwords follow the project's adopted spelling rather than a mechanical English-suffix rule. Unlisted terms follow established usage in comparable developer documentation, then stay consistent across sibling documentation.
  - Adopted spellings: 「ブラウザ」「エディタ」「パラメータ」「バッファ」「コンストラクタ」「ユーザー」「サーバー」「コンピュータ」.
  - e.g., 「ブラウザ上で実行」 — typical; mixing 「ブラウザ」 and 「ブラウザー」 for one concept in sibling pages — anti-pattern.

- Japanese text chooses parenthesis width by content: parentheses containing Japanese characters are full-width and sit flush; parentheses with ASCII-only content are half-width, separated by half-width spaces except when adjacent to punctuation.
  - e.g., 「イメージバンク（Image クラスのインスタンス）のリスト (0-2)」 — typical; 「リスト（0-2）」 — anti-pattern (full-width around ASCII-only content).

#### Translation

- The maintainer writes in Japanese; Japanese is the source of truth for translation. Translations route through English first, then to every other language.

- Each target language follows its own technical-writing conventions and retains established English loanwords where the target language conventionally uses them.
  - e.g., German, Spanish, Italian, and Portuguese keep loanwords like "Editor" and "Gamepad" in English; French instead uses native forms such as "éditeur" and "manette", keeping only product names like "Pyxel Editor" in English.

- A target-language translation is compared against the English version, not the Japanese source.
  - e.g., a German `"Installation des Pakets Anleitung"` mirrors a Japanese compound-noun chain and is rewritten as `"Paket-Installationsanleitung"`.

#### Proper Nouns

The authoritative Pyxel product names are: Pyxel, Pyxel Cube, Pyxel Editor, Pyxel Showcase, Pyxel Code Maker, Pyxel MML Studio, Pyxel Web Launcher, Pyxel User Examples, and Pyxel Composer. The abbreviations Pyxel Web (the web version), Pyxel MML (the MML variant), and Pyxel API (the public API) may stand in for their full forms.

- Listed product names are not translated and their casing is not altered.
  - e.g., `Pyxel Editor` in every language — never `pyxel editor`, `Pyxel-Editor`, or `ピクセルエディタ`.

- Every other proper noun retains the author's chosen representation, including hyphens, spacing, and casing.
  - e.g., `laser-jetman.html` keeps its hyphen; author-titled examples are not renamed to fit a `Pyxel`-prefixed pattern.

- A descriptive label may stand in for a product name when the surrounding context establishes the reference and the label reads naturally there. Outside such contexts, the product name follows the casing rule above.
  - e.g., a "Related Sites" section that introduces Pyxel Showcase as "the Pyxel community showcase" reads naturally; references to the same product elsewhere still write `Pyxel Showcase`.

### Release Notes

- A `CHANGELOG.md` entry exists when the change carries (a) a concrete user benefit, or (b) a debugger breadcrumb a future maintainer can follow. Changes that match neither are not recorded.
  - User benefits include: feature addition, bug fix, visible behavior change, performance improvement.
  - Breadcrumbs include: dependency update; shipped runtime update; build-toolchain update that affects release artifacts; build-configuration change that alters compilation; feature flag addition; internal runtime change; scoped refactor or cleanup; public API rename; release-process change.

- A breadcrumb names a concrete investigation surface; test-only, policy-only, and ignore-file changes are omitted unless they also change product, build, or release behavior.
  - e.g., `Updated pyo3 crate to version 0.29` remains useful, `Updated dependencies` is too broad, and adding tests or `.gitignore` entries is not a breadcrumb by itself.

- Sub-changes within a single commit are evaluated separately under the rule above.
  - e.g., a commit that fixes a bug and renames a public type produces two entries; a sub-change that is neither a user benefit nor a breadcrumb is omitted.

- Entries describe the change relative to the previous release. A change to code that has not shipped folds into the entry that introduces that code and does not produce its own entry.
  - e.g., a fix to a feature added earlier in the same unreleased version is absorbed by that feature's `Added` entry rather than gaining a `Fixed` entry.

- Each entry uses a language-idiomatic verb, grammar form, and level of object specificity. It matches compliant prior entries of the same change category for consistency; prevalence does not determine the form.

- Each entry fits a single line of at most 80 characters; entries typically run around 60 characters. An overlong entry is shortened without losing specificity and is split only when it contains independent sub-changes.
  - e.g., `Fixed Pyxel Editor color picker cursor shape across palette sizes` (65 chars) fits the typical band; independent user-visible changes become separate entries, while an atomic change is tightened into one line.

- Each entry is verified against the actual code diff, not the commit message. Commit messages may understate or misstate the diff.

- Documentation wording and translation touch-ups bundle into a single summary line.
  - e.g., `Update web titles and docs wording` covers a commit touching many doc strings.

## Verification

### Scope

- This policy applies to every file present in the audit target that is git-tracked or an intended addition and that `.gitattributes` does not mark as `binary`, including this file.

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

- Individual past incidents are not recorded. An enduring lesson tightens the nearest existing rule; an example changes only when it clarifies a reusable boundary, replacing or rebalancing existing examples rather than accumulating another case.
  - e.g., a one-off false-positive finding belongs in a commit message or the contributor's working notes, not as a named bullet here.

- A section with an authoritative enumeration separates the list from the rules. The list appears either in the introductory prose, followed by rule bullets, or as sub-bullets or numbered items under the rule that needs the detail.
  - e.g., `Standards > Documentation > Proper Nouns` lists product names and abbreviations in its intro and uses bullets for casing rules; `Standards > Source Code > Performance` enumerates hot paths as sub-bullets under the rule that introduces them.

- Each rule may be followed by at most one compact `e.g.,` sub-bullet containing only the examples needed to clarify a reusable boundary.
  - Hypothetical anti-patterns read clearly as anti-patterns and are not asserted to exist in the code.
  - An `e.g.` line illustrates its rule and never substitutes for it; matching the example alone does not satisfy the rule.
  - A language-specific rule names the language in its rule statement.

- After revising any section, the whole file is re-read and its balance is confirmed. Adding or splitting a rule or authoritative enumeration triggers a review of structurally comparable peers for parallel gaps; a change confined to wording or examples does not. Proportionality is checked by section length and bullet count.
