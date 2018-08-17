import pyxel

from .radio_button import RadioButton
from .screen import Screen
from .widget import Widget


class EditWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 12, 17, 128, 128)

        self.offset_x = 0
        self.offset_y = 0

        self.add_event_handler('click', self.on_click)
        self.add_event_handler('drag', self.on_drag)
        self.add_event_handler('draw', self.on_draw)

    def on_click(self, key, mx, my):
        if key == pyxel.KEY_RIGHT_BUTTON:
            img = self.parent._image_button.index
            x = self.parent.edit_x + mx // 8
            y = self.parent.edit_y + my // 8
            self.parent._color_button.index = pyxel.image(img).data[y, x]

    def on_drag(self, key, mx, my, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            img = self.parent._image_button.index
            x = self.parent.edit_x + mx // 8
            y = self.parent.edit_y + my // 8
            col = self.parent._color_button.index
            pyxel.image(img).data[y, x] = col

        elif key == pyxel.KEY_RIGHT_BUTTON:
            self.offset_x -= dx
            self.offset_y -= dy

            if abs(self.offset_x) >= 8:
                offset = (self.offset_x // 8) * 8
                self.parent.edit_x += offset
                self.offset_x -= offset

            if abs(self.offset_y) >= 8:
                offset = (self.offset_y // 8) * 8
                self.parent.edit_y += offset
                self.offset_y -= offset

            self.parent.edit_x = min(max(self.parent.edit_x, 0), 240)
            self.parent.edit_y = min(max(self.parent.edit_y, 0), 240)

    def on_draw(self):
        for i in range(16):
            y = self.y + i * 8
            for j in range(16):
                x = self.x + j * 8
                data = pyxel.image(self.parent._image_button.index).data
                col = data[self.parent.edit_y + i, self.parent.edit_x + j]
                pyxel.rect(x, y, x + 7, y + 7, col)

        pyxel.line(self.x, self.y + 63, self.x + 127, self.y + 63, 1)
        pyxel.line(self.x + 63, self.y, self.x + 63, self.y + 127, 1)


class PreviewWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 158, 17, 64, 128)

        self.preview_x = 0
        self.preview_y = 0

        self.offset_x = 0
        self.offset_y = 0

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('drag', self.on_drag)
        self.add_event_handler('draw', self.on_draw)

    def on_press(self, key, mx, my):
        if key == pyxel.KEY_LEFT_BUTTON:
            self.parent.edit_x = self.preview_x + ((mx - 4) // 8) * 8
            self.parent.edit_y = self.preview_y + ((my - 4) // 8) * 8

            self.parent.edit_x = min(max(self.parent.edit_x, 0), 240)
            self.parent.edit_y = min(max(self.parent.edit_y, 0), 240)

        if key == pyxel.KEY_RIGHT_BUTTON:
            self.offset_x = 0
            self.offset_y = 0

    def on_drag(self, key, mx, my, dx, dy):
        if key == pyxel.KEY_RIGHT_BUTTON:
            self.offset_x -= dx * 2
            self.offset_y -= dy * 2

            if abs(self.offset_x) >= 8:
                offset = (self.offset_x // 8) * 8
                self.preview_x += offset
                self.offset_x -= offset

            if abs(self.offset_y) >= 8:
                offset = (self.offset_y // 8) * 8
                self.preview_y += offset
                self.offset_y -= offset

            self.preview_x = min(max(self.preview_x, 0), 192)
            self.preview_y = min(max(self.preview_y, 0), 128)

    def on_draw(self):
        img = self.parent._image_button.index
        pyxel.blt(self.x, self.y, img, self.preview_x, self.preview_y, 64, 128)

        pyxel.clip(self.x - 1, self.y - 1, self.x + self.width,
                   self.y + self.height)
        x = self.x + self.parent.edit_x - self.preview_x
        y = self.y + self.parent.edit_y - self.preview_y
        pyxel.rectb(x - 1, y - 1, x + 16, y + 16, 7)
        pyxel.clip()


class ImageEditor(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'image_editor.png')

        self.edit_x = 0
        self.edit_y = 0

        def on_draw():
            widget = self._color_button
            x = widget.x + (widget.index % 8) * 8
            y = widget.y + (widget.index // 8) * 8
            col = 7 if widget.index < 6 else 0
            pyxel.text(x + 2, y + 1, '+', col)

        self._color_button = RadioButton(self, 12, 157, 8, 2, 8)
        self._color_button.remove_event_handler('draw',
                                                self._color_button.on_draw)
        self._color_button.add_event_handler('draw', on_draw)
        self._color_button.index = 7

        self._tool_button = RadioButton(self, 81, 161, 7, 1, 9)
        self._image_button = RadioButton(self, 191, 161, 3, 1, 10)
        self._canvas_window = EditWindow(self)
        self._preview_window = PreviewWindow(self)
