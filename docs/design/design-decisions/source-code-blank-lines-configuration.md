# Configuration Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

Read with the [shared blank-line decisions](source-code-blank-lines.md).
Configuration records, shell stages, and Makefile content.

## Configuration blocks

**Decision:** Use one empty line between the recorded
[Makefile groups](source-code-structure-and-formatting.md#makefile-groups)
and between target definitions. Keep a target and its recipe together; commands
and continued argument lists within a recipe remain continuous. Keep the
opening instructions in one comment block, with comment-only separators between
its instruction groups. In the
[ignore and attribute files](source-code-structure-and-formatting.md#ignore-and-attribute-groups),
use one empty line between labeled groups, and none between a label, its entries,
and their attached explanations.

Use these boundaries in the remaining build, editor, and GitHub configuration:

| File family | Separation |
| --- | --- |
| Workspace/crate manifests and Python packaging TOML | One empty line between tables; keep a table's entries, attached comments, and array members continuous |
| Rust toolchain and formatter TOML | Keep the small single configuration block continuous, including its attached explanation |
| `.vscode/settings.json` and `web/package.json` | Keep each nested configuration continuous; braces and indentation identify its objects |
| GitHub workflows | One empty line between top-level sections, jobs, steps, and build-matrix records; keep each step's and matrix record's fields together |
| GitHub issue/discussion forms, routing, and funding | Keep the document structure continuous, including adjacent form fields; indentation and field markers identify the entries |

Blank lines in YAML literal or folded scalars belong to the embedded shell or
prose, not to the surrounding configuration layout. In workflow shell blocks,
keep generated-file contents such as the EGL `pkg-config` heredoc intact.

**Reason:** Tables, records, and workflow steps provide stable navigation units.
Blank lines between every key would fragment each unit. Workflow steps and
platform records are useful separate checkpoints in a build sequence, while a
form's nested field declarations already expose its structure without extra
gaps. Configuration separators must not alter paragraphs, scripts, or file
payloads stored inside values.

## Shell stages and configuration content

**Decision:** Within shell scripts and workflow `run` blocks, apply the
[processing-group distinctions](source-code-blank-lines.md#processing-groups).
Keep a command's continuations, arguments, pipelines, and immediate checks
continuous. Environment settings belong with the commands that consume them.
Separate independently implemented build and inspection work, retaining each
task's own setup and result check. Setup or cleanup around one delegated command
does not acquire separate paragraphs merely because it precedes or follows
the command.

In [environment setup](../../../scripts/setup_venv), use shell options;
working-directory/path preparation and Python preflight; environment recreation;
interpreter selection and dependency installation. Keep removal and recreation
together, and keep the platform-specific interpreter choice with the pip call.
The [development requirements](../../../python/requirements.txt) are one
continuous tool inventory under their attached explanation.

In [the example runner](../../../scripts/run_examples), the path inventory,
child-output setting, process-control state, command helpers, and three launch
groups have distinct roles. Separate standalone helpers with one blank line;
keep an attached explanation with its helper. The cleanup function and trap
registration stay together. Each background launch's PID, signal handling,
wait, restoration, and returned status form one lifecycle. Keep a displayed
command's argument collection together. The example loop, packaged-app loop,
and editor launch are separate workload groups.

In the [build workflow](../../../.github/workflows/build.yml), sysroot preparation
uses path publication; X11/XCB headers; Wayland/xkbcommon/libdecor headers;
OpenGL/EGL/GLES headers; audio headers; D-Bus headers; pkg-config file copying;
path normalization; EGL metadata; stub libraries. Each subsystem's checks,
copies, and local variables stay together. The inline `stub_lib` helper belongs
with its preparation and call inventory, without standalone-helper spacing.

The SDL2 build uses archive acquisition and verification; toolchain-file
construction; configuration/build/install; backend inspection; Wayland protocol
inspection; cleanup. Keep the pkg-config environment with configuration and
keep the build/install calls with that same build group. The Linux wheel step's
environment settings, temporary library bundle, build, and removal are one
packaging lifecycle. Unlike that local bundle removal, SDL2 cleanup follows
both artifact inspections and releases the source and build trees used by the
whole pipeline; give that pipeline completion its own boundary.
Bundle inspection has acquisition/admission followed by
backend checks: the located library and missing-library guard stay together.
Keep a verification loop, its failure accumulator, and final failure check
continuous. Short version-resolution, dependency installation, import checks,
release preparation, and publishing steps each remain one command group.

Makefile recipes follow the
[target and continuation boundaries](#configuration-blocks). Recipe lines and
continued commands are not interchangeable shell paragraphs. Blank-line review
does not split a logical command, change its shell scope, or turn heredoc data
into source separators. The EGL `pkg-config` heredoc and echoed CMake directives
are file contents; keep their field/directive inventories continuous.

The [discussion form](../../../.github/DISCUSSION_TEMPLATE/user-examples.yml)
keeps its English/Japanese introduction as one bilingual instruction block.
Its folded GIF instructions are one paragraph; physical wrapping does not
create separate steps. Issue-form descriptions likewise belong to their fields.
YAML scalar style, indentation, and paragraph boundaries belong to that content,
independently of workflow-step or form-field spacing.

**Reason:** Shell readers need to locate complete build and verification tasks,
while seeing the environment, guards, and cleanup that belong to each task.
Separating every invocation obscures that relationship. Make continuations and
YAML/heredoc payloads can give whitespace operational meaning, so their
boundaries follow the consuming format rather than the surrounding source.
