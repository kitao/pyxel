# Pyxel Coding Policy

This document is the complete, project-wide coding policy and audit checklist. Apply every rule.

## Target Files

- In scope: all hand-authored files in the repository.
  - e.g., source code, configuration files, scripts, documentation
- Excluded: non-hand-authored content.
  - e.g., build artifacts (`target/`, `dist/`), generated Markdown from `scripts/generate_docs`, external dependencies (`node_modules/`)

File categories (referenced by sections below where a narrow file scope is needed):

- **Source code**: `*.rs`, `*.py`, `*.pyi`, `*.js`, `*.html`, `*.css`, `*.frag`, `*.vert`, `*.glsl`, `scripts/*`
- **Configuration**: `*.toml`, `*.yml`, `*.yaml`, `package.json`, `.vscode/*.json`, `.gitignore`, `.gitattributes`, `Makefile`, `requirements.txt`
- **Translation data**: `web/**/*.json` containing per-language fields (`ja`, `en`, etc.), parallel-language document pairs (`<name>.md` + `<name>-<lang>.md`)

When scope changes (new file type added), update this block — never edit patterns inline in sections below.

## Auditing

- When auditing any file set, walk every section below and apply every rule whose "Applies to" scope covers each file. Never build an ad-hoc subset or narrow list.
- Rules whose "Applies to" is not file-based (e.g., `development workflow` in Tools) apply at workflow transitions — in particular, before claiming any change, check, or audit is complete. Run the commands and invoke the skills listed; attach their output.
- When delegating an audit to a sub-agent, instruct the sub-agent to read this file and iterate every rule. Do not pre-curate a checklist; pre-curation suppresses rule coverage.
- Produce per-file evidence for every applied rule. Format: Markdown list, one line per in-scope file — `- <file>: PASS` or `- <file>: <specific finding>`. Summaries like "all OK" or "no issues" without per-file entries are not acceptable.
- For rules requiring judgment (naturalness of translation, natural/concise code, block-level intent, cross-file consistency), perform the judgment explicitly for every matching file. Do not defer, skip, or punt to a later reviewer.
- For mechanical rules (grep/regex checks, ordering verification, command runs), run the checks and attach the output.
- Before reporting an audit complete, re-read every section header below and confirm every rule was applied to every in-scope file with evidence recorded.

## Principles

Applies to: source code.

- Performance takes top priority. Break from language conventions when they cost performance.
  - Hot areas to watch:
    - Rendering loops (per-pixel blit, line/circle/rect primitives)
    - Audio generation (per-sample / per-frame voice, MML, BGM processing)
    - Language boundary calls (PyO3 FFI overhead)
    - Parallelization (SIMD, multi-threading)
  - Audit: for every in-scope file whose code path touches a hot area, verify it avoids unnecessary allocations, virtual dispatch, and per-iteration FFI crossings.
- In each language, write code that feels natural and concise to a professional in that language.
  - Audit: for every in-scope file, read it as a language professional would; flag stilted constructs, non-idiomatic patterns, and unnecessary verbosity.
- Keep the same intent in a single place; avoid duplication.
  - Audit: for every in-scope file, identify intent-bearing blocks; grep the repo for similar logic and flag duplicates.

## Consistency

Applies to: all hand-authored files in the repository.

- Apply the policy uniformly across all in-scope files.
  - Audit: for every in-scope file, compare against its sibling files (files matching the same directory or the same naming pattern, e.g., `*_wrapper.rs`, `editor/*.py`). Flag style, structure, or naming differences.
- Exception: sample code and inter-language interfaces may deviate from the language's conventions, but must stay internally consistent within their group.
  - e.g.,
    - Function and argument names in Rust bindings that follow the Python API
    - Call sites for SDL2 C functions
  - Audit: for every in-scope file in an exception group, compare against the other files in the same group (e.g., all `crates/pyxel-binding/src/*_wrapper.rs`); flag intra-group style, structure, or naming inconsistencies.

## Product Names

Applies to: all text (code, comments, strings, translations, docs).

- Treat the following product and website names as proper nouns. Do not translate them or alter their casing.
  - Pyxel
  - Pyxel Editor
  - Pyxel Showcase
  - Pyxel Code Maker
  - Pyxel Web Launcher
  - Pyxel User Examples
  - Pyxel Composer
  - Audit: grep each name (case-insensitive) across every in-scope file; flag any occurrence that differs from the canonical casing.
- Use the following abbreviations where helpful.
  - Pyxel Web — Pyxel's web version
  - Pyxel MML — Pyxel's MML variant
  - Pyxel API — Pyxel's API

## Naming

