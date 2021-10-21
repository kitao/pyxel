import pyxel

from .settings import PANEL_FOCUS_BORDER_COLOR, PANEL_FOCUS_COLOR
from .widgets import Widget


class TilemapViewer(Widget):
    def __init__(self, parent):
        super().__init__(parent, 157, 16, 66, 65)

        self._tilemap_image = pyxel.Image(64, 63)

        self.copy_var("help_message_var", parent)

        # focus variables
        self.new_var("focus_x_var", 0)
        self.new_var("focus_y_var", 0)
        self.new_var("focus_w_var", 8)
        self.new_var("focus_h_var", 8)

        # event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("draw", self.__on_draw)

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            self.focus_x_var, self.focus_y_var = self._screen_to_view(x, y)

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_view(x, y)
        self.help_message_var = "TARGET:CURSOR ({},{})".format(x, y)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 2, 0), 30) * 8
        y = min(max((y - self.y - 1) // 2, 0), 30) * 8
        return x, y

    # def __on_update(self):
    #    pass
    # start_y = pyxel.frame_count % 8 * 8
    # tilemap = self.tilemap  # pyxel.tilemap(self.tilemap)
    # image = self.image  # pyxel.image(self.image)
    # minimap = self.image

    # for y in range(start_y, start_y + 8):
    #    for x in range(64):
    #        val = tilemap.pget(x * 4 + 1, y * 4 + 1)
    #        col = image.pget(val[0] * 8 + 3, val[1] * 8 + 3)
    #        minimap.pset(TILEMAP_IMAGE_X + x, TILEMAP_IMAGE_Y + y, col)

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        pyxel.blt(
            self.x + 1,
            self.y + 1,
            self._tilemap_image,
            0,
            0,
            self._tilemap_image.width,
            self._tilemap_image.height,
        )

        x = self.x + self.focus_x_var // 4 + 1
        y = self.y + self.focus_y_var // 4 + 1

        pyxel.clip(self.x + 1, self.y + 1, self.width - 2, self.height - 2)
        pyxel.rectb(x, y, 4, 4, PANEL_FOCUS_COLOR)
        pyxel.rectb(x - 1, y - 1, 6, 6, PANEL_FOCUS_BORDER_COLOR)
        pyxel.clip()
