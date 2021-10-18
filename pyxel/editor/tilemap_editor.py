import pyxel

from .canvas_panel import CanvasPanel
from .editor_base import EditorBase
from .image_viewer import ImageViewer
from .settings import EDITOR_IMAGE, TEXT_LABEL_COLOR, TOOL_PENCIL
from .tilemap_viewer import TilemapViewer
from .widgets import NumberPicker, RadioButton, WidgetVariable


class TilemapEditor(EditorBase):
    def __init__(self, parent):
        super().__init__(parent)

        self.canvas_var = WidgetVariable(None, on_get=self.__on_canvas_get)
        self.color_var = None  # TODO

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

        # tilemap picker
        self._tilemap_picker = NumberPicker(
            self, 48, 161, 0, pyxel.TILEMAP_COUNT - 1, 0
        )
        self._tilemap_picker.add_event_listener(
            "change", self.__on_tilemap_picker_change
        )
        self.add_number_picker_help(self._tilemap_picker)
        self.tilemap_no_var = self._tilemap_picker.value_var

        # tilemap viewer
        self._tilemap_viewer = TilemapViewer(self)
        self.focus_x_var = self._tilemap_viewer.focus_x_var
        self.focus_y_var = self._tilemap_viewer.focus_y_var
        self.focus_w_var = self._tilemap_viewer.focus_w_var
        self.focus_h_var = self._tilemap_viewer.focus_h_var

        # image picker
        self._image_picker = NumberPicker(
            self,
            192,
            161,
            0,
            pyxel.IMAGE_COUNT - 1,
            pyxel.tilemap(self.tilemap_no_var.v).refimg,
        )
        self._image_picker.add_event_listener("change", self.__on_image_picker_change)
        self.add_number_picker_help(self._image_picker)
        self.image_no_var = self._image_picker.value_var

        # image viewer
        self._image_viewer = ImageViewer(self, ImageViewer.SIZE_SMALL)

        # canvas panel
        self._canvas_panel = CanvasPanel(self, is_tilemap_mode=True)

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
        return pyxel.tilemap(self.tilemap_no_var.v)

    def __on_tilemap_picker_change(self, value):
        self.image_no_var.v = pyxel.tilemap(value).refimg

    def __on_image_picker_change(self, value):
        pyxel.tilemap(self._tilemap_picker.value).refimg = value

    def __on_undo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]

        pyxel.tilemap(tm).set_slice(x, y, data["before"])

        self.focus_x_var.v = x
        self.focus_y_var.v = y
        self.tilemap_no_var.v = tm

    def __on_redo(self, data):
        tm = data["tilemap"]
        x, y = data["pos"]

        pyxel.tilemap(tm).set_slice(x, y, data["after"])

        self.focus_x_var.v = x
        self.focus_y_var.v = y
        self.tilemap_no_var.v = tm

    def __on_update(self):
        self.check_tool_button_shortcuts()

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(18, 162, "TILEMAP", TEXT_LABEL_COLOR)
        pyxel.text(170, 162, "IMAGE", TEXT_LABEL_COLOR)
