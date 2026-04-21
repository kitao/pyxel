# Pyxel Coding Policy

This file covers language-agnostic policies. Language-specific details belong in separate files as needed.

## Target Files

- All hand-authored files in the repository, regardless of language or file type.
  - e.g., source code, configuration files (`Cargo.toml`, `Makefile`, `.gitignore`, etc.), scripts (`scripts/`)
- Typical exclusions:
  - Build artifacts (`target/`, `dist/`)
  - Markdown files produced by `scripts/generate_docs`
  - External dependencies (`node_modules/`)

## Principles

- Performance takes top priority. When performance requires deviating from a language's conventions, choose performance.
- In each language, write code that feels natural and concise to a professional in that language.
- Avoid duplication and redundancy. Keep the same intent in a single place.

## Naming

- Function names, argument names, variable names, and definition order should follow the language's conventions and be natural and intuitive.
- Adopt idiomatic abbreviations and conventional variable names as-is.
  - e.g., `i` for a loop variable

## Proper Nouns

- Treat the following product and website names as proper nouns; do not translate them or alter their casing.
  - Pyxel
  - Pyxel Editor
  - Pyxel Showcase
  - Pyxel Code Maker
  - Pyxel Web Launcher
  - Pyxel User Examples
  - Pyxel Composer
- The following abbreviations may be used where helpful.
  - Pyxel Web — Pyxel's web version
  - Pyxel MML — Pyxel's MML variant
  - Pyxel API — Pyxel's API

## Consistency

- Apply a consistent policy across files. Do not vary style from file to file.
- Files with special circumstances — examples, interfaces to other languages — may deviate from the language's conventions, but must stay internally consistent within their group.
  - e.g.,
    - Function and argument names in Rust bindings that follow the Python API
    - Call sites for SDL2 C functions

## Comments and Blank Lines

- Write comments in English.
- Keep comments to the minimum necessary. Do not add comments that state what the code obviously does.
- Add explanations where the intent is hard to grasp, to aid understanding.
- For large or hard-to-follow code blocks, put a comment at the top describing the block's role.
- When a file contains consecutive groups of functions or methods, separate them with a standalone comment line so each group is easy to locate.
- Do not add documentation comments (Rust `///`, Python docstrings). API documentation is maintained separately.
  - Exception: the docstring portions inside `.pyi` files, which `scripts/generate_pyi_docstrings` regenerates automatically. Do not hand-edit those.
- Keep comments that follow a domain convention consistent with each other.
  - e.g., the list of events handled by the widget system
- Avoid redundant blank lines. Use a single blank line to separate meaningful chunks.

## Configuration Files

- Avoid arbitrary or incidental ordering. Follow conventional grouping and ordering.
- Within each group, sort entries by a fixed rule such as alphabetical order.

## Multi-language Documentation

- Japanese is the source of truth; translate it to English first, then use the English as the base for translations into other languages.
- Each translation should read as natural technical writing in the target language, not as a literal translation.

## Tools

- When performing rewrites, use the relevant superpowers skills properly.
  - e.g., `verification-before-completion` before claiming any change or check is complete
- Delegate surface formatting (indentation, line wrapping, quoting, etc.) to `make format`. Do not format by hand.
- Keep `make lint` and `make lint-wasm` warning-free.
- After modifying code, ensure `make test-unit` passes without errors.
