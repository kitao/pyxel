import math
import numpy as np
import OpenGL.GL as gl
from .glwrapper import GLShader, GLAttribute, GLTexture
from .shaders import (
    DRAWING_VERTEX_SHADER,
    DRAWING_FRAGMENT_SHADER,
    DRAWING_ATTRIBUTE_INFO,
    SCALING_VERTEX_SHADER,
    SCALING_FRAGMENT_SHADER,
    SCALING_ATTRIBUTE_INFO,
)
from .font import (
    MIN_FONT_CODE,
    MAX_FONT_CODE,
    FONT_WIDTH,
    FONT_HEIGHT,
    FONT_IMAGE_ROW_COUNT,
    create_font_image,
)

BANK_COUNT = 5
MAX_DRAW_COUNT = 10000

TYPE_PIX = 0
TYPE_LINE = 1
TYPE_RECT = 2
TYPE_RECTB = 3
TYPE_CIRC = 4
TYPE_CIRCB = 5
TYPE_BLT = 6
TYPE_TEXT = 7

MODE_TYPE_INDEX = DRAWING_ATTRIBUTE_INFO[0][1]
MODE_COL_INDEX = MODE_TYPE_INDEX + 1
MODE_BANK_INDEX = MODE_TYPE_INDEX + 2

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