Applies to: source code.

- Follow language and project conventions for names and definition order.
  - e.g., within a type: constructors → public API → private helpers
  - Audit: for every in-scope file, verify member order matches the pattern above (or the sibling-file pattern when different); for every identifier, verify language casing and consistency with sibling-file naming.
- Adopt idiomatic abbreviations and conventional variable names as-is.
  - e.g.,
    - Python/Rust: `i` (loop counter), `e` (exception/error variable)
    - JS: `e` (event), `el` (DOM element)
  - Audit: for every in-scope file, flag any identifier that has been needlessly expanded from its idiomatic short form (e.g., `error` in a simple catch, `element` for a DOM reference).

## Comments and Blank Lines

Applies to: source code and configuration.

- Write comments in English.
  - Audit: for every in-scope file, grep comments for non-ASCII characters (ひらがな / カタカナ / 漢字 / 한국어 / 汉字 / etc.); flag any found.
- Keep comments to the minimum necessary. Do not add comments that state what the code obviously does.
  - Audit: for every in-scope file, inspect each comment; flag any that merely restates adjacent code.
- Add explanations where the intent is hard to grasp, to aid understanding.
  - Audit: for every in-scope file, identify non-trivial logic (complex conditions, algorithms, state machines, unsafe blocks); verify each has an accompanying explanatory comment; flag any unexplained.
- For large or hard-to-follow code blocks, put a comment at the top describing the block's role.
  - Audit: for every in-scope file, identify code blocks of 30+ lines or non-trivial purpose; verify each has a top-of-block role-describing comment; flag any missing.
- When a file contains consecutive groups of functions or methods, separate them with a standalone comment line so each group is easy to locate.
  - Audit: for every in-scope file with 2+ function/method groups, verify each group boundary has a standalone comment line; flag any missing.
- Do not add documentation comments (Rust `///`, Python docstrings). API documentation is maintained separately.
  - Audit: for every in-scope file, grep for `///`, `"""`, `'''`; flag any occurrence.
  - Exception: `python/pyxel/__init__.pyi`
    - Docstrings are regenerated by `scripts/generate_pyi_docstrings` (do not hand-edit).
    - Default values reflect what the Rust implementation applies when arguments are omitted, not the PyO3 binding layer's defaults.
- Keep comments that follow a domain convention consistent with each other.
  - e.g., the list of events handled by the widget system
  - Audit: for every in-scope file, identify domain-conventional comment patterns (e.g., widget `# Variables:` / `# Events:` blocks); verify their format matches sibling files; flag any deviation.
- Avoid redundant blank lines. Use a single blank line to separate meaningful chunks.
  - Audit: for every in-scope file, grep for 3+ consecutive blank lines; flag any match.

## Configuration Files

Applies to: configuration.

- Avoid arbitrary or incidental ordering. Follow conventional grouping and ordering. Within each group, sort entries by a fixed rule such as alphabetical order.
  - Audit: for every in-scope file, identify groups; verify each group's ordering follows a stated fixed rule (alphabetical, canonical tool order, UI-flow order, etc.). Flag any group whose ordering cannot be explained by such a rule.

## Multi-language Documentation

Applies to: translation data.

- Japanese is the source of truth. Produce English from Japanese, then produce every other language from the English (not from Japanese).
- Write each translation as natural technical prose in its target language, derived from the English sentence structure — not the Japanese compound-noun chain or other Japanese syntactic patterns.
- Audit:
  - For JSON translation fields: for every field, list `ja`, `en`, and every other language value side by side. For each non-ja/non-en value, verify (a) structure follows English (phrase/clause order, qualifier placement), and (b) reads as natural target-language prose. Flag each failing value.
  - For parallel-language document pairs: read the Japanese and each non-Japanese file side by side. Verify each non-Japanese file (a) follows English sentence structure (use the English pair if present; otherwise use English-language conventions), and (b) reads as natural target-language prose.

## Tools

Applies to: development workflow.

- Before claiming any change or check is complete, invoke the relevant superpowers skills.
  - e.g., `verification-before-completion`
  - Audit: review the current session's tool-call history; confirm each applicable skill was invoked.
- Delegate surface formatting (indentation, line wrapping, quoting, etc.) to `make format`. Do not format by hand.
  - Audit: run `make format`; attach the output; confirm no file is modified (codebase was already formatted).
- Keep `make lint` and `make lint-wasm` warning-free.
  - Audit: run `make lint` and `make lint-wasm`; attach the output; confirm zero warnings.
- After modifying code, ensure `make test-unit` passes without errors.
  - Audit: run `make test-unit`; attach the output; confirm all tests pass.
