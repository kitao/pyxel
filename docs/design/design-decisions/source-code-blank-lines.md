# Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

These decisions specify where a blank line belongs and where it does not,
from the role and relationship of the surrounding code. The same conditions
apply whether that boundary currently has a blank line or not, in every source
language, in embedded content, and in tests. Where a group is named below, the code identifies it through its structure or a heading; under the [audit procedure](../design-audit.md#1-select-and-resolve-decisions), these records do not enumerate the groups of individual files.

## Across Languages

### Formatting ownership

**Decision:** Use the formatting targets in [make format](../../../Makefile).
Rustfmt owns Rust formatting, Ruff owns Python sources, stubs, and the
explicitly selected extensionless Python scripts, and Prettier owns the selected
Web/WASM and configuration files. Ruff also formats Python fenced blocks in
Markdown, including the output of
[generate_docs](../../../scripts/generate_docs). Keep their language-specific
spacing: ordinary Python method bodies and consecutive one-line stub
declarations need not have the same separation. Files outside those targets do
not acquire a formatter merely because their extension is supported by one.

These tools settle indentation, wrapping, quoting, and the blank lines they
normalize. Authored grouping remains a separate choice wherever the formatter
preserves both a separated and an unseparated form. Between authored groups,
use one blank line unless the formatter prescribes otherwise; keep the members
of each group together. A section heading does not turn everything below it
into one group.

Treat blank lines inside strings, embedded programs, and document content
according to that content's role. Formatting the containing HTML does not
establish the grouping of a Python program inside an attribute. Generated
documents and stub descriptions stay under their [source
ownership](documentation.md#sources-of-generated-guides-and-stub-docstrings);
handwritten Markdown has separate [formatting
ownership](documentation.md#handwritten-documents-and-portable-instructions).

**Reason:** Formatter normalization makes mechanical choices reproducible, but
does not select every boundary between related declarations or processing
steps. Both forms can pass formatting, so passing it does not establish that
the groups are appropriate. Distinguishing formatting from grouping also avoids
treating content whitespace as an interchangeable source separator.

### Processing groups

**Decision:** Use one blank line between paragraphs within a function, loop,
branch, macro, shell stage, or test body. A paragraph exposes a step readers can
identify before following its statements. Apply these distinctions together:

- Separate setup, substantial processing blocks, and completion. Within a
  longer block, separate successive calculations, transformations, or state
  updates that readers need to locate. Shared data, dependencies, and a common
  purpose do not remove these boundaries.
- Keep short direct calculations and call sequences compact. Keep paired
  coordinates, formula terms, related assignments, list entries, a command's
  continuations and pipelines, and a value's immediate check and recovery
  together. Different names, types, or statement kinds alone do not divide a
  paragraph.
- Keep alternatives connected by the language's control structure together.
  In a large dispatch, separate complete handlers or related handler families;
  keep a compact value-selection table continuous.

Choose paragraph size from the code readers see, including nesting and the
amount of work between boundaries. Do not demand an independent algorithm or
separately constructed output before allowing a paragraph. Conversely, do not
isolate every local variable, check, call, or return. An immediate result can
finish a compact calculation; a result after a substantial block can stand in
a completion paragraph. A borrow scope, `unsafe` block, `?`, explicit `drop`,
`try`, `finally`, `with`, `yield`, `await`, or assertion neither requires nor
prohibits a boundary. Delegated and inline work follow the same criteria.

**Reason:** Paragraphs make control flow and processing stages visible while
keeping closely related statements easy to read together. A whole function or
resource lifecycle is too broad to establish one paragraph, and a different
statement is too narrow to establish another. Rust's [style
principles](https://doc.rust-lang.org/style-guide/principles.html) and the
[standard library's I/O
implementation](https://doc.rust-lang.org/src/std/io/mod.rs.html) show the same
authored paragraphing: compact helpers beside paragraphs within substantial
loops, without fixed spacing around every loop or return.

### Declaration inventories

**Decision:** Keep a compact record, enumeration, constant range, argument
list, field group, or registration inventory continuous. A different prefix,
type, unit, visibility, attribute, `cfg`, or generated name within the
inventory does not create a group. When a larger record or catalogue has
distinct responsibilities, separate them with one blank line, use the same
groups in the declaration and in its initializers, and attach each label to
the first member it names. Separate an explicitly labeled deprecated group
from the inventory it follows. Keep a macro that repeats one registration
operation free of internal blank-line groups, and keep a final registration
call together with its success return.

Declaration and dispatch sites that expose the same concepts share their
families, as the [MML command declaration](../../../crates/pyxel-core/src/mml_command.rs)
and its [channel dispatch](../../../crates/pyxel-core/src/channel.rs) do.

Where a record, catalogue, or stub marks its groups only by blank lines, each
group covers one nameable subject. A finding names the unrelated members a group
mixes or the shared subject a boundary splits; a grouping without such a defect
is not a finding.

Number ranges, registration calls, and stub groups serve different reading
tasks and need not share boundaries: native key ranges expose encoding,
subsystem registration exposes export membership, and the stub groups help
users locate related names. Do not insert a matching blank into one merely
because another has it.

**Reason:** Dense inventories should remain scannable as inventories, and
matching groups in a declaration and its initializer make their correspondence
visible. Splitting every type, component, flag, or prefix would hide the stable
responsibilities the groups expose.

### Comments at group boundaries

**Decision:** Distinguish a heading for a group of implementations from a label
on a compact declaration list or processing section and an explanation attached
to one construct. Use the following spacing in handwritten Rust and Python,
including tests and extensionless Python tools.

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
otherwise one. Stub declaration lists retain their formatter spacing, and stub
API groups are declaration lists even when their members are functions, with
their headings attached to the first declaration. Do not insert a leading blank
line at the start of a file or a block. An attached explanation shares its
construct's preceding boundary; it does not require an additional boundary
inside a processing step. Keep decorators and Rust outer attributes (`#[...]`)
with their declaration, without blank lines between them or before the
declaration. An introduction to a whole module or class remains separate from
its first implementation member.

The distinction depends on what the comment describes, not its punctuation or
whether the next line happens to be a function. In [import
discovery](../../../python/pyxel/utils.py), `Recursive import discovery` heads a
group of traversal functions, while the note about lexical paths inside the
traversal explains one operation and stays attached.

**Reason:** Separation makes a group heading visibly different from a comment
belonging only to the next implementation. Compact declaration lists keep their
label close to the entries without adding implementation-sized gaps. Attached
explanations, decorators, and attributes must remain visibly connected to their
subject.

## Rust

### Imports, items, and calculations

**Decision:** Apply the shared processing and inventory groups inside
functions, macros, generated implementations, and native tests. Separate each
`use` group from following non-import declarations or executable statements
with one blank line, including inside blocks. Generated types,
implementations, and methods retain ordinary item separation; a storage cell
and its sole accessor remain together. Keep compact geometry literals and
short component calculations together; in a substantial expanded matrix
calculation, separate complete rows or columns in the calculation's existing
order, keeping each formula intact. Platform alternatives of one setting stay
with the setting they select, with every `cfg` attached to its declaration;
the [settings groups](source-code-structure-and-formatting.md#grouping-and-order-in-settingsrs)
own the section order.

**Reason:** Import separation distinguishes the local name environment from the
code that uses it. The remaining boundaries follow the shared criteria; Rust's
ownership syntax and attributes do not add or remove paragraphs on their own.

## Python

### Definitions, widgets, and embedded programs

**Decision:** Local definitions retain Python's definition spacing, including
helpers that use captured state; their definitions belong near the work that
uses them, and short lambdas remain part of their containing expression. Blank
source lines and empty strings emitted into generated content have different
ownership.

In editor and widget classes, keep the opening interface summary as one
comment block: a line containing only the indented `#` separates `Variables`
from `Events` and separates the common image-editing variables from the
tilemap-specific variables where both exist; entries within a group are
consecutive; one physical blank line follows the summary, including before a
class variable. The [interface-list decision](source-code-comments.md#ui-interface-list-notation-and-grouping)
owns which categories and members are listed.

In editor constructors, group base construction, stored state, and simple
variable/property wiring by responsibility. An assignment becoming `new_var`,
`copy_var`, or a variable listener does not itself create a boundary. Separate
substantial local-state, parent-interface, child-control, and own-event setup
when those are distinct portions of the construction. Keep a control's
construction with its help, listeners, and forwarded variables; related compact
controls and surface assemblies can form one group, a substantial control
configuration has its own paragraph, and a short specialization need not
separate its sole final subscription. Keep paired coordinates and corresponding
focus/size variables together. After substantial setup, separate
application-loop startup as the completion stage, and keep a minimal startup
call sequence compact.

In document generators, keep the construction of each labeled output section
together and separate the sections of a substantial document construction;
recognition variants for the same output operation belong together, while
handlers for distinct constructs form a dispatch under the shared criteria. In
drawing code, group complete visual parts and layers. A shape's coordinates,
edges, and corner pixels belong to the same part even when different drawing
APIs produce them; short symmetric component lists stay together, and larger
contour parts may have their own paragraph.

Select the spacing of embedded and quoted Python by the content's role. HTML
or JavaScript owns quoting, interpolation, and container indentation; opening
and closing literal newlines are container boundaries, not processing groups.

| Content role | Blank-line placement |
| --- | --- |
| Short runtime bridge | Keep its immediate import/use sequence compact |
| Multi-stage bootstrap | Separate dependency categories and startup/reset stages under the shared processing criteria |
| Web guide program or executable HTML program example | Use one blank between dependencies, module-level definitions, and execution groups; use one between methods |
| Contextual API usage example | Keep a compact demonstration and its supporting context together; apply ordinary processing groups when it has several stages |
| Metadata field example | Keep the field inventory continuous |

The API recipes of the base and Cube references assume surrounding context; an
import or small fixture does not make them full modules, and HTML fenced
examples retain the Python attribute's grouping.

Generated Markdown follows its own formatter, including Ruff's two blank lines
around module-level Python definitions; generate and format it from its source
and compare logical groups after that normalization. User-entered and
dynamically fetched programs retain their source's ownership; do not rewrite
them at runtime.

**Reason:** Definition spacing distinguishes a local helper from the processing
around it, and readers of a widget need to locate what it owns, how it connects
to its parent and children, and which events it handles. Complete program
examples need visible definitions and execution groups, while short API recipes
keep the demonstrated operation easy to follow; their published container and
generation path explain the different definition spacing.

## Web

### Implementations, runtime stages, styles, and markup

**Decision:** Separate script-level function and class implementations, and
methods of named classes, with one blank line in web/WASM sources and tests,
including inline HTML scripts and arrow functions assigned to individual
variables. An implementation-group heading has one blank line before and after
it, omitting only a preceding blank at the start of a file or block. Keep a
compact test double's fields and trivial methods together, whether its class is
named or anonymous, and keep object-literal callbacks that supply such a
fixture continuous; this takes precedence over method separation, and the
implementation separator does not apply to every function-valued field or
callback.

Inside runtime construction and callbacks, apply the shared processing groups
with these boundaries:

| Processing relationship | Blank-line placement |
| --- | --- |
| Fields, coordinates, bit updates, or paired subscriptions form one inventory | No separation between its members |
| A larger operation contains distinct startup stages or substantial element/acquisition work followed by a common update | One blank line between those stages or work units |
| Several local completion callbacks cooperate in one promise | No separation between the callbacks; one blank before their registration/start block |
| A short callback performs one operation and is immediately registered | No separation between its definition and registration |
| A callback contains its own acquisition and update groups | One blank between the complete callback setup and its subscription group |

Keep adjacent route or admission checks continuous, keep a catch handler
attached to the complete operation it protects, and keep a short
create/click/cleanup chain around one temporary element continuous. In a
command switch, separate substantial cases and keep each case continuous
outside its embedded Python.

In the [component stylesheet](../../../web/styles/input.css), keep the rules within each labeled `@layer
components` group continuous and separate the labeled groups with one blank
line; a rule using ordinary declarations alongside `@apply` does not start
another group. Elsewhere, including runtime element rules, page-specific inline
styles, and responsive `@media` blocks, separate rules with one blank line. Keep
a rule's property list, a selector list, and a palette's custom properties
continuous; a group label stays attached to its first rule, and attached rule
comments have no following blank line.

In HTML, use one blank line between an explicit `head` and `body`, without
padding between a region's tags and its members. Keep the head's metadata,
dependency tags, and style setup continuous; spacing inside inline CSS and
JavaScript follows those languages' decisions, without an extra blank beside the
containing tags. Within a body, separate the interface region from its inline
controller script and separate independently located auxiliary regions such as
pickers and complete dialogs; a dialog's heading, input, and action buttons
belong together. Keep a complete control's label, input, options, help, and
action buttons together, and use one blank between independently edited panels,
complete input groups, and output views; a new tag or attribute does not
establish a boundary. Keep a single runtime mounting operation together: a
target screen element with its custom element, or a loader with its immediate
launch call. In document-section templates with an `h2` title, keep the title
and its introduction together and use one blank before each `h3` subsection,
including the first; a subsection keeps its paragraphs, tables, images, code
examples, and subordinate headings together until the next `h3`. Empty section
placeholders form one inventory. Whitespace in inline text, `pre`/`textarea`
content, attribute values, and template payloads is content, checked at the
consuming element or language.

**Reason:** A named implementation is a unit readers locate and edit on its
own, while a small inline test double is read as the interface supplied to one
test. Larger runtime functions assemble several independently readable stages
or visual elements, whereas short guards, paired registrations, and dependent
value-use chains do not become separate stages because their statements have
different forms. The component catalogue is scanned as a set of reusable
classes, while individual element rules are edited separately. Head and body,
complete dialogs, and the controller are distinct document responsibilities;
the `h2`/`h3` hierarchy provides a reproducible grouping level, and a mount's
parts cooperate in one setup.

## GLSL

### Shader formulas and passes

**Decision:** In the [screen shaders](../../../crates/pyxel-core/src/shaders/),
separate function implementations and declaration groups from implementations
with one blank line. Keep version/precision directives, the uniform inventory,
and each macro inventory continuous; distinct interfaces within a filter, such
as host-name aliases and comparison macros, form separate inventories. File
boundaries do not require leading or trailing empty lines, and each fragment
ends with a newline so the loader's concatenation keeps its last token separate.

| Relationship | Blank-line placement |
| --- | --- |
| Coordinates, color channels, coefficients, and the return directly evaluate one formula | None between preparation, component calculations, and result |
| A variable is filled immediately by a component inventory | None between its declaration and that inventory, or between components |
| An immediate screen check selects a sampled color or background | Keep coordinate acquisition, selection, and output continuous |
| A complete coordinate transformation precedes filtering | One blank between the transformation and filtering |
| Separate sampling passes or coordinate domains form an effect | One blank between the complete passes or domains |
| An expanded filter has independently evaluated corner cases | One blank between complete corners; use the same internal groups for every corner |

Attribution and license blocks have their own paragraph structure, and
spacing inside pixel-map explanations is comment content, not a shader
statement boundary.

**Reason:** Component-by-component gaps obscure a single formula, while
complete sampling passes and expanded corner algorithms need locatable
boundaries that match under corner rotation. Returning or writing the
calculated value adds no new calculation stage.

## Configuration and shell

### Configuration records and shell tasks

**Decision:** Use these boundaries in build, editor, and GitHub configuration:

| File family | Separation |
| --- | --- |
| Makefile | One empty line between the recorded [groups](source-code-structure-and-formatting.md#makefile-groups) and between targets; a target, its recipe, and continued argument lists stay continuous; the opening instructions form one comment block with comment-only separators |
| Workspace/crate manifests and Python packaging TOML | One empty line between tables; keep a table's entries, attached comments, and array members continuous |
| Rust toolchain and formatter TOML | Keep the small single configuration block continuous, including its attached explanation |
| Ignore and attribute files | One empty line between labeled groups; none between a label, its entries, and their attached explanations |
| `.vscode/settings.json` and `web/package.json` | Keep each nested configuration continuous; braces and indentation identify its objects |
| GitHub workflows | One empty line between top-level sections, jobs, steps, and build-matrix records; keep each step's and matrix record's fields together |
| GitHub issue/discussion forms, routing, and funding | Keep the document structure continuous, including adjacent form fields; indentation and field markers identify the entries |

Within shell scripts and workflow `run` blocks, apply the shared processing
groups: environment settings belong with the commands that consume them;
independently implemented build and inspection tasks are separated, each with
its own setup and result check; setup or cleanup around one delegated command
does not acquire separate paragraphs merely because it precedes or follows the
command; a lifecycle such as launching, waiting for, and restoring one
background process is one group, as is a verification loop with its failure
accumulator and final check. Standalone shell helpers are separated by one
blank line with attached explanations kept with them, and a cleanup function
stays with its trap registration.

Make continuations, heredoc data, echoed file contents, and YAML literal or
folded scalars are file contents: keep their field or directive inventories
continuous and do not turn them into source separators.

**Reason:** Tables, records, workflow steps, and targets provide stable
navigation units; blank lines between every key or recipe line would fragment
them. Shell readers need to locate complete build and verification tasks with
their environment, guards, and cleanup. Make continuations and YAML or heredoc
payloads can give whitespace operational meaning, so their boundaries follow
the consuming format.

## Tests

### Test phases

**Decision:** Apply the shared processing groups to test bodies. Separate
prepared cases and observable interaction or lifecycle phases, keeping each
phase's action and its immediate observations close together; repeated use of
the same fixture does not merge distinct phases. Keep short input/call/assertion
sequences and related assertion inventories compact: component checks,
endpoints, expected-value calculations, and repeated parameter values do not
each need a paragraph, while a large setup, execution, or output inspection can
have its own. Separate substantial fixture construction or mock-environment
setup from the trials that consume it; a helper call or local variable alone
does not establish a setup paragraph. Saving state, exercising behavior,
inspecting results, and restoring state follow the same criteria; short cleanup
stays with the operation it completes. Neither mandatory arrange/act/assert
separation nor continuous formatting of an entire test applies. Local
definitions and embedded programs retain their own language's boundaries and
ownership.

**Reason:** Readers need to locate what is exercised and the evidence about it.
Test-case identity alone says nothing about the amount of code readers must
navigate; distinct phases need boundaries, while splitting each assertion or
component hides their relationship.
