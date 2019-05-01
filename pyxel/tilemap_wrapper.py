def setup_apis(module, lib):
    import ctypes
    import numpy as np

    class Tilemap:
        def __init__(self, c_obj):
            self._c_obj = c_obj

            self._data = np.ctypeslib.as_array(
                lib.tilemap_data_getter(ctypes.c_void_p(c_obj)),
                shape=(self.width * self.height,),
            )

        @property
        def width(self):
            return lib.tilemap_width_getter(ctypes.c_void_p(self._c_obj))

        @property
        def height(self):
            return lib.tilemap_height_getter(ctypes.c_void_p(self._c_obj))

        @property
        def data(self):
            return self._data

        @property
        def refimg(self):
            return lib.tilemap_refimg_getter(ctypes.c_void_p(self._c_obj))

        @refimg.setter
        def refimg(self, value):
            return lib.tilemap_refimg_setter(ctypes.c_void_p(self._c_obj), value)

        def get(self, x, y):
            return lib.tilemap_get(self._c_obj, x, y)

        def set(self, x, y, data):
            lib.tilemap_set1(self._c_obj, x, y, data)
            # todo

        def copy(self, x, y, tm, u, v, w, h):
            lib.tilemap_copy(self._c_obj, x, y, tm, u, v, w, h)

    module.Tilemap = Tilemap
