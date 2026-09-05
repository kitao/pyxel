# Pyxel Coding Policy

Use these principles and standards when reading, changing, or reviewing Pyxel.

## Principles

- **Design.** Keep Pyxel approachable. Preserve its deliberate constraints and separation of basic and advanced capabilities: they support learning and discovery. Keep differences that serve distinct users or purposes.
- **Clarity.** Keep what readers need to understand, use, or maintain the project. Prefer direct code and concise explanations; preserve necessary conditions and rationale.
- **Evidence.** Make decisions reproducible from the rules and evidence. Use mechanical checks where possible; explain judgments about naming, prose, and design through the relevant rule, source, and comparable sites. Personal taste and majority usage do not settle a disagreement.
- **Consistency.** Apply a correction to the whole affected family, including its public interfaces, translations, generated outputs, and tests.
- **Performance.** Correctness comes first. Demonstrated hot-path cost can justify a departure from language idiom; elsewhere, prefer idiomatic, readable code.
- **Self-applicability.** These standards govern every applicable part of the project, including code, documentation, tests, tooling, and the policy itself.

## Public Contract

- Preserve the maintainer's intended public contract and released compatibility. Changes to public behavior or meaning beyond the agreed scope require the maintainer's decision before changing implementation, documentation, or tests.
- Resolve disagreements using the maintainer's decisions, released behavior, implementation, documentation, and tests. No single representation establishes the intended contract. Documented requirements, accepted inputs, and editor controls may differ by design.
- API references and `.pyi` signatures show effective parameter defaults: the values used when arguments are omitted. Do not replace them with internal `None` sentinels. Use `None` only when it describes the actual default behavior, such as automatic selection or absence of a value.
  - e.g., `fps=30` documents the resolved value; `display_scale=None` documents automatic selection.

## Source Code

### Performance

- Review cost on these hot paths:
  - per-pixel blits and primitive drawing;
  - per-pixel 3D rasterization and shading;
  - per-sample voice synthesis;
  - per-frame MML and BGM voice updates;
  - per-frame 3D collision and BVH queries;
  - PyO3 argument marshaling and return paths;
  - SIMD and multi-threaded sections.

- Remove avoidable cost demonstrated by measurement or analysis of the executed path. Justify added complexity by the cost it removes; a suspicious-looking construct alone is not evidence.
  - e.g., repeated heap growth, copies, conversions, or bounds checks in an inner loop may warrant a change. `Vec::new()` alone does not allocate; inlining, unrolling, and SIMD need evidence of benefit.

- Outside these paths, prefer readability over micro-optimization. Extend the hot-path list when measurements establish another performance-critical family.

### Naming

- Follow the language's case conventions, lint rules, and idiomatic abbreviations. Keep a clear local name unless it misleads, describes an obsolete concept, or conflicts with comparable sites.
- File and directory names use consistent base names, separators, and role suffixes within each sibling group. Public URLs, generated paths, and author-titled assets retain their established spelling unless all affected references change together.
- Use consistent terminology for the same concept. Exact mirrors share names; distinct roles or language boundaries may use different names when their correspondence is clear. Preserve symmetric verb families and avoid redundant owner/type names.
  - e.g., image and tilemap drawing APIs keep matching operation names; an internal `draw_line` may implement the public `line` API. `Canvas.drawCanvas()` repeats its owner without adding meaning.

### Structure and Formatting

- Order definitions top-down: high-level structures and public types precede their supporting free functions. Required declarations and order-dependent behavior take precedence.
- Configuration uses the format's conventional groups. Sort entries alphabetically within a group unless their order carries meaning or the format prescribes another order.
- Delegate indentation, wrapping, and quoting to `make format` for supported files. Format hand-written Markdown by hand; other files follow their language or format conventions without unrelated reformatting.
- Separate meaningful chunks with one blank line unless the formatter prescribes otherwise. Do not insert blank lines inside a coherent chunk.

### Comments

