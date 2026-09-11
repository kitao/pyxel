# Rust/Python Boundary Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Public
Contract policy](../design-policy.md#public-contract)

## Errors and Failures

### Preserving established Python errors

**Decision:** Preserve the accepted inputs and error classes established in
v2.9.9 except for separately adopted changes. Convert an additional Rust failure
only when Python code needs to catch it and choose how to proceed. Otherwise,
preserve Rust-side failure handling unless a concrete requirement justifies
conversion and its added code and runtime cost.

This preserves the existing distinctions between these argument boundaries:

| Boundary | Treatment retained | Reason |
| --- | --- | --- |
| `init`, `resize`, `icon`, `Image`/`Tilemap` construction and text data | Existing `ValueError` paths | These operations already expose their own input constraints. Their presence does not authorize new limits on other operations. |
| `Sound.speed`, the speed argument of `Sound.set`, `Tone.sample_bits`, and playback start time | Existing `ValueError` paths | Keep their released parameter contracts; editor controls do not define additional API limits. |
| Sound note/tone/volume/effect strings and MML | Existing `Exception` paths | Preserve parser diagnostics and accepted syntax, including the string forms of `play` and `Channel.play`. |
| Resource selection and playback | Existing bank-index errors and playback conversions | Keep the distinctions already exposed by the overloads: non-string playback uses `ValueError`; MML playback and the `playm` core result use `Exception`. |

The relevant boundaries are owned by the
[system](../../../crates/pyxel-binding/src/system_wrapper.rs),
[audio](../../../crates/pyxel-binding/src/audio_wrapper.rs),
[channel](../../../crates/pyxel-binding/src/channel_wrapper.rs),
[sound](../../../crates/pyxel-binding/src/sound_wrapper.rs), and
[tone](../../../crates/pyxel-binding/src/tone_wrapper.rs) bindings. The image,
tilemap, and graphics bindings retain their corresponding constructor,
text-data, and resource-selection errors.

**Reason:** Released exceptions are part of the Python behavior callers can
already use. Removing them because this review found no particular caller would
itself change that behavior. For a new conversion, the possibility of imagining
a retry is insufficient: its recovery purpose must be established.

**Boundary:** Preserving a released error does not approve every current guard,
duplicate check, or newly added failure condition. Review such additions against
their own purpose and the behavior they change. Do not normalize exception
classes merely to make unrelated operations look alike.

### File-operation failures exposed to Python

**Decision:** Keep file open, read, decode, parse, write, and conversion
failures catchable at these existing boundaries.

| Operations | Exception conversion | Binding |
| --- | --- | --- |
| `load`, `save`, `load_pal`, `save_pal` | `Exception` | [Resource](../../../crates/pyxel-binding/src/resource_wrapper.rs) |
| `screenshot`, `screencast`, `user_data_dir` | `Exception` | [Resource](../../../crates/pyxel-binding/src/resource_wrapper.rs) |
| `Font(...)` | `Exception` | [Font](../../../crates/pyxel-binding/src/font_wrapper.rs) |
| `Image.from_image`, `Image.load`, `Image.save` | `Exception` | [Image](../../../crates/pyxel-binding/src/image_wrapper.rs) |
| `Tilemap.from_tmx`, `Tilemap.load` | `Exception` | [Tilemap](../../../crates/pyxel-binding/src/tilemap_wrapper.rs) |
| `Sound.pcm(filename)`, `Sound.save`, `Music.save` | `Exception` | [Sound](../../../crates/pyxel-binding/src/sound_wrapper.rs), [Music](../../../crates/pyxel-binding/src/music_wrapper.rs) |
| `Mesh.from_glb` | `ValueError` | [Mesh](../../../crates/pyxel-binding/src/cube/mesh.rs) |

**Reason:** An application needs to report an unavailable or unusable file and
continue. The editor does this when loading resources in
[App](../../../python/pyxel/editor/app.py), images in
[ImageEditor](../../../python/pyxel/editor/image_editor.py), and maps in
[TilemapEditor](../../../python/pyxel/editor/tilemap_editor.py). The same
file-failure purpose covers choosing another output location or handling a
failed export. It also requires BDF read failures to propagate through `Font`;
silently treating a read error as end-of-file could accept an incomplete font.

**Boundary:** A file operation can also fail because of its arguments or current
resource state. Its existing exception conversion does not authorize adding
dimension, duration, or reference constraints. Nor does it establish a general
promise that every failed operation leaves all state unchanged.

### Scaled PNG dimensions

**Decision:** Check multiplication of an image's dimensions by its PNG export
scale before resizing. A product that cannot fit the image library's `u32`
dimensions fails on the Rust side; it does not introduce another catchable
Python argument error.

**Reason:** In [Image.save](../../../crates/pyxel-core/src/image.rs),
multiplying width 2 by scale 2,147,483,649 would wrap back to 2 in a release
build and save the wrong size. Preventing that arithmetic error is necessary
independently of failure conversion. The existing file-error mapping does not
supply an additional recovery contract for an unrepresentable size. The
separately adopted GIF dimension check retains its own format limit and retry
behavior.

### Python conversion and sequence errors

**Decision:** Keep PyO3's argument conversion and the bindings' explicit
`PyResult` propagation. A tentative conversion used to select an overload or
implement comparison may fail without becoming the final error: preserve the
selected fallback or protocol result. Do not turn these probes into mandatory
conversions or add a blanket construction-failure wrapper.

Keep Python-standard errors for Pyxel's list operations:
`IndexError` for an unavailable element or invalid pop, `ValueError` for an
extended-slice length mismatch, and the applicable Python errors for invalid
indices and slices. Unknown module attributes raise `AttributeError`.

The [sequence binding](../../../crates/pyxel-binding/src/utils.rs) and [music
binding](../../../crates/pyxel-binding/src/music_wrapper.rs) own these
operations. Pyxel-authored standard diagnostics use CPython's exact wording;
PyO3's own diagnostics remain owned by PyO3. Do not intercept its errors solely
to rewrite their presentation.

The [math binding](../../../crates/pyxel-binding/src/math_wrapper.rs) tries integer conversion before floating-point conversion. The shared overload helpers
and sequence comparisons likewise distinguish an unsupported candidate from a
failure of the selected operation. Propagating every tentative extraction error
would prevent those supported alternatives from being reached.

**Reason:** These errors implement the Python operations offered by resource
lists, Sound sequences, `Tone.wavetable`, and `Music.seqs`. They support normal
indexing, iteration, attribute lookup, and Python conversion behavior. A Rust
panic would not implement those protocols.

**Boundary:** Ordinary list annotations alone do not decide a live view's
lifetime contract. [Music channel
views](public-contract-python.md#music-channel-views-after-removal) have an
explicit unavailable-index decision; do not infer other views' ownership or
failure behavior from their list annotation.

### Native numerical failures

**Decision:** Keep native numerical failure handling where the API has no
additional Python recovery requirement. In particular, `clamp` and `rndf` do not
gain Python-only checks for the native operations' exceptional numerical cases.

**Reason:** Adding isolated checks for NaN, infinity, or invalid ranges would
change accepted inputs and add a partial validation layer. The fact that another
API rejects a numerical argument does not supply a reason to change these APIs.
The `PyResult` used by `clamp` and `sgn` propagates their final Python
extraction errors; it does not imply that native numerical failures are wrapped.

See the [math binding](../../../crates/pyxel-binding/src/math_wrapper.rs) and
[native math](../../../crates/pyxel-core/src/math.rs).

### Normal absence and native failure ownership

**Decision:** Drawing outside the clip region, stopped playback, and spatial
queries without a hit keep their no-op, `None`, or empty-result behavior.
Missing optional palette files leave the palette unchanged. These are normal
outcomes of the respective operations, not missing error checks.

Core parsers and validators own their failure conditions and messages; the
bindings convert the returned failure at the established Python boundary.
Do not repeat those checks at each call site. Internal borrow, initialization,
built-in shader, and trusted-data invariants retain native failure handling;
they do not gain a blanket Python exception conversion. The palette's 1 to 256
color range is such an invariant, checked natively when rendering;
[saving](public-contract-python.md#palette-lookups-when-saving-images) reports
an empty palette the same way.

**Reason:** Rendering and playback repeatedly encounter empty or inactive
state. Reporting it would interrupt normal use and add work to hot paths.
Conversely, continuing after an internal invariant fails is not an established
recovery path. The existing distinctions are visible in
[drawing](../../../crates/pyxel-core/src/canvas.rs),
[playback](../../../crates/pyxel-core/src/channel.rs),
[queries](../../../crates/pyxel-core/src/cube/scene.rs), and
[palette loading](../../../crates/pyxel-core/src/resource.rs).

## Resource Access

### Image-source lookup in tilemap drawing

**Decision:** `bltm`, `bltm3d`, `Image.bltm`, and `Image.bltm3d` leave
image-source lookup to the [renderer](../../../crates/pyxel-core/src/image.rs),
preserving each drawing path's order of early returns and source access. This
applies to both tilemap objects and bank indices.

**Reason:** A zero-width draw can return without using the image source.
Rejecting an unused source first would turn a released successful no-op into an
error. Repeating the renderer's conditions in the bindings would duplicate work
and create another place for them to diverge.

**Boundary:** Validation of the `tm` argument itself remains separate. An
invalid image source that is actually dereferenced retains Rust-side failure
handling; there is no established need to add a Python recovery contract for it.

### Python reentry while accessing mutable resources

**Decision:** Complete Python conversions before taking Rust borrows or locks
that reentrant Python code may need. Release those guards before Python
allocations that can invoke callbacks or garbage collection; copy the values
needed for the result where necessary. Recheck
a resource index after Python conversion when that conversion can change its
bank.
For a sequence result, finish its native reads before allocating the Python
list. A lazy iterator over previously computed indices still accesses mutable
storage after allocation, even when it retains no borrow or lock.

**Reason:** `__index__`, iteration, and allocation-triggered garbage collection
can run Python code that accesses the same resource. Holding a borrow or mutex
across those operations can cause a borrow failure or deadlock. In `play`, for
example, converting `snd` can change `channels`, so an earlier `ch` check cannot
establish that the later access is valid. In `Music.seqs` result construction,
the Python allocation must occur after the music lock has been released.
In shared sequence reads, allocation-triggered collection can also clear the
source bank before a lazy iterator reads its next index. Collecting the native
result first prevents that invalid access; wrapped resources retain their
shared identity rather than becoming deep copies.

The [sequence binding](../../../crates/pyxel-binding/src/utils.rs), [music
binding](../../../crates/pyxel-binding/src/music_wrapper.rs), and [audio
binding](../../../crates/pyxel-binding/src/audio_wrapper.rs) apply this ordering
alongside the [ownership
decision](source-code-performance.md#shared-graphics-and-audio-ownership).

**Boundary:** Prevent reentrant access from violating Rust ownership rather than
catching the resulting panic. This ordering does not require repeated checks of
unchanged scalar values, blanket rollback, or a new exception class. A removed
Music view follows its separate [unavailable-channel
contract](public-contract-python.md#music-channel-views-after-removal).