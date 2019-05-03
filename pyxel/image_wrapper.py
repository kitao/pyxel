def setup_apis(module, lib):
    import ctypes

    class Image:
        def __init__(self, c_obj):
            self._c_obj = c_obj
            self._data = lib.image_data_getter(c_obj)

        @property
        def width(self):
            return lib.image_width_getter(self._c_obj)

        @property
        def height(self):
            return lib.image_height_getter(self._c_obj)

        @property
        def data(self):
            return self._data

        def get(self, x, y):
            return lib.image_get(self._c_obj, x, y)

        def set(self, x, y, data):
            if type(data) is int:
                lib.image_set1(self._c_obj, x, y, data)
            else:
                data_count = len(data)
                c_data = (ctypes.c_char_p * data_count)()

                for i in range(data_count):
                    c_str = ctypes.create_string_buffer(data[i].encode("utf-8"))
                    c_data[i] = ctypes.cast(c_str, ctypes.c_char_p)

                lib.image_set(self._c_obj, x, y, c_data, data_count)

        def load(self, x, y, filename):
            c_filename = ctypes.create_string_buffer(filename.encode("utf-8"))
            lib.image_load(self._c_obj, x, y, c_filename)

        def copy(self, x, y, img, u, v, w, h):
            lib.image_copy(self._c_obj, x, y, img, u, v, w, h)

    lib.image_data_getter.restype = ctypes.POINTER(ctypes.c_int32)

    module.Image = Image
