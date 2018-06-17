import OpenGL.GL as gl
import numpy as np
from .glwrapper import GLShader, GLAttribute, GLTexture
from .shaders import (DRAWING_VERTEX_SHADER, DRAWING_FRAGMENT_SHADER,
                      DRAWING_ATTRIBUTE_INFO, SCALING_VERTEX_SHADER,
                      SCALING_FRAGMENT_SHADER, SCALING_ATTRIBUTE_INFO)
from .font import (MIN_FONT_CODE, MAX_FONT_CODE, FONT_WIDTH, FONT_HEIGHT,
                   FONT_IMAGE_ROW_COUNT, create_font_image)

BANK_COUNT = 8
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

CLIP_X_INDEX = DRAWING_ATTRIBUTE_INFO[3][1]
CLIP_Y_INDEX = CLIP_X_INDEX + 1
CLIP_W_INDEX = CLIP_X_INDEX + 2
CLIP_H_INDEX = CLIP_X_INDEX + 3

PAL_A_INDEX = DRAWING_ATTRIBUTE_INFO[4][1]
PAL_B_INDEX = PAL_A_INDEX + 1
PAL_C_INDEX = PAL_A_INDEX + 2
PAL_D_INDEX = PAL_A_INDEX + 3

CLIP_PAL_INDEX = CLIP_X_INDEX
CLIP_PAL_COUNT = 8


def int_to_rgb(color):
    return ((color >> 16) & 0xff, (color >> 8) & 0xff, color & 0xff)