- Write comments in English. Explain non-obvious intent, encodings, numerical assumptions, workarounds, or non-local invariants; do not narrate the code.
- Keep a shared rationale at the site that owns the decision. Dependent sites may identify the relevant invariant or point to that explanation when needed for local understanding.
- Use the shortest explanation that preserves the reason. Comparable operations receive comparable detail; complexity, not file length, determines how much is needed.
- A one-line group label is useful only when language structure does not already identify the group. Use sentence case without decorative banners or a terminal period. Sentence comments use normal punctuation; a single sentence may omit its terminal period.
  - e.g., `# Event handlers` identifies a group; `i += 1  # increment i` adds nothing.
- Do not use Rust documentation comments, Python docstrings, or JSDoc blocks except the generated docstrings in `python/pyxel/__init__.pyi`. Edit their source data and regenerate with `scripts/generate_pyi_docstrings`.
- Explain the current reason without requiring knowledge of a past incident. Avoid self-referential or tautological explanations.

### Cross-file Consistency

- Identify sibling groups by directory, naming pattern, and shared role. A file can belong to several groups. Compare every applicable group; repository-wide prevalence does not determine the correct form.
- Preserve intentional parallel structure, including image/tilemap operations, binding/API mirrors, and language data. Fix shared defects across all affected members.
- Standing exceptions to language idiom are limited to the conventions and reasons below. Performance-related departures follow the Performance section:
  - Exposed members in `crates/pyxel-binding/src/` follow the Python API's names and argument order, including historical names such as `blt` and PyO3's constructor/property conventions. Internal helpers remain idiomatic Rust.
  - SDL2 call sites retain the external API's C names so calls remain recognizable.
  - Examples in `python/pyxel/examples/` may keep direct control flow and local names when abstraction would obscure the lesson.
- Error and warning messages form families by failure kind across files. Python-standard errors retain CPython's exact wording; parameter constraints start with the parameter's public name; other messages follow their family's consistent, idiomatic form.
  - e.g., `fps must be greater than 0` and `scale must be greater than 0` belong to the same constraint family.

## Testing

Use Rust tests for pure internal logic, Python tests for the public API, reference regressions for screenshots and rendered audio, and running samples for manual checks of look, sound, and feel. Test code follows the source standards above.

- Derive expected results from the agreed contract or independently established properties. Changing implementation, documentation, and assertions together does not establish that their shared expectation is correct.
- Choose coverage by the failure it detects. Unit/API tests are especially useful for numeric boundaries, degenerate inputs, rare branches, determinism, deprecated aliases and warnings, serialization, and errors.
- Add a test when existing coverage cannot reliably detect the failure or isolate an important contract. Do not repeat the same assertion at another layer without a distinct reason. A theoretically visible or audible failure is not proof that an existing reference case exercises it.
- Use the manual pass for appearance, sound quality, interaction feel, and device-specific behavior that automated tests cannot establish. Name its cases and record what was actually run; an unperformed manual check provides no coverage evidence.
- A test's name and comments describe what its assertions establish. Fix or remove tests that cannot fail for the claimed reason, merely repeat implementation steps, or add no protection beyond existing cases.
- Pin deterministic discrete results and exact-output contracts exactly. Use an explicit numerical tolerance only when justified by precision or the algorithm. Accept alternative outcomes only for genuine nondeterminism, with its cause stated.
  - e.g., floating-point geometry may need a tolerance; an envelope test must not accept either endpoint merely to pass. Audio-thread timing can justify alternatives for `play_pos()`.
- Every automated test runs in `make test`.

## Documentation

### Reader and Prose

- Write for the page's intended reader and task, using words they can understand. Introductory material gives enough context and a small usable example to take the next step; introduce advanced or internal details only when they help that step.
- Keep each explanation in one authoritative place and link to it when needed. Remove repetition and unnecessary qualification while preserving prerequisites, units, limits, and consequences of destructive operations.
- Make instructions usable independently of a contributor's machine or private tools. Use relative paths or generic placeholders; keep personal paths and private environment details out of published documentation and reports.
- Use natural technical writing in each language. Keep terminology consistent across related pages; translate meaning rather than the source language's sentence structure.
- In Japanese, separate Japanese characters from adjacent alphanumeric tokens with one half-width space. Preserve literal spacing inside code spans.
  - e.g., 「Web 版 Pyxel」「16 色」「.pyxres ファイル」.
