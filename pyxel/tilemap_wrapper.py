def setup_apis(module, lib):
    class Tilemap:
        width: int = 0
        height: int = 0
        data = None

        def __init__(self, tm):
            self.tm = tm

        def get(self, x: int, y: int) -> int:
            pass

        def set(self, x: int, y: int, data, refimg: int = None) -> None:
            pass

        def copy(self, x: int, y: int, tm: int, u: int, v: int, w: int, h: int) -> None:
            pass

    module.Tilemap = Tilemap
