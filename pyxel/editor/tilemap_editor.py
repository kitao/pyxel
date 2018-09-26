import pyxel

from .edit_window import EditWindow
from .editor import Editor
from .editor_radio_button import EditorRadioButton
from .image_window import ImageWindow


class TileMapEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent, "tilemap_editor.png")

        self.tilemap_button = EditorRadioButton(self, 47, 161, 10, 3, 1)
        self.tilemap_button.value = 0

        self.tool_button = EditorRadioButton(self, 81, 161, 7, 1, 2)
        self.tool_button.value = 1

        self.image_button = EditorRadioButton(self, 191, 161, 3, 1, 3)
        self.edit_window = EditWindow(self)
        self.image_window = ImageWindow(self)

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)

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
