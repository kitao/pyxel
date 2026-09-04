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


def _hsv_pyxpal_lines(count):
    lines = []
    for i in range(count):
        r, g, b = colorsys.hsv_to_rgb(i / count, 0.8, 1.0)
        lines.append(f"{int(r * 255):02x}{int(g * 255):02x}{int(b * 255):02x}")
    return lines


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
