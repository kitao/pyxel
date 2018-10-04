import pyxel
from pyxel.constants import RENDERER_IMAGE_COUNT, RENDERER_TILEMAP_COUNT
from pyxel.ui import NumberPicker, RadioButton

from .constants import TOOL_PENCIL
from .edit_frame import EditFrame
from .editor import Editor
from .image_frame import ImageFrame
from .tilemap_frame import TilemapFrame


class TileMapEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent, "tilemap_editor.png")

        self._edit_frame = EditFrame(self, is_tilemap_mode=True)
        self._tilemap_frame = TilemapFrame(self)
        self._select_frame = ImageFrame(self, is_tilemap_mode=True)
        self._tilemap_number = NumberPicker(
            self, 48, 161, 0, RENDERER_TILEMAP_COUNT - 1, 0
        )
        self._tool_button = RadioButton(self, 81, 161, 3, 63, 16, 7, TOOL_PENCIL)
        self._image_number = NumberPicker(
            self, 192, 161, 0, RENDERER_IMAGE_COUNT - 2, 0
        )

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    @property
    def tilemap(self):
        return self._tilemap_number.value

    @tilemap.setter
    def tilemap(self, value):
        self._tilemap_button.value = value

    @property
    def color(self):
        return (
            self._select_frame.select_y // 8
        ) * 32 + self._select_frame.select_x // 8

    @color.setter
    def color(self, value):
        self._select_frame.cursor_y = (value // 32) * 8
        self._select_frame.cursor_x = (value % 32) * 8

    @property
    def tool(self):
        return self._tool_button.value

    @tool.setter
    def tool(self, value):
        self._tool_button.value = value

    @property
    def image(self):
        return self._image_number.value

    @image.setter
    def image(self, value):
        self._image_button.value = value

    @property
    def edit_x(self):
        return self._edit_frame.viewport_x

    @edit_x.setter
    def edit_x(self, value):
        self._edit_frame.viewport_x = value

    @property
    def edit_y(self):
        return self._edit_frame.viewport_y

    @edit_y.setter
    def edit_y(self, value):
        self._edit_frame.viewport_y = value

    def __on_undo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        dest = pyxel.tilemap(tm).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self._edit_frame.edit_x = x
        self._edit_frame.edit_y = y
        self._tilemap_number.value = tm

    def __on_redo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        dest = pyxel.tilemap(tm).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self._edit_frame.edit_x = x
        self._edit_frame.edit_y = y
        self._tilemap_number.value = tm

    def __on_update(self):
        start_y = (pyxel.frame_count % 8) * 8
        for y in range(start_y, start_y + 8):
            for x in range(64):
                val = pyxel.tilemap(self.tilemap).data[y * 4 + 1, x * 4 + 1]
                col = pyxel.image(self.image).data[
                    (val // 32) * 8 + 3, (val % 32) * 8 + 3
                ]
                pyxel.image(3, system=True).data[y + 192, x] = col

    def __on_draw(self):
        self.draw_frame(11, 156, 136, 17)
        self.draw_frame(157, 156, 72, 17)
        pyxel.text(18, 162, "TILEMAP", 6)
        pyxel.text(18, 162, "TILEMAP", 6)
        pyxel.text(170, 162, "IMAGE", 6)
