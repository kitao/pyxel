# Web Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

Read with the [shared blank-line decisions](source-code-blank-lines.md).
JavaScript, HTML, CSS, and Web runtime groups.

## JavaScript implementation boundaries

**Decision:** Separate script-level function and class implementations, and
methods of named classes, with one blank line in Web/WASM sources and tests,
including inline HTML scripts. This applies equally to function declarations
and arrow functions assigned to individual variables. Keep the independently
called `_escapePythonString` and `_encodeUrlPath` helpers in
[the runtime](../../../wasm/pyxel.js) separate, just like the small HTML helpers
in [shared.js](../../../web/shared.js).

An implementation-group heading has one blank line before and after it. Omit
only the preceding blank when the heading starts a file or block.
An explanation of one implementation remains
attached without an intervening blank line. These boundaries do not require
adding or rewriting comments.

Keep a compact test double's fields and trivial methods together, whether its
class is named or anonymous; this takes precedence over method separation.
For example,
the anonymous `XMLHttpRequest` class in the URL-path test in
[wasm.test.js](../../../web/tests/wasm.test.js) supplies one small fixture
interface, including its request-recording method. Object-literal callbacks
that supply such a fixture likewise remain continuous. Do not apply the
independent-implementation separator to every function-valued field or callback.

**Reason:** A named implementation is a unit readers locate and edit on its
own; choosing arrow syntax or shortening its body does not change that role.
A small inline test double is read as the interface supplied to one test.
Separating every empty or trivial method would fragment that inventory. This
distinction follows the implementation's role, not its line count or the
presence of braces.

## Runtime processing groups

