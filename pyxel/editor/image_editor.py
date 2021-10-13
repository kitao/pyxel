import pyxel

from .canvas_panel import CanvasPanel
from .editor_base import EditorBase
from .image_selector import ImageSelector
from .settings import EDITOR_IMAGE, TEXT_LABEL_COLOR, TOOL_PENCIL
from .widgets import ColorPicker, NumberPicker, RadioButton


class ImageEditor(EditorBase):
    _COLOR_BUTTONS = (
        pyxel.KEY_1,
        pyxel.KEY_2,
        pyxel.KEY_3,
        pyxel.KEY_4,
        pyxel.KEY_5,
        pyxel.KEY_6,
        pyxel.KEY_7,
        pyxel.KEY_8,
    )

    def __init__(self, parent):
        super().__init__(parent)

        self.edit_x = 0
        self.edit_y = 0
        self.edit_width = 16
        self.edit_height = 16

        self._canvas_panel = CanvasPanel(self, is_tilemap_mode=False)
        self._color_picker = ColorPicker(self, 11, 156, 7, with_shadow=False)
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
        self._image_picker = NumberPicker(self, 192, 161, 0, pyxel.IMAGE_COUNT - 1, 0)
        self._image_selector = ImageSelector(self, is_image_editor=True)

        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("drop", self.__on_drop)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)
        self._color_picker.add_event_listener(
            "mouse_hover", self.__on_color_picker_mouse_hover
        )
        self.add_tool_button_help(self._tool_button)
        self.add_number_picker_help(self._image_picker)

    @property
    def color(self):
        return self._color_picker.value

    @property
    def tool(self):
        return self._tool_button.value

    @property
    def image_no(self):
        return self._image_picker.value

    # @property
    # def image(self):
    #    return pyxel.image(self._image_picker.value)

    @property
    def canvas(self):
        return pyxel.image(self._image_picker.value)

    # @color.setter
    # def color(self, value):
    #    self._color_picker.value = int(value)

    # @tool.setter
    # def tool(self, value):
    #    self._tool_button.value = value

    def __on_undo(self, data):
        img = data["image"]
        x, y = data["pos"]
        pyxel.image(img).set_slice(x, y, data["before"])

        self.canvas_x = x
        self.canvas_y = y
        self.parent.image = img

    def __on_redo(self, data):
        img = data["image"]
        x, y = data["pos"]
        pyxel.image(img).set_slice(x, y, data["after"])

        self.canvas_x = x
        self.canvas_y = y
        self.parent.image = img

    def __on_drop(self, filename):
        pyxel.image(self.image).load(0, 0, filename)

    def __on_update(self):
        self.check_tool_button_shortcuts()

        if not pyxel.btn(pyxel.KEY_ALT):
            for btn in self._COLOR_BUTTONS:
                if pyxel.btnp(btn):
                    col = btn - pyxel.KEY_1
                    if pyxel.btn(pyxel.KEY_SHIFT):
                        col += 8
                    self._color_picker.value = col
                    break

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(170, 162, "IMAGE", TEXT_LABEL_COLOR)

    def __on_color_picker_mouse_hover(self, x, y):
        self.show_help_message("COLOR:1-8/SHIFT+1-8")
