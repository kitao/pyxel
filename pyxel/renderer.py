import math

import OpenGL.GL as gl

from .constants import (
    DRAW_MAX_COUNT,
    FONT_DATA,
    FONT_HEIGHT,
    FONT_WIDTH,
    RENDERER_IMAGE_COUNT,
    RENDERER_IMAGE_HEIGHT,
    RENDERER_IMAGE_WIDTH,
    RENDERER_MIN_TEXTURE_SIZE,
    RENDERER_TILEMAP_COUNT,
    RENDERER_TILEMAP_HEIGHT,
    RENDERER_TILEMAP_WIDTH,
)
from .draw_command import DrawCommand
from .gl_wrapper import GLAttribute, GLShader, GLTexture
from .image import Image
from .shaders import (
    DRAWING_ATTRIBUTE_INFO,
    DRAWING_FRAGMENT_SHADER,
    DRAWING_VERTEX_SHADER,
    SCALING_ATTRIBUTE_INFO,
    SCALING_FRAGMENT_SHADER,
    SCALING_VERTEX_SHADER,
)
from .tilemap import Tilemap


class Renderer:
    def __init__(self, width, height):
        self._width = width
        self._height = height

        self._image_list = [
            Image(RENDERER_IMAGE_WIDTH, RENDERER_IMAGE_HEIGHT)
            for _ in range(RENDERER_IMAGE_COUNT)
        ]
        self._set_font_image(self._image_list[-1])

        self._tilemap_list = [
            Tilemap(RENDERER_TILEMAP_WIDTH, RENDERER_TILEMAP_HEIGHT)
            for _ in range(RENDERER_TILEMAP_COUNT)
        ]

        self._draw_shader = GLShader(DRAWING_VERTEX_SHADER, DRAWING_FRAGMENT_SHADER)
        self._draw_att = GLAttribute(
            DRAWING_ATTRIBUTE_INFO, DRAW_MAX_COUNT, dynamic=True
        )

        self.draw_command = DrawCommand(
            width, height, self._draw_att.data, self._tilemap_list
        )

        self._scale_shader = GLShader(SCALING_VERTEX_SHADER, SCALING_FRAGMENT_SHADER)

        tex_width = self.largest_power_of_two(width)
        tex_height = self.largest_power_of_two(height)

        self._scale_tex = GLTexture(tex_width, tex_height, 3, nearest=True)

        u = width / tex_width
        v = height / tex_height

        self._normal_scale_att = GLAttribute(SCALING_ATTRIBUTE_INFO, 4)
        data = self._normal_scale_att.data
        data[0, :] = [-1 / 3, 1 / 3, 0, v]
        data[1, :] = [-1 / 3, -1 / 3, 0, 0]
        data[2, :] = [1 / 3, 1 / 3, u, v]
        data[3, :] = [1 / 3, -1 / 3, u, 0]

        self._inverse_scale_att = GLAttribute(SCALING_ATTRIBUTE_INFO, 4)
        data = self._inverse_scale_att.data
        data[0, :] = [-1, 1, 0, 0]
        data[1, :] = [-1, -1, 0, v]
        data[2, :] = [1, 1, u, 0]
        data[3, :] = [1, -1, u, v]

    def render(self, left, bottom, width, height, palette, clear_color):
        # restore previous frame
        gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
        gl.glDisable(gl.GL_POINT_SPRITE)
        gl.glViewport(
            int(-self._width),
            int(-self._height),
            int(self._width * 3),
            int(self._height * 3),
        )

        self._scale_shader.begin(self._normal_scale_att, [self._scale_tex])
        self._scale_shader.set_uniform("u_texture", "1i", 0)
        gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
        self._scale_shader.end()

        # render drawing commands
        if self.draw_command.draw_count > 0:
            gl.glEnable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
            gl.glEnable(gl.GL_POINT_SPRITE)

            draw_tex_list = [
                (image and image._tex or None) for image in self._image_list
            ]
            self._draw_att.update(self.draw_command.draw_count)
            self._draw_shader.begin(self._draw_att, draw_tex_list)
            self._draw_shader.set_uniform(
                "u_framebuffer_size", "2f", self._width, self._height
            )

            for i, v in enumerate(palette):
                name = "u_palette[{}]".format(i)
                r, g, b = self._int_to_rgb(v)
                self._draw_shader.set_uniform(name, "3i", r, g, b)

            for i, v in enumerate(draw_tex_list):
                if v:
                    name = "u_texture[{}]".format(i)
                    self._draw_shader.set_uniform(name, "1i", i)

                    name = "u_texture_size[{}]".format(i)
                    self._draw_shader.set_uniform(name, "2f", v.width, v.height)

            gl.glDrawArrays(gl.GL_POINTS, 0, self.draw_command.draw_count)
            self._draw_shader.end()
            self._scale_tex.copy_screen(0, 0, 0, 0, self._width, self._height)

            self.draw_command.draw_count = 0

        # capture screen
        capture_image = gl.glReadPixels(
            0, 0, self._width, self._height, gl.GL_RGB, gl.GL_UNSIGNED_BYTE
        )

        # clear screen
        r, g, b = self._int_to_rgb(clear_color)
        gl.glClearColor(r / 255, g / 255, b / 255, 1)
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)

        # scaling
        gl.glDisable(gl.GL_VERTEX_PROGRAM_POINT_SIZE)
        gl.glDisable(gl.GL_POINT_SPRITE)
        gl.glViewport(int(left), int(bottom), int(width), int(height))

        self._scale_shader.begin(self._inverse_scale_att, [self._scale_tex])
        self._scale_shader.set_uniform("u_texture", "1i", 0)
        gl.glDrawArrays(gl.GL_TRIANGLE_STRIP, 0, 4)
        self._scale_shader.end()

        return capture_image

    def image(self, img, *, system=False):
        if not system and img == RENDERER_IMAGE_COUNT - 1:
            raise ValueError("image bank {} is reserved for system".format(img))

        return self._image_list[img]

    def tilemap(self, tm):
        return self._tilemap_list[tm]

    @staticmethod
    def _set_font_image(image):
        row_count = image.width // FONT_WIDTH

        for i, v in enumerate(FONT_DATA):
            left = (i % row_count) * FONT_WIDTH
            top = (i // row_count) * FONT_HEIGHT
            data = image.data

            for j in range(FONT_WIDTH * FONT_HEIGHT):
                x = left + j % FONT_WIDTH
                y = top + j // FONT_WIDTH
                data[y, x] = (v & 0x800000) and 7 or 0
                v <<= 1

    @staticmethod
    def largest_power_of_two(n):
        return max(2 ** math.ceil(math.log(n, 2)), RENDERER_MIN_TEXTURE_SIZE)

    @staticmethod
    def _int_to_rgb(color):
        return (color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF
