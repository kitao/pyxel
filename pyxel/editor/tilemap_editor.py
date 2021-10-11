import pyxel

from .canvas_panel import CanvasPanel
from .editor_base import EditorBase
from .image_selector import ImageSelector
from .settings import (
    EDITOR_IMAGE,
    TEXT_LABEL_COLOR,
    TILEMAP_IMAGE_X,
    TILEMAP_IMAGE_Y,
    TOOL_PENCIL,
)
from .tilemap_selector import TilemapSelector
from .widgets import NumberPicker, RadioButton


class TilemapEditor(EditorBase):
    def __init__(self, parent):
        super().__init__(parent)

        self.edit_x = 0
        self.edit_y = 0
        self.edit_width = 8
        self.edit_height = 8

        self._canvas_panel = CanvasPanel(self, is_tilemap_mode=True)
        self._tilemap_selector = TilemapSelector(self)
        self._image_selector = ImageSelector(self, is_image_editor=False)
        self._tilemap_picker = NumberPicker(
            self, 48, 161, 0, pyxel.TILEMAP_COUNT - 1, 0
        )
        self._tool_button = RadioButton(
            self,
            81,
            161,
            EDITOR_IMAGE,
            63,
            0,
            7,
            TOOL_PENCIL,
        )
        self._image_picker = NumberPicker(
            self,
            192,
            161,
            0,
            pyxel.IMAGE_COUNT - 1,
            pyxel.tilemap(self._tilemap_picker.value).refimg,
        )

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self._tilemap_picker.add_event_handler(
            "change", self.__on_tilemap_picker_change
        )
        self._image_picker.add_event_handler("change", self.__on_image_picker_change)
        self.add_number_picker_help(self._tilemap_picker)
        self.add_number_picker_help(self._image_picker)
        self.add_tool_button_help(self._tool_button)

    @property
    def tilemap(self):
        return pyxel.tilemap(self._tilemap_picker.value)

    @property
    def canvas(self):
        return self.tilemap

    @property
    def image(self):
        return pyxel.image(self._image_picker.value)

    @property
    def color(self):
        return self._image_viewer.focused_tiles

    @color.setter
    def color(self, value):
        self._image_viewer.set_focus(value % 32 * 8, value // 32 * 8)

    @property
    def tool(self):
        return self._tool_button.value

    @tool.setter
    def tool(self, value):
        self._tool_button.value = value

    @property
    def drawing_x(self):
        return self._canvas_panel.viewport_x

    @drawing_x.setter
    def drawing_x(self, value):
        self._canvas_panel.viewport_x = value

    @property
    def drawing_y(self):
        return self._canvas_panel.viewport_y

    @drawing_y.setter
    def drawing_y(self, value):
        self._canvas_panel.viewport_y = value

    def __on_undo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        pyxel.tilemap(tm).set_slice(x, y, data["before"])

        self._canvas_panel.viewport_x = x
        self._canvas_panel.viewport_y = y
        self._tilemap_picker.value = tm

    def __on_redo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        pyxel.tilemap(tm).set_slice(x, y, data["after"])

        self._canvas_panel.viewport_x = x
        self._canvas_panel.viewport_y = y
        self._tilemap_picker.value = tm

    def __on_update(self):
        start_y = pyxel.frame_count % 8 * 8
        tilemap = self.tilemap  # pyxel.tilemap(self.tilemap)
        image = self.image  # pyxel.image(self.image)
        minimap = self.image

        for y in range(start_y, start_y + 8):
            for x in range(64):
                val = tilemap.pget(x * 4 + 1, y * 4 + 1)
                col = image.pget(val[0] * 8 + 3, val[1] * 8 + 3)
                minimap.pset(TILEMAP_IMAGE_X + x, TILEMAP_IMAGE_Y + y, col)

        self.check_tool_button_shortcuts()

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(170, 162, "IMAGE", TEXT_LABEL_COLOR)

    def __on_tilemap_picker_change(self, value):
        self._image_picker.value = pyxel.tilemap(value).refimg

    def __on_image_picker_change(self, value):
        pyxel.tilemap(self._tilemap_picker.value).refimg = value