class Renderer:
    def __init__(self, width, height):
        self._width = width
        self._height = height
        self._cur_draw_count = 0

        self._bank_list = [None] * BANK_COUNT
        self._bank_list[-1] = create_font_image()

        self.clip_pal_data = np.ndarray(8, np.float32)
        self.clip()
        self.pal()

        self._draw_shader = GLShader(DRAWING_VERTEX_SHADER,
                                     DRAWING_FRAGMENT_SHADER)
        self._draw_att = GLAttribute(
            DRAWING_ATTRIBUTE_INFO, MAX_DRAW_COUNT, dynamic=True)

        self._scale_shader = GLShader(SCALING_VERTEX_SHADER,
                                      SCALING_FRAGMENT_SHADER)
        self._scale_tex = GLTexture(width, height, 3, nearest=True)

        self._normal_scale_att = GLAttribute(SCALING_ATTRIBUTE_INFO, 4)
        data = self._normal_scale_att.data
        data[0, :] = [-1, 1, 0, 1]
        data[1, :] = [-1, -1, 0, 0]
        data[2, :] = [1, 1, 1, 1]
        data[3, :] = [1, -1, 1, 0]

        self._inverse_scale_att = GLAttribute(SCALING_ATTRIBUTE_INFO, 4)
        data = self._inverse_scale_att.data
        data[0, :] = [-1, 1, 0, 0]
        data[1, :] = [-1, -1, 0, 1]
        data[2, :] = [1, 1, 1, 0]
        data[3, :] = [1, -1, 1, 1]

    def render(self, left, bottom, width, height, palette, clear_color):
        if self._cur_draw_count > 0:
            # restore previous frame
            gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
            gl.glDisable(gl.GL_POINT_SPRITE)
            gl.glViewport(0, 0, int(self._width), int(self._height))

            self._scale_shader.begin(self._normal_scale_att, [self._scale_tex])
            self._scale_shader.set_uniform('u_texture', '1i', 0)
            gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
            self._scale_shader.end()

            # render drawing commands
            gl.glEnable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
            gl.glEnable(gl.GL_POINT_SPRITE)

            draw_tex_list = [(image and image._tex or None)
                             for image in self._bank_list]
            self._draw_att.update(self._cur_draw_count)
            self._draw_shader.begin(self._draw_att, draw_tex_list)
            self._draw_shader.set_uniform('u_framebuffer_size', '2f',
                                          self._width, self._height)

            for i, v in enumerate(palette):
                name = 'u_palette[{}]'.format(i)
                r, g, b = self._int_to_rgb(v)
                self._draw_shader.set_uniform(name, '3i', r, g, b)

            for i, v in enumerate(draw_tex_list):
                if v:
                    name = 'u_texture[{}]'.format(i)
                    self._draw_shader.set_uniform(name, '1i', i)

                    name = 'u_texture_size[{}]'.format(i)
                    self._draw_shader.set_uniform(name, '2f', v.width,
                                                  v.height)

            gl.glDrawArrays(gl.GL_POINTS, 0, self._cur_draw_count)
            self._draw_shader.end()
            self._scale_tex.copy_screen(0, 0, 0, 0, self._width, self._height)
            capture_image = gl.glReadPixels(0, 0, self._width, self._height,
                                            gl.GL_RGB, gl.GL_UNSIGNED_BYTE)

            self._cur_draw_count = 0

        # clear screen
        r, g, b = self._int_to_rgb(clear_color)
        gl.glClearColor(r / 255, g / 255, b / 255, 1)
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)

        # scaling
        gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
        gl.glDisable(gl.GL_POINT_SPRITE)
        gl.glViewport(int(left), int(bottom), int(width), int(height))

        self._scale_shader.begin(self._inverse_scale_att, [self._scale_tex])
        self._scale_shader.set_uniform('u_texture', '1i', 0)
        gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
        self._scale_shader.end()

        return capture_image

    @staticmethod
    def _largest_power_of_two(n):
        return 2**math.ceil(math.log(n, 2))

    @staticmethod
    def _int_to_rgb(color):
        return ((color >> 16) & 0xff, (color >> 8) & 0xff, color & 0xff)

    def _next_draw_data(self):
        data = self._draw_att.data[self._cur_draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self.clip_pal_data

        if self._cur_draw_count < MAX_DRAW_COUNT - 1:
            self._cur_draw_count += 1

        return data

    def _copy_draw_data(self):
        data = self._draw_att.data[self._cur_draw_count]
        data[:] = self._draw_att.data[self._cur_draw_count - 1]

        if self._cur_draw_count < MAX_DRAW_COUNT - 1:
            self._cur_draw_count += 1

        return data

    def bank(self, index, image):
        self._bank_list[index] = image

    def clip(self, *args):
        if len(args) == 0:
            self.clip_pal_data[0] = 0
            self.clip_pal_data[1] = 0
            self.clip_pal_data[2] = self._width
            self.clip_pal_data[3] = self._height
        elif len(args) == 4:
            x1, y1, x2, y2 = args
            self.clip_pal_data[0] = x1
            self.clip_pal_data[1] = y1
            self.clip_pal_data[2] = x2
            self.clip_pal_data[3] = y2
        else:
            raise ValueError('invalid clip argument')

    def pal(self, *args):
        if len(args) == 0:
            self.clip_pal_data[4] = 0x3210
            self.clip_pal_data[5] = 0x7654
            self.clip_pal_data[6] = 0xba98
            self.clip_pal_data[7] = 0xfedc
        elif len(args) == 2:
            col1, col2 = args
            index = col1 // 4 + 4
            shift = (col1 % 4) * 4
            value = col2 << shift
            mask = 0xffff ^ (0xf << shift)
            base = int(self.clip_pal_data[index])
            self.clip_pal_data[index] = base & mask | value
        else:
            raise ValueError('invalid pal argument')

    def cls(self, col):
        self._cur_draw_count = 0

        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_RECT
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
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_PIX
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

    def line(self, x1, y1, x2, y2, col):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_LINE
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x1
        data[POS_Y1_INDEX] = y1
        data[POS_X2_INDEX] = x2
        data[POS_Y2_INDEX] = y2

    def rect(self, x1, y1, x2, y2, col):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_RECT
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x1
        data[POS_Y1_INDEX] = y1
        data[POS_X2_INDEX] = x2
        data[POS_Y2_INDEX] = y2

    def rectb(self, x1, y1, x2, y2, col):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_RECTB
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x1
        data[POS_Y1_INDEX] = y1
        data[POS_X2_INDEX] = x2
        data[POS_Y2_INDEX] = y2

    def circ(self, x, y, r, col):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_CIRC
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = r

    def circb(self, x, y, r, col):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_CIRCB
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = r

    def blt(self, x, y, bank, sx, sy, w, h, colkey=None):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_BLT
        data[MODE_COL_INDEX] = colkey or -1
        data[MODE_BANK_INDEX] = bank

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y
        data[POS_X2_INDEX] = sx
        data[POS_Y2_INDEX] = sy

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

    def text(self, x, y, str_, col):
        left = x
        first = True

        for ch in str_:
            code = ord(ch)

            if code == 10:  # new line
                first = True
                x = left
                y += FONT_HEIGHT
                continue

            if code == 32:  # space
                x += FONT_WIDTH
                continue

            if code < MIN_FONT_CODE or code > MAX_FONT_CODE:
                continue

            code -= MIN_FONT_CODE

            if first:
                data = self._next_draw_data()

                data[MODE_TYPE_INDEX] = TYPE_TEXT
                data[MODE_COL_INDEX] = col
                data[MODE_BANK_INDEX] = BANK_COUNT - 1

                data[POS_Y1_INDEX] = y

                data[SIZE_W_INDEX] = FONT_WIDTH
                data[SIZE_H_INDEX] = FONT_HEIGHT

                first = False
            else:
                data = self._copy_draw_data()

            data[POS_X1_INDEX] = x
            data[POS_X2_INDEX] = (code % FONT_IMAGE_ROW_COUNT) * FONT_WIDTH
            data[POS_Y2_INDEX] = (code // FONT_IMAGE_ROW_COUNT) * FONT_HEIGHT

            x += FONT_WIDTH
