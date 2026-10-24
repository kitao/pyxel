# Cross-file Consistency Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#cross-file-consistency)

## Across Languages

### Error and warning message families

**Decision:** Parameter-constraint messages start with the parameter's public
name. Other errors and warnings follow consistent, idiomatic phrasing within
their failure-kind family across files and languages. Python-standard
diagnostics follow the [Python protocol
decision](public-contract-rust-python.md#python-conversion-and-sequence-errors).

Repository wheel/version maintenance tools send failure diagnostics prefixed
`error: ` (usage text excepted) to standard error and successful results to
standard output. The operation-specific reason and nonzero exit status remain
useful to both a contributor and an automated caller. This applies to
`check_wasm_wheel`, `install_wasm_wheel`, and `update_version`; it does not
require wrapping every possible failure or standardizing unrelated runtime
output.

For native file operations, keep the operation and quoted filename in
`Failed to open/read/parse/create/save file '{filename}'` messages. Use the
phase the owning operation can actually identify; a library call combining
open and decode does not require speculative prechecks to distinguish them.
Format-specific diagnostics retain the needed format, field, or location:
resource validation identifies the bank and entry, and MML uses
`MML:{position}: {reason}` with the parser's byte position. The bindings retain
those messages when converting the result to the established exception class.

Runtime-owned errors keep their original diagnostic and traceback. An editor
action can add its operation context (`Failed to load resource/image/tilemap:`)
when reporting a caught loader error; that does not justify replacing the
underlying reason or repeating it at every layer. Message consistency does not
require added validation, a uniform exception class, or a common prefix for
different audiences.

**Reason:** `fps must be greater than 0` and `scale must be greater than 0`
identify the argument and its required condition in the same way. These examples
explain the wording of an existing constraint; they do not require a new guard
or determine the accepted values of every `fps` or `scale` argument.

### Standing language-idiom exceptions

**Decision:** The standing exceptions are limited to [Python API conventions at
exposed bindings and SDL2 names at external call
sites](source-code-naming.md#names-at-python-and-sdl2-boundaries), and [direct
example code when abstraction would obscure the
lesson](#teaching-scope-and-direct-code-in-examples). Internal binding
helpers remain idiomatic Rust. Performance-related departures require the cost
evidence specified by the policy.

**Reason:** These scopes preserve a recognizable external interface or a short
learning path. They do not permit every surrounding implementation to abandon
language conventions. The linked entries explain the concrete correspondences.

## Rust

### Mouse positioning and input readback

**Decision:** Keep logical-to-window mouse positioning consistent with the
inverse [input mapping](../../../crates/pyxel-core/src/input.rs), including
fractional screen scale and screen offsets: the forward conversion rounds the
scaled displacement away from zero and computes it as an `f64` product. This
records the coordinate conversion, not a guarantee about physical pointer
behavior on every window system.

**Reason:** The inverse truncates toward zero, so rounding away from zero lands
on the requested logical pixel; the `f64` product avoids premature rounding
below it, and truncating the scale first would lose fractional scaling
altogether. The [display calculation](../../../crates/pyxel-core/src/system.rs)
supplies a scale of at least one.

## Python

### Teaching scope and direct code in examples

**Decision:** Treat an example's result and its source as one teaching artifact.
Readers should be drawn to what it produces and be able to understand, modify,
and reuse the relevant technique. Preserve the behavior and expressive qualities
that define the intended demonstration when simplifying its implementation;
reducing the demonstration's scope is a separate design choice. This applies to
all example families, including basic and advanced graphics, sound, and games.

Organize code around the objects, operations, and relationships the reader needs
to follow. Choose grouping and explanations for the knowledge the intended
reader brings. Teaching code may need to expose steps that a reader familiar
with the implementation would infer without those boundaries or explanations.
A helper, class, or shared representation is useful when it gives a
coherent operation or concept a recognizable home and reduces what the reader
must keep in mind. Prefer direct code when a layer merely redirects the reader
or introduces vocabulary without helping that task. Judge the complete path
from a desired effect to the code and assets that produce it, including any
support code introduced by the example. Shorter files or fewer definitions do
not establish a simpler learning path.

Keep local values directly at their use when their meaning is clear. Name values
that expose a useful experiment or express a relationship across the program.
A local screen size can be written at `pyxel.init`; a bound shared by movement
and drawing can use a name. Neither a literal nor a repeated expression alone
requires extraction. Keep sample-specific choices distinguishable from API
requirements.

Keep the agreed demonstration focused. Do not require every possible game
sequence or exceptional state; fix defects that break the demonstrated behavior
or normal interaction. This limit on incidental machinery does not justify
removing the content that makes the demonstration worth understanding.

**Reason:** Pyxel examples make an expressive result attainable through a small,
understandable program. Readers need both the motivation to create it and a
clear route from what they experience to what they can change. Code economy
serves that connection; deleting the result or moving its implementation out of
view does not establish it.

The object classes in [Shooter](../../../python/pyxel/examples/09_shooter.py)
and the direct calls in [Transform](../../../python/pyxel/examples/16_transform.py)
serve different demonstrations. Their reasons transfer across example families;
their structures are not universal prescriptions. Technical difficulty changes
the concepts that must be taught, not the obligation to make their relationship
to the result understandable.

### Cube node identity and ownership

**Decision:** Preserve original Python node instances through tree operations,
callbacks, and query results. Handle reference cycles at the layer that owns
the references.

**Reason:** The [Rust tree](../../../crates/pyxel-core/src/cube/node.rs) owns
children and holds weak parent links to avoid an `Rc` cycle. The [Python
wrapper](../../../crates/pyxel-binding/src/cube/node.rs) retains the original
objects so subclass state, callbacks, and parent identity survive tree
operations. Its strong Python links participate in cyclic garbage collection.
[Raycast results](../../../crates/pyxel-binding/src/cube/raycast_hit.rs)
likewise retain the original node and expose that reference to the collector.
Rust's weak parent link cannot replace this Python responsibility.
