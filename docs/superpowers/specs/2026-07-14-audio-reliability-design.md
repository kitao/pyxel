# Audio Reliability Design

## Goal

Remove silent or delayed audio edge cases without adding avoidable work to the
per-sample synthesis path. Preserve the current public behavior for valid input
and add deterministic regression coverage for every corrected boundary.

## Timing Model

Voice and channel note durations use 64-bit audio clocks end to end. Rendering
continues to process bounded 32-bit chunks, but no duration is narrowed before a
chunk is selected. Additions at interpolation boundaries use saturating
arithmetic so a valid long note cannot wrap to a short or silent note.

PCM and synthesized playback share the same output timeline. Rendering splits a
bounded output step at the first channel mode transition, mixes only the samples
consumed in that span, then reevaluates channel modes. This removes the current
step-sized PCM-to-synth delay while retaining block processing away from a
transition.

## Duration and Seeking

Seconds supplied to playback must be finite and non-negative before conversion
to audio clocks. Invalid values return the same parameter-shaped `ValueError`
from the global and channel Python APIs. Seeking remains chunked and must either
consume clocks or terminate, preventing an unbounded loop.

MML duration calculation evaluates a repeat body as a duration and tempo-state
transition. A finite repeat is fast-forwarded with checked arithmetic instead of
executing each iteration. Nested repeats use the same evaluator, and an
unrepresentable duration reports no finite total rather than hanging.

## Saving and Errors

A positive save duration that rounds to zero output samples is rejected before
file creation. `Sound.save` and `Music.save` use the same message. Invalid speed
values from `Sound.set` and the speed property use the same `ValueError` family.

## Tests

Rust unit tests cover long note duration, deterministic long seeks, exact mode
transition timing, and large finite repeat evaluation. Python tests cover public
exception types and messages, non-finite playback positions, and zero-sample
save rejection. Render references remain the audible regression layer.

The Python long-seek test does not assert a single immediate `play_pos` value,
because the audio thread may consume one buffer after playback starts. Exact
seek arithmetic is asserted in the deterministic Rust layer instead.

## Policy Review

The complete uncommitted audio diff is reviewed against
`docs/coding-policy.md`, including hot-path cost, sibling wrapper structure,
error-message families, comment ownership and historical wording, deterministic
tests, and release-note accuracy. Formatting, native and WebAssembly lint, and
the full test suite are required completion gates.
