# Rust Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

Read with the [shared blank-line decisions](source-code-blank-lines.md).
These decisions apply to Rust engine code, bindings, macros, and native tests.

## Function bodies

**Decision:** Apply the shared [processing groups](source-code-blank-lines.md#processing-groups)
inside functions, loops, branches, and macros.

Keep members within each argument, component, or field group together; compact
`match` mappings do not need a blank per member. A borrow scope,
`unsafe` block, `?`, or explicit `drop` neither requires nor prohibits a boundary.
Macros use the same declaration and processing groups as handwritten code.
Generated types, implementations, and methods retain ordinary item separation;
a storage cell and its sole accessor remain together.
Separate each `use` group from following non-import declarations or executable
statements with one blank line, including inside blocks.

Keep compact geometry literals and short component calculations together. For
substantial expanded matrix calculations, separate complete rows or columns in
the calculation's existing order, keeping each formula intact.

**Reason:** Rust's [style principles](https://doc.rust-lang.org/style-guide/principles.html)
prioritize readability and scanability over minimizing vertical space, and
include simplicity of rules. The [standard library's I/O implementation](https://doc.rust-lang.org/src/std/io/mod.rs.html)
contains both compact helpers and paragraphs within substantial loops;
shared state does not prevent separation. The [Rust Book's argument-parsing examples](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)
also separate preparation and results. These demonstrate authored paragraphing,
not fixed spacing around every loop or return. Import separation distinguishes
the local name environment from the code that uses it.

## State and declaration inventories

**Decision:** Keep a compact record or enumeration continuous. For larger
records with distinct responsibilities, use the following field groups in both
the declaration and initializer. Keep members continuous within each group,
including paired axes, related flags, and differently typed fields.

| Record | Groups in order |
| --- | --- |
| [TransformProjection](../../../crates/pyxel-core/src/canvas.rs) | Mapping coefficients; source extent; destination bounds |
| [PerspectiveProjection](../../../crates/pyxel-core/src/canvas.rs) | Camera position; world-transform coefficients; view-projection coefficients; destination extent |
| [Camera](../../../crates/pyxel-core/src/cube/camera.rs) | View/display settings; retained rendering buffers |
| [DrawContext](../../../crates/pyxel-core/src/cube/scene.rs) | Target and view configuration; reusable depth and vertex buffers; per-node drawing state |
| [Mesh](../../../crates/pyxel-core/src/cube/mesh.rs) | Part structure and motions; material selection; collision caches and dirty tracking |
| [Sound](../../../crates/pyxel-core/src/sound.rs) | Note/tone/volume/effect/speed data; command/PCM and cache state |
| [Channel](../../../crates/pyxel-core/src/channel.rs) | Playlist and mix parameters; voice and playback timing; command interpretation; saved resume state; PCM position |
| [Voice](../../../crates/pyxel-core/src/voice.rs) | Synthesis components; note, tone, and clock state; crossfade state |

Visibility and attributes do not create additional groups. In settings,
platform alternatives for `AUDIO_BUFFER_SAMPLES` stay together with
`AUDIO_RENDER_STEP_SAMPLES` in the buffering subgroup. Keep every `cfg`
attached to its declaration. Other settings groups follow the
[settings decisions](source-code-structure-and-formatting.md#grouping-and-order-in-settingsrs),
and native/public key catalogues follow the
[shared catalogue decisions](source-code-blank-lines.md#constant-catalogues-and-registration).

The other declaration inventories use these groups:

| Subject | Groups in order |
| --- | --- |
| [Basic math](../../../crates/pyxel-core/src/math.rs) | Angle-conversion pair; thread-local generator state |
| [Graphics constants](../../../crates/pyxel-core/src/graphics.rs) | Shader-version alternatives; source fragments; uniform indices; uniform names; quad vertices |
| [Primitive constants](../../../crates/pyxel-core/src/cube/primitive.rs) | Topology modes; culling modes |
| [Voice constants](../../../crates/pyxel-core/src/voice.rs) | Tuning-reference pair; fixed-point rounding bias; pitch-table parameters and storage |
| [MML constants](../../../crates/pyxel-core/src/mml_parser.rs) | Reusable integer domains; command-specific ranges; default musical values |
| [MML parser state](../../../crates/pyxel-core/src/mml_parser.rs) | Stream/output and note parameters; default-command emission flags; connection/repeat history |

**Reason:** These inventories expose stable responsibilities without splitting
every type, component, or flag. Matching field groups in declarations and
initializers makes their correspondence visible. Settings subgroups preserve
their reading order and density instead of becoming an alphabetized block.

## Dispatch and registration inventories

**Decision:** The [event type](../../../crates/pyxel-core/src/platform/event.rs)
groups visibility, control values, text/files, and shutdown. Keep each label
attached to its variants. The [SDL event dispatcher](../../../crates/pyxel-core/src/platform/sdl2/poll_events.rs)
separates complete event handlers; nested value-selection alternatives remain
compact.

The [MML command declaration](../../../crates/pyxel-core/src/mml_command.rs)
and [Channel command dispatch](../../../crates/pyxel-core/src/channel.rs) share
these families: playback timing; tone/volume; pitch; envelope; vibrato; glide;
note/rest; repeats. Separate families and keep related alternatives together.

The [base Python module registration](../../../crates/pyxel-binding/src/lib.rs)
groups drawable classes, audio classes, constants/variables, API functions, and
the Cube submodule. The [Cube module registration](../../../crates/pyxel-binding/src/cube/mod.rs)
separates class registration, the public-name inventory, and publication.
Keep a registration inventory continuous, including repeated macro invocations;
separate an explicitly labeled deprecated group. The
[invisible-character predicate](../../../crates/pyxel-core/src/font.rs) groups
control characters, variation selectors, bidi controls, and zero-width specials;
keep each category's ranges together. Tests use the shared
[processing groups](source-code-blank-lines.md#processing-groups) and
[test decisions](source-code-blank-lines-tests.md).

**Reason:** Related declarations and dispatch sites expose the same concepts,
while registration groups help locate exported capabilities. These are named
inventories, not exceptions to paragraphing within an implementation.