**Decision:** In [the Web runtime](../../../wasm/pyxel.js), apply the shared
[processing-group distinctions](source-code-blank-lines.md#processing-groups).
Use these additional boundaries for runtime construction and callbacks:

| Processing relationship | Blank-line placement |
| --- | --- |
| Fields, coordinates, bit updates, or paired subscriptions form one inventory | No separation between its members |
| A larger operation contains distinct startup stages or substantial element/acquisition work followed by a common update | One blank line between those stages or work units |
| Several local completion callbacks cooperate in one promise | No separation between the callbacks; one blank before their registration/start block |
| A short callback performs one operation and is immediately registered | No separation between its definition and registration |
| A callback contains its own acquisition and update groups | One blank between the complete callback setup and its subscription group |

Global-error and drag/drop subscriptions are paired inventories.
`_setMinWidthFromRatio`'s missing-element check and width update are one compact
operation.

The following table specifies the concrete stage membership for the larger
operations. Semicolons separate groups; steps within a group are continuous,
subject to the nested groups below.

| Operation | Groups |
| --- | --- |
| `launchPyxel` | Version/parameter reporting; browser input preparation; screen/runtime creation; error/file hooks and startup gate; initialized-context fields; command execution and its error handler |
| `resetPyxel` | Route checks; overlay removal and quit request; audio suspension; main-loop cancellation; embedded state reset; command restart; scheduled audio resumption |
| `_suppressTouchZoomGestures` | Viewport setup; pinch callback and its paired subscriptions |
| `_updateScreenElementsSize` | Common screen geometry; overlay sizing and touch-geometry invalidation |
| `_createScreenElements` | Screen-container acquisition; context-menu/resize behavior; drag/drop behavior; canvas construction; logo loading, sizing, and returned canvas |
| `_loadPyodideAndPyxel` | Asset prefetches and rejection observers; Pyodide/canvas initialization; wheel installation; working directory; import-hook installation and returned runtime |
| `_displayErrorOverlay` | Reporting and overlay acquisition/construction; message and scroll update |
| `_hookFileOperations` | Filesystem capture; directory helper; mirroring helper; paired open/stat wrappers; browser-save hook |
| `copyPath` | Path qualification and normalization; source/destination and cache check; synchronous fetch and byte conversion; directory/file materialization |
| `_waitForInput` | Old-logo removal; prompt construction and sizing; input-gate promise; prompt removal and render yield |
| `_updateGamepadStateFromTouch` | Coordinates for all control regions; directional pad; action buttons; menu buttons |
| `_addVirtualGamepad` | Admission checks; canvas space and screen container; image factory; three control images; old-handler removal; rectangle-cache invalidation setup; touch callback and stored reference; touch subscriptions |
| `_executePyxelCommand` | Packages and controls; supplied file; command selection/construction; execution and error handling |

Keep adjacent route/admission checks continuous in `resetPyxel` and
`_addVirtualGamepad`. Within reset's `try`, overlay removal and the short quit
request remain one group. Keep the catch handler attached to the complete
reset operation. In the command switch, separate the substantial run, play,
and edit cases; keep each case continuous outside its embedded Python.

The callback cases in the table apply to `_loadImage` and `_waitForInput`
(cooperating callbacks), `pinchHandler` and `invalidateRects` (immediate
registration), and the gamepad touch callback (internal processing groups).
The substantial directory and mirroring helpers in `_hookFileOperations`
remain separate implementations.

In `_hookPythonError`, separate the captured batch state from the returned
receiver. Within that receiver, keep the admission check, loop cancellation,
and text accumulation together, then separate the deferred flush setup.
In the gamepad's `touchHandler`, separate rectangle acquisition/cache fallback
from button-state recomputation and event completion. Keep coordinate sets,
directional-bit updates, callback cleanup, and other short inventories intact.

Keep `window._savePyxelFile` continuous from anchor construction through click
and scheduled cleanup. Its attached cleanup explanation stays inside that
operation. None of these boundaries changes blank lines inside Python strings.

**Reason:** The larger functions assemble several independently readable
stages or visual elements. Short guards, paired registrations, and dependent
value-use chains do not become separate stages merely because their statements
have different forms. Local callbacks that share one promise are a small
cooperating set; their later registration is a useful boundary after that set,
while one short callback plus its subscription needs no extra split.

A download's short create/click/cleanup chain concerns one temporary
anchor; labeling those steps as different lifecycle phases would not make
three separated groups easier to read. The overlay and touch handlers contain
larger acquisition work that is useful to locate separately from their updates.

## CSS rule groups

**Decision:** In [the component stylesheet](../../../web/styles/input.css), keep
the rules within each labeled `@layer components` group continuous. Separate
the shared primitives, API reference, user guide, editor manual, showcase,
launcher, Code Maker, MML playback, MML manual, and MML command-reference groups
with one blank line. Keep each label attached to its first rule. A rule using
ordinary declarations alongside `@apply` does not start another group.

Outside that component catalogue, separate rules with one blank line: the
pixel-rendering, details/summary, and highlighting rules in the same file;
the runtime element rules in [pyxel.css](../../../wasm/pyxel.css); and the
page-specific inline styles in Code Maker and the MML Studio manuals.
Use the same rule separation inside their responsive `@media` blocks.
Keep a rule's property list, a selector list, and the runtime palette's custom
properties continuous. Attached rule comments have no following blank line.

**Reason:** The component catalogue is scanned as a set of reusable classes
for each page or shared purpose. A blank after every short class would expand
that catalogue without improving navigation. Runtime and page-specific rules
describe individual elements, layout adjustments, or browser states that
readers edit separately. Their rule boundaries are useful even for a short
override; the number of declarations and the use of `@apply` do not decide
the spacing. Properties and selectors within one rule describe a single
styling operation and stay together.

## HTML document regions

**Decision:** When a document explicitly contains both `head` and `body`, use
one blank line between them. This identifies document setup and application
content equally in ordinary pages, launch examples, and internal iframe pages.
Do not add padding between a region's opening tag and its first member or
between its last member and closing tag.

Keep the head's metadata, dependency tags, and page-style setup continuous.
Changing from a `meta` or `link` to a `script` or `style` tag does not create
another inventory. Spacing inside inline CSS and JavaScript follows those
languages' decisions, without an extra blank beside the containing tags.
The [WASM redirects](../../../wasm/user-guide/index.html) need only a head;
the two-region separator does not require inventing a body.

Within a body, separate the interface region from its inline controller script.
Separate independently located auxiliary regions as well: in
[Code Maker](../../../web/code-maker/index.html), the main application, file
picker, four complete dialogs, and controller are separate groups. A dialog's
heading, input, and action buttons belong to that dialog; their different tag
names do not require separate paragraphs.

Keep a single runtime mounting operation together. In
[script-test](../../../web/showcase/examples/script-test.html), the target
screen element and `pyxel-run` belong to the same mount. The other
[launch examples](../../../web/showcase/examples/01-hello-pyxel.html) and
[iframe screens](../../../web/mml-studio/pyxel-screen.html) have a single
custom-element region. Likewise, the loader and immediate launch call in the
[app2html output](../../../python/pyxel/cli.py) form one short bootstrap.
The [short HTML example decision](documentation.md#short-html-examples) governs
their minimal scaffolding; region spacing does not add missing wrappers.

Do not treat whitespace in inline text, `pre`/`textarea` content, attribute
values, or template payloads as an interchangeable source separator. Its role
must be checked at the consuming element or language. In particular, adding a
document-region boundary does not reformat a `pyxel-run` Python attribute.

**Reason:** Head and body represent distinct document responsibilities even
when the page runs inside an iframe. The head's members collectively configure
that document; a blank per tag category makes the inventory harder to scan.
Interface regions, complete dialogs, and their controller are useful separate
editing units. A mount's target and declaration, or a loader and its sole
launch call, are cooperating parts of one setup rather than independent regions.
Visible text and embedded programs have whitespace semantics of their own.

## HTML controls and document templates

**Decision:** Within an interface, keep a complete control's label, input,
options, help, and action buttons together. Tags and indentation already expose
those parts. Use one blank between independently edited panels, complete input
groups, and output views. A new tag or attribute does not establish a boundary.

In [Code Maker](../../../web/code-maker/index.html), toolbar and focus strip
form the header group, followed by the split workspace. Within that workspace,
the left pane, divider, and runtime pane are separate groups; the code and
resource editors are separate alternatives inside the left pane. Keep the
toolbar's menus, tabs, run button, and each menu's entries continuous. They
form one command inventory, while the panes are independently selected editors
and runtime content. The focus strip's three segments are one indicator.

In [MML Studio](../../../web/mml-studio/index.html), use title/introduction;
playback/preview; each complete channel editor; sharing output. Within
playback/preview, separate the controls and links from the preview. Keep each
channel's label, solo/mute buttons, and textarea together, without internal blanks.
Within sharing, the labeled URL and QR image are separate output views. In the
[URL builder](../../../web/launcher/url-builder.html), separate header, form,
and footer. The form groups are startup input with help, run/play options,
edit options, and the resulting URL. Keep each group's labels and options
together. The [MML command page](../../../web/mml-studio/mml-commands.html)
has a heading/language/subtitle group followed by the complete command table;
the table's header and body remain continuous.

For document-section templates with an `h2` section title, use one blank before
each `h3` subsection, including the first. Keep the section title and its
introduction together. A subsection contains its paragraphs, tables, images,
code examples, and subordinate `h4` or area-title details until the next `h3`.
Do not add a blank for every paragraph, helper interpolation, table row, or
heading below that level. Apply this structure to the
[user guide](../../../web/user-guide/index.html),
[editor manual](../../../web/editor-manual/index.html), and
[Web usage guide](../../../web/web-usage/index.html). The custom-element
overview paragraph belongs with its setup subsection, before the run subsection.

Empty section placeholders form one declaration inventory in the user, Cube,
and Web usage guides and the editor manual. Keep them continuous. In the editor
manual, separate that inventory from the following collapsible editor regions;
each complete `details` region is a group. Its summary, icon/line, and content
placeholder remain together.

The [Code Maker](../../../web/code-maker/manual.html) and
[MML Studio](../../../web/mml-studio/manual.html) layout illustrations have
three groups: introduction, complete interface preview, and generated legend.
Keep each preview's toolbar, pane/channel, and sharing mockup together as one
illustration, and each legend row together. Other short explanatory sections,
thumbnail galleries, table helpers, icon fragments, and repeated comparison
rows stay continuous unless they contain the subsection boundary above.
Single-line fragments need no expansion to introduce blank lines.

These choices concern literal HTML structure. JavaScript expressions embedded
in a template retain their own grouping, and code examples retain their content
ownership. A literal separator between block elements must not be inserted
inside text, an attribute, a code example, or a JavaScript interpolation.

**Reason:** A reader locates whole panels and controls, not isolated labels or
attributes. The document's `h2`/`h3` hierarchy provides a reproducible grouping
level; subordinate headings and rendering helpers do not each become a source
paragraph. Empty placeholders are an inventory rather than implementations of
their eventual content. A layout illustration contains a complete interface
structure worth locating independently from its explanation and legend, while
rows and thumbnails are compared as one collection.
