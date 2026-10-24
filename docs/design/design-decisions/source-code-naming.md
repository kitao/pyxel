# Naming Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#naming)

## Across Languages

### Names shared by interfaces

**Decision:** Python API names and argument names remain identical through their
bindings, stubs, reference source data, and generated descriptions. Web routes,
serialized keys, event names, DOM identifiers, shader uniforms, and
build-provider names retain the spelling their producers and consumers share.
Language case conventions apply to the surrounding implementation, not to
renaming those connections independently.

**Reason:** Callers and consumers depend on the shared spelling. For example,
[shader uniform names](../../../crates/pyxel-core/src/shaders/) match their
[Rust lookups](../../../crates/pyxel-core/src/graphics.rs); Python snippets in
web data still use Python names. Public API abbreviations are not expanded in
only one representation.

### Directional operation families

**Decision:** Related operations use the same object term and the direction
appropriate to each operation. In [Code
Maker](../../../web/code-maker/index.html), use `loadFromGist` with
`saveToGist`, and `loadFromUrl` for loading a URL. Native resource operations
use `load_resource` and `save_resource`; coordinate conversions use `to_local`
and `to_world`.

**Reason:** The repeated object identifies the family; the verb and preposition
identify its direction. Identical prepositions would not express opposite
directions. A name does not require implementing an inverse operation, and an
unrelated lifecycle does not acquire a new name solely to fit a verb-pair table.

### File and asset name families

**Decision:** Web page directories use their established hyphenated routes;
Python modules and packaged example basenames use underscores. Showcase pages
retain their explicit mapping to the packaged examples. Capture references keep
the corresponding example/editor name and frame suffix; dimension suffixes
identify asset variants. Preserve author and upstream asset names.

**Reason:** A page URL, importable module, and generated capture have different
consumers. Their mapping must remain recognizable without imposing one separator
across those roles. Generated names are changed at their producer together with
their references, under the [documentation ownership
decisions](documentation.md).

## Rust

### Names at Python and SDL2 boundaries

**Decision:** The exposed members in the
[Python bindings](../../../crates/pyxel-binding/src/) follow Python API names
and argument order, including `blt`, and use PyO3's constructor and property
conventions. Their internal helpers remain idiomatic Rust. The
[SDL2 implementation](../../../crates/pyxel-core/src/platform/sdl2/) retains the
external API's C names at its call sites.

Image and tilemap drawing APIs keep matching operation names. An internal
`draw_line` may implement the public `line` API: the layers have distinct naming
roles. Conversely, a name such as `Canvas.drawCanvas()` repeats its owner
without adding meaning. These examples distinguish a useful correspondence from
mechanical textual identity or redundant qualification.

**Reason:** A binding should be recognizable from the interface it implements.
Renaming external terms to suit an internal naming convention would make the
mapping harder to follow. This exception belongs to the interface boundary,
not every function in its surrounding file.

### Shared handles and value kernels

**Decision:** Keep the shared-resource `Rc` aliases with the meaning recorded
under [shared graphics and audio
ownership](source-code-performance.md#shared-graphics-and-audio-ownership). They
do not encode one concrete smart-pointer implementation.

For a Cube operation with both a shared-object result and a value kernel, use
the same calculation or result vocabulary with `_value`: `inverse` /
`inverse_value`, `to_matrix` / `matrix_value`. The suffix identifies the
value result; it does not require a character-for-character operation
stem. Keep the borrowed receiver used by these calculations. Do not introduce
a `to_` prefix that requires a receiver change or lint suppression solely to
match the allocating entry's name. This does not require a companion operation
or a suffix on every value-returning helper.

**Reason:** The [matrix](../../../crates/pyxel-core/src/cube/mat4.rs),
[quaternion](../../../crates/pyxel-core/src/cube/quat.rs), and
[node](../../../crates/pyxel-core/src/cube/node.rs) pairs perform the same
calculation with different result ownership. Their names should expose that
correspondence; the [value-kernel
decision](source-code-performance.md#cube-value-calculations) owns when the
separate representation is useful.

## Python

### Registered handlers and overridable hooks

**Decision:** Editor and widget callbacks registered on an instance use
`__on_...` names. Their registration identifies the event they handle.
Overridable Cube `Node` hooks keep the public `on_...` names invoked by
traversal. Reactive widget properties keep their `_var` names through
registration, access, and the [interface summaries](source-code-comments.md).

**Reason:** [Registered widget
callbacks](../../../python/pyxel/editor/widgets/button.py) belong to their
defining class; a subclass can register another handler without accidentally
replacing that method. [Node
hooks](../../../crates/pyxel-binding/src/cube/node.rs) are instead selected
through overriding. Applying the same underscore scheme to both would change
their dispatch semantics.

### Test names and value semantics

**Decision:** The Mat4 and Vec3 transformation-result groups use
`TestTransform`. Mutation terminology belongs to tests that exercise changes to
an existing mutable object, such as Camera or Shading state.

**Reason:** The [matrix](../../../python/tests/cube/test_mat4.py) and
[vector](../../../python/tests/cube/test_vec3.py) groups verify returned
transformation results. Calling them mutation tests would misdescribe the
immutable math interface. This does not require reorganizing unrelated test
groups or adding assertions solely for a class-name correction.

## Web

### JavaScript helpers and cross-language data

**Decision:** Project-owned JavaScript functions and ordinary variables use
camelCase; custom-element classes use PascalCase. Configuration constants such
as `PYXEL_WORKING_DIRECTORY` use UPPER_SNAKE_CASE; a `const` binding alone does
not select that form. The private helpers in the
[web runtime](../../../wasm/pyxel.js) retain their leading underscore, while its
host entry points and [shared page helpers](../../../web/shared.js) keep their
callable names. Cross-frame and Python bridge identifiers retain their shared
spelling; an underscore does not make such a connection safe to rename locally.

Web source-data keys keep their schema's snake_case names through renderers and
generators. The language-selection key `cn` maps to `zh` when setting the HTML
language; storage/source keys and HTML language tags are separate interfaces.

**Reason:** Runtime helpers, page-shared operations, and cross-language lookups
have different callers. Keeping those boundaries explicit prevents a case or
privacy-prefix cleanup from breaking the connection or rewriting embedded
Python as JavaScript.
