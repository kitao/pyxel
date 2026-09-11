# Distribution Contract Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Public Contract policy](../design-policy.md#public-contract)

## Crates and Wheels

### Registry publication of the Rust crates

**Decision:** Keep `publish = false` in the
[core](../../../crates/pyxel-core/Cargo.toml) and
[binding](../../../crates/pyxel-binding/Cargo.toml) manifests. These crates are not
distributed through crates.io.

**Reason:** The core and binding are built together for Pyxel; maintaining a
separately published Rust package is outside the intended distribution scope.
A `pub` Rust item does not by itself establish a supported crates.io API.

**Boundary:** This does not waive compatibility checks for Python, saved data,
or repository callers. Rust signature changes still affect source callers and
must be assessed on that basis, without inventing a separate package promise.

### Python ABI and web runtime compatibility

**Decision:** Keep Python 3.11 as the minimum runtime and build the extension
against its stable ABI. Treat the web runtime, Emscripten build, and web wheel
platform tag as a separate compatibility unit whose producers and consumers
change together.

**Reason:** The [binding manifest](../../../crates/pyxel-binding/Cargo.toml)
selects `abi3-py311`, matching
[project metadata](../../../python/pyproject.toml). The interpreter used by
[wheel builds](../../../.github/workflows/build.yml) can be newer without raising
the package minimum. Web compatibility also depends on the
[build flags](../../../Makefile), the Pyodide version selected by the
[web runtime](../../../wasm/pyxel.js), and the tags checked by the
[wheel tools](../../../scripts/check_wasm_wheel).
Matching version strings alone does not establish that the toolchain works;
build and runtime verification still apply to the selected targets.

### SDL2 linkage and Linux fallback

**Decision:** Link SDL2 statically for macOS and Windows. On Linux, load the
system SDL2 library first and fall back to the packaged copy. Web builds link
the PIC SDL2 port into the side module.

**Reason:** The [Python loader](../../../python/pyxel/__init__.py) makes Linux
SDL2 symbols available to the extension through `RTLD_GLOBAL`; the fallback
supports systems without the library. The
[core build](../../../crates/pyxel-core/build.rs), crate feature forwarding,
[Makefile](../../../Makefile), and
[wheel builds](../../../.github/workflows/build.yml) supply the corresponding
linkage and dependencies. These platform differences serve the distributions;
one linking rule cannot replace them. Import success establishes neither device
behavior nor audio and graphics quality.

### Web wheel validation and installation

**Decision:** Validate the selected web wheel before installing it into `wasm/`.
Check its identity and platform tag, packaged Python sources and metadata, and
unwanted caches and build-host paths. Install the wheel and update the web
runtime's wheel reference before removing obsolete wheels.

**Reason:** A successful import can still load stale Python sources.
[check_wasm_wheel](../../../scripts/check_wasm_wheel) compares the package with
its source owners, while [install_wasm_wheel](../../../scripts/install_wasm_wheel)
updates the artifact and its consumer. The
[build target](../../../Makefile) runs them in that order. Packaging exclusions
prevent cache inclusion; inspecting the built artifact verifies that boundary.

The installer uses separate replacements, not a transaction covering both files.
A failure can leave an unreferenced new wheel, or replace a same-version wheel
before the update of the web runtime's wheel reference fails. Source and
metadata comparisons do not establish compiled Rust/source equivalence. Path
remapping and `SOURCE_DATE_EPOCH` control build-host paths and timestamps; they
do not by themselves promise byte-identical builds on every platform.

### Release version correspondence

**Decision:** Use Python release spelling in the runtime version, Python
metadata, web wheel filename, and release tag. Map `a`, `b`, and `rc`
prereleases to Cargo's SemVer spelling. Version updates and wheel generation
remain separate operations.

**Reason:** `3.0.0a1` and `3.0.0-alpha.1` represent the same release in their
respective package systems. [update_version](../../../scripts/update_version)
maintains that correspondence across the
[workspace manifest](../../../crates/Cargo.toml),
[runtime settings](../../../crates/pyxel-core/src/settings.rs),
[project metadata](../../../python/pyproject.toml), and
[web runtime](../../../wasm/pyxel.js).
[Release automation](../../../.github/workflows/release.yml) uses the
`v`-prefixed tag. Literal equality across the two version syntaxes would reject
valid prereleases. The mapping does not promise every PEP 440 form, and editing
version strings does not rebuild the wheel.

## Executable Export

### Optional executable-export dependencies

**Decision:** Keep PyInstaller in the `app2exe` extra rather than requiring it
for ordinary Pyxel installation. The export command explains how to install
the extra when the tool is unavailable.

**Reason:** Only executable export needs the packager. The
[CLI](../../../python/pyxel/cli.py) invokes it for that operation, while
[project metadata](../../../python/pyproject.toml) keeps it optional.
[Development requirements](../../../python/requirements.txt) can include the
tool because contributors test packaging. Their contents do not define the
runtime dependency set.

### Executable-export workspaces

**Decision:** Give each executable export its own temporary workspace, including
the bootstrap script and PyInstaller build/spec files. Clean up that workspace,
leaving unrelated user files and the requested distribution output intact.

**Reason:** A shared work directory or deleting `build/` and a same-named spec
file in the caller's directory can affect another export or the user's work.
The [export command](../../../python/pyxel/cli.py) owns only its temporary files.
Preserve the requested app's filename in the bootstrap and data mapping;
resolving a symlink for filesystem access must not silently rename that packaged
input.

### Static dependency discovery for executable export

**Decision:** Discover imports from source syntax without executing the
application or imported packages. Include parent package initializers and
resolvable submodules, retaining the lexical local paths while using resolved
file identity to stop repeated source inspection.

**Reason:** [The import scanner](../../../python/pyxel/utils.py) supplies hidden
imports to [executable export](../../../python/pyxel/cli.py). Running a package
initializer just to discover its dependencies could start the application or
cause unrelated side effects. Parent initializers have their own imports even
when the source names only a leaf module. A `from` target can also be an
attribute, so it is not automatically another system module. Static discovery
does not promise to find arbitrary dynamically constructed imports. Its
visited-file identity serves a different purpose from
[preserving directory aliases during packaging](public-contract-python.md#directory-links-in-packaging-and-watching).
