import pyxel
from pyxel.constants import RENDERER_IMAGE_COUNT
from pyxel.ui import ColorPicker, NumberPicker, RadioButton

from .edit_frame import EditFrame
from .editor import Editor
from .image_frame import ImageFrame


class ImageEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent, "image_editor.png")

        self._edit_frame = EditFrame(self, is_tilemap_mode=False)
        self._image_frame = ImageFrame(self, is_tilemap_mode=False)
        self._color_picker = ColorPicker(self, 12, 157)
        self._tool_button = RadioButton(self, 81, 161, 3, 81, 173, 7)
        self._image_number = NumberPicker(self, 192, 161, 0, RENDERER_IMAGE_COUNT - 2)

        self.color = 7
        self.tool = 1

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)
        self.add_event_handler("draw", self.__on_draw)

    @property
    def color(self):
        return self._color_picker.value

    @color.setter
    def color(self, value):
        self._color_picker.value = value

    @property
    def tool(self):
        return self._tool_button.value

    @tool.setter
    def tool(self, value):
        self._tool_button.value = value

    @property
    def image(self):
        return self._image_number.value

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
        img = data["image"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self.edit_x = x
        self.edit_y = y
        self.parent.image = img

    def __on_redo(self, data):
        img = data["image"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self.edit_x = x
        self.edit_y = y
        self.parent.image = img

    def __on_draw(self):
        self.draw_frame(11, 156, 146, 172)
        self.draw_frame(157, 156, 228, 172)
        pyxel.text(170, 162, "IMAGE", 6)
