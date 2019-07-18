import pyxel
from pyxel.ui import NumberPicker, RadioButton

from .constants import (
    EDITOR_IMAGE_X,
    EDITOR_IMAGE_Y,
    TILEMAP_IMAGE_X,
    TILEMAP_IMAGE_Y,
    TOOL_PENCIL,
)
from .drawing_panel import DrawingPanel
from .editor import Editor
from .image_panel import ImagePanel
from .tilemap_panel import TilemapPanel


class TileMapEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self._drawing_panel = DrawingPanel(self, is_tilemap_mode=True)
        self._tilemap_panel = TilemapPanel(self)
        self._image_panel = ImagePanel(self, is_tilemap_mode=True)
        self._tilemap_picker = NumberPicker(
            self, 48, 161, 0, pyxel.TILEMAP_BANK_COUNT - 1, 0
        )
        self._tool_button = RadioButton(
            self,
            81,
            161,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X + 63,
            EDITOR_IMAGE_Y,
            7,
            TOOL_PENCIL,
        )
        self._image_picker = NumberPicker(
            self,
            192,
            161,
            0,
            pyxel.IMAGE_BANK_COUNT - 1,
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
        return self._tilemap_picker.value

    @tilemap.setter
    def tilemap(self, value):
        self._tilemap_button.value = value

    @property
    def color(self):
        return self._image_panel.focused_tiles

    @color.setter
    def color(self, value):
        self._image_panel.set_focus(value % 32 * 8, value // 32 * 8)

    @property
    def tool(self):
        return self._tool_button.value

    @tool.setter
    def tool(self, value):
        self._tool_button.value = value

    @property
    def image(self):
        return self._image_picker.value

    @image.setter
    def image(self, value):
        self._image_button.value = value

    @property
    def drawing_x(self):
        return self._drawing_panel.viewport_x

    @drawing_x.setter
    def drawing_x(self, value):
        self._drawing_panel.viewport_x = value

    @property
    def drawing_y(self):
        return self._drawing_panel.viewport_y

    @drawing_y.setter
    def drawing_y(self, value):
        self._drawing_panel.viewport_y = value

    def __on_undo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        dest = pyxel.tilemap(tm).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self._drawing_panel.viewport_x = x
        self._drawing_panel.viewport_y = y
        self._tilemap_picker.value = tm

    def __on_redo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        dest = pyxel.tilemap(tm).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self._drawing_panel.viewport_x = x
        self._drawing_panel.viewport_y = y
        self._tilemap_picker.value = tm

    def __on_update(self):
        start_y = pyxel.frame_count % 8 * 8
        tilemap_data = pyxel.tilemap(self.tilemap).data
        image_data = pyxel.image(self.image).data
        minimap_data = pyxel.image(3, system=True).data

        for y in range(start_y, start_y + 8):
            for x in range(64):
                val = tilemap_data[y * 4 + 1, x * 4 + 1]
                col = image_data[val // 32 * 8 + 3, val % 32 * 8 + 3]
                minimap_data[TILEMAP_IMAGE_Y + y, TILEMAP_IMAGE_X + x] = col

        self.check_tool_button_shortcuts()

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(18, 162, "TILEMAP", 6)
        pyxel.text(18, 162, "TILEMAP", 6)
        pyxel.text(170, 162, "IMAGE", 6)

    def __on_tilemap_picker_change(self, value):
        self._image_picker.value = pyxel.tilemap(value).refimg

    def __on_image_picker_change(self, value):
        pyxel.tilemap(self._tilemap_picker.value).refimg = value
