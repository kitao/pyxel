# Python Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

Read with the [shared blank-line decisions](source-code-blank-lines.md).
These decisions apply to Python programs, tools, embedded code, and editor
construction. Tests and their support code also use the [test decisions](source-code-blank-lines-tests.md).

## Function bodies

**Decision:** Apply the shared [processing groups](source-code-blank-lines.md#processing-groups)
inside functions, loops, and branches.

Local definitions retain Python's definition spacing, including helpers that
use captured state. Their definitions belong near the work that uses them;
short lambdas remain part of their containing expression. Blank source lines
and empty strings emitted into generated content have different ownership.

**Reason:** Python's indentation shows nesting, while definition spacing
distinguishes a local helper from the processing around it. Captured state does
not change that role.

## Embedded Python and reference examples

**Decision:** Select spacing by the Python content's role. HTML or JavaScript
owns quoting, interpolation, and container indentation. Opening and closing
literal newlines are container boundaries, not processing groups.

| Content role | Blank-line placement |
| --- | --- |
| Short runtime bridge | Keep its immediate import/use sequence compact |
| Multi-stage bootstrap | Separate dependency categories and startup/reset stages under the shared processing criteria |
| Web guide program or executable HTML program example | Use one blank between dependencies, module-level definitions, and execution groups; use one between methods |
| Contextual API usage example | Keep a compact demonstration and its supporting context together; apply ordinary processing groups when it has several stages |
| Metadata field example | Keep the field inventory continuous |

The program convention applies to the [user guide](../../../web/user-guide/index.html),
[Cube guide](../../../web/user-guide/cube/index.html),
[Web usage guide](../../../web/web-usage/index.html), and
[script test](../../../web/showcase/examples/script-test.html).
The [base](../../../web/api-reference/api-reference.json) and
[Cube](../../../web/api-reference/cube/api-reference.json) API recipes assume
surrounding context; an import or small fixture does not make them full modules.

Generated Markdown follows its own formatter, including Ruff's two blank lines
around module-level Python definitions. Generate and format it from its source;
compare logical groups and Python content after that normalization. HTML fenced
examples retain the Python attribute's grouping. User-entered and dynamically
fetched programs retain their source's ownership; do not rewrite them at runtime.

**Reason:** Complete program examples need visible definitions and execution
groups, while short API recipes keep the demonstrated operation easy to follow.
Their published container and generation path explain the different definition
spacing; they do not justify different rules for ordinary processing.

## Editor interface lists

**Decision:** Keep each class's opening interface summary as one comment block.
Use a line containing only the indented `#` between `Variables` and `Events`,
and between the common image-editing and tilemap-specific variable groups in
`CanvasPanel` and `TilemapEditor`. Keep entries within each group consecutive.
Separate the complete summary from the following class member with one physical
blank line, including a class variable such as `Widget._mouse_capture_info`.
The [interface-list decision](source-code-comments.md#ui-interface-list-notation-and-grouping)
owns which categories and members are listed.

**Reason:** Comment-only separators expose the categories while keeping the
summary visibly distinct from executable class members. Physical blank lines
separate that summary from the implementation; they do not split it into
unrelated comments.

## State and editor construction

**Decision:** Keep compact state and declaration inventories together, including
`MouseCaptureInfo` and `FieldCursor`. Different units, types, prefixes, or
attached field explanations do not each create another group. Settings follow
the [editor and widget settings groups](source-code-structure-and-formatting.md#editor-and-widget-settings-groups).

In editor constructors, group base construction, stored state, and simple
variable/property wiring by responsibility. An assignment becoming `new_var`,
`copy_var`, or a variable listener does not itself create a boundary. Separate
substantial local-state, parent-interface, child-control, and own-event setup
when those are distinct portions of the construction.

Keep a control's construction with its help, listeners, and forwarded variables.
Related compact controls and surface assemblies can form one group. A substantial
control configuration has its own paragraph; a short specialization need not
separate its sole final subscription. Keep paired coordinates and corresponding
focus/size variables together. After substantial setup, separate application-loop
startup as the completion stage. Keep a minimal startup call sequence compact.

**Reason:** Readers need to locate what a widget owns, how it connects to its
parent and children, and which events it handles. Grouping by those roles
preserves the whole control setup without a rule for every constructor or a
blank before every variable, child, or listener.

## Output and drawing inventories

**Decision:** In document generators, keep construction of each labeled output
section together, including its separators and bookkeeping. Separate the
sections of a substantial document construction. Recognition variants for the
same output operation belong together; handlers for distinct constructs form
a dispatch under the shared criteria. Preserve generated-content whitespace
independently of source paragraphing.

In drawing code, group complete visual parts and layers. A shape's coordinates,
edges, and corner pixels belong to the same part even when different drawing
APIs produce them. Keep short symmetric component lists together. Separate
larger contour parts where their construction needs its own paragraph.

The [gamepad guide drawing](../../../scripts/generate_gamepad_images) follows
left-to-right control areas: cross, menu, action buttons. Cross and action-button
areas each have margin and corner groups; the menu has left-button and
right-button margin groups. Keep repeated rectangles within each group together.

**Reason:** Output sections and visual parts are what a maintainer locates to
change a generator. Their membership follows the produced content, not the
statement kind, drawing API, or coordinate name.
