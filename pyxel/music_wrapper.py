def setup_apis(module, lib):
    class Music:
        ch0 = []
        ch1 = []
        ch2 = []
        ch3 = []

        def __init__(self, msc):
            self.msc = msc

        def set(self, ch0, ch1, ch2, ch3) -> None:
            pass

        def set_ch0(self, data) -> None:
            pass

        def set_ch1(self, data) -> None:
            pass

        def set_ch2(self, data) -> None:
            pass

        def set_ch3(self, data) -> None:
            pass

    module.Music = Music
