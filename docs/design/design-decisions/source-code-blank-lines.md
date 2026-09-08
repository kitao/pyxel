# Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

These decisions specify where a blank line belongs and where it does not,
from the role and relationship of the surrounding code. The same conditions
apply whether that boundary currently has a blank line or not.

Read these shared decisions together with the subject files selected through
the [audit file index](../design-audit.md#decision-records). Embedded content and
tests can require a subject file in addition to the containing language.

## Formatting ownership

**Decision:** Use the formatting targets in [make format](../../../Makefile).
Rustfmt owns Rust formatting, Ruff owns Python sources, stubs, and the explicitly
selected extensionless Python scripts, and Prettier owns the selected Web/WASM
and configuration files. Ruff also formats Python fenced blocks in Markdown,
including the output of [generate_docs](../../../scripts/generate_docs).
Keep their language-specific spacing: ordinary Python
method bodies and consecutive one-line stub declarations need not have the same
separation. Files outside those targets do not acquire a formatter merely
because their extension is supported by one.

These tools settle indentation, wrapping, quoting, and the blank lines they
normalize. Authored grouping remains a separate choice wherever the formatter
preserves both a separated and an unseparated form. Between authored groups,
use one blank line unless the formatter prescribes otherwise; keep the members
of each group together. The
[settings decisions](source-code-structure-and-formatting.md#grouping-and-order-in-settingsrs)
specify their groups, including subgroups within a section. A section heading
does not turn everything below it into one group.

Treat blank lines inside strings, embedded programs, and document content
according to that content's role. For example, the Python in
[Code Maker's screen](../../../web/code-maker/pyxel-screen.html) is a program
inside an HTML attribute. Formatting the containing HTML does not establish its
Python grouping. Keep generated documents and stub descriptions under their
[source ownership](documentation.md#sources-of-generated-guides-and-stub-docstrings);
handwritten Markdown has separate
[formatting ownership](documentation.md#handwritten-documents-and-portable-instructions).

**Reason:** Formatter normalization makes mechanical choices reproducible, but
does not select every boundary between related declarations or processing
steps. Both forms can pass formatting, so passing it does not establish that
the groups are appropriate. Distinguishing formatting from grouping also avoids
removing meaningful settings subgroups or treating content whitespace as an
interchangeable source separator.

## Processing groups

**Decision:** Use one blank line between paragraphs within a function, loop,
or branch. A paragraph exposes a step readers can identify before following
its statements. Apply these distinctions together:

- Separate setup, substantial processing blocks, and completion. Within a
  longer block, separate successive calculations, transformations, or state
  updates that readers need to locate. Shared data, dependencies, and a common
  purpose do not remove these boundaries.
- Keep short direct calculations and call sequences compact. Keep paired
  coordinates, formula terms, related assignments, and list entries together.
  Keep a value's immediate check and recovery with it. Different names, types,
  or statement kinds alone do not divide a paragraph.
- Keep alternatives connected by the language's control structure together.
  In a large dispatch, separate complete handlers or related handler families;
  keep a compact value-selection table continuous.

Choose paragraph size from the code readers see, including nesting and the
amount of work between boundaries. Do not demand an independent algorithm or
separately constructed output before allowing a paragraph. Conversely, do not
isolate every local variable, check, call, or return. An immediate result can
finish a compact calculation; a result after a substantial block can stand in
a completion paragraph. These choices apply equally to delegated and inline
work, and inside macros.

**Reason:** Paragraphs make control flow and processing stages visible while
keeping closely related statements easy to read together. A whole function or
resource lifecycle is too broad to establish one paragraph, and a different
statement is too narrow to establish another. Avoid an undivided sequence
that hides substantial processing stages and a succession of isolated statements.

## Comments at group boundaries

**Decision:** Distinguish a heading for a group of implementations from a label
on a compact declaration list or processing section and an explanation attached
to one construct.
Use the following spacing in handwritten Rust and Python, including tests and
extensionless Python tools.

| Comment role | Blank lines after the comment |
| --- | --- |
| Heading for module-level Python implementation groups | Two |
| Heading for Python method groups | One |
| Heading for Rust function, type, or implementation groups | One |
| Label for a compact constant, field, or stub declaration group | None |
| Label for a processing section | None |
| Explanation attached to one declaration or statement | None |

Before a group heading or label, keep the normal separation from the preceding
group: two blank lines between module-level Python implementation definitions,
otherwise one. Stub declaration lists retain their formatter spacing.
Do not insert a leading blank line at the start of a file or a block. An attached
explanation shares its construct's preceding boundary; it does not require an
additional boundary inside a processing step. Keep decorators and Rust outer
attributes (`#[...]`) with their declaration, without blank lines between them
or before the declaration.

The distinction depends on what the comment describes, not its punctuation or
whether the next line happens to be a function. In
[import discovery](../../../python/pyxel/utils.py), `Recursive import discovery`
labels the traversal and tracking functions together; `Module path resolution`
labels the following path helpers. Both are group headings. The note about
lexical paths inside the traversal explains one operation and stays attached.
The `.pyi` API groups are compact declaration lists even when their members are
functions; their headings remain attached to the first declaration.

An introduction to a whole module or class remains separate from its first
implementation member. For example, the hold/repeat explanation in
[input tests](../../../python/tests/test_input.py) covers the class's test cases,
and the fresh-process note in [the capture runner](../../../python/tests/_runner.py)
covers the runner rather than its first exception class.

**Reason:** Separation makes a group heading visibly different from a comment
belonging only to the next implementation. Compact declaration lists keep their
label close to the entries without adding implementation-sized gaps. Attached
explanations, decorators, and attributes must remain visibly connected to their
subject. These distinctions preserve the comment's scope without adding labels
or rewriting its text.

## Constant catalogues and registration

**Decision:** In [native key definitions](../../../crates/pyxel-core/src/platform/key.rs),
separate the key/value type aliases, SDL keycodes, virtual-key range, mouse-key
range, and gamepad range parameters. Each range's base and entries are continuous;
digits, letters, function keys, keypad keys, axes, and buttons do not create
additional blank-line groups within that range. The gamepad macro's definition
and its four invocations are separate groups; keep the offset entries inside
the macro continuous and the invocation list continuous. Range labels attach
to their first declaration.

In [module constant registration](../../../crates/pyxel-binding/src/constant_wrapper.rs),
separate settings, graphics, audio, keyboard, virtual keys, mouse, and each of
the four gamepads. Each registration call is one argument inventory, with its
label attached. Do not reproduce every underlying settings subgroup inside
that call. Keep the final registration and success return together. The macro
that repeats one registration operation has no internal blank-line groups.

In the [base stub](../../../python/pyxel/__init__.pyi), the `Constants` section
has identity/runtime-location values, file-format values, graphics dimensions,
palette/color values, font dimensions, audio-bank counts, tone values, and
effect values as separate groups. In `Keys`, keep keyboard and virtual modifier
names together, followed by mouse and one group for each gamepad. The public
key list groups by input device, so the native distinction between SDL and
Pyxel-assigned numbers does not require a matching blank there. Keep members
continuous within each group, including differently typed constants.

The [constant test inventories](../../../python/tests/test_system.py) separate
color, keyboard, mouse, and gamepad name lists. Expanded literal names and
comprehensions form the same continuous inventory. The gamepad comprehension
uses one suffix list across devices, without a paragraph per axis/button kind.
`TestConstants`' assertions and per-name loops each stay with their case inputs
and results; enumerating different values does not create extra test phases.

**Reason:** Number ranges expose native encoding, subsystem registration calls
expose export membership, and stub groups help users locate related names.
Those distinct reading tasks justify different group boundaries without
changing the names or their correspondence. Dense inventories should remain
scannable as inventories; a different prefix, type, or generated name alone
does not identify another task or useful navigation unit.
