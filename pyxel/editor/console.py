from .screen import Screen


class Console(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'console.png')

        self.add_event_handler('draw', self.draw_not_implemented_message)