class Renderer:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.cur_draw_count = 0

        self.bank_list = [None] * BANK_COUNT
        self.bank_list[-1] = create_font_image()

        self.clip_pal_data = np.ndarray(8, np.float32)
        self.clip()
        self.pal()

        self.draw_shader = GLShader(DRAWING_VERTEX_SHADER,
                                    DRAWING_FRAGMENT_SHADER)
        self.draw_att = GLAttribute(
            DRAWING_ATTRIBUTE_INFO, MAX_DRAW_COUNT, dynamic=True)

        self.scale_shader = GLShader(SCALING_VERTEX_SHADER,
                                     SCALING_FRAGMENT_SHADER)
        self.scale_tex = GLTexture(width, height, 3, nearest=True)

        self.normal_scale_att = GLAttribute(SCALING_ATTRIBUTE_INFO, 4)
        data = self.normal_scale_att.data
        data[0, :] = [-1, 1, 0, 1]
        data[1, :] = [-1, -1, 0, 0]
        data[2, :] = [1, 1, 1, 1]
        data[3, :] = [1, -1, 1, 0]

        self.inverse_scale_att = GLAttribute(SCALING_ATTRIBUTE_INFO, 4)
        data = self.inverse_scale_att.data
        data[0, :] = [-1, 1, 0, 0]
        data[1, :] = [-1, -1, 0, 1]
        data[2, :] = [1, 1, 1, 0]
        data[3, :] = [1, -1, 1, 1]

    def reset_drawing_command(self):
        self.cur_draw_count = 0

    def render(self, left, bottom, width, height, palette, clear_color):
        if self.cur_draw_count > 0:
            # restore previous frame
            gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
            gl.glDisable(gl.GL_POINT_SPRITE)
            gl.glViewport(0, 0, self.width, self.height)

            self.scale_shader.begin(self.normal_scale_att, [self.scale_tex])
            self.scale_shader.set_uniform('u_texture', '1i', 0)
            gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
            self.scale_shader.end()

            # render drawing commands
            gl.glEnable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
            gl.glEnable(gl.GL_POINT_SPRITE)

            draw_tex_list = [
                image._tex if image else None for image in self.bank_list
            ]
            self.draw_att.refresh(self.cur_draw_count)
            self.draw_shader.begin(self.draw_att, draw_tex_list)
            self.draw_shader.set_uniform('u_framebuffer_size', '2f',
                                         self.width, self.height)

            for i, v in enumerate(palette):
                name = 'u_palette[{}]'.format(i)
                r, g, b = int_to_rgb(v)
                self.draw_shader.set_uniform(name, '3i', r, g, b)

            for i, v in enumerate(draw_tex_list):
                if v:
                    name = 'u_texture[{}]'.format(i)
                    self.draw_shader.set_uniform(name, '1i', i)

                    name = 'u_texture_size[{}]'.format(i)
                    self.draw_shader.set_uniform(name, '2f', v.width, v.height)

            gl.glDrawArrays(gl.GL_POINTS, 0, self.cur_draw_count)
            self.draw_shader.end()
            self.scale_tex.copy_screen(0, 0, 0, 0, self.width, self.height)

            self.cur_draw_count = 0

        # clear screen
        r, g, b = int_to_rgb(clear_color)
        gl.glClearColor(r / 255, g / 255, b / 255, 1)
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)

        # scaling
        gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
        gl.glDisable(gl.GL_POINT_SPRITE)
        gl.glViewport(left, bottom, width, height)

        self.scale_shader.begin(self.inverse_scale_att, [self.scale_tex])
        self.scale_shader.set_uniform('u_texture', '1i', 0)
        gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
        self.scale_shader.end()

    def _next_draw_data(self):
        data = self.draw_att.data[self.cur_draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self.clip_pal_data

        if self.cur_draw_count < MAX_DRAW_COUNT - 1:
            self.cur_draw_count += 1

        return data

    def _copy_draw_data(self):
        data = self.draw_att.data[self.cur_draw_count]
        data[:] = self.draw_att.data[self.cur_draw_count - 1]

        if self.cur_draw_count < MAX_DRAW_COUNT - 1:
            self.cur_draw_count += 1

        return data

    def bank(self, index, image):
        self.bank_list[index] = image

    def clip(self, *args):
        if len(args) == 4:
            x, y, z, w = args
            self.clip_pal_data[0] = x
            self.clip_pal_data[1] = y
            self.clip_pal_data[2] = z
            self.clip_pal_data[3] = w
        else:
            self.clip_pal_data[0] = 0
            self.clip_pal_data[1] = 0
            self.clip_pal_data[2] = self.width
            self.clip_pal_data[3] = self.height

    def pal(self, *args):
        if len(args) == 2:
            c1, c2 = args
            index = c1 // 4 + 4
            shift = (c1 % 4) * 4
            value = c2 << shift
            mask = 0xffff ^ (0xf << shift)
            base = int(self.clip_pal_data[index])
            self.clip_pal_data[index] = base & mask | value
        else:
            self.clip_pal_data[4] = 0x3210
            self.clip_pal_data[5] = 0x7654
            self.clip_pal_data[6] = 0xba98
            self.clip_pal_data[7] = 0xfedc

    def cls(self, col):
        self.cur_draw_count = 0

        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_RECT
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = 0
        data[POS_Y1_INDEX] = 0

        data[SIZE_W_INDEX] = self.width
        data[SIZE_H_INDEX] = self.height

        data[CLIP_X_INDEX] = 0
        data[CLIP_Y_INDEX] = 0
        data[CLIP_W_INDEX] = self.width
        data[CLIP_H_INDEX] = self.height

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

    def rect(self, x, y, w, h, col):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_RECT
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

    def rectb(self, x, y, w, h, col):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_RECTB
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

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

    def blt(self, x, y, bank, sx, sy, w, h, colkey=-1):
        data = self._next_draw_data()

        data[MODE_TYPE_INDEX] = TYPE_BLT
        data[MODE_COL_INDEX] = colkey
        data[MODE_BANK_INDEX] = bank

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y
        data[POS_X2_INDEX] = sx
        data[POS_Y2_INDEX] = sy

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

    def text(self, x, y, s, col):
        for i, c in enumerate(s):
            code = min(max(ord(c), MIN_FONT_CODE),
                       MAX_FONT_CODE) - MIN_FONT_CODE

            if i == 0:
                data = self._next_draw_data()

                data[MODE_TYPE_INDEX] = TYPE_TEXT
                data[MODE_COL_INDEX] = col
                data[MODE_BANK_INDEX] = BANK_COUNT - 1

                data[POS_Y1_INDEX] = y

                data[SIZE_W_INDEX] = FONT_WIDTH
                data[SIZE_H_INDEX] = FONT_HEIGHT
            else:
                data = self._copy_draw_data()

            data[POS_X1_INDEX] = x
            data[POS_X2_INDEX] = (code % FONT_IMAGE_ROW_COUNT) * FONT_WIDTH
            data[POS_Y2_INDEX] = (code // FONT_IMAGE_ROW_COUNT) * FONT_HEIGHT

            x += FONT_WIDTH
