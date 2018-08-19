import pyxel

from .radio_button import RadioButton
from .screen import Screen
from .scroll_bar import ScrollBar
from .widget import Widget


class EditWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 12, 17, 128, 128)

        self.edit_x = 0
        self.edit_y = 0

        self.offset_x = 0
        self.offset_y = 0

        self.h_scroll_bar = ScrollBar(self, 11, 145, 130, 7, 2, 32)
        self.h_scroll_bar.add_event_handler('change', self.on_change_x)

        self.v_scroll_bar = ScrollBar(self, 140, 16, 7, 130, 2, 32)
        self.v_scroll_bar.add_event_handler('change', self.on_change_y)

        self.add_event_handler('click', self.on_click)
        self.add_event_handler('drag', self.on_drag)
        self.add_event_handler('draw', self.on_draw)

    def on_change_x(self, value):
        self.edit_x = value * 8

    def on_change_y(self, value):
        self.edit_y = value * 8

    def on_click(self, key, mx, my):
        if key == pyxel.KEY_RIGHT_BUTTON:
            img = self.parent.image_button.value
            x = self.edit_x + mx // 8
            y = self.edit_y + my // 8
            self.parent.color_button.value = pyxel.image(img).data[y, x]

    def on_drag(self, key, mx, my, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            img = self.parent.image_button.value
            x = self.edit_x + mx // 8
            y = self.edit_y + my // 8
            col = self.parent.color_button.value
            pyxel.image(img).data[y, x] = col

        elif key == pyxel.KEY_RIGHT_BUTTON:
            self.offset_x -= dx
            self.offset_y -= dy

            if abs(self.offset_x) >= 8:
                offset = (self.offset_x // 8) * 8
                self.edit_x += offset
                self.offset_x -= offset

            if abs(self.offset_y) >= 8:
                offset = (self.offset_y // 8) * 8
                self.edit_y += offset
                self.offset_y -= offset

            self.edit_x = min(max(self.edit_x, 0), 240)
            self.edit_y = min(max(self.edit_y, 0), 240)

            self.h_scroll_bar.value = self.edit_x // 8
            self.v_scroll_bar.value = self.edit_y // 8

    def on_draw(self):
        for i in range(16):
            y = self.y + i * 8
            for j in range(16):
                x = self.x + j * 8
                data = pyxel.image(self.parent.image_button.value).data
                col = data[self.edit_y + i, self.edit_x + j]
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

        self.h_scroll_bar = ScrollBar(self, 157, 145, 66, 7, 8, 32)
        self.h_scroll_bar.add_event_handler('change', self.on_change_x)

        self.v_scroll_bar = ScrollBar(self, 222, 16, 7, 130, 16, 32)
        self.v_scroll_bar.add_event_handler('change', self.on_change_y)

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('drag', self.on_drag)
        self.add_event_handler('draw', self.on_draw)

    def on_change_x(self, value):
        self.preview_x = value * 8

    def on_change_y(self, value):
        self.preview_y = value * 8

    def on_press(self, key, mx, my):
        if key == pyxel.KEY_LEFT_BUTTON:
            self.parent.edit_window.edit_x = self.preview_x + (
                (mx - 4) // 8) * 8
            self.parent.edit_window.edit_y = self.preview_y + (
                (my - 4) // 8) * 8

            self.parent.edit_window.edit_x = min(
                max(self.parent.edit_window.edit_x, 0), 240)
            self.parent.edit_window.edit_y = min(
                max(self.parent.edit_window.edit_y, 0), 240)

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
        img = self.parent.image_button.value
        pyxel.blt(self.x, self.y, img, self.preview_x, self.preview_y, 64, 128)

        pyxel.clip(self.x - 1, self.y - 1, self.x + self.width,
                   self.y + self.height)
        x = self.x + self.parent.edit_window.edit_x - self.preview_x
        y = self.y + self.parent.edit_window.edit_y - self.preview_y
        pyxel.rectb(x - 1, y - 1, x + 16, y + 16, 7)
        pyxel.clip()


class ImageEditor(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'image_editor.png')

        def on_draw():
            widget = self.color_button
            x = widget.x + (widget.value % 8) * 8
            y = widget.y + (widget.value // 8) * 8
            col = 7 if widget.value < 6 else 0
            pyxel.text(x + 2, y + 1, '+', col)

        self.color_button = RadioButton(self, 12, 157, 8, 2, 8)
        self.color_button.remove_event_handler('draw',
                                               self.color_button.on_draw)
        self.color_button.add_event_handler('draw', on_draw)
        self.color_button.value = 7

        self.tool_button = RadioButton(self, 81, 161, 7, 1, 9)
        self.image_button = RadioButton(self, 191, 161, 3, 1, 10)
        self.edit_window = EditWindow(self)
        self.preview_window = PreviewWindow(self)
