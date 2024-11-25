import pyxel

from .canvas_panel import CanvasPanel
from .editor_base import EditorBase
from .image_viewer import ImageViewer
from .settings import EDITOR_IMAGE, TEXT_LABEL_COLOR, TOOL_PENCIL
from .tilemap_viewer import TilemapViewer
from .widgets import NumberPicker, RadioButton


class TilemapEditor(EditorBase):
    """
    Variables:
        color_var
        tool_var
        image_index_var
        canvas_var
        focus_x_var
        focus_y_var

        tilemap_index_var
        tile_x_var
        tile_y_var
        tile_w_var
        tile_h_var

    Events:
        undo (data)
        redo (data)
        drop (filename)
    """

    def __init__(self, parent):
        super().__init__(parent)

        # Initialize canvas_var
        self.new_var("canvas_var", None)
        self.add_var_event_listener("canvas_var", "get", self.__on_canvas_get)

        # Initialize color_var
        self.new_var("color_var", (255, 255))

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

        # Initialize tilemap picker
        self._tilemap_picker = NumberPicker(
            self, 48, 161, min_value=0, max_value=pyxel.NUM_TILEMAPS - 1, value=0
        )
        self._tilemap_picker.add_event_listener(
            "change", self.__on_tilemap_picker_change
        )
        self._tilemap_picker.add_event_listener(
            "mouse_hover", self.__on_tilemap_picker_mouse_hover
        )
        self.add_number_picker_help(self._tilemap_picker)
        self.copy_var("tilemap_index_var", self._tilemap_picker, "value_var")

        # Initialize tilemap viewer
        self._tilemap_viewer = TilemapViewer(self)
        self.copy_var("focus_x_var", self._tilemap_viewer, "focus_x_var")
        self.copy_var("focus_y_var", self._tilemap_viewer, "focus_y_var")

        # Initialize image picker
        self._image_picker = NumberPicker(
            self,
            192,
            161,
            min_value=0,
            max_value=pyxel.NUM_IMAGES - 1,
            value=pyxel.tilemaps[self.tilemap_index_var].imgsrc,
        )
        self._image_picker.add_event_listener("change", self.__on_image_picker_change)
        self.add_number_picker_help(self._image_picker)
        self.copy_var("image_index_var", self._image_picker, "value_var")

        # Initialize image viewer
        self._image_viewer = ImageViewer(self)
        self.copy_var("tile_x_var", self._image_viewer, "focus_x_var")
        self.copy_var("tile_y_var", self._image_viewer, "focus_y_var")
        self.copy_var("tile_w_var", self._image_viewer, "focus_w_var")
        self.copy_var("tile_h_var", self._image_viewer, "focus_h_var")

        # Initialize canvas panel
        self._canvas_panel = CanvasPanel(self)

        # Set event listeners
        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("drop", self.__on_drop)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def __on_canvas_get(self, value):
        return pyxel.tilemaps[self.tilemap_index_var]

    def __on_tilemap_picker_change(self, value):
        self.image_index_var = pyxel.tilemaps[value].imgsrc

    def __on_tilemap_picker_mouse_hover(self, x, y):
        self.help_message_var = "COPY_ALL:CTRL+SHIFT+C/X/V"

    def __on_image_picker_change(self, value):
        pyxel.tilemaps[self.tilemap_index_var].imgsrc = value

    def __on_undo(self, data):
        self.tilemap_index_var = data["tilemap_index"]
        if "old_data" in data:
            pyxel.tilemaps[self.tilemap_index_var].set_slice(0, 0, data["old_data"])
            self.image_index_var = data["old_imgsrc"]
        else:
            self.focus_x_var, self.focus_y_var = data["focus_pos"]
            self.canvas_var.set_slice(
                self.focus_x_var * 8, self.focus_y_var * 8, data["old_canvas"]
            )

    def __on_redo(self, data):
        self.tilemap_index_var = data["tilemap_index"]
        if "new_data" in data:
            pyxel.tilemaps[self.tilemap_index_var].set_slice(0, 0, data["new_data"])
            self.image_index_var = data["new_imgsrc"]
        else:
            self.focus_x_var, self.focus_y_var = data["focus_pos"]
            self.canvas_var.set_slice(
                self.focus_x_var * 8, self.focus_y_var * 8, data["new_canvas"]
            )

    def __on_drop(self, filename):
        pyxel.tilemaps[self.tilemap_index_var].load(
            self.focus_x_var * 8, self.focus_y_var * 8, filename, 0
        )

    def __on_update(self):
        self.check_tool_button_shortcuts()

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(170, 162, "IMAGE", TEXT_LABEL_COLOR)
