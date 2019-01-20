from .constants import SOUND_EFFECT_TABLE, SOUND_NOTE_TABLE, SOUND_TONE_TABLE


class Sound:
    def __init__(self):
        self._note = []
        self._tone = []
        self._volume = []
        self._effect = []
        self.speed = 30

    @property
    def note(self):
        return self._note

    @property
    def tone(self):
        return self._tone

    @property
    def volume(self):
        return self._volume

    @property
    def effect(self):
        return self._effect

    def set(self, note, tone, volume, effect, speed):
        self.set_note(note)
        self.set_tone(tone)
        self.set_volume(volume)
        self.set_effect(effect)
        self.speed = speed

    def set_note(self, data):
        param_list = []
        data = data.replace(" ", "").replace("\n", "").replace("\t", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_NOTE_TABLE.get(c, None)

            if param is not None:
                c = data[0]
                data = data[1:]

                if c == "#" or c == "-":
                    param += c == "#" and 1 or -1

                    c = data[0]
                    data = data[1:]

                if "0" <= c <= "4":
                    param += int(c) * 12
                else:
                    raise ValueError("invalid sound note")
            elif c == "r":
                param = -1
            else:
                raise ValueError("invalid sound note")

            param_list.append(param)

        self._note[:] = param_list

    def set_tone(self, data):
        param_list = []
        data = data.replace(" ", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_TONE_TABLE.get(c, None)

            if param is None:
                raise ValueError("invalid sound tone")

            param_list.append(param)

        self._tone[:] = param_list

    def set_volume(self, data):
        param_list = []
        data = data.replace(" ", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            if "0" <= c <= "7":
                param = int(c)
            else:
                raise ValueError("invalid sound volume")

            param_list.append(param)

        self._volume[:] = param_list

    def set_effect(self, data):
        param_list = []
        data = data.replace(" ", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_EFFECT_TABLE.get(c, None)

            if param is None:
                raise ValueError("invalid sound effect")

            param_list.append(param)

        self._effect[:] = param_list
