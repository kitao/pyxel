import pyxel

from .settings import OCTAVE_BAR_BACKGROUND_COLOR, OCTAVE_BAR_COLOR
from .widgets import Widget


class OctaveBar(Widget):
    """
    Variables:
        octave_var
        help_message_var
    """

    def __init__(self, parent, x, y):
        super().__init__(parent, x, y, 4, 123)

        self.field_cursor = parent.field_cursor

        self.copy_var("octave_var", parent)
        self.copy_var("help_message_var", parent)

        # event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("draw", self.__on_draw)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self.field_cursor.move_to(self.field_cursor.x, 0)
        self.octave_var = min(max(3 - ((y - self.y - 12) // 24), 0), 3)

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        self.help_message_var = "OCTAVE:PAGEUP/PAGEDOWN"

    def __on_draw(self):
        x = self.x + 1
        y = self.y + 1 + (3 - self.octave_var) * 24

        pyxel.rect(self.x, self.y, self.width, self.height, OCTAVE_BAR_BACKGROUND_COLOR)
        pyxel.rect(x, y, 2, 47, OCTAVE_BAR_COLOR)
