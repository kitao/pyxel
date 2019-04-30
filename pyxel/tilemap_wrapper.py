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

        def get(self, x, y):
            return lib.tilemap_get(self._c_obj, x, y)

        def set(self, x, y, data, refimg):
            pass

        def copy(self, x, y, tm, u, v, w, h):
            lib.tilemap_copy(self._c_obj, x, y, tm, u, v, w, h)

    module.Tilemap = Tilemap
