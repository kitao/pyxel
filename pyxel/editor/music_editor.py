from .screen import Screen


class MusicEditor(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'music_editor.png')

        self.add_event_handler('draw', self.draw_not_implemented_message)
