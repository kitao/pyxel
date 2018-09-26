import pyxel

from .edit_window import EditWindow
from .editor import Editor
from .editor_radio_button import EditorRadioButton
from .image_window import ImageWindow


class TileMapEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent, "tilemap_editor.png")

        self._edit_window = EditWindow(self, is_tilemap_mode=True)
        self._image_window = ImageWindow(self, is_tilemap_mode=True)
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
            self._tilemap_button.value * 1000
            + (self._image_window.cursor_y // 8) * 32
            + (self._image_window.cursor_x // 8) * 8
        )

    @color.setter
    def color(self, value):
        self._tilemap_button.value = value // 1000

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
    def edit_x(self):
        return self._edit_window.edit_x

    @edit_x.setter
    def edit_x(self, value):
        self._edit_window.edit_x = value

    @property
    def edit_y(self):
        return self._edit_window.edit_y

    @edit_y.setter
    def edit_y(self, value):
        self._edit_window.edit_y = value

    @property
    def image_x(self):
        return self._image_window.image_x

    @image_x.setter
    def image_x(self, value):
        self._image_window.image_x = value

    @property
    def image_y(self):
        return self._image_window.image_y

    @image_y.setter
    def image_y(self, value):
        self._image_window.image_y = value

    def __on_undo(self, data):
        img = data["img"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self.edit_window.edit_x = x
        self.edit_window.edit_y = y
        self.image_button.value = img

    def __on_redo(self, data):
        img = data["img"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self.edit_window.edit_x = x
        self.edit_window.edit_y = y
        self.image_button.value = img
