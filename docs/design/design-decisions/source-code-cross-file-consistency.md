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
lesson](#inline-dimensions-and-editable-constants-in-examples). Internal binding
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

### Inline dimensions and editable constants in examples

**Decision:** Keep screen dimensions directly in `pyxel.init` in examples such
as [Hello Pyxel](../../../python/pyxel/examples/01_hello_pyxel.py), rather than
creating constants merely to name the arguments. Use named values where they
expose something the learner is expected to change and play with, such as
`MOTION_SPEED` in [Mesh and
Motion](../../../python/pyxel/examples/cube/c04_mesh_and_motion.py).

**Reason:** Beginners should be able to follow the short program and try a
change without navigating layers of constants and helpers. The screen size is
already clear at the call site. The motion speed exposes a value the learner
can change to compare animation playback, making that experiment available
through one edit.

Applying production abstraction habits mechanically would obscure the lesson.
Direct control flow and local names and values are deliberate choices here.
A number or small repeated expression does not by itself establish a need for
extraction.

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