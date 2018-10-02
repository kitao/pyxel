from .widget import Widget


class Frame(Widget):
    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        self._draw_frame()
