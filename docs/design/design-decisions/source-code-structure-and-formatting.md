# Structure and Formatting Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

## Across Languages

### Definition order

**Decision:** Place high-level structures and public types before their
supporting free functions. Required declarations and order-dependent behavior
take precedence. The [Python example
layout](#utility-functions-and-classes-in-examples) gives module-level teaching
helpers a separate position.

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
time. Visibility and spelling do not identify that task. The [drawing
implementations](../../../crates/pyxel-core/src/image.rs) expose these operation
pairs without separating their implementation support. A reorder needs a
specific misplaced relationship; either order being legal is not such a reason.

### Configuration groups and ordering

**Decision:** Keep build manifests in identity, build-target, dependency, and
tool-setting groups. Cargo manifests introduce the workspace or package, then
library/benchmark targets, dependency kinds, and features where present. Python
packaging introduces the build system, project metadata, optional dependencies,
URLs and scripts, then Maturin settings and target specializations. Web package
metadata precedes scripts and development dependencies. Ignore and attribute
files group entries by the kind of product or asset they cover, with the group
label identifying a distinct source ownership and each explanation attached to
the affected entry.

Sort independent metadata, dependency, ignore, and attribute entries
alphabetically within each group; an attribute file puts the default text
treatment first and orders its asset groups alphabetically by kind. Schema
records, ordered arrays, command arguments, and workflow steps keep their
meaningful order. GitHub workflows introduce name, triggers, permissions,
environment, then jobs; shared anchor definitions precede their uses. Form
fields follow the submission task.

**Reason:** Alphabetical order makes independent entries predictable. It must
not hide dependencies or the intended sequence; a shared prefix alone does not
establish one sortable group, and one broad alphabetical list would hide the
distinction between project build products and local environment material.

### Local outputs and versioned deliverables

**Decision:** Ignore local caches, build outputs, environment files, and
assistant workspace material. Retain generated artifacts consumed directly
from the repository, including the web wheel, generated guides, and stylesheet.
Keep the root `/examples` shortcut separate from the packaged examples.

The root README and LICENSE own the package copies created by
[`make build`](../../../Makefile). Ignore those copies rather than maintaining
another editable authority. Git ignore rules neither remove tracked files nor
define wheel contents; package exclusions and artifact checks have their own
distribution role.

**Reason:** The [ignore rules](../../../.gitignore) keep local working material
out of source discovery. The [web runtime](../../../wasm/pyxel.js), web pages,
and documentation readers consume versioned generated files without building
them first. The examples under `python/pyxel/examples/` are teaching material
used by packaging, `copy_examples`, tests, and web pages; a root shortcut does
not justify ignoring that directory family.

### Makefile groups

**Decision:** In the [Makefile](../../../Makefile), definitions precede the
targets that use them, inputs precede the options and environment derived from
them, and common build options stay apart from target-specific additions.
Targets keep each workflow together: the default build and the maintenance
targets (cleaning, dependency updates, formatting) come first, then the native,
WASM, and web-page workflows. The `.PHONY` list uses the same target groups.

**Reason:** Definitions introduce paths and build inputs before the flags and
environment derived from them, then expose the commands that use them. Native
commands own the shared build process and WASM commands specialize it through
recursive Make calls, so keeping each workflow together makes that
correspondence visible. Alphabetizing variables would separate inputs from their
derivations; alphabetizing targets would scatter the workflow. The [opening
instructions](documentation.md#contributor-workflow-and-command-instructions)
have a separate reader-oriented sequence.

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

**Decision:** Group [`settings.rs`](../../../crates/pyxel-core/src/settings.rs)
by subsystem section, with one blank line between a section's subgroups, each
covering one nameable subject under the [inventory
decision](source-code-blank-lines.md#declaration-inventories). System defaults
put public startup behavior before internal coordination, resource files keep
each format's metadata together, graphics puts capacities before built-in
payloads, and audio puts timing before the data it drives. Within a subgroup,
inputs precede derived values, dimensions precede their payload, identifiers
follow their encoded numeric order, startup and capture defaults follow their
public parameter order, and bank counts and limits follow the object or
parameter family they describe.

**Reason:** Readers first find the subsystem, then the setting's role. These
orders make related declarations recognizable without reinterpreting
importance or alphabetizing their names at each review. They are selected
reading orders, not a claim that every other order is inherently unreadable.

## Python

### Component and framework members

**Decision:** Concrete editor controls present construction and exposed state,
ordinary operations and support, registered event handlers, then drawing, where
those groups exist. Keep visual-part helpers with drawing. The broader Widget
and FieldCursor facilities instead keep their responsibility groups together:
geometry, events, and variable binding; or movement, editing, and input
handling. Use the [comment group
vocabulary](source-code-comments.md#method-groups-in-the-editor-and-widget-framework)
for groups that need labels, without creating empty groups or moving methods to
consolidate labels.

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

**Reason:** Type help, reference navigation, and reviewing evidence are
different reading tasks. Alphabetizing tests or copying the reference
catalogue's order would not establish better coverage. Existing case and fixture
groups remain recognizable when the same contract needs another case.

### Editor and widget settings groups

**Decision:** In the [widget
settings](../../../python/pyxel/editor/widgets/settings.py), group settings by
component, shared widget settings first and then those of the button and input
controls, keeping a component's input behavior apart from its colors. In the
[editor settings](../../../python/pyxel/editor/settings.py), keep editor-wide
assets, dimensions, identifiers, limits, and text before component colors, with
one section per component. Within a state family, put its primary fill or frame
before its boundary, normal data before selected data, and a live playback
cursor before editing and selection cursors; components without a playback
cursor omit it. Separate each group with one blank line.

**Reason:** Input behavior and appearance are separate editing tasks, and a
component's colors are read together rather than by a shared prefix or an
alphabetical sort. These distinct reading tasks settle the groups; the subgroups
themselves follow the [inventory
decision](source-code-blank-lines.md#declaration-inventories).

### Utility functions and classes in examples

**Decision:** In Python examples, group module-level utilities after imports,
constants, and shared state, before the classes. Keep application startup code
last, including a function whose role is starting or switching applications.
Required declarations and order-dependent behavior still take precedence.

**Reason:** Small reusable operations such as the entity helpers in
[Shooter](../../../python/pyxel/examples/09_shooter.py) give readers the
vocabulary used by the classes. Keeping that utility group before the class
group avoids hiding shared operations after several hundred lines of game
behavior. The classes then form a continuous account of the program leading to
its startup.

This is the selected layout for teaching examples, not a claim that Python
requires one universal definition order. It does not require extracting new
helpers or moving behavior out of a class. The launcher's
[switch_app](../../../python/pyxel/examples/17_app_launcher.py) belongs at the
end because it selects and starts the application; resetting an existing game is
ordinary gameplay behavior, not application startup.

Published examples are also quoted in books with line numbers. Assess a change's
teaching or correctness benefit against that disruption, including changes to
order, blank lines, and comments that leave execution unchanged. A layout
preference alone does not justify revising an established example; a necessary
change still needs a carefully limited scope.

## Web

### Page definitions and ordered execution

**Decision:** Keep the runtime's host classes and public entry points before
their support groups, and a page's operations and construction before its local
helpers. Startup calls follow the bindings they need.

Keep HTML controls in their visual and keyboard-navigation order. Preserve
script dependencies, class inheritance, state initialization, and listener
registration. Keep CSS base, variant, state, and override relationships;
shader fragments retain the host's concatenation and declaration order.

**Reason:** These groups expose what the page does before its supporting
details, using the [selected group
labels](source-code-comments.md#groups-in-shared-helpers-and-applications).
Function hoisting permits some definition moves but does not protect
initialization or subscriptions. CSS cascade, shader availability, and DOM focus
order affect the result, so an alphabetical sort is not a layout-only operation.

### Localized page data

**Decision:** Put language declarations and shared UI metadata before page
collections. Group section-owned text, controls, and tables in the owning
page's section and subsection order, keeping shared labels and cross-section
tables together once; a renderer helper's declaration position does not change
the section that owns its data. Keep the language selector's English fallback
first, followed by the other established data codes in code order, and use the
same sequence for translation alternatives. Content arrays retain their
selected display, parameter, or encoded order; they are not independent
dictionary keys to alphabetize.

**Reason:** A maintainer adding an instruction should find its text and table in
the group of the page section that shows it, beside the surrounding
instructions. Grouping all headings first and appending their bodies elsewhere
separates one editing task. The pages own their presentation order; source
layout follows it without changing the rendered text.
