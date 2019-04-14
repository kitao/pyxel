def setup_apis(module, lib):
    import ctypes
    import numpy as np

    class Image:
        def __init__(self, c_obj):
            self._c_obj = c_obj

            self._data = np.ctypeslib.as_array(
                lib.image_data_getter(ctypes.c_void_p(c_obj)),
                shape=(self.width * self.height,),
            )

        @property
        def width(self):
            return lib.image_width_getter(ctypes.c_void_p(self._c_obj))

        @property
        def height(self):
            return lib.image_height_getter(ctypes.c_void_p(self._c_obj))

        @property
        def data(self):
            return self._data

        def get(self, x, y):
            return lib.image_get(self._c_obj, x, y)

        def set(self, x, y, data):
            if type(data) is int:
                lib.image_set1(
                    self._c_obj,
                    x,
                    y,
                    data.ctypes.data_as(ctypes.POINTER(ctypes.c_int32)),
                )
            else:
                lib.image_set(
                    self._c_obj,
                    x,
                    y,
                    data.ctypes.data_as(ctypes.POINTER(ctypes.c_int32)),
                    data.shape[1],
                    data.shape[0],
                )

        def load(self, x, y, filename):
            c_filename = ctypes.create_string_buffer(filename.encode("utf-8"))
            lib.image_load(ctypes.c_void_p(self._c_obj), x, y, c_filename)

        def copy(self, x, y, img, u, v, w, h):
            lib.image_copy(self._c_obj, x, y, img, u, v, w, h)

    module.Image = Image
