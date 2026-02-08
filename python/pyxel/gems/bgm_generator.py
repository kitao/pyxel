"""
Pyxel BGM Generator

Based on 8bit BGM Generator by frenchbread:
https://github.com/shiromofufactory/8bit-bgm-generator
"""

from __future__ import annotations

import random
from typing import Dict, List, Optional, Sequence, Tuple

BARS_COUNT = 8

# Tone IDs used by presets (Pyxel tone indices)
TONE_CANDIDATES = [11, 8, 2, 10, 6, 4]
LIST_MELO_LOWEST_NOTE = [28, 29, 30, 31, 32, 33]
LIST_MELO_DENSITY = [0, 2, 4]
LIST_BASE_QUANTIZE = [12, 13, 14, 15]

PATTERN_BASIC = 0
PATTERN_FINAL = 1

CHORD_LOC = 0
CHORD_NOTES = 1
CHORD_REPEAT = 2

PRESET_SPEED = 0
PRESET_CHORD = 1
PRESET_BASE = 2
PRESET_BASE_QUANTIZE = 3
PRESET_DRUMS = 4
PRESET_MELO_TONE = 5
PRESET_SUB_TONE = 6
PRESET_MELO_LOWEST_NOTE = 7
PRESET_MELO_DENSITY = 8
PRESET_MELO_USE16 = 9
PRESET_INSTRUMENTATION = 10

# 16-step bass patterns per bar:
# '.' = rest, '0' = stop, '1'-'4' = degree
BASS_PATTERNS = [
    ("4.....4.....0.44", "4.....4.....44.."),
    ("2.3.4.3.2.3.4.3.", "2.3.4.3.2...4..."),
    ("440...440...2...", "440...440...442."),
    ("0.2.4.2.0.2.4.2.", "0.2.4.2.2.4.2.4."),
    ("2.402.402.402.40", "2.402.402.40242."),
    ("2034203420342034", "2034203420444440"),
    ("2044204420442044", "2044204420442.4."),
    ("4.444.444.444.44", "4.444.444.442.22"),
]

# Chord progression:
# loc, notes, repeat (notes = 12-digit string per semitone)
CHORD_PROGRESSIONS = [
    # Ⅰ - Ⅶ♭
    [
        (0, "209019030909", None),
        (16, "901093090920", None),
        (32, "209019030909", None),
        (48, "901093090920", None),
        (64, None, 0),
        (80, None, 1),
        (96, "209019030909", None),
        (112, "901093090920", None),
        (120, "901099010902", None),
    ],
    # Ⅳ - Ⅴ／Ⅳ
    [
        (0, "309092090109", None),
        (16, "903092010901", None),
        (32, "309092090109", None),
        (48, "903092010901", None),
        (64, "909209030930", None),
        (80, "309201090190", None),
        (96, "909209030930", None),
        (112, "309201090190", None),
    ],
    # Ⅰ - Ⅴ - Ⅵ♭ - Ⅶ♭
    [
        (0, "209019030909", None),
        (16, "903099020901", None),
        (32, "109309092090", None),
        (48, "901903099020", None),
        (64, None, 0),
        (80, None, 1),
        (96, "109309092090", None),
        (104, "901903099020", None),
        (112, "209091030909", None),
        (120, "209019030909", None),
    ],
    # ⅣＭ７ - ⅢＭ７ - Ⅵ７ - Ⅰ
    [
        (0, "309012090109", None),
        (16, "901020901903", None),
        (32, "109039010209", None),
        (48, "209019030909", None),
        (64, None, 0),
        (80, None, 1),
        (96, "901039090209", None),
        (112, "019039010209", None),
    ],
    # Ⅵｍ - Ⅴ - Ⅳ - Ⅴ
    [
        (0, "109039090209", None),
        (16, "903099020901", None),
        (32, "309092090109", None),
        (48, "903099020901", None),
        (64, None, 0),
        (80, None, 1),
        (96, None, 2),
        (112, "903099020901", None),
        (120, "903090902901", None),
    ],
    # Ⅵｍ - Ⅳ - Ⅴ - Ⅰ
    [
        (0, "109039090209", None),
        (16, "309092090109", None),
        (32, "903099020901", None),
        (48, "209019030909", None),
        (56, "109019030902", None),
        (64, None, 0),
        (80, None, 1),
        (96, None, 2),
        (112, "209019030909", None),
        (120, "903093090302", None),
        (124, "903029030903", None),
    ],
    # Ⅵｍ - Ⅱ
    [
        (0, "109030909209", None),
        (16, "902090109309", None),
        (32, "109030909209", None),
        (48, "902090109309", None),
        (64, None, 0),
        (80, None, 1),
        (96, "109030909209", None),
        (112, "902090109309", None),
        (120, "909020901903", None),
    ],
    # Ⅲｍ７ - ⅣＭ７ - Ⅴ６ - Ⅵｍ９
    [
        (0, "901029010903", None),
        (16, "309012090109", None),
        (32, "903019020901", None),
        (48, "109039030201", None),
        (64, None, 0),
        (80, None, 1),
        (96, "903019020901", None),
        (112, "019030909209", None),
    ],
]

# Drum patterns
DRUM_PATTERNS = [
    ("1000000000001000", "1000000000001030"),
    ("1000001000001000", "1000001000007750"),
    ("3030003330300033", "3030003330303013"),
    ("1000001000003330", "1000001000003310"),
    ("1000301010003013", "1000301010003333"),
    ("3033303330333033", "3033303330336655"),
    ("3033203330332033", "3033203330332032"),
    ("1033203310332033", "1033203310332220"),
]

