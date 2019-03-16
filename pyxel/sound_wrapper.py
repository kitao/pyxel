def setup_apis(module, lib):
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

    module.Sound = Sound
