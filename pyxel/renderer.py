import OpenGL.GL as gl
from .glwrapper import GLShader, GLAttribute, GLTexture
from .shaders import (
    DRAWING_COMMAND_VERTEX_SHADER, DRAWING_COMMAND_FRAGMENT_SHADER,
    DRAWING_COMMAND_ATTRIBUTE_INFO, FRAMEBUFFER_VERTEX_SHADER,
    FRAMEBUFFER_FRAGMENT_SHADER, FRAMEBUFFER_ATTRIBUTE_INFO)
from .bank import Bank
import numpy as np

TYPE_PIX = 0
TYPE_LINE = 1
TYPE_RECT = 2
TYPE_RECTB = 3
TYPE_CIRC = 4
TYPE_CIRCB = 5
TYPE_BLT = 6

MODE_TYPE_INDEX = DRAWING_COMMAND_ATTRIBUTE_INFO[0][1]
MODE_COL_INDEX = MODE_TYPE_INDEX + 1
MODE_BANK_INDEX = MODE_TYPE_INDEX + 2

POS_X1_INDEX = DRAWING_COMMAND_ATTRIBUTE_INFO[1][1]
POS_Y1_INDEX = POS_X1_INDEX + 1
POS_X2_INDEX = POS_X1_INDEX + 2
POS_Y2_INDEX = POS_X1_INDEX + 3

SIZE_W_INDEX = DRAWING_COMMAND_ATTRIBUTE_INFO[2][1]
SIZE_H_INDEX = SIZE_W_INDEX + 1

CLIP_PAL_INDEX = DRAWING_COMMAND_ATTRIBUTE_INFO[3][1]
CLIP_PAL_COUNT = 8


def int_to_rgb(color):
    return ((color >> 16) & 0xff, (color >> 8) & 0xff, color & 0xff)


class Renderer:
    def __init__(self, width, height, bank_size, bank_count, draw_count):
        self.width = width
        self.height = height
        self.bank_list = [Bank(*bank_size) for _ in range(bank_count)]
        self.max_bank_count = bank_count
        self.max_draw_count = draw_count
        self.cur_draw_count = 0
        self.need_to_refresh = True

        self.clip_pal_data = np.ndarray(8, np.float32)
        self.clip()
        self.pal()

        self.dc_shader = GLShader(DRAWING_COMMAND_VERTEX_SHADER,
                                  DRAWING_COMMAND_FRAGMENT_SHADER)
        self.dc_att = GLAttribute(
            DRAWING_COMMAND_ATTRIBUTE_INFO,
            draw_count,
            integer=True,
            dynamic=True)
        self.dc_tex_list = [bank._tex for bank in self.bank_list]

        self.fb_shader = GLShader(FRAMEBUFFER_VERTEX_SHADER,
                                  FRAMEBUFFER_FRAGMENT_SHADER)
        self.fb_tex = GLTexture(width, height, 3, nearest=True)

        self.forward_att = GLAttribute(FRAMEBUFFER_ATTRIBUTE_INFO, 4)
        data = self.forward_att.data
        data[0, :] = [-1, 1, 0, 1]
        data[1, :] = [-1, -1, 0, 0]
        data[2, :] = [1, 1, 1, 1]
        data[3, :] = [1, -1, 1, 0]

        self.inverse_att = GLAttribute(FRAMEBUFFER_ATTRIBUTE_INFO, 4)
        data = self.inverse_att.data
        data[0, :] = [-1, 1, 0, 0]
        data[1, :] = [-1, -1, 0, 1]
        data[2, :] = [1, 1, 1, 0]
        data[3, :] = [1, -1, 1, 1]

    def begin(self):
        self.cur_draw_count = 0
        self.need_to_refresh = True

    def end(self):
        self.dc_att.refresh(self.cur_draw_count)

    def render(self, left, bottom, width, height, palette, bg_color):
        # clear screen
        r, g, b = int_to_rgb(bg_color)
        gl.glClearColor(r / 255, g / 255, b / 255, 1)
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)

        # drawing
        if self.need_to_refresh:
            # restore previous frame
            gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
            gl.glDisable(gl.GL_POINT_SPRITE)
            gl.glViewport(0, 0, self.width, self.height)

            self.fb_shader.begin(self.forward_att, [self.fb_tex])
            self.fb_shader.set_uniform('u_texture', '1i', 0)
            gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
            self.fb_shader.end()

            # render drawing commands
            gl.glEnable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
            gl.glEnable(gl.GL_POINT_SPRITE)

            self.dc_shader.begin(self.dc_att, self.dc_tex_list)
            self.dc_shader.set_uniform('u_res', '2f', self.width, self.height)

            for i, v in enumerate(palette):
                name = 'u_color[{}]'.format(i)
                r, g, b = int_to_rgb(v)
                self.dc_shader.set_uniform(name, '3i', r, g, b)

            for i, v in enumerate(self.dc_tex_list):
                if v:
                    name = 'u_texture[{}]'.format(i)
                    self.dc_shader.set_uniform(name, '1i', i)

                    name = 'u_texture_size[{}]'.format(i)
                    self.dc_shader.set_uniform(name, '2f', v.width, v.height)

            gl.glDrawArrays(gl.GL_POINTS, 0, self.cur_draw_count)
            self.dc_shader.end()
            self.fb_tex.copy_screen(0, 0, self.width, self.height)

            self.need_to_refresh = False

        # scaling
        gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
        gl.glDisable(gl.GL_POINT_SPRITE)
        gl.glViewport(left, bottom, width, height)

        self.fb_shader.begin(self.inverse_att, [self.fb_tex])
        self.fb_shader.set_uniform('u_texture', '1i', 0)
        gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
        self.fb_shader.end()

    def _next_dc_data(self):
        data = self.dc_att.data[self.cur_draw_count]
        data[CLIP_PAL_INDEX:CLIP_PAL_INDEX +
             CLIP_PAL_COUNT] = self.clip_pal_data

        if self.cur_draw_count < self.max_draw_count:
            self.cur_draw_count += 1

        return data

    def bank(self, index):
        return self.bank_list[index]

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
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_RECT
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = 0
        data[POS_Y1_INDEX] = 0

        data[SIZE_W_INDEX] = self.width
        data[SIZE_H_INDEX] = self.height

    def pix(self, x, y, col):
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_PIX
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

    def line(self, x1, y1, x2, y2, col):
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_LINE
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x1
        data[POS_Y1_INDEX] = y1
        data[POS_X2_INDEX] = x2
        data[POS_Y2_INDEX] = y2

    def rect(self, x, y, w, h, col):
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_RECT
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

    def rectb(self, x, y, w, h, col):
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_RECTB
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

    def circ(self, x, y, r, col):
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_CIRC
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = r

    def circb(self, x, y, r, col):
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_CIRCB
        data[MODE_COL_INDEX] = col

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y

        data[SIZE_W_INDEX] = r

    def blt(self, x, y, w, h, bank, sx, sy, colkey=-1):
        data = self._next_dc_data()

        data[MODE_TYPE_INDEX] = TYPE_BLT
        data[MODE_COL_INDEX] = colkey
        data[MODE_BANK_INDEX] = bank

        data[POS_X1_INDEX] = x
        data[POS_Y1_INDEX] = y
        data[POS_X2_INDEX] = sx
        data[POS_Y2_INDEX] = sy

        data[SIZE_W_INDEX] = w
        data[SIZE_H_INDEX] = h

    def text(self, x, y, str, col):
        pass
