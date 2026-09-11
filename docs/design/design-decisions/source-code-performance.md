# Performance Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#performance)

## Rust

### Pixel, palette, and tile storage

**Decision:** Keep the distinct storage types used by
[Image](../../../crates/pyxel-core/src/image.rs) and
[Tilemap](../../../crates/pyxel-core/src/tilemap.rs).

| Data | Representation | Reason |
| --- | --- | --- |
| Palette index (`Color`) | `u8` | One byte per indexed pixel |
| Packed RGB (`Rgb24`) | `u32` | Three color components need 24 bits |
| Image-tile coordinate (`ImageTileCoord`) | `u16` | Each tile stores two coordinates, distinct from a pixel's palette index |

**Reason:** These widths also reach Python through `data_ptr`: the [image
binding](../../../crates/pyxel-binding/src/image_wrapper.rs) exposes `c_uint8`
elements, while the [tilemap
binding](../../../crates/pyxel-binding/src/tilemap_wrapper.rs) exposes two
`c_uint16` elements per tile. Unifying the integer types would change buffer
layout and memory use, not merely coding style.

The editor's displayed ranges do not define these storage widths. Likewise,
the name `Rgb24` does not justify rejecting every value with a high byte;
[capture conversion](../../../crates/pyxel-core/src/screencast.rs) extracts the
three RGB components from the stored value.

### Audio values and their units

**Decision:** Preserve the distinctions between compact editable sound data
and the values used for synthesis.

| Data | Representation | Reason |
| --- | --- | --- |
| Legacy sound notes | `i8` | Negative values represent rests |
| Legacy tones, volumes, and effects | `u8` | Separate compact, unsigned component lists |
| Legacy sound speed | `u16` | Ticks per legacy note or rest, independent of the editor control's range |
| Editable tone samples | `u32` | Raw sample values are normalized using the tone's selected bit depth |
| Channel detune | `i32` cents | A signed integral control, converted to fractional semitones for synthesis |
| Tone/channel gain and MML modulation controls | `f32` | Fractional multipliers and pitch offsets before fixed-point mixing |

**Reason:** The [sound](../../../crates/pyxel-core/src/sound.rs),
[tone](../../../crates/pyxel-core/src/tone.rs), and
[channel](../../../crates/pyxel-core/src/channel.rs) implementations establish
these units and conversions. In contrast to legacy notes, an [MML
command](../../../crates/pyxel-core/src/mml_command.rs) has separate `Note` and
`Rest` variants, so its unsigned note field needs no negative rest sentinel.

These are representation choices, not a requirement to add range checks.
The Python sound lists accept their storage types, and a tone's editable
wavetable can contain values above its selected bit depth; normalization occurs
when producing the waveform. Editor limits and parser syntax are separate
contracts.

### Storage widths and arithmetic intermediates

**Decision:** Keep the storage type separate from the width needed to combine
values or accumulate time.

| Calculation | Representation | Reason |
| --- | --- | --- |
| [Canvas coordinates](../../../crates/pyxel-core/src/canvas.rs) | Camera positions: `i32`; dimensions: `u32`; differences and clipping intermediates: `i64` where needed; buffer indices after clipping: `usize`; ordinary transformed coordinates: `f32` | Positions can be offscreen, dimensions are nonnegative, and differences between two valid coordinates can exceed `i32`. |
| [Rectangle intersections](../../../crates/pyxel-core/src/rect_area.rs) | Stored bounds: `i32`; extents: `u32`; logical endpoints: `i64` | A saturated stored endpoint cannot recover the original extent. |
| [Font metrics](../../../crates/pyxel-core/src/font.rs) | Glyph advances and offsets: `i32`; text widths and positioned pixels: `i64`; one BDF bitmap row: `u32` | Several valid metrics can sum beyond `i32`; the bitmap row's width is independent of text width. |
| [Playback time](../../../crates/pyxel-core/src/channel.rs) | Short clock intervals: `u32`; accumulated playback: `u64`; [repeat-duration arithmetic](../../../crates/pyxel-core/src/mml_parser.rs): checked `u128` intermediates | Accumulated time needs a wider range; repeat durations are combined without expanding every repeat. |
| [Audio mixing](../../../crates/pyxel-core/src/voice.rs) | Quantized waveform: `i16`; amplitude and fixed-point gain: `i32`; products before shifting: `i64` | Products need a wider intermediate. Fractional modulation and long-running phase calculations remain separate from the integer mixer. |
| [Native scheduling](../../../crates/pyxel-core/src/platform/sdl2/platform_sdl2.rs) | Clock readings: `u64`; scheduling times: `f64` | Retain timing precision over long uptimes without widening public frame counters or every short elapsed-time value. |

**Reason:** Each wider calculation follows from the operation performed on its
inputs, not a preference for large types. Promoting every stored field would
change memory use and conversions without addressing the same problem.
Conversely, narrowing an intermediate to match its inputs can lose a valid
result. These choices do not create new accepted-input limits or Python error
classes.

### Primitive raster precision and clipping

**Decision:** Preserve ordinary `f32` pixel rounding in
[Canvas](../../../crates/pyxel-core/src/canvas.rs). Select wider calculations at
these boundaries:

