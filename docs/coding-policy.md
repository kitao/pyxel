# Pyxel Coding Policy

This file is the Pyxel coding policy. It applies to every in-scope file (see Verification > Scope) and is enforced through the steps in Verification.

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

- Mechanical naming rules apply first; subjective taste is a last resort.
  - e.g., case convention, prefix/suffix patterns, and length limits are checked before asking "does this read well?".

- A symbol referenced from more than one file uses the same base name at every site (function and type names, CSS classes, HTML IDs, i18n keys, public API entries). Suffixed variants of the base name are allowed when each variant is exposed as a separate public entry.
  - e.g., a `pyxel-core` function `generate_bgm` keeps the same base name in `pyxel-binding/src/*_wrapper.rs` and `python/pyxel/__init__.pyi`; if it is split for separate exposure, the split uses suffixes (`generate_bgm_mml`, `generate_bgm_json`) rather than a renaming.
  - When a sibling file uses a different name, the cross-referenced file's name wins.

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

- A comment exists only when it adds intent the code cannot show. Required cases: a mechanical or non-obvious operation (bit-twiddling, format-specific encoding); a non-local invariant.
  - e.g., `i += 1  # increment i` — anti-pattern (stated by the code); `i += 1  # wrap at frame boundary` — typical (states intent).

- A non-trivial block of statements (≥ 30 lines, or a `match`/`switch` with heterogeneous arms) is preceded by a one-line comment naming the block's role.
  - e.g., a 40-line `match` with many arms gains a one-line header naming the dispatch.

- A file with multiple groups of functions or methods places a one-line separator comment before each group, using the language's idiomatic single-line comment form (no decorative dashes or banners).
  - e.g., Python `# Event handlers`, Rust `// Constructors`, JavaScript `// HTML helpers`.

- No documentation comments (Rust `///`, Python docstrings, JSDoc `/** */`) anywhere except `python/pyxel/__init__.pyi`. The `.pyi` docstrings are regenerated by `scripts/generate_pyi_docstrings`; do not hand-edit.

- Domain conventions are uniform across all sites that follow them.
  - e.g., the editor widget convention uses `# Variables:` and `# Events:` blocks (`python/pyxel/editor/widgets/widget.py` and every widget file).

- Every comment stands alone out of context. No self-referential gloss, no tautological phrasing.
  - e.g., `the Pyxel API (the API of Pyxel)` — anti-pattern (gloss restates the term); `// explanations to aid understanding` — anti-pattern (tautology).

#### Formatting

- Surface formatting (indentation, line wrapping, quoting) is delegated to `make format`. No hand-formatting.
  - e.g., a Rust match arm is not hand-aligned; a `Cargo.toml` table is not hand-reformatted.

- Exactly one blank line separates meaningful chunks unless `make format` prescribes otherwise. Runs of blank lines and blank lines inside a chunk are not used.
  - e.g., one blank line between class methods; no double blank between imports; no blank line between a function signature and its first statement.

#### Consistency

- Each file belongs to a sibling group: same directory, same naming pattern, or shared role. Consistency is judged within the group, not against the rest of the codebase.
  - Sibling groups in this repo include: `crates/pyxel-binding/src/*_wrapper.rs`; `python/pyxel/editor/widgets/*.py`; `python/pyxel/editor/*_editor.py`; HTML pages under `web/*/index.html`; language JSON files under `web/**/*.json`.

- A sibling group may be an *exception group*: a deliberate deviation from the language's default conventions for an interface or other self-contained reason. Within an exception group, the group's internal style, its cross-file naming choices toward the mirrored interface, and the framework-level binding conventions it relies on govern.
  - e.g., the `*_wrapper.rs` group mirrors the Python API (snake_case names, Python-style argument ordering, and Pyxel-historical short names like `blt`/`cls`/`pset` rather than the Rust-idiomatic counterparts in `pyxel-core`) rather than Rust conventions, and adopts the PyO3 binding conventions (`#[new]` for `__init__`, `#[getter]`/`#[setter]` for Python attributes); SDL2 call sites use C-style names; samples in `python/pyxel/examples/` may simplify production patterns for educational clarity.

- Parallel mirrors — shapes deliberately repeated across sibling files for API symmetry or data-structure parallelism — are preserved as-is.
  - e.g., binding wrappers mirror the Python API one-to-one; image and tilemap drawing primitives mirror each other; the `languages` array is independently loaded by each i18n JSON.

### Documentation

#### Prose

