def setup_apis(module, lib):
    #
    # Sound class
    #
    class Sound:
        note = []
        tone = []
        volume = []
        effect = []
        speed = 0

        def __init__(self, snd):
            self.snd = snd

        def set(
            self, note: str, tone: str, volume: str, effect: str, speed: int
        ) -> None:
            pass

        def set_note(self, data: str) -> None:
            pass

        def set_tone(self, data: str) -> None:
            pass

        def set_volume(self, data: str) -> None:
            pass

        def set_effect(self, data: str) -> None:
            pass

    #
    # Music class
    #
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
