import pyxel

from .editor import Editor


class MusicEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent, "music_editor.png")

        self.add_event_handler("draw", self.__on_draw)
        self.add_event_handler("draw", self.draw_not_implemented_message)

    def __on_draw(self):
        self.draw_frame(11, 16, 228, 24)
        pyxel.text(23, 18, "MUSIC", 6)

        for i in range(4):
            x = i * 59 + 11
            self.draw_frame(x, 30, x + 40, 172)
            pyxel.text(x + 15, 32, "CH{}".format(i), 6)
