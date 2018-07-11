from .constants import (
    SOUND_NOTE_TABLE,
    SOUND_TONE_TABLE,
    SOUND_EFFECT_TABLE,
)


class Sound:
    def __init__(self):
        self._note = [0]
        self._tone = [0]
        self._volume = [0]
        self._effect = [0]
        self._speed = 1

    def set(self, note, tone, volume, effect, speed):
        self._note = self._parse_note(note)
        self._tone = self._parse_tone(tone)
        self._volume = self._parse_volume(volume)
        self._effect = self._parse_effect(effect)
        self._speed = speed

    def _parse_note(self, data):
        param_list = []
        last_param = 0
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_NOTE_TABLE.get(c, None)

            if param is not None:
                c = data[0]
                data = data[1:]

                if c == '#' or c == '-':
                    param += c == '#' and 1 or -1

                    c = data[0]
                    data = data[1:]

                if c >= '0' and c <= '4':
                    param += int(c) * 12
                else:
                    raise ValueError('invalid sound note')

            elif c == '.':
                param = last_param
            elif c == 'r':
                param = -1
            else:
                raise ValueError('invalid sound note')

            param_list.append(param)
            last_param = param

        return param_list

    def _parse_tone(self, data):
        param_list = []
        last_param = 0
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_TONE_TABLE.get(c, None)

            if param is None:
                if c == '.':
                    param = last_param
                else:
                    raise ValueError('invalid sound tone')

            param_list.append(param)
            last_param = param

        return self._complement_param_list(param_list)

    def _parse_volume(self, data):
        param_list = []
        last_param = 0
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            if c >= '0' and c <= '7':
                param = int(c)
            elif c == '.':
                param = last_param
            else:
                raise ValueError('invalid sound volume')

            param_list.append(param)
            last_param = param

        return self._complement_param_list(param_list)

    def _parse_effect(self, data):
        param_list = []
        last_param = 0
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_EFFECT_TABLE.get(c, None)

            if param is None:
                if c == '.':
                    param = last_param
                else:
                    raise ValueError('invalid sound effect')

            param_list.append(param)
            last_param = param

        return self._complement_param_list(param_list)

    def _complement_param_list(self, param_list):
        diff = len(param_list) - len(self._note)

        if diff < 0:
            param_list += [param_list[-1]] * -diff
        elif diff > 0:
            param_list = param_list[:len(self._note)]

        return param_list
