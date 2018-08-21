from .screen import Screen


class SoundEditor(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'sound_editor.png')

        self.add_event_handler('draw', self.draw_not_implemented_message)
