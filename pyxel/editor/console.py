from .screen import Screen


class Console(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'console.png')
