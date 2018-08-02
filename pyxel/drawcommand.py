import numpy as np

from .constants import (DRAW_TYPE_BLT, DRAW_TYPE_CIRC, DRAW_TYPE_CIRCB,
                        DRAW_TYPE_LINE, DRAW_TYPE_PIX, DRAW_TYPE_RECT,
                        DRAW_TYPE_RECTB, DRAW_TYPE_TEXT, FONT_HEIGHT,
                        FONT_MAX_CODE, FONT_MIN_CODE, FONT_WIDTH)
from .shaders import DRAWING_ATTRIBUTE_INFO

MODE_TYPE_INDEX = DRAWING_ATTRIBUTE_INFO[0][1]
MODE_COL_INDEX = MODE_TYPE_INDEX + 1
MODE_IMAGE_INDEX = MODE_TYPE_INDEX + 2

POS_X1_INDEX = DRAWING_ATTRIBUTE_INFO[1][1]
POS_Y1_INDEX = POS_X1_INDEX + 1
POS_X2_INDEX = POS_X1_INDEX + 2
POS_Y2_INDEX = POS_X1_INDEX + 3

SIZE_W_INDEX = DRAWING_ATTRIBUTE_INFO[2][1]
SIZE_H_INDEX = SIZE_W_INDEX + 1

CLIP_X1_INDEX = DRAWING_ATTRIBUTE_INFO[3][1]
CLIP_Y1_INDEX = CLIP_X1_INDEX + 1
CLIP_X2_INDEX = CLIP_X1_INDEX + 2
CLIP_Y2_INDEX = CLIP_X1_INDEX + 3

PAL_A_INDEX = DRAWING_ATTRIBUTE_INFO[4][1]
PAL_B_INDEX = PAL_A_INDEX + 1
PAL_C_INDEX = PAL_A_INDEX + 2
PAL_D_INDEX = PAL_A_INDEX + 3

CLIP_PAL_INDEX = CLIP_X1_INDEX
CLIP_PAL_COUNT = 8


class DrawCommand:
    def __init__(self, width, height, draw_att_data):
        self._width = width
        self._height = height
        self._draw_att_data = draw_att_data
        self._clip_pal_data = np.ndarray(8, np.float32)
        self._max_draw_count = len(draw_att_data)
        self.draw_count = 0

        self.clip()
        self.pal()

    def clip(self, x1=None, y1=None, x2=None, y2=None):
        if x1 is None:
            self._clip_pal_data[0] = 0
            self._clip_pal_data[1] = 0
            self._clip_pal_data[2] = self._width - 1
            self._clip_pal_data[3] = self._height - 1
        else:
            self._clip_pal_data[0] = x1
            self._clip_pal_data[1] = y1
            self._clip_pal_data[2] = x2
            self._clip_pal_data[3] = y2

    def pal(self, col1=None, col2=None):
        if col1 is None:
            self._clip_pal_data[4] = 0x3210
            self._clip_pal_data[5] = 0x7654
            self._clip_pal_data[6] = 0xba98
            self._clip_pal_data[7] = 0xfedc
        else:
            index = col1 // 4 + 4
            shift = (col1 % 4) * 4
            value = col2 << shift
            mask = 0xffff ^ (0xf << shift)
            base = int(self._clip_pal_data[index])
            self._clip_pal_data[index] = base & mask | value

    def cls(self, col):
        self.draw_count = 0

        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_RECT
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = 0
        data[POS_Y1_INDEX] = 0
        data[POS_X2_INDEX] = self._width - 1
        data[POS_Y2_INDEX] = self._height - 1

        data[CLIP_X1_INDEX] = 0
        data[CLIP_Y1_INDEX] = 0
        data[CLIP_X2_INDEX] = self._width - 1
        data[CLIP_Y2_INDEX] = self._height - 1

    def pix(self, x, y, col):
        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_PIX
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

    def line(self, x1, y1, x2, y2, col):
        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_LINE
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x1
        data[POS_Y1_INDEX] = y1
        data[POS_X2_INDEX] = x2
        data[POS_Y2_INDEX] = y2

    def rect(self, x1, y1, x2, y2, col):
        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_RECT
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x1
        data[POS_Y1_INDEX] = y1
        data[POS_X2_INDEX] = x2
        data[POS_Y2_INDEX] = y2

    def rectb(self, x1, y1, x2, y2, col):
        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_RECTB
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x1
        data[POS_Y1_INDEX] = y1
        data[POS_X2_INDEX] = x2
        data[POS_Y2_INDEX] = y2

    def circ(self, x, y, r, col):
        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_CIRC
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = r

    def circb(self, x, y, r, col):
        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_CIRCB
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = r

    def blt(self, x, y, img, sx, sy, w, h, colkey=None):
        if self.draw_count >= self._max_draw_count:
            return
        data = self._draw_att_data[self.draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self._clip_pal_data
        self.draw_count += 1

        data[MODE_TYPE_INDEX] = DRAW_TYPE_BLT
        data[MODE_COL_INDEX] = -1 if colkey is None else colkey
        data[MODE_IMAGE_INDEX] = img

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y
        data[POS_X2_INDEX] = sx
        data[POS_Y2_INDEX] = sy

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

    def text(self, x, y, s, col):
        left = x
        first = True

        for ch in s:
            code = ord(ch)

            if code == 10:  # new line
                first = True
                x = left
                y += FONT_HEIGHT
                continue

            if code == 32:  # space
                x += FONT_WIDTH
                continue

            if code < FONT_MIN_CODE or code > FONT_MAX_CODE:
                continue

            code -= FONT_MIN_CODE

            if first:
                if self.draw_count >= self._max_draw_count:
                    return
                data = self._draw_att_data[self.draw_count]
                data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
                     CLIP_PAL_COUNT] = self._clip_pal_data
                self.draw_count += 1

                data[MODE_TYPE_INDEX] = DRAW_TYPE_TEXT
                data[MODE_COL_INDEX] = col

                data[POS_Y1_INDEX] = y

                first = False
            else:
                if self.draw_count >= self._max_draw_count:
                    return
                data = self._draw_att_data[self.draw_count]
                data[:] = self._draw_att_data[self.draw_count - 1]
                self.draw_count += 1

            data[POS_X1_INDEX] = x
            data[POS_X2_INDEX] = code

            x += FONT_WIDTH
