import OpenGL.GL as gl
from OpenGL.GL import shaders
import numpy as np
import ctypes


class GLShader:
    def __init__(self, vertex_shader, fragment_shader):
        self.program = shaders.compileProgram(
            shaders.compileShader(vertex_shader, gl.GL_VERTEX_SHADER),
            shaders.compileShader(fragment_shader, gl.GL_FRAGMENT_SHADER))
        self.att = None
        self.tex_list = []

    def begin(self, att, tex_list):
        self.att = att
        self.tex_list = tex_list

        gl.glUseProgram(self.program)

        if att:
            att._begin(self.program)

        for i, tex in enumerate(tex_list):
            if tex:
                tex._begin(i)

    def end(self):
        for i, tex in enumerate(self.tex_list):
            if tex:
                tex._end(i)

        if self.att:
            self.att._end(self.program)

        gl.glUseProgram(0)

        self.att = None
        self.tex_list = []

    def set_uniform(self, name, param_type, *params):
        loc = gl.glGetUniformLocation(self.program, name)
        getattr(gl, 'glUniform' + param_type)(loc, *params)


class GLAttribute:
    def __init__(self, att_info, count, *, integer=False, dynamic=False):
        self.att_info = att_info[:]
        self.size = sum(att[2] for att in att_info)
        self.stride = self.size * 4
        self.count = count
        self.usage = gl.GL_DYNAMIC_DRAW if dynamic else gl.GL_STATIC_DRAW

        shape = (count, self.size)
        if integer:
            self.dtype = gl.GL_INT
            self.data = np.zeros(shape, np.int32)
        else:
            self.dtype = gl.GL_FLOAT
            self.data = np.zeros(shape, np.float32)

        self.buf = gl.glGenBuffers(1)
        self.need_to_refresh = True

    def refresh(self, count=0):
        self.count = self.data.shape[0] if count == 0 else count
        self.need_to_refresh = True

    def _begin(self, program):
        gl.glBindBuffer(gl.GL_ARRAY_BUFFER, self.buf)

        if self.need_to_refresh:
            gl.glBufferData(gl.GL_ARRAY_BUFFER, self.stride * self.count,
                            self.data.tobytes(), self.usage)
            self.need_to_refresh = False

        for att in self.att_info:
            loc = gl.glGetAttribLocation(program, att[0])
            gl.glVertexAttribPointer(loc, att[2], self.dtype, gl.GL_FALSE,
                                     self.stride, ctypes.c_void_p(att[1] * 4))
            gl.glEnableVertexAttribArray(loc)

    def _end(self, program):
        for att in self.att_info:
            loc = gl.glGetAttribLocation(program, att[0])
            gl.glDisableVertexAttribArray(loc)

        gl.glBindBuffer(gl.GL_ARRAY_BUFFER, 0)


class GLTexture:
    def __init__(self, width, height, size, *, nearest=False):
        if size == 1:
            self.format = gl.GL_LUMINANCE
            shape = (height, width)
        elif size == 3:
            self.format = gl.GL_RGB
            shape = (height, width, 3)
        elif size == 4:
            self.format = gl.GL_RGBA
            shape = (height, width, 4)
        else:
            raise ValueError('invalid texture format')

        self.width = width
        self.height = height
        self.filter = gl.GL_NEAREST if nearest else gl.GL_LINEAR
        self.data = np.zeros(shape, np.uint8)
        self.tex = gl.glGenTextures(1)
        self.need_to_refresh = True

    def refresh(self):
        self.need_to_refresh = True

    def copy_screen(self, x, y, width, height):
        gl.glBindTexture(gl.GL_TEXTURE_2D, self.tex)
        gl.glCopyTexImage2D(gl.GL_TEXTURE_2D, 0, self.format, x, y, width,
                            height, 0)
        gl.glBindTexture(gl.GL_TEXTURE_2D, 0)

    def _begin(self, i):
        gl.glActiveTexture(gl.GL_TEXTURE0 + i)
        gl.glBindTexture(gl.GL_TEXTURE_2D, self.tex)

        if self.need_to_refresh:
            gl.glTexImage2D(gl.GL_TEXTURE_2D, 0, self.format, self.width,
                            self.height, 0, self.format, gl.GL_UNSIGNED_BYTE,
                            self.data.tobytes())
            self.need_to_refresh = False

        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_S,
                           gl.GL_CLAMP_TO_EDGE)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_T,
                           gl.GL_CLAMP_TO_EDGE)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER,
                           self.filter)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER,
                           self.filter)
        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)

    def _end(self, i):
        gl.glActiveTexture(gl.GL_TEXTURE0 + i)
        gl.glBindTexture(gl.GL_TEXTURE_2D, 0)