# Style presets fields:
# speed, chord, base, base_quantize, drums, melo_tone, sub_tone
# melo_lowest_note, melo_density, melo_use16, instrumentation
STYLE_PRESETS = [
    (216, 0, 4, 14, 4, 0, 0, 28, 2, True, 3),
    (216, 1, 6, 12, 5, 3, 3, 28, 4, True, 3),
    (312, 2, 1, 15, 0, 5, 5, 30, 2, False, 0),
    (276, 3, 2, 15, 3, 4, 4, 28, 0, False, 3),
    (240, 4, 0, 14, 2, 0, 1, 29, 2, False, 3),
    (216, 5, 3, 14, 4, 1, 1, 30, 2, True, 2),
    (192, 6, 5, 13, 6, 0, 0, 28, 4, True, 1),
    (168, 7, 7, 15, 7, 3, 3, 28, 4, True, 3),
]
RHYTHM_PATTERN_STRINGS = [
    "0....0000.......",
    "...0..0.0...0...",
    "0..0..0.0.......",
    "-.......-.......",
    "0....0000.......",
    "...0..0.0..0..0.",
    "0.....0...000...",
    "0...............",
    "0....0000.......",
    "....0...0...0...",
    "0......0...000...",
    "0.........0.00..",
    "0.........0.0.0.",
    "0.....0...000...",
    "0...............",
    "-.......-.0.0.0.",
    "0.....0.....0...",
    "0..0..0....00...",
    "0.....0.....0...",
    "........-.000000",
    "0..0..0.0...0...",
    "0..0..0.0..0..0.",
    "0...............",
    "........-0000000",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.........0.0.0.",
    "0...0...0.....0.",
    "0.......0...0...",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.........0.0.0.",
    "0...0...0.....0.",
    "0...............",
    "-...............",
    "0..0..0.0.......",
    "-.....000..0..0.",
    "0..0..0.0.......",
    "...0..0.0...0...",
    "0.....0.....0...",
    "0.0.0.0...000...",
    "0..0..0.0.......",
    "-.....000..0..0.",
    "0..0..0.0.......",
    "...0..0.0...0...",
    "0.....0.....0...",
    "0.0.0.0...000...",
    "0.....000.....00",
    "0.000.......0.0.",
    "0.....000.....00",
    "0.000...........",
    "0.....000.....00",
    "0.000.......0.0.",
    "0.....000.....00",
    "0.000...........",
    "000000000.......",
    "000000000.......",
    "00000...00000...",
    "0.....0.....0...",
]


def _parse_rhythm_line(line: str) -> List[Optional[int]]:
    mapping = {".": None, "0": 0, "-": -1}
    return [mapping[ch] if ch in mapping else int(ch) for ch in line]


RHYTHM_PATTERNS = tuple(_parse_rhythm_line(s) for s in RHYTHM_PATTERN_STRINGS)

# Tone library:
# wave, attack, decay, sustain, release, vibrato
# wave: 0=Triangle, 1=Square, 2=Pulse, 3=Noise
TONE_WAVE = 0
TONE_ATTACK = 1
TONE_DECAY = 2
TONE_SUSTAIN = 3
TONE_RELEASE = 4
TONE_VIBRATO = 5

TONE_LIBRARY = [
    (0, 0, 0, 100, 0, 0),
    (2, 0, 30, 50, 10, 60),
    (2, 20, 20, 70, 10, 60),
    (2, 40, 0, 100, 20, 90),
    (1, 15, 60, 50, 10, 90),
    (1, 0, 30, 30, 10, 0),
    (1, 0, 15, 10, 20, 0),
    (0, 0, 0, 100, 0, 60),
    (2, 0, 40, 20, 10, 0),
    (0, 15, 60, 60, 10, 0),
    (1, 0, 60, 80, 10, 0),
    (2, 0, 60, 80, 10, 0),
    (0, 0, 0, 0, 0, 0),
    (0, 0, 0, 0, 0, 0),
    (0, 0, 0, 0, 0, 0),
    (3, 0, 12, 0, 0, 0),
]

# Drum kit:
# key, wave, notes, decay, sustain, velocity
# wave: 0=Triangle, 1=Square, 2=Pulse, 3=Noise
DRUM_KEY = 0
DRUM_WAVE = 1
DRUM_NOTES = 2
DRUM_DECAY = 3
DRUM_SUSTAIN = 4
DRUM_VELOCITY = 5

DRUM_KIT = [
    (":1", 3, (36, 24), 8, 0, 100),
    (":2", 3, (46,), 16, 0, 100),
    (":3", 3, (58,), 10, 0, 30),
    (":5", 0, (21, 19, 18, 17, 16, 15, 14, 13, 12), 16, 0, 100),
    (":6", 0, (27, 25, 24, 23, 22, 21, 20, 19, 18), 16, 0, 100),
    (":7", 0, (33, 31, 30, 29, 28, 27, 26, 25, 24), 16, 0, 100),
]