- Use these adopted Japanese spellings: 「ブラウザ」「エディタ」「パラメータ」「バッファ」「コンストラクタ」「ユーザー」「サーバー」「コンピュータ」. For other terms, choose established technical usage and apply it consistently.
- Japanese parentheses containing Japanese characters are full-width and sit flush. ASCII-only parentheses are half-width and separated by spaces except next to punctuation.
  - e.g., 「イメージバンク（Image クラスのインスタンス）のリスト (0-2)」.

### Translation and Product Names

- For translation, Japanese is the maintainer's source of truth. Translate through English, then use the English version to check other languages. Resolve suspected meaning loss against Japanese and correct the whole affected chain.
- Each language uses its own technical conventions and established loanwords. Preserve product names and placeholders; translated descriptions must retain the same conditions and behavior.
- The product names are Pyxel, Pyxel Cube, Pyxel Editor, Pyxel Showcase, Pyxel Code Maker, Pyxel MML Studio, Pyxel Web Launcher, Pyxel User Examples, and Pyxel Composer. Pyxel Web, Pyxel MML, and Pyxel API may identify the web version, MML variant, and public API respectively.
- Preserve product-name spelling and casing in every language. A descriptive label may replace a product name where the context makes its reference clear. Other proper nouns keep their author's spelling, spacing, hyphens, and casing.
  - e.g., write `Pyxel Editor`, not `Pyxel-Editor` or `ピクセルエディタ`; retain an author's title such as `laser-jetman`.

## Release Notes

- Record a concrete user benefit or a useful investigation clue. These include features, fixes, performance changes, public API changes, and specific dependency, runtime, toolchain, build, release-process, or internal-structure changes.
- Omit test-only, policy-only, and ignore-file changes unless they also change product, build, or release behavior. A maintenance entry names the affected component and change; generic cleanup claims are insufficient.
- Evaluate independent changes separately, relative to the previous release, and verify them against the code diff. Changes to an unreleased feature fold into its introductory entry.
- Use consistent, natural verbs and specificity within each change category. Each entry occupies one line of at most 80 characters; split it only for independent changes.
- Bundle qualifying documentation and translation improvements into one summary entry.

## Verification

### Scope

- Include every git-tracked file and intended addition not marked `binary` by `.gitattributes`. Review every applicable aspect of a file: path, code, comments, prose, translations, configuration, and test content.
- Exclude these generated or tool-maintained files:
  - `*.tmx` and `*.bdf`;
  - `Cargo.lock` and `*-lock.json`;
  - `web/styles.css`;
  - Markdown whose first line begins with `<!-- This file is generated`.
- An exclusion removes direct style review, not dependency checks. When source changes affect a generated document, package, or other distributed artifact, regenerate it and verify its required correspondence with the source.

### Required Checks

- Run `make format` after code or formatter-managed document changes and before committing them.
- Keep `make lint` and `make lint-wasm` warning-free. Clippy warnings fail the check; each suppression needs a specific justification.
- After code changes, `make test` must pass before claiming completion. Investigate failures; do not waive them as flaky. Record environment blockers explicitly.
- For documentation or structured-data changes, run applicable parsers, generators, consistency checks, and `git diff --check`. Verify generation at its source rather than hand-editing output.
- After substantive changes to the standards or verification process, review the changed obligations and their verification together, and recheck every subject whose verdict may change. Record the impact on repository practice; a revision review does not certify repository compliance.
- Run an exhaustive audit when explicitly requested and before a release tag. Cover every applicable rule and subject without sampling, with reproducible evidence and independent review. Unresolved findings or unperformed checks keep the audit pending.

## Maintaining This Policy

- Integrate enduring lessons into the relevant rule. Preserve agreed requirements when shortening or reorganizing them. Changes to requirements need the maintainer's decision, with their reason and practical effect recorded alongside the change. Keep incident histories and case-specific caveats out of the policy.
- Keep authoritative lists at their owning rule or section. Use at most one compact example per rule, only to clarify a boundary; an example does not create a separate requirement.
- After an edit, reread the policy for contradictions, gaps, and uneven detail. Review analogous rules and their verification when an obligation changes. Judge concision by how clearly the text governs decisions, not by its length or bullet count.
