import pyxel
from pyxel.editor.widgets.widget_variable import WidgetVariable

from .canvas_panel import CanvasPanel
from .editor_base import EditorBase
from .image_viewer import ImageViewer
from .settings import EDITOR_IMAGE, TEXT_LABEL_COLOR, TOOL_PENCIL
from .widgets import ColorPicker, NumberPicker, RadioButton


class ImageEditor(EditorBase):
    """
    Variables:
        color_var
        tool_var
        image_no_var
    """

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

        self.canvas_var = WidgetVariable(None, on_get=self.__on_canvas_get)

        # color picker
        self._color_picker = ColorPicker(self, 11, 156, 7, with_shadow=False)
        self._color_picker.add_event_listener(
            "mouse_hover", self.__on_color_picker_mouse_hover
        )
        self.color_var = self._color_picker.value_var

        # tool button
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
        self.add_tool_button_help(self._tool_button)
        self.tool_var = self._tool_button.value_var

        # image picker
        self._image_picker = NumberPicker(self, 192, 161, 0, pyxel.IMAGE_COUNT - 1, 0)
        self.add_number_picker_help(self._image_picker)
        self.image_no_var = self._image_picker.value_var

        # image viewer
        self._image_viewer = ImageViewer(self, ImageViewer.SIZE_LARGE)
        self.focus_x_var = self._image_viewer.focus_x_var
        self.focus_y_var = self._image_viewer.focus_y_var
        self.focus_w_var = self._image_viewer.focus_w_var
        self.focus_h_var = self._image_viewer.focus_h_var

        # canvas panel
        self._canvas_panel = CanvasPanel(self, is_tilemap_mode=False)

        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("drop", self.__on_drop)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    @property
    def canvas(self):
        return pyxel.image(self._image_picker.value)

    def __on_canvas_get(self, value):
        return pyxel.image(self.image_no_var.v)

    def __on_color_picker_mouse_hover(self, x, y):
        self.help_message_var.v = "COLOR:1-8/SHIFT+1-8"

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
