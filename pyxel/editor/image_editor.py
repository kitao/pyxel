import pyxel

from .radio_button import RadioButton
from .screen import Screen
from .widget import Widget


class CanvasWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 12, 17, 128, 128)

    def on_draw(self):
        for i in range(16):
            y = self.y + i * 8
            for j in range(16):
                x = self.x + j * 8
                col = pyxel.image(self.parent._image_button.value).data[i, j]
                pyxel.rect(x, y, x + 7, y + 7, col)

        pyxel.line(self.x, self.y + 63, self.x + 127, self.y + 63, 1)
        pyxel.line(self.x + 63, self.y, self.x + 63, self.y + 127, 1)

    def on_click(self, key, mx, my):
        if key == pyxel.KEY_RIGHT_BUTTON:
            img = self.parent._image_button.value
            x = mx // 8
            y = my // 8
            self.parent._color_button.value = pyxel.image(img).data[y, x]

    def on_drag(self, key, mx, my, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            img = self.parent._image_button.value
            x = mx // 8
            y = my // 8
            col = self.parent._color_button.value
            pyxel.image(img).data[y, x] = col


class PreviewWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 158, 17, 64, 128)

    def on_draw(self):
        img = self.parent._image_button.value
        pyxel.blt(self.x, self.y, img, 0, 0, 64, 128)


class ImageEditor(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'image_editor.png')

        self._color_button = RadioButton(self, 12, 157, 8, 2, 8)

        def on_draw():
            widget = self._color_button
            x = widget.x + (widget.value % 8) * 8
            y = widget.y + (widget.value // 8) * 8
            col = 7 if widget.value < 6 else 0
            pyxel.text(x + 2, y + 1, '+', col)

        self._color_button.on_draw = on_draw

        self._tool_button = RadioButton(self, 81, 161, 7, 1, 9)
        self._image_button = RadioButton(self, 191, 161, 3, 1, 10)

        self._canvas_window = CanvasWindow(self)
        self._preview_window = PreviewWindow(self)
