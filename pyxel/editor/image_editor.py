import pyxel

from pyxel.editor.screen import Screen
from .radio_button import RadioButton

COLOR_BUTTON_X = 12
COLOR_BUTTON_Y1 = 157
COLOR_BUTTON_Y2 = 165
COLOR_BUTTON_INTERVAL = 8
COLOR_BUTTON_COUNT = 8

TOOL_BUTTON_X = 81
TOOL_BUTTON_Y = 161
TOOL_BUTTON_INTERVAL = 9
TOOL_BUTTON_COUNT = 7

IMAGE_BUTTON_X = 191
IMAGE_BUTTON_Y = 161
IMAGE_BUTTON_INTERVAL = 10
IMAGE_BUTTON_COUNT = 3

CANVAS_X = 12
CANVAS_Y = 17

PREVIEW_X = 158
PREVIEW_Y = 17


class ImageEditor(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'image_editor.png')

        self._color = 0
        self._tool = 0
        self._img = 0

    def on_update(self):
        super().on_update()

        #col = self.check_button(COLOR_BUTTON_X, COLOR_BUTTON_Y1,
        #                        COLOR_BUTTON_INTERVAL, COLOR_BUTTON_COUNT)
        #if col is not None:
        #    self._color = col
        #else:
        #    col = self.check_button(COLOR_BUTTON_X, COLOR_BUTTON_Y2,
        #                            COLOR_BUTTON_INTERVAL, COLOR_BUTTON_COUNT)
        #    if col is not None:
        #        self._color = col + COLOR_BUTTON_COUNT

        #tool = self.check_button(TOOL_BUTTON_X, TOOL_BUTTON_Y,
        #                         TOOL_BUTTON_INTERVAL, TOOL_BUTTON_INTERVAL)
        #if tool is not None:
        #    self._tool = tool

        #img = self.check_button(IMAGE_BUTTON_X, IMAGE_BUTTON_Y,
        #                        IMAGE_BUTTON_INTERVAL, IMAGE_BUTTON_COUNT)
        #if img is not None:
        #    self._img = img

        #if pyxel.btn(pyxel.KEY_LEFT_BUTTON):
        #    mx = pyxel.mouse_x
        #    my = pyxel.mouse_y

        #    if (mx >= CANVAS_X and mx < CANVAS_X + 8 * 16 and my >= CANVAS_Y
        #            and my < CANVAS_Y + 8 * 16):
        #        x = (mx - CANVAS_X) // 8
        #        y = (my - CANVAS_Y) // 8
        #        pyxel.image(self._img).data[y, x] = self._color

    def on_draw(self):
        super().on_draw()
        #for i in range(16):
        #    y = CANVAS_Y + i * 8
        #    for j in range(16):
        #        x = CANVAS_X + j * 8
        #        col = pyxel.image(self._img).data[i, j]
        #        pyxel.rect(x, y, x + 7, y + 7, col)

        #pyxel.blt(PREVIEW_X, PREVIEW_Y, self._img, 0, 0, 64, 128)

        #super().draw()

        #if self._color < COLOR_BUTTON_COUNT:
        #    x = COLOR_BUTTON_X + self._color * COLOR_BUTTON_INTERVAL
        #    y = COLOR_BUTTON_Y1
        #else:
        #    x = COLOR_BUTTON_X + (
        #        self._color - COLOR_BUTTON_COUNT) * COLOR_BUTTON_INTERVAL
        #    y = COLOR_BUTTON_Y2
        #col = 7 if self._color < 6 else 0
        #pyxel.text(x + 2, y + 1, '+', col)

        #self.draw_button(TOOL_BUTTON_X, TOOL_BUTTON_Y, TOOL_BUTTON_INTERVAL,
        #                 self._tool)

        #self.draw_button(IMAGE_BUTTON_X, IMAGE_BUTTON_Y, IMAGE_BUTTON_INTERVAL,
        #                 self._img)