- Documentation prose reads as natural technical writing in its own language, using the target language's standard conventions for compound-noun chains rather than literal translation from another language.
  - e.g., English "package installation guide" — typical; "installation of the package guide" — anti-pattern (translationese).

#### Translation

- The maintainer writes in Japanese; Japanese is the source of truth for translation. Translations route through English first, then to every other language. Routing through English keeps target phrasing free of Japanese compound-noun structure.

- Each target language follows its own technical-writing conventions and retains established English loanwords where the target language conventionally uses them.
  - e.g., German and Romance languages (de, es, fr, it, pt) keep "Editor", "Launcher", "Gamepad", and similar technical terms in English.

- Translations are produced from English; comparison (including audit) is made against the English version, not the Japanese.
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
  - Breadcrumbs include: build-config change (e.g., a new `cfg(...)` gate); feature flag addition; internal runtime change; scoped refactor or cleanup; public API rename; release-process change.

- Sub-changes within a single commit are evaluated separately under the rule above.
  - e.g., a commit that fixes a bug and renames a public type produces two entries (or confirms the rename is not user-visible and merges them).

- Each entry's verb, grammar form, and object specificity match prior entries of the same change category.
  - e.g., for an audio fix, the entry mirrors prior `Fixed` entries' tense and object specificity.

- Each entry fits a single line of at most 80 characters; entries typically run around 60. Longer descriptions are split into sub-changes per the rule above.
  - e.g., `Fixed Pyxel Editor color picker cursor shape across palette sizes` (66 chars) fits the typical band; entries needing more detail become two short entries instead of one long line.

- Each entry is verified against the actual code diff, not the commit message. Commit messages may understate or misstate the diff.

- Documentation wording and translation touch-ups bundle into a single summary line.
  - e.g., `Update web titles and docs wording` covers a commit touching many doc strings.

- Every rule above is reapplied on every revision. An earlier draft is not rubber-stamped.

## Verification

### Scope

- This policy and its audit cover every git-tracked file that `.gitattributes` does not mark as `binary`. This policy file (`docs/coding-policy.md`) is in scope.

- Excluded by tool-chain origin:
  - `*.tmx` (Tiled tilemap editor output)
  - `*.bdf` (font tooling output)
  - `Cargo.lock` and `*-lock.json` (package-manager lockfiles)
  - `web/styles.css` (a Tailwind CSS build artifact)
  - `.md` files whose first line contains `<!-- This file is generated -->` (output of `scripts/generate_docs`)

- A file's code-side aspects (structure, syntax, identifiers, non-prose elements) remain in scope even when its prose content has been handed off for separate work; the handoff covers content, not the file.

### Build and Lint

- After a code change, run `make format` before committing.

- `make lint` (native build) and `make lint-wasm` (WebAssembly build) must be warning-free at all times. The two builds use different feature sets and target environments; both must pass.
  - Clippy warnings count as failures. Suppression with `#[allow(...)]` requires that the suppression itself be justified.

- After a code change, `make test` must pass before completion is claimed. A flaky failure does not waive the rule; reproduce the failure and fix the underlying cause.

### Audit (primarily for AI agents)

#### When to Run

The audit runs:
- when an audit is explicitly requested;
- before a release tag, as part of the release checklist;
- after a substantive revision of this policy, on the files affected by the revision.

#### Phases

The audit runs as ordered phases. Each phase gates the next; the meta-rules apply throughout.

A *false positive* in this procedure is a fix candidate that, on closer inspection, follows the policy's intent and is therefore not modified.

1. Build a (file × criterion) matrix using `superpowers:writing-plans`, listing every cell.
   - Each cell resolves to `pass`, `fix`, or `pending`, with one line of evidence (one line per field × language for translations). Aggregate summaries are not evidence; no cell is dropped silently.
   - Each cell's evidence verifies both (a) the rule body's broader intent and (b) the `e.g.` line's specific patterns. A cell addressing only (b) is marked `pending`, not `pass`.
   - e.g., one row per file, one column per criterion; each cell carries evidence such as `(a) the file's comments contain no unstated intent; (b) grep '^\s*///' returns no match` (pass), or the concrete problem (fix).

