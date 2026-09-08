# Comments Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#comments)

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
generated docstrings in `python/pyxel/__init__.pyi`, whose descriptions come from
the [documentation sources](documentation.md#sources-of-generated-guides-and-stub-docstrings).

**Reason:** English provides a shared source-comment language. Source comments
explain local intent; public API descriptions have their own authoritative
sources and reach editor help through generation. Maintaining another set of
API prose inside implementation files would create competing descriptions.

A one-line group label uses sentence case without decorative banners or a terminal period.
Sentence comments use normal punctuation; a single sentence may omit its terminal
period. These are presentation conventions, separate from deciding whether a
comment supplies information the reader needs.

### Definition-group headings

**Decision:** Use a heading to name a consecutive group of definitions by its
shared responsibility. Put it before the group's first member, including that
member's attributes, decorators, or explanatory comment. Keep supporting
constants and helpers with the group they serve. A class, trait implementation,
module, or single clearly named operation does not need another label merely to
repeat its declaration. Nor does an operation and its private helpers need a
heading when their enclosing structure already identifies that responsibility.

Headings provide navigation, not lexical scope. A standalone constructor,
protocol method, registration function, or startup block does not need a closing
heading just because the preceding group has one. Conversely, when a new role
spans several definitions, do not rely on readers to reconstruct that group from
its implementation. Use the same level of responsibility for peer headings;
keep body-step labels inside the function they describe.

Keep accurate established labels. Use the public operation or subsystem's
terminology for a missing group, and reuse the label of the corresponding group
in its peers. Do not replace a valid label with a synonym, reorder definitions
to fit headings, or add a heading before every method.

**Reason:** Readers need landmarks for responsibilities spread across several
definitions. Treating a heading as an open scope creates labels whose only
purpose is to close another label; treating all declarations as self-explanatory
loses the navigation that function syntax does not supply. The groups below
record the selected vocabulary where language syntax alone does not settle it.
For example, `# Event handlers` names a role shared by methods;
`i += 1  # increment i` merely repeats one statement.

## Rust

### Python conversion and lifetime boundaries

**Decision:** Keep the [Python reentry](public-contract-rust-python.md#python-reentry-while-accessing-mutable-resources)
explanation at the shared conversion and sequence helpers in
[binding utils](../../../crates/pyxel-binding/src/utils.rs). Their comments identify
when Python can invalidate a resource index or access a borrowed resource;
individual forwarding methods do not repeat the general explanation.
Describe the `_pyxel_owner` reference where the ctypes view is created: it
retains the owner, without promising protection against buffer reallocation.

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

### Variables and Events in editor classes

**Decision:** Keep compact introductory lists of a component's relevant variables
and events in the [editor](../../../python/pyxel/editor/) and
[widget framework](../../../python/pyxel/editor/widgets/), including relevant inherited
members. Their purpose is to make the component understandable in that file.

**Reason:** `Widget.new_var` creates properties dynamically, `Widget.copy_var`
shares them, and event interfaces emerge from registration and dispatch.
Reconstructing that interface from implementation code repeatedly sends readers
to other files. The opening lists collect it in one place.

For example, [Button](../../../python/pyxel/editor/widgets/button.py) presents
`is_pressed_var` and `press`;
[EditorBase](../../../python/pyxel/editor/editor_base.py) presents `help_message_var`,
`undo (data)`, `redo (data)`, and `drop (filename)`. Relevance to the component, rather
than the location where a member was first implemented, determines the list's
usefulness. This does not require copying every common `Widget` facility into
every subclass.

The lists describe an interface; they are not repeated explanations of how the
framework implements it. Inheritance or repetition alone does not remove their
purpose.

Include `is_pressed_var` and `press` in `ImageButton` and `TextButton`, and
`is_checked_var`, `checked`, and `unchecked` in `ImageToggleButton`. These are
the controls callers instantiate; changing their drawing does not remove their
button interface. List `undo (data)` and `redo (data)` in all four concrete
editors, and `drop (filename)` in the image and tilemap editors that handle it.
The event list covers the component's integration points, not only events
emitted by its own methods. Common mouse, visibility, update, and draw events
remain described in `Widget` rather than repeated in every component.

### UI interface list notation and grouping

**Decision:** Put `# Variables:` before `# Events:` at the start of the class.
Indent each listed name after the comment marker, retain the `_var` suffix, and
write event arguments as `change (value)`. Use `-> value` for the transforming
`get` and `set` listeners in
[WidgetVar](../../../python/pyxel/editor/widgets/widget_var.py). Keep both
categories and write `none` when a category has no entries in the component's
interface summary.

The [blank-line decision](source-code-blank-lines-python.md#editor-interface-lists)
specifies the separators within and after the summary.

**Reason:** The names must match the properties and event strings used by
callers. Arguments describe what a listener receives; the return notation
distinguishes value transformation from notification. Explicit `none`
distinguishes an empty category from missing documentation, so readers need
not inspect the implementation to establish that absence. It applies to the
component-specific summary; common inherited `Widget` facilities remain listed
in `Widget`. The lists are an interface summary, not Python declarations or
another API reference.

Keep related coordinates together and preserve the common image-editing group
before the tilemap-specific group in
[CanvasPanel](../../../python/pyxel/editor/canvas_panel.py) and
[TilemapEditor](../../../python/pyxel/editor/tilemap_editor.py). Preserve readable
semantic order elsewhere; neither alphabetical order nor the order of variable
creation is a reason to rearrange these interface summaries.

For sound-editor views, list the forwarded editing inputs in the parent's
`speed_var`, `octave_var`, `note_var` order, omitting inputs the view does not
use, before playback state and help. `PianoKeyboard` puts `note_var` first:
it provides that value to the editor and other views, whereas `PianoRoll`
receives it. This exposes the control's output before its supporting inputs.
It does not impose an owned-before-inherited order on every widget; a viewer
needs its image or tilemap index before the focus within that resource.

### Method groups in the editor and widget framework

**Decision:** In editor components, use `# Public methods`, `# Helpers`,
`# Event handlers`, and `# Drawing` where they identify distinct method groups.
Use the framework's functional groups, such as `Geometry`, `Event listeners`,
and `Var binding`, for the broader responsibilities of
[Widget](../../../python/pyxel/editor/widgets/widget.py), and `Movement`,
`Editing`, and `Input processing` in
[FieldCursor](../../../python/pyxel/editor/field_cursor.py).
Do not add a label before a constructor, every property, or a lone draw callback
in a small button subclass merely to give every method a heading.

**Reason:** Python syntax identifies a method or property but does not identify
the role of a consecutive group. These labels provide navigation through a
class. A label adds little when the entire remaining class is already a single
short callback. Larger framework classes and small concrete controls need
different amounts of navigation, not identical heading inventories.

### Labels inside drawing and input routines

**Decision:** Retain short labels for visual parts assembled from primitive
drawing calls and for distinct input-operation groups. Examples include the
arrow and slider groups in
[ScrollBar](../../../python/pyxel/editor/widgets/scroll_bar.py), the grid and note
groups in [PianoRoll](../../../python/pyxel/editor/piano_roll.py), and bank versus
selection operations in
[CanvasPanel](../../../python/pyxel/editor/canvas_panel.py).
Do not label an individual assignment or call when its name and arguments
already convey the same information.

**Reason:** Coordinates, palette values, and primitive calls do not immediately
identify the visual object they compose. Naming that object helps the reader
find the relevant drawing work without reconstructing the picture. Input-group
labels similarly distinguish operations on different data scopes. These labels
name the purpose of a block; they do not justify narrating every drawing call,
repeating a clearly named helper, or expanding the comments into a detailed
design description.

### Groups in stubs and development tools

**Decision:** In the Cube stub, use functional API groups for the math types and
Node's hierarchy, motion, lifecycle, and drawing facilities. Keep short
constructors and protocol declarations together without labeling each one.
Node's drawing groups name the drawing operations, as do their binding groups;
the compact stub need not reproduce implementation-only helper sections.

In the capture tools, distinguish capture entry points from screenshot
comparison and shared capture helpers. In the document generator, use
`Expression helpers`, `Markdown render helpers`, and `Data lookup helpers` for
the evaluator's corresponding facilities, and keep `Document generators`,
`Shared helpers`, and `Markdown conversion helpers` at module scope. Repeat a
functional heading for a later non-contiguous group when needed; do not move
methods merely to consolidate headings.

**Reason:** These are the tasks a reader looks for across multiple definitions.
The stub exposes the API, the capture tools separate subprocess use from image
comparison, and the generators separate evaluation from output assembly. A
function's local explanation serves a different purpose from these group names.

## JavaScript

### Groups in shared helpers and applications

**Decision:** In `web/shared.js`, use `Page controls`, `HTML helpers`, `Data
transfer`, and `Page setup`. These distinguish DOM controls, markup fragments,
URL and binary transfer helpers, and page readiness and language setup. Keep
markup localization with the HTML helpers that use it.

In Code Maker, use `Project operations`, `Initialization`, `Project helpers`,
and `UI helpers`: user-facing project load/save entry points; editor, frame,
control, and input setup; project archive and transfer support; menu, modal,
focus, and feedback behavior. Initialization calls after the definitions form
the startup block and do not need another group heading.

In the Web runtime, keep the public API separate from initialization, screen
elements, Pyodide loading, error handling, file operations, input and startup,
virtual gamepad, and command execution. Keep custom-element classes together;
use `Custom element helpers` for their shared launch and registration helpers.
Small page scripts and test groups already identified by their declarations do
not need the application's heading inventory.

**Reason:** These files contain several facilities at the same declaration
level. Responsibility names let readers find a facility without reading every
function. The browser setup and project-transfer concerns are distinct from
each other even when both are implemented with ordinary JavaScript functions.
