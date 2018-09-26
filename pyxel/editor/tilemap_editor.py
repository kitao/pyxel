import pyxel

from .edit_window import EditWindow
from .editor import Editor
from .editor_radio_button import EditorRadioButton
from .select_window import SelectWindow


class TileMapEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent, "tilemap_editor.png")

        self._edit_window = EditWindow(self, is_tilemap_mode=True)
        self._select_window = SelectWindow(self, is_tilemap_mode=True)
        self._tilemap_button = EditorRadioButton(self, 47, 161, 3, 1, 3)
        self._tool_button = EditorRadioButton(self, 81, 161, 7, 1, 2)
        self._image_button = EditorRadioButton(self, 191, 161, 3, 1, 3)

        self.color = 0
        self.tool = 1

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)

    @property
    def tilemap(self):
        return self._tilemap_button.value

    @tilemap.setter
    def tilemap(self, value):
        self._tilemap_button.value = value

    @property
    def color(self):
        return (
            self._select_window.cursor_y // 8
        ) * 32 + self._select_window.cursor_x // 8

    @color.setter
    def color(self, value):
        self._select_window.cursor_y = (value // 32) * 8
        self._select_window.cursor_x = (value % 32) * 8

    @property
    def tool(self):
        return self._tool_button.value

    @tool.setter
    def tool(self, value):
        self._tool_button.value = value

    @property
    def image(self):
        return self._image_button.value

    @image.setter
    def image(self, value):
        self._image_button.value = value

    @property
    def select_x(self):
        return self._select_window.select_x

    @property
    def select_y(self):
        return self._select_window.select_y

    def __on_undo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        dest = pyxel.tilemap(tm).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self._edit_window.edit_x = x
        self._edit_window.edit_y = y
        self._tilemap_button.value = tm

    def __on_redo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]
        dest = pyxel.tilemap(tm).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self._edit_window.edit_x = x
        self._edit_window.edit_y = y
        self._tilemap_button.value = tm