def _tempo_from_preset_speed(speed_value: int) -> int:
    return int(28800 // speed_value)


def _quantize_percent(q: int) -> int:
    return max(0, min(100, int(round(q / 16 * 100))))


def _volume_to_mml(vol: int) -> int:
    return max(0, min(127, int(round(vol * 16))))


def _tone_to_mml_tone(tone_idx: int, tones: Sequence[Tuple]) -> int:
    return int(tones[tone_idx][TONE_WAVE])


def _note_to_mml_octave_semi(note: int) -> Tuple[int, int]:
    octave = (note // 12) + 2
    semitone = note % 12
    return octave, semitone


def _semitone_to_note_str(semitone: int) -> str:
    return ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"][semitone]


def _env_def_from_tone(tone: Tuple) -> str:
    attack = max(0, int(tone[TONE_ATTACK]))
    decay = max(0, int(tone[TONE_DECAY]))
    sustain = max(0, min(100, int(tone[TONE_SUSTAIN])))
    release = max(0, int(tone[TONE_RELEASE]))
    sustain_level = int(round(sustain / 100 * 127))

    if attack == 0 and decay == 0 and release == 0 and sustain_level == 127:
        return "@ENV{slot}{127}"

    segments = [(attack, 127), (decay, sustain_level)]
    if release > 0:
        segments.append((release, 0))

    seg_str = "".join(f",{dur},{vol}" for dur, vol in segments if dur >= 0)
    return f"@ENV{{slot}}{{0{seg_str}}}"


def _env_def_from_pattern(pattern: Tuple) -> str:
    decay = max(0, int(pattern[DRUM_DECAY]))
    sustain = max(0, min(100, int(pattern[DRUM_SUSTAIN])))
    sustain_level = int(round(sustain / 100 * 127))
    velocity = max(0, min(100, int(pattern[DRUM_VELOCITY])))
    init = int(round(velocity / 100 * 127))

    if decay == 0 and sustain_level == 0:
        return "@ENV{slot}{" + f"{init}" + "}"

    seg_str = f",{decay},{sustain_level}"
    return f"@ENV{{slot}}{{{init}{seg_str}}}"


def _vib_def_from_tone(tone: Tuple) -> Optional[str]:
    vibrato = max(0, int(tone[TONE_VIBRATO]))
    if vibrato == 0:
        return None
    delay_ticks = vibrato
    period_ticks = 20
    depth_cents = 25
    return f"@VIB{{slot}}{{{delay_ticks},{period_ticks},{depth_cents}}}"


def _length_units_to_tokens(units: int) -> List[str]:
    table = [
        (16, "1"),
        (12, "2."),
        (8, "2"),
        (6, "4."),
        (4, "4"),
        (3, "8."),
        (2, "8"),
        (1, "16"),
    ]
    tokens: List[str] = []
    remaining = units
    for u, token in table:
        while remaining >= u:
            tokens.append(token)
            remaining -= u
        if remaining == 0:
            break
    return tokens


def _length_token_to_units(token: str) -> int:
    base = {"1": 16, "2": 8, "4": 4, "8": 2, "16": 1}
    if not token:
        return 0
    dotted = token.endswith(".")
    core = token[:-1] if dotted else token
    units = base.get(core, 0)
    if dotted:
        units += units // 2
    return units


def _note_token(note_str: str, units: int, tie_out: bool, default_len: str) -> str:
    lens = _length_units_to_tokens(units)
    length_head = "" if lens[0] == default_len else lens[0]
    token = f"{note_str}{length_head}" + "".join(f"&{tok}" for tok in lens[1:])
    if tie_out:
        token += "&"
    return token


def _select_default_length(notes: List[Optional[object]]) -> str:
    len_counts = {"1": 0, "2": 0, "4": 0, "8": 0, "16": 0}
    idx = 0
    while idx < len(notes):
        note_len = notes[idx]
        if note_len is None:
            idx += 1
            continue
        length = 1
        j = idx + 1
        while j < len(notes) and notes[j] is None:
            length += 1
            j += 1
        idx = j
        tokens = _length_units_to_tokens(length)
        if tokens:
            head = tokens[0]
            if head in len_counts:
                len_counts[head] += 1
    return max(len_counts.items(), key=lambda kv: (kv[1], int(kv[0])))[0]


def _merge_rest_tokens(tokens: List[str], default_len: str) -> List[str]:
    merged: List[str] = []
    rest_units = 0
    for tok in tokens + ["__END__"]:
        is_rest = tok.startswith("R")
        if is_rest:
            tail = tok[1:]
            parts = tail.split("&") if tail else [""]
            total = 0
            for part in parts:
                part_len = part if part else default_len
                total += _length_token_to_units(part_len)
            rest_units += total
            continue
        if rest_units:
            merged.append(_note_token("R", rest_units, False, default_len))
            rest_units = 0
        if tok != "__END__":
            merged.append(tok)
    return merged


def _compress_token_runs(tokens: List[str], group_size: int) -> List[str]:
    if group_size <= 1:
        out: List[str] = []
        i = 0
        while i < len(tokens):
            j = i + 1
            while j < len(tokens) and tokens[j] == tokens[i]:
                j += 1
            count = j - i
            if tokens[i] in ("<", ">") or count <= 1:
                out.append(tokens[i])
            else:
                expanded = tokens[i] * count
                bracketed = f"[{tokens[i]}]{count}"
                out.append(expanded if len(expanded) <= len(bracketed) else bracketed)
            i = j
        return out

    out = []
    i = 0
    while i < len(tokens):
        if i + group_size <= len(tokens):
            chunk = tokens[i : i + group_size]
            if any(tok in ("<", ">") for tok in chunk):
                out.append(tokens[i])
                i += 1
                continue
            j = i + group_size
            while j + group_size <= len(tokens) and tokens[j : j + group_size] == chunk:
                j += group_size
            count = (j - i) // group_size
            if count > 1:
                expanded = "".join(chunk) * count
                bracketed = f"[{''.join(chunk)}]{count}"
                out.append(expanded if len(expanded) <= len(bracketed) else bracketed)
                i = j
                continue
        out.append(tokens[i])
        i += 1
    return out


def _format_tokens(tokens: List[str]) -> str:
    out = []
    last_tok = ""
    for tok in tokens:
        if tok.startswith("@") or tok.startswith("O"):
            if out and out[-1] != " ":
                out.append(" ")
            out.append(tok)
            last_tok = tok
            continue
        if (
            tok in ("<", ">")
            and last_tok
            and (last_tok.startswith("@") or last_tok.startswith("O"))
        ):
            if out and out[-1] != " ":
                out.append(" ")
        out.append(tok)
        last_tok = tok
    return "".join(out).strip()


def _compress_chunks(lines: List[str], chunk_size: int) -> List[str]:
    if chunk_size <= 1:
        out: List[str] = []
        i = 0
        while i < len(lines):
            j = i + 1
            while j < len(lines) and lines[j] == lines[i]:
                j += 1
            count = j - i
            if count > 1:
                expanded = lines[i] * count
                bracketed = f"[{lines[i]}]{count}"
                out.append(expanded if len(expanded) <= len(bracketed) else bracketed)
            else:
                out.append(lines[i])
            i = j
        return out

    out = []
    i = 0
    while i < len(lines):
        if i + chunk_size <= len(lines):
            chunk = lines[i : i + chunk_size]
            j = i + chunk_size
            while j + chunk_size <= len(lines) and lines[j : j + chunk_size] == chunk:
                j += chunk_size
            count = (j - i) // chunk_size
            if count > 1:
                expanded = "".join(chunk) * count
                bracketed = f"[{''.join(chunk)}]{count}"
                out.append(expanded if len(expanded) <= len(bracketed) else bracketed)
                i = j
                continue
        out.append(lines[i])
        i += 1
    return out


class _Composer:
    def __init__(self, seed: Optional[int], params: Dict[str, object]):
        self.rng = random.Random(seed)
        self.settings = params.copy()

        self.tones = TONE_LIBRARY
        self.patterns = DRUM_KIT
        self.chord_progressions = CHORD_PROGRESSIONS
        self.bass_patterns = BASS_PATTERNS
        self.drum_patterns = DRUM_PATTERNS
        self.style_presets = STYLE_PRESETS
        self.melody_rhythms = RHYTHM_PATTERNS

        self.melody_notes: List[Optional[int]] = []
        self.submelody_notes: List[Optional[int]] = []
        self.base_notes: List[Optional[int]] = []
        self.timeline: List[List[Optional[object]]] = []

    @property
    def total_len(self) -> int:
        return BARS_COUNT * 16

    @property
    def with_submelody(self) -> bool:
        return self.settings["instrumentation"] in (2, 3)

    @property
    def with_drum(self) -> bool:
        return self.settings["instrumentation"] in (1, 3)

    def apply_preset(self):
        preset = self.style_presets[self.settings["style"]]
        tempo = _tempo_from_preset_speed(preset[PRESET_SPEED])
        tempo += int(self.settings.get("bpm_offset", 0))
        self.settings["tempo"] = max(1, tempo)
        self.settings["chord"] = preset[PRESET_CHORD]
        self.settings["base"] = preset[PRESET_BASE]
        self.settings["base_quantize"] = preset[PRESET_BASE_QUANTIZE]
        self.settings["drums"] = preset[PRESET_DRUMS]
        self.settings["melo_tone"] = preset[PRESET_MELO_TONE]
        self.settings["sub_tone"] = preset[PRESET_SUB_TONE]
        self.settings["melo_lowest_note"] = preset[PRESET_MELO_LOWEST_NOTE]
        self.settings["melo_density"] = preset[PRESET_MELO_DENSITY]
        self.settings["melo_use16"] = preset[PRESET_MELO_USE16]
        self.settings["instrumentation"] = preset[PRESET_INSTRUMENTATION]

    def compose_timeline(self, make_melody: bool = True):
        settings = self.settings
        bass_patterns = self.bass_patterns[settings["base"]]
        drum_patterns = self.drum_patterns[settings["drums"]]
        self.build_chord_plan()
        timeline: List[List[Optional[object]]] = []
        self.base_notes = []
        self.cur_chord_idx = -1
        for loc in range(self.total_len):
            timeline.append([None for _ in range(19)])
            (chord_idx, _) = self.chord_at(loc)
            if chord_idx > self.cur_chord_idx:
                chord_list = self.chord_plan[chord_idx]
                self.cur_chord_idx = chord_idx
                self.cur_chord_loc = loc
            item = timeline[loc]
            tick = loc % 16
            if loc == 0:
                self._init_timeline_header(item)

            if chord_list["repeat"] is not None:
                repeat_loc = self.chord_plan[chord_list["repeat"]]["loc"]
                target_loc = repeat_loc + loc - self.cur_chord_loc
                item[10] = timeline[target_loc][10]
                base_note = item[10]
            else:
                base_note = self._build_base_note(bass_patterns, chord_list, tick, loc)
                item[10] = base_note
            self.base_notes.append(base_note)

            if self.with_drum:
                self._apply_drum(drum_patterns, item, tick, loc)

        while make_melody:
            self.build_melody()
            if self.melody_has_required_tones():
                break
            self.build_chord_plan()

        if self.with_submelody:
            self.build_submelody()

        for loc in range(self.total_len):
            item = timeline[loc]
            item[6] = self.melody_notes[loc]
            if self.with_submelody:
                item[14] = self.submelody_notes[loc]
            elif not self.with_drum:
                item[14] = self.melody_notes[
                    (loc + self.total_len - 1) % self.total_len
                ]

        self.timeline = timeline

    def _init_timeline_header(self, item: List[Optional[object]]):
        settings = self.settings
        item[0] = settings["tempo"]
        item[1] = 48
        item[2] = 3
        item[3] = TONE_CANDIDATES[settings["melo_tone"]]
        item[4] = 6
        item[5] = 14
        item[7] = 7
        item[8] = 7
        item[9] = settings["base_quantize"]
        if self.with_submelody:
            item[11] = TONE_CANDIDATES[settings["sub_tone"]]
            item[12] = 4
            item[13] = 15
            if self.with_drum:
                item[15] = 15
                item[16] = 5
                item[17] = 15
        elif self.with_drum:
            item[11] = 15
            item[12] = 5
            item[13] = 15
        else:
            item[11] = item[3]
            item[12] = 2
            item[13] = item[5]

    def _build_base_note(self, bass_patterns, chord_list, tick: int, loc: int):
        settings = self.settings
        pattern_idx = PATTERN_BASIC if loc // 16 < 7 else PATTERN_FINAL
        base_str = bass_patterns[pattern_idx][tick]
        if base_str == ".":
            return None
        if base_str == "0":
            return -1
        highest = settings["base_highest_note"]
        base_root = 12 + settings["transpose"] + chord_list["base"]
        while base_root + 24 > highest:
            base_root -= 12
        notes = chord_list["notes_origin"]
        adjust_list = [0, -1, 1, -2, 2, -3, 3]
        adjust_idx = 0
        base_add = {"1": 7, "2": 12, "3": 19, "4": 24}[base_str]
        while notes:
            adjust = adjust_list[adjust_idx]
            base_note = base_root + base_add + adjust
            if notes[(base_note + settings["transpose"]) % 12] in [1, 2, 3]:
                return base_note
            adjust_idx += 1
        return -1

    def _apply_drum(
        self, drum_patterns, item: List[Optional[object]], tick: int, loc: int
    ):
        pattern_idx = PATTERN_BASIC if (loc // 16) % 4 < 3 else PATTERN_FINAL
        idx = 18 if self.with_submelody else 14
        drum_str = drum_patterns[pattern_idx][tick]
        item[idx] = None if drum_str == "0" else ":" + drum_str

    def build_chord_plan(self):
        chord = self.chord_progressions[self.settings["chord"]]
        self.chord_plan = []
        for progression in chord:
            chord_list = {
                "loc": progression[CHORD_LOC],
                "base": 0,
                "no_root": False,
                "notes": [],
                "notes_origin": [],
                "repeat": progression[CHORD_REPEAT],
            }
            if progression[CHORD_REPEAT] is not None:
                chord_list["base"] = self.chord_plan[progression[CHORD_REPEAT]]["base"]
            if progression[CHORD_NOTES] is not None:
                notes = [int(s) for s in progression[CHORD_NOTES]]
                chord_list["notes_origin"] = notes
                note_chord_cnt = 0
                for idx in range(12):
                    if notes[idx] == 2:
                        chord_list["base"] = idx
                    if notes[idx] in [1, 2, 3]:
                        note_chord_cnt += 1
                chord_list["no_root"] = note_chord_cnt > 3
                chord_list["notes"] = self.build_chord_notes(notes)
            self.chord_plan.append(chord_list)

    def build_chord_notes(self, notes, tone_up: int = 0):
        settings = self.settings
        note_highest = None
        idx = 0
        results = []
        while True:
            note_type = int(notes[idx % 12])
            note = 12 + idx + settings["transpose"]
            if note >= settings["melo_lowest_note"] + tone_up:
                if note_type in [1, 2, 3, 9]:
                    results.append((note, note_type))
                    if note_highest is None:
                        note_highest = note + 15
            if note_highest and note >= note_highest:
                break
            idx += 1
        return results

    def build_melody(self):
        self.melody_notes = [-2 for _ in range(self.total_len)]
        self.submelody_notes = [-2 for _ in range(self.total_len)]
        rhythm_main_list = []
        for _ in range(5):
            rhythm_main_list.append(self.pick_rhythm())
        rhythm_main_list.sort(key=len)
        rhythm_main = rhythm_main_list[self.settings["melo_density"]]
        for loc in range(self.total_len):
            if self.melody_notes[loc] != -2:
                continue
            notesets = self.next_note_events(rhythm_main, loc)
            if notesets is None:
                repeat_loc = self.chord_plan[self.chord_list["repeat"]]["loc"]
                target_loc = repeat_loc + loc - self.cur_chord_loc
                repeat_note = self.melody_notes[target_loc]
                self.place_melody(loc, repeat_note, 1)
                repeat_subnote = self.submelody_notes[target_loc]
                self.submelody_notes[loc] = repeat_subnote
            else:
                notesets_len = 0
                for noteset in notesets:
                    self.place_melody(noteset[0], noteset[1], noteset[2])
                    notesets_len += noteset[2]
                self.place_harmony(loc, -2, notesets_len)

    def build_submelody(self):
        rhythm_sub = self.pick_rhythm(True)
        prev_note_loc = -1
        for loc in range(self.total_len):
            note = self.submelody_notes[loc]
            if note is not None and note >= 0:
                prev_note_loc = loc
                self.prev_note = note
            elif loc - prev_note_loc >= 4 and loc % 4 == 0:
                notesets = self.next_note_events(rhythm_sub, loc, True)
                if notesets is not None:
                    for noteset in notesets:
                        self.place_harmony(noteset[0], noteset[1], noteset[2])
                    prev_note_loc = loc

    def pick_rhythm(self, is_sub: bool = False):
        self.cur_chord_idx = -1
        self.cur_chord_loc = -1
        self.is_repeat = False
        self.chord_list = []
        self.prev_note = -1
        self.first_in_chord = True
        results = []
        used16 = False
        while True:
            for bar in range(BARS_COUNT):
                if is_sub:
                    pat_line = [
                        0,
                        None,
                        None,
                        None,
                        0,
                        None,
                        None,
                        None,
                        0,
                        None,
                        None,
                        None,
                        0,
                        None,
                        None,
                        None,
                    ]
                else:
                    while True:
                        pat_line = self.melody_rhythms[
                            self.rng.randint(0, len(self.melody_rhythms) - 1)
                        ]
                        if self.rhythm_has_16th(pat_line):
                            if not self.settings["melo_use16"]:
                                continue
                            used16 = True
                        if pat_line[0] is not None:
                            break
                for idx, pat_one in enumerate(pat_line):
                    loc = bar * 16 + idx
                    if pat_one is not None:
                        results.append((loc, pat_one))
            if is_sub or not self.settings["melo_use16"] or used16:
                break
        for _ in range(2):
            results.append((self.total_len, -1))
        return results

    def rhythm_has_16th(self, line):
        prev_str = None
        for i in line:
            if i == 0 and prev_str == 0:
                return True
            prev_str = i
        return False

    def next_note_events(self, rhythm_set, loc, is_sub: bool = False):
        pat = None
        for pat_idx, rhythm in enumerate(rhythm_set):
            if loc == rhythm[0]:
                pat = rhythm[1]
                break
            elif loc < rhythm[0]:
                break
        note_len = rhythm_set[pat_idx + 1][0] - loc
        change_chord = False
        lookahead = False
        (next_chord_idx, next_chord_loc) = self.chord_at(loc)
        if next_chord_idx > self.cur_chord_idx:
            change_chord = True
        elif not self.is_repeat and loc + note_len > next_chord_loc:
            (next_chord_idx, next_chord_loc) = self.chord_at(loc + note_len)
            change_chord = True
            lookahead = True
        if change_chord:
            self.chord_list = self.chord_plan[next_chord_idx]
            self.cur_chord_idx = next_chord_idx
            self.cur_chord_loc = loc
            self.first_in_chord = True
            self.is_repeat = self.chord_list["repeat"] is not None
        if self.is_repeat:
            return [] if lookahead else None
        if pat == -1:
            return [(loc, -1, note_len)]
        self.chord_notes = self.chord_list["notes"]
        next_idx = self.pick_target_note(is_sub, loc)
        following = []
        prev_loc = loc
        while True:
            pat_loc = rhythm_set[pat_idx + 1 + len(following)][0]
            no_next = pat_loc >= next_chord_loc or pat_loc - prev_loc > 4
            if not following or not no_next:
                following.append((prev_loc, pat_loc - prev_loc))
            if no_next:
                break
            prev_loc = pat_loc
        loc, note_len = following[0]
        cur_idx = None
        if not lookahead:
            for idx, note in enumerate(self.chord_notes):
                if self.prev_note == note[0]:
                    cur_idx = idx
                    break
        if self.prev_note < 0 or cur_idx is None:
            note = self.chord_notes[next_idx][0]
            return [(loc, note, note_len)]
        results = []
        diff = abs(next_idx - cur_idx)
        direction = 1 if next_idx > cur_idx else -1
        if diff == 0:
            cnt = len(following) // 2
            if cnt and self.rng.randint(0, 1) and not is_sub:
                for i in range(cnt):
                    while next_idx == cur_idx:
                        next_idx = self.pick_target_note()
                    direction = 1 if next_idx > cur_idx else -1
                    note = self.chord_notes[cur_idx + direction][0]
                    prev_note = self.prev_note
                    note_follow = following[i * 2]
                    results.append((note_follow[0], note, note_follow[1]))
                    note_follow = following[i * 2 + 1]
                    results.append((note_follow[0], prev_note, note_follow[1]))
                return results
            return [(loc, self.prev_note, note_len)]
        if abs(next_idx - cur_idx) > len(following):
            note = self.chord_notes[next_idx][0]
            return [(loc, note, note_len)]
        i = 0
        while next_idx != cur_idx:
            cur_idx += direction
            note = self.chord_notes[cur_idx][0]
            note_follow = following[i]
            results.append((note_follow[0], note, note_follow[1]))
            i += 1
        return results

    def melody_has_required_tones(self):
        cur_chord_idx = -1
        need_notes_list = []
        for loc in range(self.total_len):
            (next_chord_idx, _) = self.chord_at(loc)
            if next_chord_idx > cur_chord_idx:
                if len(need_notes_list) > 0:
                    return False
                cur_chord_idx = next_chord_idx
                notes_list = self.chord_plan[cur_chord_idx]["notes"]
                need_notes_list = []
                for chord in notes_list:
                    note = chord[0] % 12
                    if chord[1] == 1 and note not in need_notes_list:
                        need_notes_list.append(note)
            note = self.melody_notes[loc]
            if note is not None and note >= 0 and note % 12 in need_notes_list:
                need_notes_list.remove(note % 12)
        return True

    def chord_at(self, loc):
        chord_lists_cnt = len(self.chord_plan)
        next_chord_loc = 16 * BARS_COUNT
        for rev_idx in range(chord_lists_cnt):
            idx = chord_lists_cnt - rev_idx - 1
            if loc >= self.chord_plan[idx]["loc"]:
                break
            else:
                next_chord_loc = self.chord_plan[idx]["loc"]
        return idx, next_chord_loc

    def pick_target_note(self, is_sub: bool = False, loc=None):
        no_root = self.first_in_chord or self.chord_list["no_root"]
        allowed_types = [1, 3] if no_root else [1, 2, 3]
        notes = self.harmony_note_pool(loc) if is_sub else self.chord_list["notes"]
        highest_note = 0
        highest_idx = 0
        for idx, noteset in enumerate(notes):
            if noteset[0] > highest_note and noteset[1] in allowed_types:
                highest_note = noteset[0]
                highest_idx = idx
        if self.prev_note - highest_note > 12:
            return highest_idx
        while True:
            idx = self.rng.randint(0, len(notes) - 1)
            if notes[idx][1] not in allowed_types:
                continue
            note = notes[idx][0]
            if self.prev_note >= 0:
                diff = abs(self.prev_note - note)
                if diff > 12:
                    continue
                factor = diff if diff != 12 else diff - 6
                if self.rng.randint(0, 15) < factor and not is_sub:
                    continue
            return idx

    def place_melody(self, loc, note, note_len: int = 1):
        for idx in range(note_len):
            self.melody_notes[loc + idx] = note if idx == 0 else None
        if note is not None:
            self.prev_note = note
            self.first_in_chord = False

    def place_harmony(self, loc, note, note_len: int = 1):
        master_note = None
        subnote = note
        master_loc = loc
        while master_loc >= 0:
            master_note = self.melody_notes[master_loc]
            if master_note is not None and master_note >= 0:
                prev_note = master_note if note == -2 else note
                subnote = self.find_lower_harmony(prev_note, master_note, master_loc)
                break
            master_loc -= 1
        prev_subnote = None
        for idx in range(note_len):
            if (
                self.melody_notes[loc + idx] is not None
                and self.melody_notes[loc + idx] >= 0
            ):
                master_note = self.melody_notes[loc + idx]
            duplicate = (
                master_note is not None
                and subnote is not None
                and (abs(subnote - master_note) < 3)
            )
            if duplicate:
                subnote = self.find_lower_harmony(subnote, master_note, loc + idx)
            self.submelody_notes[loc + idx] = (
                subnote if subnote != prev_subnote else None
            )
            prev_subnote = subnote

    def find_lower_harmony(self, prev_note, master_note, loc):
        if self.with_submelody and master_note >= 0:
            notes = self.harmony_note_pool(loc)
            if prev_note is not None and abs(prev_note - master_note) >= 3:
                return prev_note
            cur_note = master_note - 3
            while cur_note >= self.settings["melo_lowest_note"]:
                for n in notes:
                    if n[0] == cur_note and n[1] in [1, 2, 3]:
                        return cur_note
                cur_note -= 1
        return -1

    def harmony_note_pool(self, start_loc):
        master_note = None
        base_note = None
        loc = start_loc
        while master_note is None or base_note is None:
            if master_note is None and self.melody_notes[loc] != -1:
                master_note = self.melody_notes[loc]
            if base_note is None and self.base_notes[loc] != -1:
                base_note = self.base_notes[loc]
            loc = (loc + self.total_len - 1) % self.total_len
        notes = self.chord_list["notes_origin"].copy()
        results = []
        has_important_tone = False
        idx = 0
        while notes:
            note_type = notes[idx % 12]
            if note_type in [1, 2, 3, 9]:
                note = 12 + idx + self.settings["transpose"]
                if note > master_note - 3 and has_important_tone:
                    break
                if note >= base_note + 3:
                    results.append((note, note_type))
                    if note_type in [1, 3]:
                        has_important_tone = True
            idx += 1
        self.chord_notes = results
        return results


def _timeline_to_mml(
    timeline: List[List[Optional[object]]],
    tones: Sequence[Tuple],
    patterns: Sequence[Tuple],
) -> List[str]:
    pattern_map = {p[DRUM_KEY]: p for p in patterns}
    pattern_slots = {p[DRUM_KEY]: i + 1 for i, p in enumerate(patterns)}

    def channel_items(ch: int) -> Tuple[int, int, int, List[Optional[object]]]:
        base = 3 + ch * 4
        tone = 0
        volume = 7
        quantize = 16
        for item in timeline:
            if item[base] is not None:
                tone = item[base]
                break
        for item in timeline:
            if item[base + 1] is not None:
                volume = item[base + 1]
                break
        for item in timeline:
            if item[base + 2] is not None:
                quantize = item[base + 2]
                break
        notes = [item[base + 3] for item in timeline]
        return tone, volume, quantize, notes

    def build_mml_for_channel(ch: int) -> str:
        tone, volume, quantize, notes = channel_items(ch)
        tempo = timeline[0][0] or 120
        q = _quantize_percent(quantize)
        vol = _volume_to_mml(volume)
        mml_tone = _tone_to_mml_tone(tone, tones)

        used_tone_slots = {tone + 1}
        used_pattern_slots = {
            pattern_slots[n] for n in notes if isinstance(n, str) and n in pattern_map
        }

        default_len = _select_default_length(notes)

        tone_slot_map = {
            slot: idx + 1 for idx, slot in enumerate(sorted(used_tone_slots))
        }
        pattern_offset = len(tone_slot_map)
        pattern_slot_map = {
            slot: pattern_offset + idx + 1
            for idx, slot in enumerate(sorted(used_pattern_slots))
        }

        prelude = [f"T{tempo}", f"L{default_len}"]
        for slot in sorted(used_tone_slots):
            env_def = _env_def_from_tone(tones[slot - 1]).replace(
                "{slot}", str(tone_slot_map[slot])
            )
            prelude.append(env_def)
            vib_def = _vib_def_from_tone(tones[slot - 1])
            if vib_def:
                prelude.append(vib_def.replace("{slot}", str(tone_slot_map[slot])))
        for key, slot in sorted(
            ((k, pattern_slots[k]) for k in pattern_map.keys()), key=lambda x: x[1]
        ):
            if slot in used_pattern_slots:
                env_def = _env_def_from_pattern(pattern_map[key]).replace(
                    "{slot}", str(pattern_slot_map[slot])
                )
                prelude.append(env_def)

        prelude.extend(
            [
                f"Q{q}",
                f"V{vol}",
                f"@{mml_tone}",
                f"@ENV{tone_slot_map[tone + 1]}",
            ]
        )
        vib_def = _vib_def_from_tone(tones[tone])
        prelude.append(f"@VIB{tone_slot_map[tone + 1]}" if vib_def else "@VIB0")

        cur_oct: Optional[int] = None
        cur_tone: Optional[int] = mml_tone
        cur_env: Optional[int] = tone_slot_map[tone + 1]
        cur_vib: Optional[int] = tone_slot_map[tone + 1] if vib_def else 0

        bar_tokens: List[List[str]] = [[]]
        bar_units = 0
        pattern_note_idx: Dict[str, int] = {k: 0 for k in pattern_map.keys()}

        idx = 0
        while idx < len(notes):
            note = notes[idx]
            if note is None:
                idx += 1
                continue
            length = 1
            j = idx + 1
            while j < len(notes) and notes[j] is None:
                length += 1
                j += 1
            idx = j

            is_drum = False
            if isinstance(note, str):
                pattern_key = note
                pattern = pattern_map.get(pattern_key)
                if pattern:
                    drum_tone = int(pattern[DRUM_WAVE])
                    drum_slot = pattern_slots[pattern_key]
                    if cur_tone != drum_tone:
                        bar_tokens[-1].append(f"@{drum_tone}")
                        cur_tone = drum_tone
                    env_slot = pattern_slot_map[drum_slot]
                    if cur_env != env_slot:
                        bar_tokens[-1].append(f"@ENV{env_slot}")
                        cur_env = env_slot
                    if cur_vib != 0:
                        bar_tokens[-1].append("@VIB0")
                        cur_vib = 0
                    note_list = pattern[DRUM_NOTES] or []
                    if note_list:
                        idx0 = pattern_note_idx[pattern_key] % len(note_list)
                        note = int(note_list[idx0])
                        pattern_note_idx[pattern_key] += 1
                    else:
                        note = -1
                    is_drum = True
                else:
                    note = -1
            else:
                if cur_tone != mml_tone:
                    bar_tokens[-1].append(f"@{mml_tone}")
                    cur_tone = mml_tone
                tone_env_slot = tone_slot_map[tone + 1]
                if cur_env != tone_env_slot:
                    bar_tokens[-1].append(f"@ENV{tone_env_slot}")
                    cur_env = tone_env_slot
                if cur_vib != (tone_env_slot if vib_def else 0):
                    bar_tokens[-1].append(
                        f"@VIB{tone_env_slot}" if vib_def else "@VIB0"
                    )
                    cur_vib = tone_env_slot if vib_def else 0

            if note == -1:
                remaining = length
                while remaining > 0:
                    space = 16 - bar_units
                    seg = min(space, remaining)
                    token = _note_token("R", seg, False, default_len)
                    bar_tokens[-1].append(token)
                    bar_units += seg
                    remaining -= seg
                    if bar_units == 16:
                        bar_tokens.append([])
                        bar_units = 0
                continue

            remaining = length
            first_segment = True
            while remaining > 0:
                space = 16 - bar_units
                seg = min(space, remaining)
                tie_out = remaining > seg
                if first_segment or not is_drum:
                    octave, semitone = _note_to_mml_octave_semi(int(note))
                    if cur_oct is None:
                        bar_tokens[-1].append(f"O{octave}")
                        cur_oct = octave
                    elif cur_oct != octave:
                        diff = octave - cur_oct
                        if abs(diff) == 1:
                            bar_tokens[-1].append(">" if diff > 0 else "<")
                        else:
                            bar_tokens[-1].append(f"O{octave}")
                        cur_oct = octave
                    note_str = _semitone_to_note_str(semitone)
                token = _note_token(note_str, seg, tie_out, default_len)
                bar_tokens[-1].append(token)
                bar_units += seg
                remaining -= seg
                first_segment = False
                if bar_units == 16:
                    bar_tokens.append([])
                    bar_units = 0

        if bar_tokens and not bar_tokens[-1]:
            bar_tokens.pop()

        bar_tokens = [_merge_rest_tokens(bar, default_len) for bar in bar_tokens]

        for idx, bar in enumerate(bar_tokens):
            compressed = _compress_token_runs(bar, 4)
            if compressed == bar:
                compressed = _compress_token_runs(bar, 2)
            if compressed == bar:
                compressed = _compress_token_runs(bar, 1)
            bar_tokens[idx] = compressed

        bar_strs = [_format_tokens(b) for b in bar_tokens]

        compressed = _compress_chunks(bar_strs, 2)
        if compressed == bar_strs:
            compressed = _compress_chunks(bar_strs, 1)

        return " ".join(prelude + compressed)

    return [build_mml_for_channel(ch) for ch in range(4)]


def _generate_bgm_mml(
    *,
    style: int = 0,
    layout: int = 0,
    transpose: int = 0,
    bpm_offset: int = 0,
    seed: Optional[int] = None,
) -> List[str]:
    if seed is None:
        seed = random.randrange(1 << 30)
    settings = {
        "style": style,
        "transpose": transpose,
        "instrumentation": layout,
        "bpm_offset": bpm_offset,
        "chord": 0,
        "base": 0,
        "base_quantize": LIST_BASE_QUANTIZE[2],
        "drums": 0,
        "melo_tone": 0,
        "sub_tone": 0,
        "melo_lowest_note": LIST_MELO_LOWEST_NOTE[3],
        "melo_density": LIST_MELO_DENSITY[1],
        "melo_use16": True,
        "base_highest_note": 26,
    }
    composer = _Composer(seed, settings)
    composer.apply_preset()
    composer.settings["instrumentation"] = layout
    composer.compose_timeline(make_melody=True)
    return _timeline_to_mml(composer.timeline, composer.tones, composer.patterns)


def generate_bgm(
    *,
    style: int = 0,
    layout: int = 0,
    transpose: int = 0,
    bpm_offset: int = 0,
    seed: Optional[int] = None,
    play: bool = False,
    loop: bool = False,
) -> List[str]:
    """
    Generate a BGM and return MML strings.

    style: 0-7
    layout: 0-3
    transpose: semitone shift (0 = no shift)
    bpm_offset: BPM delta added to preset tempo
    seed: random seed (None for random)
    play: play immediately if True
    loop: loop playback if play is True
    """
    import pyxel

    if style not in (0, 1, 2, 3, 4, 5, 6, 7):
        raise ValueError("Style must be 0-7")
    if layout not in (0, 1, 2, 3):
        raise ValueError("Layout must be 0-3")

    mml_list = _generate_bgm_mml(
        style=style,
        transpose=transpose,
        layout=layout,
        bpm_offset=bpm_offset,
        seed=seed,
    )
    if play:
        play_channels = [(0, 1), (0, 1, 2), (0, 1, 2), (0, 1, 2, 3)][layout]
        for ch in play_channels:
            mml = mml_list[ch]
            snd = pyxel.Sound()
            snd.mml(mml)
            pyxel.play(ch, snd, loop=loop)
    return mml_list
