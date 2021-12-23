import pyxel

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
        canvas_var
        focus_x_var
        focus_y_var
        help_message_var

    Events:
        undo (data)
        redo (data)
        drop (filename)
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
        self.copy_var("help_message_var", parent)

        # Initialize canvas_var
        self.new_var("canvas_var", None)
        self.add_var_event_listener("canvas_var", "get", self.__on_canvas_get)

        # Initialize color picker
        self._color_picker = ColorPicker(self, 11, 156, 7, with_shadow=False)
        self._color_picker.add_event_listener(
            "mouse_hover", self.__on_color_picker_mouse_hover
        )
        self.copy_var("color_var", self._color_picker, "value_var")

        # Initialize tool button
        self._tool_button = RadioButton(
            self,
            81,
            161,
            img=EDITOR_IMAGE,
            u=63,
            v=0,
            num_buttons=7,
            value=TOOL_PENCIL,
        )
        self.add_tool_button_help(self._tool_button)
        self.copy_var("tool_var", self._tool_button, "value_var")

        # Initialize image picker
        self._image_picker = NumberPicker(
            self, 192, 161, min_value=0, max_value=pyxel.NUM_IMAGES - 1, value=0
        )
        self.add_number_picker_help(self._image_picker)
        self.copy_var("image_no_var", self._image_picker, "value_var")

        # Initialize image viewer
        self._image_viewer = ImageViewer(self)
        self.copy_var("focus_x_var", self._image_viewer)
        self.copy_var("focus_y_var", self._image_viewer)

        # Initialize canvas panel
        self._canvas_panel = CanvasPanel(self)

        # Set event listeners
        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("drop", self.__on_drop)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def __on_canvas_get(self, value):
        return pyxel.image(self.image_no_var)

    def __on_color_picker_mouse_hover(self, x, y):
        self.help_message_var = "COLOR:1-8/SHIFT+1-8"

    def __on_undo(self, data):
        self.image_no_var = data["image_no"]
        self.focus_x_var, self.focus_y_var = data["focus_pos"]
        self.canvas_var.set_slice(
            self.focus_x_var * 8, self.focus_y_var * 8, data["old_canvas"]
        )

    def __on_redo(self, data):
        self.image_no_var = data["image_no"]
        self.focus_x_var, self.focus_y_var = data["focus_pos"]
        self.canvas_var.set_slice(
            self.focus_x_var * 8, self.focus_y_var * 8, data["new_canvas"]
        )

    def __on_drop(self, filename):
        pyxel.image(self.image_no_var).load(0, 0, filename)

    def __on_update(self):
        self.check_tool_button_shortcuts()

        # Check color shortcuts
        if not pyxel.btn(pyxel.KEY_ALT):
            for btn in self._COLOR_BUTTONS:
                if pyxel.btnp(btn):
                    col = btn - pyxel.KEY_1
                    if pyxel.btn(pyxel.KEY_SHIFT):
                        col += 8
                    self.color_var = col
                    break

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(170, 162, "IMAGE", TEXT_LABEL_COLOR)