| Primitive | Use `f64` when |
| --- | --- |
| Lines and triangles | Any camera-adjusted x or y coordinate has absolute value greater than `2^23` |
| Circles, including the [Cube rasterizer](../../../crates/pyxel-core/src/cube/raster.rs) | Radius is at least `2^22` |
| Ellipses | Any camera-adjusted bounding edge has magnitude at least `2^22` |

Keep integer offsets wide until clipping. Retain the interpolation origin,
rounding convention, and curve boundary bias. Restrict iteration to samples
that can reach the clip rectangle, accounting for both symmetric axes of curves.

**Reason:** `f32` spacing reaches one pixel at `2^23` and half a pixel at
`2^22`. Large curves lose fractional boundary information, while lines spanning
large coordinates need to retain individual pixel steps. Widening every
calculation would also change ordinary raster rounding. These boundaries select
the wider path while preserving the ordinary path; they are neither input limits
nor a claim that every intermediate below them is exact. Clipping the iteration
range reduces offscreen work without restarting interpolation or changing which
visible samples contribute.

### Shared graphics and audio ownership

**Decision:** Graphics resources use `Rc<RefCell<T>>`; mutable audio resources
use `Arc<Mutex<T>>`, as defined by the
[resource macros](../../../crates/pyxel-core/src/utils.rs).

Immutable [MML command](../../../crates/pyxel-core/src/mml_command.rs) snapshots
use `Arc` to share command and envelope data without copying it for each
playback. [Sound](../../../crates/pyxel-core/src/sound.rs) owns the editable
source and cached snapshot; [Channel](../../../crates/pyxel-core/src/channel.rs)
keeps its own playback position.

**Reason:** Graphics resources share mutable state locally. Audio resources are
also accessed during the [audio
callback](../../../crates/pyxel-core/src/audio.rs), requiring shared ownership
and synchronization across that boundary. Their `Rc`-prefixed aliases identify
shared resources rather than promising the concrete `std::rc::Rc` type.

### Cache invalidation for editable sources

**Decision:** Compare source contents when callers can edit them without an
observed mutation hook. Use a revision only where every relevant mutation
updates it, and include external inputs that affect the derived result.

**Reason:** [Sound's command cache](../../../crates/pyxel-core/src/sound.rs)
depends on editable lists, speed, and tone modes; its MML commands instead have
an owned revision. [Tone](../../../crates/pyxel-core/src/tone.rs) compares
waveform inputs before publishing a new revision to playback. Replacing those
comparisons with an incomplete dirty flag would reuse stale output. This
distinction does not require a common cache abstraction or copying immutable
playback data.

### Cube numeric representations

**Decision:** Keep `f32` for Cube's `Vec3`, `Mat4`, and `Quat` components,
geometry attributes, ordinary geometry calculations, depth buffers, and motion
values. Preserve the distinction between these values and integral indices or
pixel coordinates.

World geometry uses world units and projected coordinates use pixels. Angle
arguments and angle results use degrees; imported animation times are converted
from seconds to frames using `fps`; playback speed is frames per update. Mesh
parents and BVH child links use signed indices to represent `-1` sentinels,
while array access uses `usize`. These distinctions should survive any local
change in width.

Widen intermediates when the calculation needs it. Circle precision and `i64`
offsets until clipping follow the [primitive raster
decision](#primitive-raster-precision-and-clipping). Representation choices do
not add parameter restrictions.

**Reason:** The [Python math
bindings](../../../crates/pyxel-binding/src/cube/vec3.rs) convert components to
`f32`, and the [GLB importer](../../../crates/pyxel-core/src/cube/glb_parser.rs)
produces matching position, normal, texture-coordinate, and animation arrays.
Keeping that representation through calculation preserves their rounding
behavior and four-byte component storage. Widening the whole pipeline would
change memory use and numerical results; Python's `float` annotation alone does
not require such a change.

### Cube value calculations

**Decision:** Keep temporary transforms and collision bounds as values where
the calculation needs no shared identity. Preserve immutable Python math
objects and shared mutable scene resources.

**Reason:** The [matrix value
kernels](../../../crates/pyxel-core/src/cube/mat4.rs) produce fixed-size results
without an `Rc` allocation for each intermediate. Vertex transforms, ancestor
composition, and [collision
bounds](../../../crates/pyxel-core/src/cube/collision.rs) use those values
repeatedly. Public arithmetic returns a new value; a mutable `Primitive` or
`Mesh` instead shares changes between its users. This distinction justifies
value kernels without requiring a wholesale rewrite of wrapper storage or
claiming that every existing `Rc<RefCell<_>>` is necessary.

### Shared primitive collision caches

**Decision:** Notify every dependent mesh when shared primitive geometry
changes, without keeping discarded dependents alive.

**Reason:** A [Primitive](../../../crates/pyxel-core/src/cube/primitive.rs) can
belong to several [meshes](../../../crates/pyxel-core/src/cube/mesh.rs). Weak
subscriptions to their dirty flags invalidate each collision cache and allow
expired subscribers to disappear. The current `Arc<AtomicBool>` representation
also keeps `Primitive` compatible with the global `OnceLock<Primitive>`
templates; it does not make the mutable scene thread-safe. Any replacement must
account for sharing, invalidation, and template storage, rather than
substituting `Rc` or `Arc` by convention alone.
