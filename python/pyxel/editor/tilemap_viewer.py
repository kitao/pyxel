import pyxel

from .settings import PANEL_FOCUS_BORDER_COLOR, PANEL_FOCUS_COLOR
from .widgets import Widget


class TilemapViewer(Widget):
    """
    Variables:
        tilemap_no_var
        focus_x_var
        focus_y_var
        help_message_var
    """

    def __init__(self, parent):
        super().__init__(parent, 157, 16, 66, 65)
        self._tilemap_image = pyxel.Image(64, 63)
        self.copy_var("tilemap_no_var", parent)
        self.copy_var("help_message_var", parent)

        # Initialize focus_x_var
        self.new_var("focus_x_var", 0)
        self.add_var_event_listener("focus_x_var", "set", self.__on_focus_x_set)

        # Initialize focus_y_var
        self.new_var("focus_y_var", 0)
        self.add_var_event_listener("focus_y_var", "set", self.__on_focus_y_set)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def _screen_to_focus(self, x, y):
        x = min(max((x - self.x - 1) // 2, 0), 31)
        y = min(max((y - self.y - 1) // 2, 0), 31)
        return x, y

    def __on_focus_x_set(self, value):
        return min(max(value, 0), 30)

    def __on_focus_y_set(self, value):
        return min(max(value, 0), 30)

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            self.focus_x_var, self.focus_y_var = self._screen_to_focus(x, y)

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_focus(x, y)
        self.help_message_var = f"TARGET:CURSOR ({x * 8},{y * 8})"

    def __on_update(self):
        tilemap = pyxel.tilemap(self.tilemap_no_var)
        image = tilemap.image
        start_y = pyxel.frame_count % 8 * 8
        for y in range(start_y, start_y + 8):
            for x in range(64):
                tile = tilemap.pget(x * 4 + 1, y * 4 + 1)
                col = image.pget(tile[0] * 8 + 3, tile[1] * 8 + 3)
                self._tilemap_image.pset(x, y, col)

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        # Draw tilemap
        pyxel.blt(
            self.x + 1,
            self.y + 1,
            self._tilemap_image,
            0,
            0,
            self._tilemap_image.width,
            self._tilemap_image.height,
        )

        # Draw focus
        x = self.x + self.focus_x_var * 2 + 1
        y = self.y + self.focus_y_var * 2 + 1
        pyxel.clip(self.x + 1, self.y + 1, self.width - 2, self.height - 2)
        pyxel.rectb(x, y, 4, 4, PANEL_FOCUS_COLOR)
        pyxel.rectb(x - 1, y - 1, 6, 6, PANEL_FOCUS_BORDER_COLOR)
        pyxel.clip()
