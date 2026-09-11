import colorsys
import shutil
import subprocess
import sys
from pathlib import Path
from types import SimpleNamespace

import pytest
import pyxel
from _capture import (  # type: ignore[reportMissingImports]
    EDITOR_REFS_DIR,
    collect_editor_results,
    run_editor_subprocess,
)
from pyxel.editor.field_cursor import FieldCursor
from pyxel.editor.image_editor import ImageEditor
from pyxel.editor.image_viewer import ImageViewer
from pyxel.editor.music_field import MusicField
from pyxel.editor.sound_editor import SoundEditor
from pyxel.editor.tilemap_editor import TilemapEditor
from pyxel.editor.widgets import NumberPicker, ScrollBar, Widget

RESOURCE_FILE = str(
    Path(__file__).parent.parent / "pyxel" / "examples" / "assets" / "sample.pyxres"
)

_EDITOR_PALETTE_PARAMS = [
    ("image", None),
    ("tilemap", None),
    ("sound", None),
    ("music", None),
    ("image", 16),
    ("image", 32),
    ("image", 64),
]


def _param_id(editor, palette_count):
    if palette_count is None:
        return editor
    return f"{editor}_{palette_count}colors"


class TestEditor:
    def test_music_view_and_save_preserve_extra_channels(self, tmp_path):
        path = str(tmp_path / "six-channels.pyxres")
        music = pyxel.musics[0]
        seqs = [list(seq) for seq in music.seqs]
        try:
            music.seqs[:] = [[0], [1], [2], [3], [4, 5], [6, 7]]
            pyxel.save(path)
        finally:
            music.seqs[:] = seqs

        code = """
import sys

import pyxel
from pyxel.editor.app import App

path = sys.argv[1]
expected = [[0], [1], [2], [3], [4, 5], [6, 7]]
init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None

app = App(path, "sound")
assert [list(seq) for seq in pyxel.musics[0].seqs] == expected

app.editor_type_var = 3
app.draw_all()
viewed = [list(seq) for seq in pyxel.musics[0].seqs]

app._save_button.trigger_event("press")
pyxel.load(path)
assert viewed == expected, viewed
assert [list(seq) for seq in pyxel.musics[0].seqs] == expected
"""

        result = subprocess.run(
            [sys.executable, "-c", code, path],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_failed_save_preserves_edits_and_allows_retry(self, tmp_path):
        code = """
import sys
from pathlib import Path

import pyxel
from pyxel.editor.app import App

path = Path(sys.argv[1])
init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None
app = App(str(path), "image")
editor = app._editor
old_canvas = pyxel.images[0].get_slice(0, 0, 16, 16)
pyxel.images[0].pset(0, 0, 7)
editor.add_history({
    "image_index": 0,
    "focus_pos": (0, 0),
    "old_canvas": old_canvas,
    "new_canvas": pyxel.images[0].get_slice(0, 0, 16, 16),
})

# A directory at the destination causes a real file-creation failure.
path.mkdir()
app._save_button.trigger_event("press")
assert app._editor is editor
assert pyxel.images[0].pget(0, 0) == 7
editor.undo()
assert pyxel.images[0].pget(0, 0) == old_canvas[0][0]
editor.redo()
assert pyxel.images[0].pget(0, 0) == 7
app.update_all()
messages = []
text = pyxel.text
pyxel.text = lambda x, y, message, color: messages.append(message)
app.draw_all()
assert "SAVE FAILED: SEE CONSOLE" in messages

path.rmdir()
app._save_button.trigger_event("press")
messages.clear()
app.draw_all()
assert "SAVE FAILED: SEE CONSOLE" not in messages
pyxel.text = text
pyxel.images[0].pset(0, 0, 0)
pyxel.load(str(path))
assert pyxel.images[0].pget(0, 0) == 7
"""
        result = subprocess.run(
            [sys.executable, "-c", code, str(tmp_path / "retry.pyxres")],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr
        assert result.stdout.count("Failed to save resource:") == 1
        assert "Failed to create file" in result.stdout

    @pytest.mark.parametrize("case", ["music", "sound", "note-below", "note-above"])
    def test_audio_drawing_stays_inside_fields(self, tmp_path, case):
        code = """
import sys

import pyxel
from pyxel.editor.app import App

path, case = sys.argv[1:]
init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None
app = App(path, "music" if case == "music" else "sound")

sound = pyxel.sounds[0]
if case == "music":
    pyxel.musics[0].seqs[:] = [[], [], [], [0] * 32]
else:
    sound.set("c2" * 48, "p" * 48, "7" * 48, "n" * 48, 30)
app.draw_all()
before = [[pyxel.pget(x, y) for x in range(pyxel.width)]
          for y in range(pyxel.height)]

if case == "music":
    pyxel.musics[0].seqs[3].extend([0] * 16)
elif case == "sound":
    sound.set("c2" * 52, "p" * 52, "7" * 52, "n" * 52, 30)
else:
    sound.notes[:] = [-2 if case == "note-below" else 60]
    pyxel.play_pos = lambda ch: (0, 0.0)

fields = ("notes", "tones", "volumes", "effects")
expected = [list(getattr(sound, field)) for field in fields]
seqs = [list(seq) for seq in pyxel.musics[0].seqs]
app.draw_all()

for y in range(pyxel.height):
    for x in range(pyxel.width):
        if case.startswith("note-") and 17 <= x < 223 and 25 <= y < 148:
            continue
        assert pyxel.pget(x, y) == before[y][x], (case, x, y)
assert [list(getattr(sound, field)) for field in fields] == expected
assert [list(seq) for seq in pyxel.musics[0].seqs] == seqs

pyxel.save(path)
pyxel.sounds[0] = pyxel.Sound()
pyxel.musics[0].seqs[:] = []
pyxel.load(path)
assert [list(getattr(pyxel.sounds[0], field)) for field in fields] == expected
assert [list(seq) for seq in pyxel.musics[0].seqs] == seqs
"""

        result = subprocess.run(
            [sys.executable, "-c", code, str(tmp_path / "audio.pyxres"), case],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_resource_drop_preserves_editor_palette(self, tmp_path):
        code = """
import sys
from pathlib import Path

import pyxel
from pyxel.editor.app import App

folder = Path(sys.argv[1])
init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None

app = App(str(folder / "empty.pyxres"), "image")
system = list(pyxel.colors[:pyxel.NUM_COLORS])
editor = app._editors[0]
picker = editor._color_picker

for count in (64, 8, None):
    app.editor_type_var = int(count == 8)
    path = str(folder / f"colors-{count}.pyxres")
    pyxel.save(path)
    colors = list(pyxel.colors)
    expected = colors[pyxel.NUM_COLORS:]
    if count is not None:
        expected = [0x010101 * i for i in range(count)]
        pyxel.colors[:] = expected
        pyxel.save_pal(path)
        pyxel.colors[:] = colors

    previous = editor.color_var
    pyxel._dropped_files = [path]
    app.update_all()
    assert list(pyxel.colors) == system + expected
    assert pyxel.num_user_colors == len(expected)
    assert editor.color_var == min(previous, len(expected) - 1)

    cw = 4 if len(expected) > 16 else 8
    ch = 4 if len(expected) > 32 else 8
    last = len(expected) - 1
    assert picker.check_value(
        picker.x + 1 + last % (64 // cw) * cw,
        picker.y + 1 + last // (64 // cw) * ch,
    ) == last

    editor.color_var = last
    app.draw_all()

pyxel._dropped_files = [str(folder / "missing.pyxres")]
app.update_all()
app.draw_all()
assert list(pyxel.colors) == system + expected
assert pyxel.num_user_colors == len(expected)
"""

        result = subprocess.run(
            [sys.executable, "-c", code, str(tmp_path)],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr
        missing = tmp_path / "missing.pyxres"
        assert result.stdout.splitlines()[-1] == (
            f"Failed to load resource: Failed to open file '{missing}'"
        )

    def test_resource_drop_keeps_title_on_save_destination(self, tmp_path):
        code = """
import sys
from pathlib import Path

import pyxel
from pyxel.editor.app import App

folder = Path(sys.argv[1])
init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None
titles = []
title = pyxel.title
pyxel.title = lambda text: (titles.append(text), title(text))

resource = str(folder / "edited.pyxres")
dropped = str(folder / "dropped.pyxres")
app = App(resource, "image")
pyxel.save(dropped)
pyxel._dropped_files = [dropped]
app.update_all()
assert titles == [f"Pyxel Editor - {resource}"], titles
"""

        result = subprocess.run(
            [sys.executable, "-c", code, str(tmp_path)],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    @pytest.mark.parametrize("editor_type", [ImageEditor, TilemapEditor])
    def test_failed_drop_reports_load_error(self, editor_type, tmp_path, capsys):
        state = SimpleNamespace(
            image_index_var=0, tilemap_index_var=0, focus_x_var=0, focus_y_var=0
        )
        path = str(tmp_path / "missing.file")
        colors = list(pyxel.colors)
        handler = getattr(editor_type, f"_{editor_type.__name__}__on_drop")
        handler(state, path)
        kind = "image" if editor_type is ImageEditor else "tilemap"
        assert capsys.readouterr().out == (
            f"Failed to load {kind}: Failed to open file '{path}'\n"
        )
        assert list(pyxel.colors) == colors

    @pytest.mark.parametrize(
        "editor,palette_count",
        _EDITOR_PALETTE_PARAMS,
        ids=[_param_id(e, p) for e, p in _EDITOR_PALETTE_PARAMS],
    )
    def test_editor(self, editor, palette_count, tmp_path, compare_screenshots):
        if palette_count is None:
            resource = RESOURCE_FILE
            ref_name = f"editor_{editor}"
        else:
            pyxres = tmp_path / "test.pyxres"
            shutil.copy(RESOURCE_FILE, pyxres)
            pyxpal = tmp_path / "test.pyxpal"
            pyxpal.write_text(
                "\n".join(_hsv_pyxpal_lines(palette_count)) + "\n", encoding="utf-8"
            )
            resource = str(pyxres)
            ref_name = f"editor_{editor}_{palette_count}colors"

        run_editor_subprocess(editor, resource, tmp_path)
        results = collect_editor_results(tmp_path)
        compare_screenshots(ref_name, results, EDITOR_REFS_DIR)


class TestSoundEditor:
    @pytest.fixture
    def sound_editor(self):
        originals = [pyxel.sounds[i] for i in range(2)]
        try:
            for i in range(2):
                pyxel.sounds[i] = pyxel.Sound()

            parent = Widget(None, 0, 0, 240, 180)
            parent.new_var("help_message_var", "")
            yield SoundEditor(parent)
        finally:
            for i, sound in enumerate(originals):
                pyxel.sounds[i] = sound

    @pytest.mark.parametrize(
        "y,note", [(2, 59), (6, 57), (10, 55), (13, 53), (16, 52), (20, 50)]
    )
    def test_keyboard_click_matches_white_key_bottom(self, sound_editor, y, note):
        keyboard = sound_editor._piano_keyboard
        keyboard.trigger_event(
            "mouse_down", pyxel.MOUSE_BUTTON_LEFT, keyboard.x + 8, keyboard.y + y
        )
        assert keyboard._mouse_note == note

    @pytest.mark.parametrize(
        "field,values,expected",
        [
            ("tones", [0, 1, 2, 3, 4, 9, 255], "TSPN???"),
            ("volumes", [0, 7, 8, 255], "07??"),
            ("effects", [0, 1, 2, 3, 4, 5, 6, 255], "NSVFHQ??"),
        ],
    )
    def test_draw_marks_unknown_values_without_changing_data(
        self, sound_editor, monkeypatch, field, values, expected
    ):
        sound = pyxel.sounds[0]
        sound.set("c2", "t", "7", "n", 30)
        getattr(sound, field)[:] = values
        drawn_text = []
        monkeypatch.setattr(
            pyxel, "text", lambda x, y, text, col: drawn_text.append(text)
        )

        sound_editor._sound_field.trigger_event("draw")
        assert expected in drawn_text
        assert list(getattr(sound, field)) == values

    @pytest.mark.parametrize(
        "speed,expected_text",
        [(1, " 1"), (99, "99"), (100, " ?"), (150, " ?"), (2**16 - 1, " ?")],
    )
    def test_view_and_bank_switch_preserve_speed(
        self, sound_editor, monkeypatch, speed, expected_text
    ):
        pyxel.sounds[0].speed = speed
        sound_editor.trigger_event("update")
        assert pyxel.sounds[0].speed == speed
        assert sound_editor.speed_var == speed

        pyxel.sounds[1].speed = speed
        sound_editor.sound_index_var = 1
        sound_editor.trigger_event("update")
        assert pyxel.sounds[1].speed == speed
        assert sound_editor.speed_var == speed

        drawn_text = []
        monkeypatch.setattr(
            pyxel, "text", lambda x, y, text, col: drawn_text.append(text)
        )
        sound_editor._speed_picker.trigger_event("draw")
        assert drawn_text == [expected_text]

    def test_speed_is_clamped_only_when_user_edits(self, sound_editor):
        sound_editor.speed_var = 150
        sound_editor.trigger_event("update")
        assert pyxel.sounds[0].speed == 150
        assert sound_editor.speed_var == 150

        sound_editor._speed_picker.dec_button.trigger_event("press")
        assert pyxel.sounds[0].speed == 99
        assert sound_editor.speed_var == 99

        sound_editor._speed_picker.dec_button.trigger_event("press")
        assert pyxel.sounds[0].speed == 98
        assert sound_editor.speed_var == 98


class TestNumberPicker:
    def test_initial_buttons_reflect_range_bounds(self):
        picker = NumberPicker(None, 0, 0, min_value=0, max_value=3, value=0)
        assert not picker.dec_button.is_enabled_var
        assert picker.inc_button.is_enabled_var

        picker = NumberPicker(None, 0, 0, min_value=0, max_value=3, value=3)
        assert picker.dec_button.is_enabled_var
        assert not picker.inc_button.is_enabled_var

    def test_initial_value_is_clamped(self):
        picker = NumberPicker(None, 0, 0, min_value=0, max_value=3, value=99)
        assert picker.value_var == 3
        assert not picker.inc_button.is_enabled_var


class TestScrollBar:
    @pytest.mark.parametrize("slider_amount", [2, 8, 16, 32])
    @pytest.mark.parametrize("value", [-1, 0, 12, 32, 99])
    def test_initial_value_stays_within_the_viewport(self, slider_amount, value):
        bar = ScrollBar(
            None,
            0,
            0,
            width=66,
            scroll_amount=32,
            slider_amount=slider_amount,
            value=value,
        )
        assert bar.value_var == min(max(value, 0), 32 - slider_amount)

    @pytest.mark.parametrize("slider_amount", [2, 8, 16, 32])
    def test_assigned_value_uses_the_same_bounds_as_buttons(self, slider_amount):
        bar = ScrollBar(
            None, 0, 0, width=66, scroll_amount=32, slider_amount=slider_amount, value=0
        )
        bar.value_var = 99
        assert bar.value_var == 32 - slider_amount
        bar.inc_button.trigger_event("press")
        assert bar.value_var == 32 - slider_amount

        bar.value_var = -1
        assert bar.value_var == 0
        bar.dec_button.trigger_event("press")
        assert bar.value_var == 0


class TestImageViewer:
    @pytest.mark.parametrize("tilemap_mode", [False, True])
    def test_right_drag_keeps_the_viewport_inside_the_image(self, tilemap_mode):
        parent = Widget(None, 0, 0, 256, 192)
        parent.new_var("image_index_var", 0)
        parent.new_var("help_message_var", "")
        if tilemap_mode:
            parent.new_var("tilemap_index_var", 0)
        viewer = ImageViewer(parent)

        viewer.trigger_event("mouse_down", pyxel.MOUSE_BUTTON_RIGHT, viewer.x, viewer.y)
        viewer.trigger_event(
            "mouse_drag", pyxel.MOUSE_BUTTON_RIGHT, viewer.x, viewer.y, -1000, -1000
        )
        assert viewer.viewport_x_var == 24
        assert viewer.viewport_y_var == (24 if tilemap_mode else 16)

        viewer.trigger_event(
            "mouse_drag", pyxel.MOUSE_BUTTON_RIGHT, viewer.x, viewer.y, 1000, 1000
        )
        assert viewer.viewport_x_var == 0
        assert viewer.viewport_y_var == 0

    def test_tile_drag_selection_keeps_the_pressed_tile(self):
        parent = Widget(None, 0, 0, 256, 192)
        parent.new_var("image_index_var", 0)
        parent.new_var("help_message_var", "")
        parent.new_var("tilemap_index_var", 0)
        viewer = ImageViewer(parent)
        viewer.viewport_x_var = 8
        viewer.viewport_y_var = 8
        press_x = viewer.x + 1 + 7 * 8
        press_y = viewer.y + 1 + 7 * 8
        viewer.trigger_event("mouse_down", pyxel.MOUSE_BUTTON_LEFT, press_x, press_y)
        assert (viewer.focus_x_var, viewer.focus_y_var) == (15, 15)

        viewer.trigger_event(
            "mouse_drag", pyxel.MOUSE_BUTTON_LEFT, press_x - 1000, press_y - 1000, 0, 0
        )
        assert (viewer.focus_x_var, viewer.focus_y_var) == (8, 8)
        assert (viewer.focus_w_var, viewer.focus_h_var) == (8, 8)

        viewer.trigger_event(
            "mouse_drag", pyxel.MOUSE_BUTTON_LEFT, press_x + 1000, press_y + 1000, 0, 0
        )
        assert (viewer.focus_x_var, viewer.focus_y_var) == (15, 15)
        assert (viewer.focus_w_var, viewer.focus_h_var) == (8, 8)


class TestFieldCursor:
    @pytest.fixture
    def cursor_fields(self):
        fields = [[], [10, 11, 12, 13, 14, 15], [3]]
        cursor = FieldCursor(
            None,
            max_field_length=32,
            field_wrap_length=16,
            max_field_values=[63] * len(fields),
            get_field=fields.__getitem__,
            add_pre_history=lambda *args, **kwargs: None,
            add_post_history=lambda *args, **kwargs: None,
            enable_cross_field_copy=True,
        )
        return cursor, fields

    @pytest.mark.parametrize("with_select_key", [False, True])
    @pytest.mark.parametrize(
        "source,destination,x,expected_x", [(0, 1, 4, 4), (2, 1, 4, 4), (1, 2, 4, 1)]
    )
    def test_cross_field_move_uses_destination_length(
        self, cursor_fields, source, destination, x, expected_x, with_select_key
    ):
        cursor, _ = cursor_fields
        cursor.move_to(0, source, False)
        cursor.move_to(x, destination, with_select_key)
        assert (cursor.x, cursor.y) == (expected_x, destination)
        assert not cursor.is_selecting

    def test_shift_only_changes_existing_values(self, cursor_fields):
        cursor, fields = cursor_fields
        cursor.move_to(6, 1, False)
        cursor.shift(1)
        cursor.move_to(0, 0, False)
        cursor.shift(1)
        assert fields == [[], [10, 11, 12, 13, 14, 15], [3]]

        cursor.move_to(4, 1, False)
        cursor.move_to(5, 1, True)
        cursor.shift(1)
        assert fields[1] == [10, 11, 12, 13, 15, 16]

    def test_first_music_field_click_inserts_at_clicked_position(
        self, cursor_fields, monkeypatch
    ):
        cursor, fields = cursor_fields
        parent = Widget(None, 0, 0, 256, 192)
        parent.field_cursor = cursor
        parent.get_field = fields.__getitem__
        parent.new_var("is_playing_var", False)
        parent.new_var("help_message_var", "")
        field = MusicField(parent, 0, 0, 1)
        monkeypatch.setattr(pyxel, "btn", lambda key: False)

        field.trigger_event("mouse_down", pyxel.MOUSE_BUTTON_LEFT, 21 + 4 * 12, 2)
        cursor.insert(63)
        assert fields == [[], [10, 11, 12, 13, 63, 14, 15], [3]]


class TestUserPal:
    def test_user_pal_maps_all_user_colors(self):
        saved_colors = list(pyxel.colors)
        saved_num_user = getattr(pyxel, "num_user_colors", None)
        try:
            user_colors = [0x100000 + i for i in range(64)]
            pyxel.colors[:] = saved_colors + user_colors
            pyxel.num_user_colors = len(user_colors)  # type: ignore[attr-defined]
            pyxel.user_pal()  # type: ignore[attr-defined]
            try:
                for i in range(pyxel.num_user_colors):  # type: ignore[attr-defined]
                    pyxel.cls(0)
                    pyxel.rect(0, 0, 1, 1, i)
                    assert pyxel.pget(0, 0) == pyxel.NUM_COLORS + i
            finally:
                pyxel.pal()
        finally:
            pyxel.colors[:] = saved_colors
            if saved_num_user is None:
                if hasattr(pyxel, "num_user_colors"):
                    delattr(pyxel, "num_user_colors")
            else:
                pyxel.num_user_colors = saved_num_user  # type: ignore[attr-defined]


def _hsv_pyxpal_lines(count):
    lines = []
    for i in range(count):
        r, g, b = colorsys.hsv_to_rgb(i / count, 0.8, 1.0)
        lines.append(f"{int(r * 255):02x}{int(g * 255):02x}{int(b * 255):02x}")
    return lines
