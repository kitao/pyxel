import pyxel

from .canvas_panel import CanvasPanel
from .editor_base import EditorBase
from .image_viewer import ImageViewer
from .settings import EDITOR_IMAGE, TEXT_LABEL_COLOR, TOOL_PENCIL
from .tilemap_viewer import TilemapViewer
from .widgets import NumberPicker, RadioButton


class TilemapEditor(EditorBase):
    def __init__(self, parent):
        super().__init__(parent)

        # canvas_var
        self.new_var("canvas_var", None)
        self.add_var_event_listener("canvas_var", "set", self.__on_canvas_get)

        # color_var
        self.new_var("color_var", None)  # TODO

        # tool button
        self._tool_button = RadioButton(
            self,
            81,
            161,
            img=EDITOR_IMAGE,
            u=63,
            v=0,
            btn_count=7,
            value=TOOL_PENCIL,
        )
        self.add_tool_button_help(self._tool_button)
        self.copy_var("tool_var", self._tool_button, "value_var")

        # tilemap picker
        self._tilemap_picker = NumberPicker(
            self, 48, 161, min_value=0, max_value=pyxel.TILEMAP_COUNT - 1, value=0
        )
        self._tilemap_picker.add_event_listener(
            "change", self.__on_tilemap_picker_change
        )
        self.add_number_picker_help(self._tilemap_picker)
        self.copy_var("tilemap_no_var", self._tilemap_picker, "value_var")

        # tilemap viewer
        self._tilemap_viewer = TilemapViewer(self)
        self.copy_var("focus_x_var", self._tilemap_viewer, "focus_x_var")
        self.copy_var("focus_y_var", self._tilemap_viewer, "focus_y_var")
        self.copy_var("focus_w_var", self._tilemap_viewer, "focus_w_var")
        self.copy_var("focus_h_var", self._tilemap_viewer, "focus_h_var")

        # image picker
        self._image_picker = NumberPicker(
            self,
            192,
            161,
            min_value=0,
            max_value=pyxel.IMAGE_COUNT - 1,
            value=pyxel.tilemap(self.tilemap_no_var).refimg,
        )
        self._image_picker.add_event_listener("change", self.__on_image_picker_change)
        self.add_number_picker_help(self._image_picker)
        self.copy_var("image_no_var", self._image_picker, "value_var")

        # image viewer
        self._image_viewer = ImageViewer(self)

        # canvas panel
        self._canvas_panel = CanvasPanel(self)

        # event listeners
        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    # @property
    # def color(self):
    #    return self._image_viewer.focused_tiles

    # @color.setter
    # def color(self, value):
    #    self._image_viewer.set_focus(value % 32 * 8, value // 32 * 8)

    # @property
    # def drawing_x(self):
    #    return self._canvas_panel.viewport_x

    # @drawing_x.setter
    # def drawing_x(self, value):
    #    self._canvas_panel.viewport_x = value

    # @property
    # def drawing_y(self):
    #    return self._canvas_panel.viewport_y

    # @drawing_y.setter
    # def drawing_y(self, value):
    #    self._canvas_panel.viewport_y = value

    def __on_canvas_get(self, value):
        return pyxel.tilemap(self.tilemap_no_var)

    def __on_tilemap_picker_change(self, value):
        self.image_no_var = pyxel.tilemap(value).refimg

    def __on_image_picker_change(self, value):
        pyxel.tilemap(self._tilemap_picker.value).refimg = value

    def __on_undo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]

        pyxel.tilemap(tm).set_slice(x, y, data["before"])

        self.focus_x_var = x
        self.focus_y_var = y
        self.tilemap_no_var = tm

    def __on_redo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]

        pyxel.tilemap(tm).set_slice(x, y, data["after"])

        self.focus_x_var = x
        self.focus_y_var = y
        self.tilemap_no_var = tm

    def __on_update(self):
        self.check_tool_button_shortcuts()

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(170, 162, "IMAGE", TEXT_LABEL_COLOR)
