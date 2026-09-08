# Structure and Formatting Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

## Across Languages

### Definition order

**Decision:** Place high-level structures and public types before their supporting
free functions. Required declarations and order-dependent behavior take precedence.
The [Python example layout](#utility-functions-and-classes-in-examples) gives
module-level teaching helpers a separate position.

**Reason:** This layout presents the main structures before their implementation
support. It is the selected reading order, not a consequence of syntax or a
claim that every alternative is unreadable. A different layout needs a scoped
reason; moving definitions alone does not improve the code.

### Related operations and execution order

**Decision:** Keep operations of one responsibility together, including their
local helpers. Do not split a responsibility merely to put all public members
before private members. Within a matched family, keep a getter beside its
setter, a filled shape before its outline, and axis variants in X, Y, Z order.
The family's parameter and field order follows the interface or representation
it implements; declaration order does not authorize changing that contract.

Keep execution sequences distinct from definition layout. Initialization,
registration, callbacks, resource lifecycles, and ordered data retain their
required sequence. A dependency-sorted list of every function is not the reading
order: shared helpers can serve several entry points.

**Reason:** Readers compare corresponding operations and follow one task at a
time. Visibility and spelling do not identify that task. The
[drawing implementations](../../../crates/pyxel-core/src/image.rs) and
[binding properties](../../../crates/pyxel-binding/src/cube/) expose these
operation pairs without separating their implementation support. A reorder
needs a specific misplaced relationship; either order being legal is not such
a reason.

### Configuration groups and ordering

**Decision:** Keep build manifests in identity, build-target, dependency, and
tool-setting groups. Cargo manifests introduce the workspace or package, then
library/benchmark targets, dependency kinds, and features where present. Python
packaging introduces the build system, project metadata, optional dependencies,
URLs and scripts, then Maturin settings and target specializations. Web package
metadata precedes scripts and development dependencies.

Sort independent metadata and dependency entries alphabetically within each
group. Schema records, ordered arrays, command arguments, and workflow steps
keep their meaningful order. GitHub workflows introduce name, triggers,
permissions, environment, then jobs; shared anchor definitions precede their
uses. Form fields follow the submission task.

**Reason:** Alphabetical order makes independent entries predictable. It must
not hide dependencies or the intended sequence. The
[settings groups](#grouping-and-order-in-settingsrs) preserve those distinctions;
a shared prefix alone does not establish one sortable group.

### Local outputs and versioned deliverables

**Decision:** Ignore local caches, build outputs, environment files, and
assistant workspace material. Retain generated artifacts consumed directly
from the repository, including the Web wheel, generated guides, and stylesheet.
Keep the root `/examples` shortcut separate from the packaged examples.

The root README and LICENSE own the package copies created by
[make build](../../../Makefile). Ignore those copies rather than maintaining
another editable authority. Git ignore rules neither remove tracked files nor
define wheel contents; package exclusions and artifact checks have their own
distribution role.

**Reason:** The [ignore rules](../../../.gitignore) keep local working material
out of source discovery. The [Web loader](../../../wasm/pyxel.js), Web pages,
and documentation readers consume versioned generated files without building
them first. The examples under `python/pyxel/examples/` are teaching material
used by packaging, `copy_examples`, tests, and Web pages; a root shortcut does
not justify ignoring that directory family.

### Makefile groups

**Decision:** Keep the [Makefile](../../../Makefile) definitions in these groups:
project directories; extensionless Python script discovery; build targets and
reproducibility settings; WASM path remapping; build options; tool options;
PyO3 environment; declared targets. Within build options, keep common options,
WASM-specific additions, and native-platform SDL2 selection as separate groups.
Within the PyO3 conditional, separate computing the Python environment from
exporting it to the applicable targets.

Keep the target sequence: default build; cleaning; dependency updates;
formatting; native lint/build/install/test/run; WASM clean/lint/build/run;
Web-page generation. The `.PHONY` list uses the same target groups.

**Reason:** Definitions introduce paths and build inputs before the flags and
environment derived from them, then expose the commands that use them. Native
commands own the shared build process; WASM commands specialize it through
recursive Make calls. Keeping each workflow together makes that correspondence
visible. Alphabetizing variables across these groups would separate inputs from
their derivations; alphabetizing targets would scatter the workflow. The
[opening instructions](documentation.md#contributor-workflow-and-command-instructions)
have a separate reader-oriented sequence.

### Ignore and attribute groups

**Decision:** Keep [.gitignore](../../../.gitignore) in this sequence: Python
outputs; package copies; Rust outputs; Node outputs; environment files;
AI-tool workspace files; local shortcuts; OS files. Keep entries within each
group together and alphabetically ordered. Package copies follow Python outputs
because they are produced for the Python package, with their distinct source
ownership identified by the group label.

In [.gitattributes](../../../.gitattributes), keep the default text treatment
first, then asset groups in this order: archives, audio, fonts, images, models,
Pyxel assets. Keep each group's extensions alphabetical and its explanation
attached to the affected entry. BDF and palette explanations do not split their
font and Pyxel-asset groups.

**Reason:** Ignore groups distinguish project build products from local
environment and workspace material; one broad alphabetical list would hide
that distinction. Attribute groups let readers find a file's treatment by asset
kind. Text exceptions belong beside their related formats, where readers can
see why they differ, without turning each exception into another section.

### Text and binary asset representation

**Decision:** Treat BDF fonts and `.pyxpal` palettes as text. Give BDF explicit
LF endings. Keep encoded image, audio, font, model, and archive payloads binary
in [.gitattributes](../../../.gitattributes).

**Reason:** The [font reader](../../../crates/pyxel-core/src/font.rs) consumes
line-oriented BDF keywords and bitmap rows. The
[palette reader and writer](../../../crates/pyxel-core/src/resource.rs) use
hexadecimal color lines. Text diffs expose meaningful changes in these formats;
text normalization would corrupt encoded binary payloads. BDF's explicit LF
choice does not impose that setting on every text asset. Text representation
and direct style-review scope are separate questions, as are binary
representation and source/artifact correspondence.

## Rust

### Types, implementations, and operation kernels

**Decision:** Keep a type's supporting field types and ownership aliases with
its declaration, followed by its implementation. Keep distinct responsibility
groups within an implementation together. Put an allocating Cube operation
immediately before its corresponding `_value` kernel, retaining attributes and
comments with the declaration they describe.

Keep module declarations and re-exports alphabetical within their existing
dependency groups. Macro declarations needed by later modules precede those
consumers; conditional declarations stay with their alternative or affected
group. Unit-test modules follow the implementation they exercise.

**Reason:** The type and its representation form one reading unit. Small field
types can explain that representation before its owner; moving every private
type below every public type would break the unit. The
[Cube value pairs](source-code-naming.md#shared-handles-and-value-kernels)
then show the callable operation and its calculation together. The
[crate root](../../../crates/pyxel-core/src/lib.rs) introduces shared macros
before the modules that use them, while ordinary module lookup remains
predictable.

### Resource record conversions

**Decision:** In the image, tilemap, sound, and music records in
[resource data](../../../crates/pyxel-core/src/resource_data.rs), put the
conversion from a runtime object first, the conversion back second, then the
record's validation method where present. Keep the record families in image,
tilemap, sound, music order, matching the resource banks.

**Reason:** Capturing and reconstructing a resource are the record's paired
operations. Validation is a separate preflight performed by the aggregate
resource loader before it publishes any selected bank. This placement exposes
the same operations in the same order without suggesting that one conversion
implicitly validates. It does not put every validator last or impose this
record layout on the aggregate loader.

### Grouping and order in settings.rs

**Decision:** Keep the main sections of
[`settings.rs`](../../../crates/pyxel-core/src/settings.rs) in this order:
system defaults, resource files, graphics, audio. Use the subgroup sequences
below, with one blank line between subgroups.

| Section | Subgroups, in reading order |
| --- | --- |
| System | Version and base directory; startup title/FPS/quit key; capture scale/duration; window sizing/background/screen modes; frame-delay measurement; icon dimensions/options/data; window/watch/reset coordination |
| Resource files | App extension and startup marker; resource extension/archive name/format version; palette extension |
| Graphics | Color counts; image count/size; tilemap count/size; tile size/shift/mask; palette data and color identifiers; cursor dimensions/data; font character range/layout/data |
| Audio | Clock/sample format and derived clocks per sample; buffering/render step; gain representation; control/interpolation and musical timing; vibrato; bank counts; channel/sound defaults; tone identifiers; effect identifiers; volume/effect limits; default tone data |

Within these groups, inputs precede derived values, dimensions precede their
payload, and identifiers follow their encoded numeric order. Startup and capture
defaults follow their public parameter order. Audio bank counts follow the object
family `Channel`, `Tone`, `Sound`, `Music`, as do the
[resource initializers](../../../crates/pyxel-core/src/pyxel.rs).
Control timing presents voice rate, note interpolation, ticks per quarter note,
then ticks per second; vibrato presents period then depth. Volume precedes effect
in the limits, matching `Sound.set`'s field order.

**Reason:** Readers first find the subsystem, then the setting's role. Public
startup behavior precedes internal coordination; each resource format keeps its
metadata together; graphics capacities precede built-in payloads; audio timing
precedes the sound data it drives. Parameter, representation, and object-family
orders make related declarations recognizable without reinterpreting importance
or alphabetizing their names at each review. These are selected reading orders,
not a claim that every other order is inherently unreadable.

## Python

### Component and framework members

**Decision:** Concrete editor controls present construction and exposed state,
ordinary operations and support, registered event handlers, then drawing, where
those groups exist. Keep visual-part helpers with drawing. The broader Widget
and FieldCursor facilities instead keep their responsibility groups together:
geometry, events, and variable binding; or movement, editing, and input handling.
Use the [comment group vocabulary](source-code-comments.md) for groups that need
labels, without creating empty groups or moving methods to consolidate labels.

**Reason:** A control reader learns its interface, interaction, and appearance.
A framework reader needs the entry points and support for one facility together.
Putting every helper at the end would split that facility. Child construction
and event registration also affect traversal and input handling, so this member
layout does not reorder statements inside constructors.

### Stubs and test groups

**Decision:** Stubs introduce the type and data vocabulary, then keep API
operations in functional groups. Reference pages can introduce those operations
in a user-task order. Preserve parameter positions and paired properties across
the representations; do not impose identical whole-file declaration order.

Tests keep their case families and supporting fixtures together, with the
declarations needed by decorators, defaults, and parametrization available
before use. Keep class and fixture setup separate from the order in which
test cases are collected or executed.

**Reason:** Type help, reference navigation, and reviewing evidence are different
reading tasks. Alphabetizing tests or copying the reference catalogue's order
would not establish better coverage. Existing case and fixture groups remain
recognizable when the same contract needs another case.

### Editor and widget settings groups

**Decision:** In [widget settings](../../../python/pyxel/editor/widgets/settings.py),
use this sequence: hold/repeat timing and click time/distance;
panel/background/shadow colors; button press duration;
enabled/disabled/pressed button colors and text color; input text/field colors.
Separate each group with one blank line.

In [editor settings](../../../python/pyxel/editor/settings.py), keep the section
sequence: editor image, app dimensions, tool identifiers, sound/music field
lengths, text colors, panel focus/selection, piano keyboard, piano roll, octave
bar, sound field, music field. Use these member groups within the color sections:

| Section | Groups and member order |
| --- | --- |
| Text | Label, then help message |
| Panel | Focus fill/border; selection frame/border |
| Piano keyboard | Rest, then playback |
| Piano roll | Playback/edit/selection cursors; background; note/rest data |
| Octave bar | Background, then bar |
| Sound field | Normal/selected data; edit/selection cursors |
| Music field | Background; normal/selected sound data; playback/edit/selection cursors |

**Reason:** Widget input behavior and appearance are separate editing tasks.
Editor-wide assets, dimensions, identifiers, limits, and text precede component
colors. The component order follows the editor's keyboard/roll/octave controls
and sound/music fields. Within a state family, put its primary fill or frame
before its boundary, and normal data before selected data. Cursor groups expose
live playback first, then editing and selection; components without a playback
cursor omit it. The piano roll emphasizes its moving cursors, whereas the compact
sound and music fields present their data first. These distinct reading tasks
settle the order; a shared prefix or alphabetical sort does not.

### Utility functions and classes in examples

**Decision:** In Python examples, group module-level utilities after imports,
constants, and shared state, before the classes. Keep application startup code
last, including a function whose role is starting or switching applications.
Required declarations and order-dependent behavior still take precedence.

**Reason:** Small reusable operations such as the entity helpers in
[Shooter](../../../python/pyxel/examples/09_shooter.py) give readers the vocabulary
used by the classes. Keeping that utility group before the class group avoids
hiding shared operations after several hundred lines of game behavior. The
classes then form a continuous account of the program leading to its startup.
Larger groups, such as the figure-building functions in
[Offscreen](../../../python/pyxel/examples/11_offscreen.py), remain together and
can be read separately from the application loop.

This is the selected layout for teaching examples, not a claim that Python
requires one universal definition order. It does not require extracting new
helpers or moving behavior out of a class. The launcher's
[switch_app](../../../python/pyxel/examples/17_app_launcher.py) belongs at the end
because it selects and starts the application; resetting an existing game is
ordinary gameplay behavior, not application startup.

Published examples are also quoted in books with line numbers. Assess a change's
teaching or correctness benefit against that disruption, including changes to
order, blank lines, and comments that leave execution unchanged. A layout
preference alone does not justify revising an established example; a necessary
change still needs a carefully limited scope.

## Web

### Page definitions and ordered execution

**Decision:** Keep the runtime's host classes and public entry points before
their support groups. Shared page helpers use page controls, HTML helpers,
data transfer, then page setup. Code Maker uses project operations,
initialization, project helpers, then UI helpers. Smaller pages present page
construction and text updates before their local rendering helpers. Startup
calls follow the bindings they need.

Keep HTML controls in their visual and keyboard-navigation order. Preserve
script dependencies, class inheritance, state initialization, and listener
registration. Keep CSS base, variant, state, and override relationships;
shader fragments retain the host's concatenation and declaration order.

**Reason:** These groups expose what the page does before its supporting
details, using the [selected group labels](source-code-comments.md). Function
hoisting permits some definition moves but does not protect initialization
or subscriptions. CSS cascade, shader availability, and DOM focus order affect
the result, so an alphabetical sort is not a layout-only operation.

### Localized page data

**Decision:** Put language declarations and shared UI metadata before page
collections. Group section-owned text, controls, and tables in the owning
page's section and subsection order. Keep shared labels and cross-section
tables together once. The editor manual's shared tables precede image,
tilemap, sound, then music data; a renderer helper's declaration position does
not change the section that owns its data.

Keep the language selector's English fallback first, followed by the other
established data codes in code order. Translation alternatives use that same
sequence. Content arrays retain their selected display, parameter, or encoded
order; they are not independent dictionary keys to alphabetize.

**Reason:** A maintainer adding a piano-roll instruction should find its text
and table in the sound-editor group, beside the surrounding instructions.
Grouping all headings first and appending their bodies elsewhere separates
one editing task. The [editor manual](../../../web/editor-manual/index.html),
[Code Maker manual](../../../web/code-maker/manual.html), and
[launcher form](../../../web/launcher/url-builder.html) own their presentation
order. Source-map layout follows it without changing the rendered text or
reordering shared translation alternatives.
