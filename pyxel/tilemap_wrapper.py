def setup_apis(module, lib):
    import ctypes

    class Tilemap:
        def __init__(self, c_obj):
            self._c_obj = c_obj
            self._data = lib.tilemap_data_getter(c_obj)

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
        def refimg(self, img):
            return lib.tilemap_refimg_setter(self._c_obj, img)

        def get(self, x, y):
            return lib.tilemap_get(self._c_obj, x, y)

        def set(self, x, y, data):
            if type(data) is int:
                lib.tilemap_set1(self._c_obj, x, y, data)
            else:
                data_count = len(data)
                c_data = (ctypes.c_char_p * data_count)()

                for i in range(data_count):
                    c_str = ctypes.create_string_buffer(data[i].encode("utf-8"))
                    c_data[i] = ctypes.cast(c_str, ctypes.c_char_p)

                lib.tilemap_set(self._c_obj, x, y, c_data, data_count)

        def copy(self, x, y, tm, u, v, w, h):
            lib.tilemap_copy(self._c_obj, x, y, tm, u, v, w, h)

    lib.tilemap_data_getter.restype = ctypes.POINTER(ctypes.c_int32)

    module.Tilemap = Tilemap
