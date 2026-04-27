<!-- This file is generated from web/mml-studio/mml-commands.json. -->

# Pyxel MML Commands

*This document was auto-generated from the [Pyxel MML Commands](https://kitao.github.io/pyxel/web/mml-studio/) web page, which also offers multilingual support.*

| Command | Description |
| --- | --- |
| `T <bpm>` | Sets the tempo (BPM). Range: 1-. Default is 120. |
| `Q <gate_percent>` | Sets the gate time as a percentage. Range: 0-100. 100 means the note is played with no gap, 0 means it is not played at all. Default is 80. |
| `@ <tone>` | Sets the tone. Range: 0-3 (by default: 0:Triangle / 1:Square / 2:Pulse / 3:Noise). Default is 0. |
| `V <vol>` | Sets the volume. Range: 0-127. Default is 100. |
| `K <key_offset>` | Sets the transpose amount in semitones. 12 raises the pitch by one octave. Default is 0. |
| `Y <offset_cents>` | Sets detune in cents. 100 raises by a semitone, -100 lowers by a semitone. Default is 0. |
| `@ENV <slot>` | Switches the envelope (volume curve) slot. Range: 0-. 0 turns it off. |
| `@ENV <slot> { init_vol, dur_ticks1, vol1, dur_ticks2, vol2, ... }` | Slot range: 1-. Sets and switches to the specified envelope slot. Slot 0 cannot be specified. Inside `{ }`, specify "initial volume (once)", then repeat "duration (tick), volume (vol)". 1 tick is 1/48 of a quarter note. Example: `@ENV1 { 30, 20, 100, 50, 0 }` |
| `@VIB <slot>` | Switches the vibrato (pitch modulation) slot. Range: 0-. 0 turns it off. |
| `@VIB <slot> { delay_ticks, period_ticks, depth_cents }` | Slot range: 1-. Sets and switches to the specified vibrato slot. Slot 0 cannot be specified. Inside `{ }`, specify "delay (tick), period (tick), depth (cent)". 1 tick is 1/48 of a quarter note. Example: `@VIB1 {24, 12, 100}` |
| `@GLI <slot>` | Switches the glide (pitch slide) slot. Range: 0-. 0 turns it off. |
| `@GLI <slot> { offset_cents, dur_ticks }` | Slot range: 1-. Sets and switches to the specified glide (pitch slide) slot. Slot 0 cannot be specified. Inside `{ }`, specify "initial pitch offset (cent), time to return to 0 (tick)". 1 tick is 1/48 of a quarter note. Specifying `*` for each parameter automatically applies the pitch offset as the difference from the previous note and the return time as the playback duration of each note, respectively. Example: `@GLI1 { -100, 24 }` |
| `O <oct>` | Sets the octave. Range: -1-9. `O4`'s A is 440 Hz. Default is 4. |
| `>` | Raises the octave by 1 (max 9). |
| `<` | Lowers the octave by 1 (min -1). |
| `L <len>` | Sets the default note/rest length. Range: 1-192. `L4` is a quarter note, `L8` is an eighth note, and `L12` is a quarter note triplet. Default is 4. |
| `C/D/E/F/G/A/B` | Plays the specified note. You can specify the length after the note, e.g., `F16`. |
| `R` | Rest. Range: 1-192. You can specify the length after the rest, e.g., `R8`. |
| `#` | Raises the note by a semitone. |
| `+` | Raises the note by a semitone. |
| `-` | Lowers the note by a semitone. |
| `.` | Dotted note/rest. Extends the length by half. Can be repeated for multiple dots. |
| `&` | Tie/Slur. When connecting the same pitch, it ties the notes into one. When connecting different pitches, it plays them legato (no gap). You can also specify only the length, e.g., `C4&16`. |
| `[` | Start of repeat section. |
| `] <count>` | End of repeat section. Count range: 1-. Repeats the section between `[` and `]` the specified number of times. If omitted, repeats infinitely. Nested repeats are supported. |
