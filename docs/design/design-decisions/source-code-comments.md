# Comment Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code
policy](../design-policy.md#comments)

## Across Languages

### Comment ownership

**Decision:** Apply prose conventions to explanatory comments authored for this
code. Before changing comment-shaped text, check whether a tool, application,
generator, or external notice owns its form. Keep required directives, parsed
metadata, and attribution or license notices in their required form; their
necessity is a separate question from prose clarity. Review embedded code in
its own language, and change generated explanations at their documented source.

**Reason:** Comment delimiters do not establish that text is freely editable
prose. Such text can select execution, control checks, carry application data,
or preserve an external notice.

### Comment language and forms

**Decision:** Write comments in English. Use ordinary comments rather than Rust
documentation comments, Python docstrings, or JSDoc blocks. The exception is the
generated docstrings in `python/pyxel/__init__.pyi`, whose descriptions come
from the [documentation
sources](documentation.md#sources-of-generated-guides-and-stub-docstrings).

A one-line group label uses sentence case without decorative banners or a
terminal period. Sentence comments use normal punctuation; a single sentence
may omit its terminal period. These are presentation conventions, separate
from deciding whether a comment supplies information the reader needs.

**Reason:** English provides a shared source-comment language. Source comments
explain local intent; public API descriptions have their own authoritative
sources and reach editor help through generation. Maintaining another set of
API prose inside implementation files would create competing descriptions.

### Definition-group headings

**Decision:** Use a heading to name a consecutive group of definitions by its
shared responsibility. Put it before the group's first member, including that
member's attributes, decorators, or explanatory comment. A comment that explains
the whole group follows the heading directly; the [blank lines after the
heading](source-code-blank-lines.md#comments-at-group-boundaries) follow that
block. Keep supporting constants and helpers with the group they serve. A class,
trait implementation, module, or single clearly named operation does not need
another label merely to repeat its declaration, and an operation with its
private helpers needs no heading when the enclosing structure already identifies
that responsibility.

Headings provide navigation, not lexical scope. A standalone constructor,
protocol method, registration function, or startup block does not need a closing
heading just because the preceding group has one. Conversely, when a new role
spans several definitions, do not rely on readers to reconstruct that group from
its implementation. Use the same level of responsibility for peer headings, keep
body-step labels inside the function they describe, and repeat a functional
heading for a later non-contiguous group rather than moving definitions to
consolidate headings.

Keep accurate established labels. Use the public operation or subsystem's
terminology for a missing group, and reuse the label of the corresponding group
in its peers. Do not replace a valid label with a synonym, reorder definitions
to fit headings, or add a heading before every method. Small page scripts and
test groups already identified by their declarations do not need a heading
inventory.

**Reason:** Readers need landmarks for responsibilities spread across several
definitions. Treating a heading as an open scope creates labels whose only
purpose is to close another label; treating all declarations as self-explanatory
loses the navigation that function syntax does not supply. `# Event handlers`
names a role shared by methods; `i += 1  # increment i` merely repeats one
statement.

## Rust

### Python conversion and lifetime boundaries

**Decision:** Keep the [Python
reentry](public-contract-rust-python.md#python-reentry-while-accessing-mutable-resources)
explanation at the shared conversion and sequence helpers in [binding
utils](../../../crates/pyxel-binding/src/utils.rs). Their comments identify when
Python can invalidate a resource index or access a borrowed resource; individual
forwarding methods do not repeat the general explanation. Describe the
`_pyxel_owner` reference where the ctypes view is created: it retains the owner,
without promising protection against buffer reallocation.

**Reason:** Conversion and allocation can execute Python code despite looking
like ordinary Rust operations. A ctypes view made from an address likewise
does not establish the allocation's lifetime. These hidden boundaries justify
short owner comments; an explanation of each conversion or accessor does not.

### Numerical and geometry contracts

**Decision:** Explain numerical meaning at its owner: units and sentinels at
their conversion or definition, rounding and depth biases at the constants,
and geometry bounds at the buffer or primitive they constrain. Keep algorithm
and coordinate conventions at the implementing function. Do not repeat the
same derivation at callers or tests; fixture-specific explanations remain local.

**Reason:** A numeric expression exposes the calculation but can leave its unit,
rounding convention, or geometric meaning implicit. Place that explanation
where the representation is introduced, so callers can rely on it without
repeating its derivation. A fixture can need a different explanation of why its
chosen values exercise the tested property.

## Python

### UI interface list notation and grouping

**Decision:** Each component class in the
[editor](../../../python/pyxel/editor/) and [widget
framework](../../../python/pyxel/editor/widgets/) opens with a compact summary
of its interface: `# Variables:` followed by `# Events:`, with each listed name
indented after the comment marker. Retain the `_var` suffix, write event
arguments as `change (value)`, and use `-> value` for the transforming `get` and
`set` listeners of
[WidgetVar](../../../python/pyxel/editor/widgets/widget_var.py). Keep both
categories and write `none` when a category has no entries.

List the members relevant to using the component in that file, including
inherited members that form its interface, such as `is_pressed_var` and `press`
on the concrete buttons, `undo (data)` and `redo (data)` on the editors, and
`drop (filename)` on the editors that handle it; the event list covers the
component's integration points, not only events emitted by its own methods.
Relevance to the component, not the class where a member was first implemented,
decides the list; common mouse, visibility, update, and draw facilities remain
described in `Widget` rather than repeated in every component. Keep related
coordinates together and preserve a readable semantic order: a control's output
before its supporting inputs, and common groups before specialized groups. This
does not impose an owned-before-inherited order on every widget; a viewer lists
its image or tilemap index before the focus within that resource. Neither
alphabetical order nor the order of variable creation is a reason to rearrange a
summary. The [blank-line decision](source-code-blank-lines.md#python) specifies
the separators within and after the summary.

**Reason:** `Widget.new_var` creates properties dynamically, `Widget.copy_var`
shares them, and event interfaces emerge from registration and dispatch, so
reconstructing an interface from implementation code repeatedly sends readers
to other files. The names must match the properties and event strings callers
use; arguments describe what a listener receives, the return notation
distinguishes transformation from notification, and an explicit `none`
distinguishes an empty category from missing documentation. The lists are an
interface summary, not Python declarations or another API reference.

### Method groups in the editor and widget framework

**Decision:** In editor components, use `# Public methods`, `# Helpers`, `#
Event handlers`, and `# Drawing` where they identify distinct method groups.
Broader framework classes use their functional responsibilities instead, such as
`Geometry`, `Event listeners`, and `Var binding` in
[Widget](../../../python/pyxel/editor/widgets/widget.py), or `Movement`,
`Editing`, and `Input processing` in
[FieldCursor](../../../python/pyxel/editor/field_cursor.py). Do not add a label
before a constructor, every property, or a lone draw callback in a small
subclass merely to give every method a heading.

**Reason:** Python syntax identifies a method or property but does not identify
the role of a consecutive group. These labels provide navigation through a
class; larger framework classes and small concrete controls need different
amounts of navigation, not identical heading inventories.

### Labels inside drawing and input routines

**Decision:** Retain short labels for visual parts assembled from primitive
drawing calls and for distinct input-operation groups, such as the grid and
note groups in [PianoRoll](../../../python/pyxel/editor/piano_roll.py). Do not
label an individual assignment or call when its name and arguments already
convey the same information.

**Reason:** Coordinates, palette values, and primitive calls do not immediately
identify the visual object they compose, and input-group labels distinguish
operations on different data scopes. These labels name the purpose of a block;
they do not justify narrating every drawing call or expanding the comments into
a design description.

### Groups in stubs and development tools

**Decision:** In the Cube stub, use functional API groups for the math types and
for Node's hierarchy, motion, lifecycle, and drawing facilities, keeping short
constructors and protocol declarations together without labeling each one; the
compact stub need not reproduce implementation-only helper sections. In the
development tools, distinguish facilities a reader looks for across several
definitions, such as capture entry points from screenshot comparison, or the
document generator's evaluation helpers from its output assembly.

**Reason:** The stub exposes the API, the capture tools separate subprocess use
from image comparison, and the generators separate evaluation from output
assembly. A function's local explanation serves a different purpose from these
group names.

## JavaScript

### Groups in shared helpers and applications

**Decision:** Where a web source contains several facilities at the same
declaration level, use responsibility headings such as `Page controls`, `HTML
helpers`, `Data transfer`, and `Page setup` in
[shared.js](../../../web/shared.js), and keep markup localization with the HTML
helpers that use it. In the web runtime, keep the public API apart from its
internal facilities, and keep the custom-element classes together with their
shared launch and registration helpers. Initialization calls after the
definitions form the startup block and do not need another heading.

**Reason:** Responsibility names let readers find a facility without reading
every function. Page setup and data-transfer concerns are distinct from each
other even when both are implemented with ordinary JavaScript functions.
