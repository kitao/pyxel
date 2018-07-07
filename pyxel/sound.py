NOTE_TABLE = {'c': 0, 'd': 2, 'e': 4, 'f': 5, 'g': 7, 'a': 9, 'b': 11}

TONE_TRIANGLE = 0
TONE_SQUARE = 1
TONE_PULSE = 2
TONE_NOISE = 3

TONE_TABLE = {
    't': TONE_TRIANGLE,
    's': TONE_SQUARE,
    'p': TONE_PULSE,
    'n': TONE_NOISE,
}

EFFECT_NONE = 0
EFFECT_SLIDE = 1
EFFECT_VIBRATO = 2
EFFECT_FADEOUT = 3

EFFECT_TABLE = {
    'n': EFFECT_NONE,
    's': EFFECT_SLIDE,
    'v': EFFECT_VIBRATO,
    'f': EFFECT_FADEOUT,
}


class Sound:
    def __init__(self, speed, data):
        self.speed = max(speed, 1)
        self.note = self._parse_note(data[0])
        self.tone = self._parse_tone(data[1])
        self.volume = self._parse_volume(data[2])
        self.effect = self._parse_effect(data[3])

    def _parse_note(self, data):
        param_list = []
        last_param = None
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            param = NOTE_TABLE.get(c, None)

            if param is not None:
                c = data[0]
                data = data[1:]

                if c == '#':
                    param += 1

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
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            param = TONE_TABLE.get(c, None)

            if param is None:
                raise ValueError('invalid sound data')

            param_list.append(param)

        return self._complement_param_list(param_list)

    def _parse_volume(self, data):
        param_list = []
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            if c >= '0' and c <= '7':
                param = int(c)
            else:
                raise ValueError('invalid sound data')

            param_list.append(param)

        return self._complement_param_list(param_list)

    def _parse_effect(self, data):
        param_list = []
        data = data.replace(' ', '').lower()

        while data:
            c = data[0]
            data = data[1:]

            param = EFFECT_TABLE.get(c, None)

            if param is None:
                raise ValueError('invalid sound data')

            param_list.append(param)

        return self._complement_param_list(param_list)

    def _complement_param_list(self, param_list):
        diff = len(param_list) - len(self.note)

        if diff < 0:
            param_list += [param_list[-1]] * -diff
        elif diff > 0:
            param_list = param_list[:len(self.note)]

        return param_list
