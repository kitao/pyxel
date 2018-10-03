import pyxel

from .editor import Editor


class SoundEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent, "sound_editor.png")

        self.add_event_handler("draw", self.__on_draw)
        self.add_event_handler("draw", self.draw_not_implemented_message)

    def __on_draw(self):
        self.draw_frame(11, 16, 228, 172)
        pyxel.text(23, 18, "SOUND", 6)
        pyxel.text(83, 18, "SPEED", 6)
