import pyxel

from .canvas_panel import CanvasPanel
from .editor_base import EditorBase
from .image_viewer import ImageViewer
from .settings import EDITOR_IMAGE, TEXT_LABEL_COLOR, TOOL_PENCIL
from .widgets import ColorPicker, NumberPicker, RadioButton

_COLOR_SHORTCUT_KEYS = tuple(pyxel.KEY_1 + i for i in range(8))


class ImageEditor(EditorBase):
    # Variables:
    #   color_var
    #   tool_var
    #   image_index_var
    #   canvas_var
    #   focus_x_var
    #   focus_y_var
    #   help_message_var
    #
    # Events:
    #   undo (data)
    #   redo (data)
    #   drop (filename)

    def __init__(self, parent):
        super().__init__(parent)

        self.new_var("canvas_var", None)
        self.add_var_event_listener("canvas_var", "get", self.__on_canvas_get)

        # Initialize color picker
        self._color_picker = ColorPicker(
            self,
            11,
            156,
            min(7, pyxel.num_user_colors - 1),
            with_shadow=False,
        )
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
        self._image_picker.add_event_listener(
            "mouse_hover", self.__on_image_picker_mouse_hover
        )
        self.add_number_picker_help(self._image_picker)
        self.copy_var("image_index_var", self._image_picker, "value_var")

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

    # Helpers

    def _restore_state(self, data, prefix):
        self.image_index_var = data["image_index"]
        if f"{prefix}_data" in data:
            pyxel.images[self.image_index_var].set_slice(0, 0, data[f"{prefix}_data"])
        else:
            self.focus_x_var, self.focus_y_var = data["focus_pos"]
            self.canvas_var.set_slice(
                self.focus_x_var * 8, self.focus_y_var * 8, data[f"{prefix}_canvas"]
            )

    # Event handlers

    def __on_canvas_get(self, value):
        return pyxel.images[self.image_index_var]

    def __on_color_picker_mouse_hover(self, _x, _y):
        self.help_message_var = "COLOR:1-8/SHIFT+1-8"

    def __on_image_picker_mouse_hover(self, _x, _y):
        self.help_message_var = "COPY_ALL:CTRL+SHIFT+C/X/V"

    def __on_undo(self, data):
        self._restore_state(data, "old")

    def __on_redo(self, data):
        self._restore_state(data, "new")

    def __on_drop(self, filename):
        colors = list(pyxel.colors)
        user_colors = colors[pyxel.NUM_COLORS :]
        pyxel.colors[:] = user_colors
        try:
            pyxel.images[self.image_index_var].load(
                self.focus_x_var * 8, self.focus_y_var * 8, filename
            )
        except (OSError, ValueError) as e:
            print(f"Failed to load image: {e}")
        finally:
            pyxel.colors[:] = colors

    def __on_update(self):
        self.check_tool_button_shortcuts()

        # Check color shortcuts
        if not pyxel.btn(pyxel.KEY_ALT):
            for key in _COLOR_SHORTCUT_KEYS:
                if pyxel.btnp(key):
                    col = key - pyxel.KEY_1
                    if pyxel.btn(pyxel.KEY_SHIFT):
                        col += 8
                    self.color_var = col
                    break

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(170, 162, "IMAGE", TEXT_LABEL_COLOR)
