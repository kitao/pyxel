# Testing Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Testing policy](../design-policy.md#testing)

## Rust, Python, and JavaScript

### Division between `make test` and `make run`

**Decision:** Use the automated suites under `make test` for behavior with
mechanically checkable expectations, and running examples under `make run` for
appearance, sound, and interaction that require human judgment.

Use Rust tests for pure internal logic, Python tests for the public API,
reference regressions for screenshots and rendered audio, and running samples
for manual checks of look, sound, and feel.

The [Makefile](../../../Makefile) provides one automated entry point, `make
test`, for Python, Rust, and JavaScript. The current JavaScript tests run on
Node.js; calling them `test-wasm` would imply execution of the web runtime they
do not perform. `make run` installs the current native package, then runs the
[startup check](../../../scripts/check_window_startup), which opens a window in
every screen mode, draws, exits, and reports the installed build; the automated
suites run headless and never create a GL context. It then uses the [example
runner](../../../scripts/run_examples) for native examples, bundled apps, and
the editor. `make run-wasm` builds the web wheel and serves the web runtime
locally through [start_showcase](../../../scripts/start_showcase), which also
serves the working tree with the tracked wheel for page checks. Those two
commands distinguish execution environments, not programming languages.

Archive creation, startup paths, watcher restarts, and executable export have
automated coverage in the [CLI tests](../../../python/tests/test_cli.py). The
app2exe tests build and launch executables to check resources, imports, and exit
status; routine creation and startup do not also require a manual checklist.
Visual, audible, or platform behavior outside those assertions still needs its
own observation. A launcher screenshot does not cover playing every bundled app.

A Cube draw call outside an active drawing context checks the accepted call
and no-op behavior, not rendered geometry. Keep those assertions distinct from
renderer and reference tests.

**Reason:** A numerical boundary, list operation, or resource round trip can be
checked unattended. Whether an animation looks right, an interaction feels
usable, or a sound is satisfactory needs observation. Passing image or audio
reference comparisons establishes the selected output's correspondence, not the
quality of the reference or all behavior outside the captured case.

### Screenshot capture and actual launch coverage

**Decision:** Capture examples, bundled apps, and editor screens in fresh
headless subprocesses with fixed seeds and explicit frame/input plans. The
[capture runner](../../../python/tests/_runner.py) intercepts `run` and `show`
to drive those plans; the `flip` example uses its own loop. Compare the captured
PNG bytes with the committed references without updating them during ordinary
verification.
Give each capture subprocess a timeout so a stalled example cannot block the
suite; this bounds the harness wait, not the program's expected performance.

**Reason:** Pyxel initialization belongs to the process, while controlled frames
and inputs make a reference comparison repeatable. The harness deliberately
controls launch and timing, so these captures do not establish normal CLI
startup, reset, or interactive responsiveness. The
[packaged-app execution tests](../../../python/tests/test_pyxapp_execution.py)
and [executable-export tests](../../../python/tests/test_cli.py) exercise those
separate launch paths. Updating a reference records a new expectation; it does
not establish that the new image is correct.

### Current native package and target-specific checks

**Decision:** Build and install the current native package before running the
Python, Rust, and web suites through `make test`. Keep native and Emscripten
checks separate, using the selected development environment consistently.

**Reason:** Editing Rust source does not update the extension imported by Python
tests. The [build/install/test dependencies](../../../Makefile) prevent those
tests from ordinarily exercising an older installed extension. Native and web
builds select different code, dependencies, and linking options; checking one
target does not establish the other. These routes do not replace browser
interaction or device checks.

### Extensionless Python tools

**Decision:** Include executable Python scripts in both formatting and linting
even when their names have no `.py` suffix. Share the script-family selection
between the two commands; keep Bash scripts outside the Python tool inputs.

**Reason:** The documentation and release tools under `scripts/` affect
distributed output. Suffix-only discovery would omit them.
[PYTHON_SCRIPTS](../../../Makefile) identifies the current Python tools by their
interpreter declaration and supplies them to both Ruff commands. Check that
selection against the actual script inventory when adding tools; the selector
does not prove that every future shebang form is covered.

### Repository utility scripts

**Decision:** Verify the simple environment bootstrap in
[setup_venv](../../../scripts/setup_venv) by running it in an isolated setup
location. Do not maintain a simulated Python installation and virtual
environment merely to assert that this command sequence was issued. Keep focused
automated tests for the transformations in documentation generators, version
updates, wheel validation and installation, and the example runner's failure
propagation.

**Reason:** Those tests can detect a wrong generated value, stale package,
misdirected update, or hidden command failure. A large imitation of an external
tool mainly tests the imitation and does not establish that setup works on
another operating system. A script does not need a dedicated test file simply
because it exists, nor should it be split into helpers only to enable mocking.

### Interface and documentation expectations

**Decision:** Use executable Python API cases and [type-checker
cases](../../../python/tests/test_stub_compatibility.py) to verify the behavior
and type help callers use. Check that the signatures in the API reference data
agree with the stub signatures through an automated comparison; a mismatch is a
data defect, not a wording question for reviewers. Keep [generator
tests](../../../python/tests/test_generate_docs.py) focused on meaningful
conversion results, such as preserved link pairs and literal text. Do not freeze
the entire API as a generated inventory or pin editorial wording, document
counts, workflow text, or promotional image layout as substitutes for checking
their meaning or actual output.

**Reason:** A large source snapshot can reject a harmless representation change
while missing a broken setter or accepted call. Conversely, exact exception
messages and binary reference outputs have their own explicit contracts; this
choice does not weaken those comparisons. Assertions derived from extracted
source can be useful when they execute that source against independent expected
behavior, but source text alone does not establish another platform's execution.

### Numerical tolerances and audio timing

**Decision:** Use tolerances for floating-point geometry when the calculation's
precision or algorithm justifies them. State the numerical basis once where a
tolerance is defined for reuse, at a constant, a helper, or the first of
adjacent helpers sharing the value; an inline tolerance beside the compared
quantities needs no separate statement. Record the cause of nondeterminism at
the test or assertion helper that accepts alternative outcomes. An envelope test
must not accept either endpoint merely to pass.

**Reason:** Accepting either envelope endpoint would hide an unresolved
expectation. In
contrast, `play_pos()` queried during live audio playback can depend on callback
timing, so alternative results may be valid when that timing is the reason.
A headless calculation does not gain the same allowance merely because it
belongs to the audio subsystem.

Live playback range checks establish that playback remains active within its
loop after seeking. They do not establish the exact seek offset; detached
[Channel cases](../../../python/tests/test_channel.py) and the
[core channel tests](../../../crates/pyxel-core/src/channel.rs) cover that
calculation without callback timing.

### Audio callback concurrency

**Decision:** Exercise callback-related playback in a subprocess with a timeout,
using SDL's dummy audio device for unattended coverage, as in
[play-and-stop coverage](../../../python/tests/test_audio.py).

**Reason:** The audio callback can contend with the main thread for resources.
A timeout confines a deadlock to the child process; same-thread conversion tests
cannot exercise that contention. Combine this coverage with inspection of lock
ordering. A successful run does not prove every thread schedule or sound quality
on a physical device.
