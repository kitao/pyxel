def setup_apis(module, lib):
    import ctypes
    import numpy as np

    class Tilemap:
        def __init__(self, c_obj):
            self._c_obj = c_obj

            self._data = np.ctypeslib.as_array(
                lib.tilemap_data_getter(c_obj), shape=(self.width * self.height,)
            )

        @property
        def width(self):
            return lib.tilemap_width_getter(self._c_obj)

        @property
        def height(self):
            return lib.tilemap_height_getter(self._c_obj)

        @property
        def data(self):
            return self._data

        @property
        def refimg(self):
            return lib.tilemap_refimg_getter(self._c_obj)

        @refimg.setter
        def refimg(self, value):
            return lib.tilemap_refimg_setter(self._c_obj, value)

        def get(self, x, y):
            return lib.tilemap_get(self._c_obj, x, y)

        def set(self, x, y, val):
            if type(val) is int:
                lib.tilemap_set1(self._c_obj, x, y, val)
            else:
                val_count = len(val)
                c_val = (ctypes.c_char_p * val_count)()

                for i in range(val_count):
                    c_str = ctypes.create_string_buffer(val[i].encode("utf-8"))
                    c_val[i] = ctypes.cast(c_str, ctypes.c_char_p)

                lib.tilemap_set(self._c_obj, x, y, c_val, val_count)

        def copy(self, x, y, tm, u, v, w, h):
            lib.tilemap_copy(self._c_obj, x, y, tm, u, v, w, h)

    module.Tilemap = Tilemap
