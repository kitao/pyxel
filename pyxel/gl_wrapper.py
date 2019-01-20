import ctypes

import numpy as np
import OpenGL.GL as gl
from OpenGL.GL import shaders


class GLShader:
    def __init__(self, vertex_shader, fragment_shader):
        self._program = shaders.compileProgram(
            shaders.compileShader(vertex_shader, gl.GL_VERTEX_SHADER),
            shaders.compileShader(fragment_shader, gl.GL_FRAGMENT_SHADER),
        )
        self._att = None
        self._tex_list = []

    def begin(self, att, tex_list):
        self._att = att
        self._tex_list = tex_list

        gl.glUseProgram(self._program)

        if att:
            att._begin(self._program)

        for i, tex in enumerate(tex_list):
            if tex:
                tex._begin(i)

    def end(self):
        for i, tex in enumerate(self._tex_list):
            if tex:
                tex._end(i)

        if self._att:
            self._att._end(self._program)

        gl.glUseProgram(0)

        self._att = None
        self._tex_list = []

    def set_uniform(self, name, param_type, *params):
        loc = gl.glGetUniformLocation(self._program, name)
        getattr(gl, "glUniform" + param_type)(loc, *params)


class GLAttribute:
    def __init__(self, att_info, count, *, dynamic=False):
        self._att_info = att_info[:]
        self._size = sum(att[2] for att in att_info)
        self._stride = self._size * 4
        self._count = count
        self._usage = dynamic and gl.GL_DYNAMIC_DRAW or gl.GL_STATIC_DRAW
        self._dtype = gl.GL_FLOAT
        self._data = np.zeros((count, self._size), np.float32)
        self._buf = gl.glGenBuffers(1)
        self._should_update_data = True

    @property
    def data(self):
        return self._data

    def update(self, count=0):
        self._count = (count == 0) and self._data.shape[0] or count
        self._should_update_data = True

    def _begin(self, program):
        gl.glBindBuffer(gl.GL_ARRAY_BUFFER, self._buf)

        if self._should_update_data:
            size = self._stride * self._count
            gl.glBufferData(
                gl.GL_ARRAY_BUFFER, size, self._data[:size].tobytes(), self._usage
            )
            self._should_update_data = False

        for att in self._att_info:
            loc = gl.glGetAttribLocation(program, att[0])
            gl.glVertexAttribPointer(
                loc,
                att[2],
                self._dtype,
                gl.GL_FALSE,
                self._stride,
                ctypes.c_void_p(att[1] * 4),
            )
            gl.glEnableVertexAttribArray(loc)

    def _end(self, program):
        for att in self._att_info:
            loc = gl.glGetAttribLocation(program, att[0])
            gl.glDisableVertexAttribArray(loc)

        gl.glBindBuffer(gl.GL_ARRAY_BUFFER, 0)


class GLTexture:
    def __init__(self, width, height, size, *, nearest=False):
        if size == 1:
            self._format = gl.GL_LUMINANCE
            shape = (height, width)
        elif size == 3:
            self._format = gl.GL_RGB
            shape = (height, width, 3)
        elif size == 4:
            self._format = gl.GL_RGBA
            shape = (height, width, 4)
        else:
            raise ValueError("invalid texture format")

        self._width = width
        self._height = height
        self._filter = nearest and gl.GL_NEAREST or gl.GL_LINEAR
        self._data = np.zeros(shape, np.uint8)
        self._tex = gl.glGenTextures(1)
        self._should_update_data = True

    @property
    def width(self):
        return self._width

    @property
    def height(self):
        return self._height

    @property
    def data(self):
        return self._data

    def update(self):
        self._should_update_data = True

    def copy_screen(self, x, y, left, bottom, width, height):
        gl.glBindTexture(gl.GL_TEXTURE_2D, self._tex)
        gl.glCopyTexSubImage2D(gl.GL_TEXTURE_2D, 0, x, y, left, bottom, width, height)
        gl.glBindTexture(gl.GL_TEXTURE_2D, 0)

    def _begin(self, i):
        gl.glActiveTexture(gl.GL_TEXTURE0 + i)
        gl.glBindTexture(gl.GL_TEXTURE_2D, self._tex)

        if self._should_update_data:
            gl.glTexImage2D(
                gl.GL_TEXTURE_2D,
                0,
                self._format,
                self._width,
                self._height,
                0,
                self._format,
                gl.GL_UNSIGNED_BYTE,
                self._data.tobytes(),
            )
            self._should_update_data = False

        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_S, gl.GL_CLAMP_TO_EDGE)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_T, gl.GL_CLAMP_TO_EDGE)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, self._filter)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, self._filter)
        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)

    @staticmethod
    def _end(i):
        gl.glActiveTexture(gl.GL_TEXTURE0 + i)
        gl.glBindTexture(gl.GL_TEXTURE_2D, 0)