2. Run the cross-file consistency check.
   - Every file pair or group sharing a concern appears as an explicit matrix row with evidence; an unrepresented pair counts as a skipped check.
   - Each row's evidence verifies both (a) the cross-file dependency's broader intent and (b) the specific items compared. A row addressing only (b) is marked `pending`, not `pass`.
   - The auditor expands each category below into the concrete file pairs in the repo and identifies any pair not listed.
   - Cross-file pairs in this repo include:
     - sibling files (`*_wrapper.rs`, editor widgets, `web/*/index.html`);
     - HTML ↔ i18n JSON key sets;
     - Rust core ↔ binding ↔ `python/pyxel/__init__.pyi` signatures;
     - widget convention markers (`# Variables:` / `# Events:`) ↔ `copy_var` / `new_var` usage in `python/pyxel/editor/widgets/widget.py`;
     - the `languages` array across `web/**/*.json`.

3. Verify every matrix cell by reading its evidence and assessing the verdict. Format checks (row count, regex, banned-word grep) cannot substitute. When a phase has been delegated, read the delegated work's per-cell evidence, not its overall self-verification summary.
   - e.g., an evidence line `line 12: no issue` passes a regex but fails substance unless it names what was examined and why it is clean.

4. Run the design-intent self-check on every fix candidate. A candidate that hits any of the following intents is a false positive. Standards-derived intents come first; design-derived intents follow.
   - product name casing (Standards > Documentation > Proper Nouns);
   - section-context label substitution for a product name (Standards > Documentation > Proper Nouns);
   - wording reused intentionally from prior entries of the same change category, including generic phrasing repeated across versions (Standards > Release Notes);
   - parallel-mirror design (Standards > Source Code > Consistency);
   - intentional platform-conditional code (`cfg(...)` gates);
   - code duplicated for self-contained distribution (e.g., samples);
   - defensive code at system boundaries.

5. Gate completion in two stages.
   - (a) The original auditor runs `superpowers:verification-before-completion` to re-run Phases 1-4 against its own matrix and confirm consistency.
   - (b) An independent second reviewer — separate from the original auditor, in a fresh agent or session — re-runs the full audit from the in-scope files using `superpowers:code-reviewer`, rather than from the auditor's matrix. If the reviewer reports zero findings — or only a small number of judgment-call findings whose fix is not clearly net-positive — completion is gated. Otherwise the auditor incorporates the findings and a new second-reviewer cycle is run; the loop repeats until the gating condition is met.

#### Meta-rules

- Every criterion applies to every in-scope file. Sampling, spot-check, and ad-hoc scope narrowing are not permitted, whether during the audit itself or during verification of delegated work.
  - e.g., for "Comments in English", every `.rs` and `.py` file is checked — not "a representative sample". The same applies to verifying delegated per-cell verdicts: every cell is read, not a chosen subset.

- Finding imbalance is a non-execution signal. When findings concentrate in one category while structurally comparable categories return zero, the imbalance triggers a re-run on the zero-finding categories with stricter probing before proceeding.
  - e.g., if a documentation pass surfaces every fix candidate while every code-side group returns zero, the imbalance is the signal — the zero groups are re-inspected; the distribution is not accepted as-is.

- When a phase or pair-check is delegated, the rule text is passed verbatim, the file list is passed in full, and every cross-file dependency the group must cover is named explicitly. Shortening any of these causes silent sampling.
  - e.g., the HTML ↔ i18n JSON pairing, the Rust core ↔ binding ↔ pyi pairing, and the translation JSON keys across languages each pass as explicit pair lists with file paths.

## Conventions of This File

- A new concern joins an existing section before a new section is added. A new section is warranted only when no existing section fits.
  - e.g., a wording guideline for CHANGELOG entries belongs under `Standards > Release Notes`, not as a top-level section.

- Individual past incidents are not recorded. The lesson folds into the nearest existing rule or its example.
  - e.g., a one-off false-positive audit finding belongs in a commit message or the contributor's working notes, not as a named bullet here.

- A section that carries an authoritative enumeration separates the enumeration from the rules: either as intro-prose stating the enumeration followed by rule-bullets, or as a rule with sub-bullets or a numbered list enumerating the items when each needs detail.
  - e.g., `Standards > Documentation > Proper Nouns` lists product names and abbreviations in its intro and uses bullets for casing rules; `Standards > Source Code > Performance` enumerates hot paths as sub-bullets under the rule that introduces them.

- Each rule may be followed by an `e.g.,` sub-bullet that lists typical examples and, when useful, boundary cases or hypothetical anti-patterns.
  - Hypothetical anti-patterns read clearly as anti-patterns and are not asserted to exist in the code.
  - An `e.g.` line cites one or more illustrative examples; a language-specific rule names the language in its rule statement.

- After revising any section, the whole file is re-read and balance confirmed. Substantial growth in one part triggers a review of its structurally comparable peers for parallel gaps; minor edits do not. Proportionality is checked by section length and bullet count.
