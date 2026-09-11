# Web Contract Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Public
Contract policy](../design-policy.md#public-contract)

## Code Maker

### Latest project selection in Code Maker

**Decision:** Keep the load token in [Code
Maker](../../../web/code-maker/index.html) so an earlier download or archive
unpack cannot replace a later project selection or report its failure as the
current operation's failure. Local, dialog, and startup error-reporting sites
check the same token before displaying a load error. Startup Gist, GitHub, and
URL loads use those same loaders; the loaders still reject for direct callers.

**Reason:** Downloads and archive processing can finish out of order. The token
keeps the result consistent with the user's latest project selection.

### Saving resources before Code Maker actions

**Decision:** Once the embedded resource editor is initialized, Run, ZIP saving,
and Gist sharing use resource bytes only after saving in its Python context
succeeds. Before initialization, use the project's existing resource bytes.
Invoke that save directly and read the resulting file after it returns; a
keyboard event followed by a fixed delay is not a completion signal. Yield out
of an existing Python callback before invoking Python again, and suppress the
save-export callback's recursive entry while copying resources.

**Reason:** [Code Maker](../../../web/code-maker/index.html) must not run or
export stale resource data as though the user's current edits were saved.
The action reports a failure once and stops, keeping the project and editor
available for retry. The copy helper propagates the failure to that owner.
Picker cancellation remains silent; other file-write failures do not trigger
an additional download. This recovery does not change `pyxel.save`'s exception
contract or introduce a transaction across all project files.

### Code Maker project filenames

**Decision:** Allow ordinary filenames containing consecutive dots, such as
`data..txt`, when copying a Code Maker project into its runtime. Exclude a path
component equal to `..`, which denotes parent-directory traversal.

**Reason:** Files accepted by the [project loaders and archive
writer](../../../web/code-maker/index.html) should remain usable in the [runtime
project copy](../../../web/code-maker/pyxel-screen.html). A double dot within a
filename has a different meaning from a parent-directory component. This
distinction does not define an archive sandbox or prescribe the validation used
by other file-loading interfaces.

### Code Maker dropped-file destinations

**Decision:** Choose the destination of a dropped `.py` or `.pyxres` file by
its extension across the page and both embedded Pyxel views. A Python file
replaces the code; a resource file goes to the resource editor. Do not require
the user to drop it over the destination pane.

**Reason:** The extension already identifies the destination, so requiring a
particular pane adds an unnecessary location-dependent rule. The
[drop handler](../../../web/code-maker/index.html) owns this routing, and the
[manual](../../../web/code-maker/manual.json) describes it across translations.
Other file types retain their embedded-app handling; this decision does not
extend project drops to ZIP archives or multiple files.

## MML Studio

### Initial MML restoration

**Decision:** Keep [MML Studio](../../../web/mml-studio/index.html)'s initial
restoration of locally encoded MML without per-channel edit tracking.

**Reason:** This startup operation decodes content already in the URL. Managing
additional edit state solely during that local decoding adds complexity out of
proportion to its purpose. Its share-link generation guard instead protects
later asynchronous encoding results; it does not preserve typing during initial
restoration. These operations have distinct responsibilities.

## Runtime and Pages

### Runtime error presentation

**Decision:** The [web runtime](../../../wasm/pyxel.js) retains an uncaught
error's name, message, and available stack in its fatal display. If the stack
already begins with that complete summary, display it once; a stack containing
only frames still needs the summary. Preserve runtime-provided wording.

**Reason:** The summary identifies the failure and the stack locates it.
Repeating the summary adds no information, while relying on every stack to
include it can hide the actual error. This presentation choice does not add
exception handlers or make recoverable tool operations fatal.

### Published page redirects

**Decision:** Retain the legacy pages under `wasm/` as redirects to their web
counterparts, carrying the query string and fragment.

**Reason:** Published links can contain launch options or a reference location.
The [redirect pages](../../../wasm/launcher/index.html) preserve that state
while keeping content at one owner. A directory reorganization does not
invalidate those links or justify dropping their options.

### Undocumented packages option

**Decision:** The [Web launcher](../../../web/launcher/index.html) forwards a
`packages` URL parameter, and the `pyxel-run` and `pyxel-play` custom elements
in the [web runtime](../../../wasm/pyxel.js) accept a `packages` attribute that
loads the named Pyodide packages before the command runs. Keep these paths as an
undocumented compatibility route: the URL builder, guides, and reference do not
describe them, and neither their presence in the runtime nor their absence from
the documentation is a defect.

**Reason:** The option was removed from the documented launcher interface in
2.8.7, while published pages and links can still carry it. Removing the runtime
support would break those pages for a feature Pyxel no longer promotes, and
documenting it again would reverse that removal.

### Local Showcase runtime and served files

**Decision:** Serve the public web, documentation, runtime, and example assets
needed by the local Showcase. Keep repository-internal paths outside those
served roots. Replace executable CDN runtime references in page headers with
the local runtime; preserve instructional examples in page bodies.

**Reason:** [`make run-wasm`](../../../Makefile) builds a local runtime for
inspection. Loading an unrelated CDN build would defeat that purpose.
[start_showcase](../../../scripts/start_showcase) limits resolved file paths to
the asset roots, rewrites page headers, and supplies a service worker where the
browser supports it. Its network-accessible listener does not make it a general
repository file server. These choices concern the showcase server; they neither
prescribe public hosting nor guarantee service-worker support on every HTTP
origin.

Service-worker registration is optional. Its existing warning handles both a
synchronous throw and a rejected registration promise; neither reaches the
runtime's fatal-error handler. The rewritten page can still use the local
runtime without successful registration.
