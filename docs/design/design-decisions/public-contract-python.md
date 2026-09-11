# Python Contract Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Public
Contract policy](../design-policy.md#public-contract)

## Python Interfaces

### Legacy formats removed in 3.0

**Decision:** Pyxel 3.0 does not support the old MML grammar, `Sound.old_mml`,
or resource archives from before Pyxel 2.0. Keep the current MML grammar and
the TOML resource family, including its older supported format versions. Retain
the other deprecated Python APIs, including
`tick` and the sequence `from_list` and `to_list` methods.

**Reason:** These are the maintainer's selected compatibility removals for 3.0,
giving MML one supported grammar and resources one archive layout. The [sound
binding](../../../crates/pyxel-binding/src/sound_wrapper.rs), [MML
parser](../../../crates/pyxel-core/src/mml_parser.rs), and [resource
loader](../../../crates/pyxel-core/src/resource.rs) define the affected
interfaces. This does not remove the note, tone, volume, and effect lists or
`Sound.set`; those are distinct from the old MML grammar. Other deprecated APIs
still serve existing apps; deprecation alone does not justify removing them.

### List annotations for native sequence proxies

**Decision:** Keep these familiar list annotations in the [base
stub](../../../python/pyxel/__init__.pyi) and [Cube
stub](../../../python/pyxel/cube/__init__.pyi), although the corresponding
runtime objects can be native sequence proxies.

| Surface | Stub annotation |
| --- | --- |
| `colors` | `list[int]` |
| `images`, `tilemaps` | `list[Image]`, `list[Tilemap]` |
| `channels`, `tones`, `sounds`, `musics` | Lists of their respective public classes |
| `Tone.wavetable` | `list[int]` |
| `Sound.notes`, `Sound.tones`, `Sound.volumes`, `Sound.effects` | `list[int]` |
| `Music.seqs` | `list[list[int]]` |
| `Primitive.positions`, `Primitive.normals`, `Primitive.uvs` | `list[float]` |
| `Primitive.indices` | `list[int]` |

**Reason:** Users read these signatures in editor completion and type help.
`list[int]` immediately suggests indexing, iteration, and assignment with
integer elements. An implementation-specific name such as `NotesList` would make
users open another reference just to understand the type help.

The [sequence binding](../../../crates/pyxel-binding/src/utils.rs), [music
binding](../../../crates/pyxel-binding/src/music_wrapper.rs), and [Primitive
binding](../../../crates/pyxel-binding/src/cube/primitive.rs) retain their
implementation types. The annotation does not claim exact built-in `list`
identity or determine whether another property returns a live view or a copy;
those behavior differences belong in the relevant API description. Ordinary
lists returned by other APIs are not thereby classified as proxies.

### Integer and floating results from clamp and sgn

**Decision:** `clamp` and `sgn` use the integer path for inputs convertible to
`i64` and return Python integers. Otherwise they use `f64` extraction and return
Python floats, as implemented by the
[math binding](../../../crates/pyxel-binding/src/math_wrapper.rs).

**Reason:** The result remains usable as an integer when the operation took
integer inputs, while fractional inputs retain floating-point results. This
does not promise arbitrary-precision integer computation: integers outside the
integer path's range can take the floating path. The dispatch is separate from
the engine's ordinary `f32` math and from native failure conversion.

### Effective defaults in references and stubs

**Decision:** API references and `.pyi` signatures show effective parameter
defaults. Do not replace them with internal `None` sentinels. Use `None` only
for actual default behavior such as automatic selection or absence of a value.
Reference type labels name the public type in Python union notation (`int |
None`) and do not expose an internal sentinel either.

The reference and stub for `pyxel.init` therefore show the values selected when
fixed-default arguments are omitted: `fps=30`, `capture_scale=2`, and
`capture_sec=10`. They do not replace those values with the wrapper's `None`.

`display_scale` differs: its value is selected automatically for the display and
requested screen size. Its `None` annotation therefore denotes automatic
selection, which the parameter description explains. There is no single fixed
magnification to substitute.

The [system binding](../../../crates/pyxel-binding/src/system_wrapper.rs),
[initialization](../../../crates/pyxel-core/src/pyxel.rs), and [resource
defaults](../../../crates/pyxel-core/src/resource.rs) establish the
correspondence. Public descriptions are owned by the [API source
data](../../../web/api-reference/api-reference.json) and propagated by the
[generate_docs](../../../scripts/generate_docs) and
[generate_pyi_docstrings](../../../scripts/generate_pyi_docstrings) generators.

**Reason:** The user needs to know the actual frame rate, capture magnification,
and recording duration. An internal sentinel would force the reader to inspect
code before knowing the behavior of the documented call.

## Input

### Named keyboard constants

**Decision:** Keep the selected keyboard constant set in
[platform keys](../../../crates/pyxel-core/src/platform/key.rs), its
[Python exports](../../../crates/pyxel-binding/src/constant_wrapper.rs), and
the [base stub](../../../python/pyxel/__init__.pyi). Do not restore the omitted
special-key family, such as `KEY_AUDIOPLAY`, solely to mirror every SDL key.

**Reason:** Defining a public constant for every specialized key would expand
the API without a corresponding common use. This is the maintainer's selected
scope for that family; it does not authorize deleting existing constants based
on how often they appear in this repository.

### Combined modifier keys

**Decision:** `KEY_SHIFT`, `KEY_CTRL`, `KEY_ALT`, and `KEY_GUI` stay pressed
while either corresponding side is held. Pressing the second side does not
produce another combined press; releasing one side does not release the combined
key while the other remains held. The side-specific keys retain their own
events.

**Reason:** A combined key represents either side, not the most recent side's
event. The [SDL event
handler](../../../crates/pyxel-core/src/platform/sdl2/poll_events.rs) uses each
queued event's modifier state to preserve transitions even when several events
are processed together. Reading only the final keyboard state would lose those
intermediate transitions.

## Apps and CLI

### App metadata and execution compatibility

**Decision:** When writing app metadata, `title` and `author` are required;
other fields are optional. The launcher still runs existing apps with no
metadata or with either field missing.

**Reason:** The [distribution guide](../../../web/user-guide/user-guide.json)
describes what an author should provide. Making that documentation requirement
a check at app startup would prevent existing apps from running. The
[metadata reader and launcher](../../../python/pyxel/cli.py) therefore keep
execution independent of metadata completeness.

### CLI execution and reset context

**Decision:** Run scripts and extracted apps as `__main__`, with their script
directory available for sibling imports. Preserve the CLI arguments in
`sys.argv`; `pyxel.init` changes the working directory to its caller's
directory. For native reset, capture the interpreter, original command line, and
working directory before that change, so relative launch arguments can be
reused.

**Reason:** The [CLI](../../../python/pyxel/cli.py) and
[system binding](../../../crates/pyxel-binding/src/system_wrapper.rs)
have separate responsibilities. Rewriting arguments to imitate a direct script
invocation would change what existing apps observe. Restarting a relative
`pyxel play` command from its extracted directory would lose the original app
path. [Execution tests](../../../python/tests/test_pyxapp_execution.py) check
those relationships through the actual command; web runtime reset has its own
lifecycle.

### Directory links in packaging and watching

**Decision:** Follow directory links in the shared [CLI
traversal](../../../python/pyxel/cli.py), retaining each app-relative alias and
stopping when a resolved directory is already an ancestor. Package exclusion
filters and startup-script path requirements remain separate from traversal.

**Reason:** Packaging and watching then discover linked files the program can
use, while ancestor checks prevent cycles. A global visited-directory set would
incorrectly omit a second alias of the same directory.

### Command errors and application failures

**Decision:** The [CLI](../../../python/pyxel/cli.py) diagnoses its own command
and app-layout prerequisites with a concise message and nonzero exit. Once it
executes an application, leave that program's exceptions, traceback, and
`SystemExit` to Python. The watcher keeps application execution in its child
process and remains available for subsequent source changes.

**Reason:** A command error needs invocation guidance; an application failure
needs its original code location and exit semantics. Catching both at the CLI
entry point would obscure that distinction. Optional metadata and disappeared
watch files are ordinary absence, not additional user-facing failures.

### App packaging and source preservation

**Decision:** Write the app's startup marker into the archive without creating
or replacing a marker in the source directory. Build the archive beside its
destination and replace the destination only after the archive is complete.
Keep package exclusion rules separate from directory traversal.

**Reason:** Packaging must not overwrite a source marker or destroy an existing
app when reading another input fails. The
[packager](../../../python/pyxel/cli.py) can preserve both by writing a
temporary archive and generating its own marker entry. Dotfiles, `__pycache__`
components, GIF/ZIP files, and the output app are excluded from package inputs;
an ancestor directory's name must not accidentally exclude an app-relative file.
The chosen startup script must survive those filters. These are package rules,
not rules for what the watcher observes or what the Python runtime can load.

### Portable app startup paths

**Decision:** Write startup markers with archive-style `/` separators. Resolve
existing markers as native paths first, then try Windows-style separators where
appropriate. Require the resolved startup file to stay within its extracted app.

**Reason:** Apps move between operating systems, but a backslash can also be a
literal POSIX filename character. The
[startup-path resolver](../../../python/pyxel/cli.py) preserves that native name
before trying the portable interpretation. Absolute or escaping pointers do not
identify a script belonging to the app. Marker reading strips surrounding
whitespace, so packaging rejects a startup spelling that would change during
that round trip. These path checks do not make executing a Pyxel app a sandbox.

## Resources and Capture

### Screenshot and screencast arguments

**Decision:** Keep the filename-first `screenshot` and `screencast` signatures.
Both can be called without arguments, generating a filename and using the
configured capture scale. Callers specify a scale with `scale=2`, for example;
an integer first argument is not a second positional convention for scale.

**Reason:** The filename-first API is an adopted public interface. Adding
type-dependent positional interpretations would make the same argument position
mean different things. The [resource
binding](../../../crates/pyxel-binding/src/resource_wrapper.rs) and [capture
settings](../../../crates/pyxel-core/src/resource.rs) own the signatures and
configured defaults.

### GIF dimensions before opening the output

**Decision:** Check scaled GIF dimensions before opening the output file in
[screencast saving](../../../crates/pyxel-core/src/screencast.rs). This
guarantee concerns unsupported dimensions; it does not make every later I/O
failure transactional.

**Reason:** GIF dimensions must fit `u16`; narrowing an oversized value would
change the dimensions, and opening the file first would truncate an existing
destination. Rejecting that case first preserves both the destination and the
captured frames so the caller can retry with a smaller scale.

### Palette lookups when saving images

**Decision:** `screenshot`, `screencast`, and `Image.save` write a pixel value
beyond the current palette as the palette's last color, matching the display. An
empty palette remains a native failure with the renderer's `Number of colors
must be between 1 and 256` diagnostic; the list operations of `colors` do not
gain an emptiness check.

**Reason:** A capture records what the screen shows, and the
[renderer](../../../crates/pyxel-core/src/graphics.rs) samples the palette
texture with `CLAMP_TO_EDGE`, so out-of-range pixels display as the last color.
Drawing accepts any pixel value as data, so the
[image](../../../crates/pyxel-core/src/image.rs) and
[screencast](../../../crates/pyxel-core/src/screencast.rs) writers follow the
display rather than validating stored pixels. `set_icon` validates its input
because it converts string data to RGBA at construction; that is an input
boundary, not a save. Palette size is a runtime invariant: rendering checks the
1 to 256 range, and saving reports an empty palette with the same diagnostic.
Refusing `clear()` or an empty assignment would give `colors` a check no Python
list has, guarding a state a program only passes through while rebuilding the
palette; the [sequence proxies](../../../crates/pyxel-binding/src/utils.rs) keep
list behavior instead.

### Excluded resource banks during saving

**Decision:** Saving selected resource banks does not convert banks excluded by
the caller. A sound-only save can succeed even when excluded image or tilemap
data is invalid.

An included zero-width or zero-height image or tilemap retains its native
conversion failure. That conversion already precedes opening the destination;
preserving the destination therefore does not require a new Python exception.
Do not add an export-side dimension-validation layer without an established
Python recovery need.

**Reason:** Exclusion selects the data to save, not just the fields to emit
after converting every bank. Inspecting excluded data would make an unrelated
resource prevent the requested save. The [resource
conversion](../../../crates/pyxel-core/src/resource_data.rs) therefore applies
each exclusion before converting that bank. This does not change how invalid
data in an included bank is handled.

## Audio

### Switching a Sound between MML and PCM

**Decision:** Parse new MML or load a new PCM file successfully before clearing
the other mode in [Sound](../../../crates/pyxel-core/src/sound.rs).
If that preparation fails, leave the previous sound data in place. The
no-argument `Sound.mml()` and `Sound.pcm()` calls still explicitly clear their
respective modes.

**Reason:** A failed replacement should not discard usable sound data when
preserving it requires only changing the operation order. The parser and loader
already return the replacement data, so preparing it first avoids data loss
without backup or restoration machinery.

**Boundary:** This decision covers `Sound.mml(code)` and `Sound.pcm(filename)`.
It adds no copied backup, rollback framework, or exception layer, and makes no
general atomicity promise for every Sound editing operation. Successful
replacement and existing error diagnostics retain their meanings.

### Stored music sequences and playback channels

**Decision:** [Music.set](../../../crates/pyxel-core/src/music.rs) retains every
supplied sequence, including sequences beyond the current playback-channel
count. It pads shorter input with empty sequences. Real-time
[playm](../../../crates/pyxel-core/src/audio.rs) uses the available channels;
their count does not limit how much music data can be stored.

**Reason:** Limiting simultaneous playback is not a reason to delete a song's
later sequences. Preserving all supplied data also agrees with direct sequence
editing and resource loading.

**Boundary:** [Resource
serialization](../../../crates/pyxel-core/src/resource_data.rs) retains its
established omission of trailing empty sequences, and loading normalizes the
same empty tail. This changes neither the indices nor the contents of nonempty
sequences; empty sequences before a later nonempty one remain. The [resource
format](../../../docs/pyxres-format.md) therefore need not preserve the exact
number of trailing unused channels. The real-time channel limit does not
introduce a new limit on offline `Music.save` rendering.

### Music channel views after removal

**Decision:** A `Music.seqs` inner view addresses a channel index in its parent
Music. If that index no longer exists, its list operations raise `IndexError`
with `list index out of range`, including when Python argument conversion
removes the channel before the operation accesses it.

**Reason:** This is the maintainer's selected handling of an unavailable Music
channel view. The [music
binding](../../../crates/pyxel-binding/src/music_wrapper.rs) retains the parent
and index, and the [sequence
binding](../../../crates/pyxel-binding/src/utils.rs) checks availability at the
point of access. The view does not keep a removed sequence as independent data.
Ordinary element indexing, conversion errors, and releasing locks before Python
allocation remain separate requirements.

### Same-pitch MML ties

**Decision:** Preserve the parser's merging of tied notes at the same pitch into
one note with their combined duration. A note followed by `&` sounds at full
gate; the merged note ends with the gate ratio of the Q in effect at its final
tied note, so `Q50 C4& C4` sounds like `Q50 C4&4`. Keep the released command
ordering rather than introducing control changes inside the combined note.

**Reason:** The Japanese and English [MML
reference](../../../web/mml-studio/mml-commands.json) describe same-pitch ties
as one note. The [parser](../../../crates/pyxel-core/src/mml_parser.rs) extends
the earlier note's duration. For example, in `C4& T140 C4`, the tempo command
remains after that combined note; it does not divide the note into two tempo
sections. Merging and command ordering are the v2.9.9 behavior. A tie only
suppresses the key-off between the joined notes, not the articulation of the
note that ends them, so the merged note's gate matches the length-only tie.

**Boundary:** The reference does not specify scheduling a control command at
its written position within a tied note. Adding that capability would require
a separate MML language decision. Different-pitch slurs and length-only ties
retain their own documented behavior.

### The Python BGM generator and Composer JSON

**Decision:** Treat `pyxel.gen_bgm(preset, transp, instr, seed, play=False)` as
the public Python generation interface. Its
[reference](../../../web/api-reference/api-reference.json) defines the preset,
transposition, instrumentation, seed, and optional playback inputs. Raw Composer
JSON is a separate interface, not another input form of `pyxel.gen_bgm`.

**Reason:** The [audio
binding](../../../crates/pyxel-binding/src/audio_wrapper.rs) exposes those
arguments. The shared
[generator](../../../crates/pyxel-core/src/bgm_generator.rs) gives Composer
separate JSON entry points, which the normal Pyxel build excludes through its
`pyxel_core` configuration. Their presence in shared source or tests does not
add them to the Python API.

**Boundary:** This distinction does not exempt the shared generator from review
or establish that every Composer document is valid. Check Composer inputs and
producer behavior in their own interface context. Do not add validation or
expand accepted Python inputs merely to handle arbitrary edited JSON that the
Python function does not accept.

## Editor

### Editor save failures

**Decision:** A failed editor save leaves the editing session, resource data,
and undo history available for a retry. Report the save operation and its reason
once, and retain a visible failure notice until saving succeeds.

**Reason:** An unavailable destination must not discard work held in memory.
The [editor](../../../python/pyxel/editor/app.py) owns that recovery;
`pyxel.save` keeps its existing exception contract for other callers. Catch the
save call's ordinary exception at the action handler, without suppressing
unrelated update errors or changing the loader's behavior. This does not promise
that every failed file write preserves the previous on-disk file.

### Sound editor display and speed limits

**Decision:** The sound editor shows the standard four tone symbols and uses `?`
for tones outside that set. Unsupported tone, volume, effect, and speed values
are displayed as `?` without rewriting their data or expanding the input keys.
The speed control accepts 1–99 when the user edits it; viewing or switching
banks preserves speeds outside that range.

**Reason:** The editor offers a deliberately small set of controls. Preserving
unsupported data lets it coexist with the wider API without changing an app's
sound merely by opening it. The speed picker therefore distinguishes syncing an
existing value from stepping that value with its buttons. Its editing range is
not a global API cap or permission for unrelated normalization.
The [sound editor](../../../python/pyxel/editor/sound_editor.py) and
[field display](../../../python/pyxel/editor/sound_field.py) are the affected
implementation sites.

## Cube

### Imported motion and scene placement

**Decision:** Applying a motion assigns sampled local transforms to its
matching imported nodes. Keep application placement and scale on a separate
parent when they must remain independent of that animation.

**Reason:** Imported transforms describe animated model parts; an application's
placement describes the model's position in its scene. Composing each sampled
frame with the node's previous transform would accumulate movement and scale.
The [motion binding](../../../crates/pyxel-binding/src/cube/node.rs) replaces
the sampled transforms; a parent preserves scene placement without changing the
motion contract. The [collision
sample](../../../python/pyxel/examples/cube/c05_3d_collision.py) uses this
separation for its chosen model scale.

### Zero-mass Cube bodies

**Decision:** A Cube collider with `mass=0` is not pushed by other bodies, but
an assigned velocity can move it. Zero mass does not mean that the scene must
ignore its velocity.

**Reason:** Moving platforms need prescribed movement without being displaced
by the bodies they carry. The
[collider](../../../crates/pyxel-core/src/cube/collider.rs) and
[scene update](../../../crates/pyxel-core/src/cube/scene.rs) separate contact
response from movement. This records zero-mass behavior, not the whole physics
contract.
