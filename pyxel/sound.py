NOTE_TABLE = {'c': 0, 'd': 2, 'e': 4, 'f': 5, 'g': 7, 'a': 9, 'b': 11}
TONE_TABLE = {'t': 0, 's': 1, 'p': 2, 'n': 3}
EFFECT_TABLE = {'-': 0, 's': 1, 'v': 2, 'f': 3}


class Sound:
    def __init__(self, speed, data):
        self.speed = max(speed, 1)
        self.note = self._parse_note(data[0])
        self.length = len(self.note)
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

                if c >= '0' and c <= '9':
                    param += int(c) * 12
                else:
                    raise ValueError('invalid sound note')

            elif c == '.':
                param = last_param
            elif c == 'r':
                param = None
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

        param_list = self._complement_param_list(param_list)
        return param_list

    def _parse_volume(self, data):
        param_list = []
        last_param = 0
        data = data.replace(' ', '')

        while data:
            c = data[0]
            data = data[1:]

            if c >= '0' and c <= '7':
                param = int(c)
            elif c == '.':
                param = last_param
            else:
                raise ValueError('invalid sound data')

            param_list.append(param)

        param_list = self._complement_param_list(param_list)
        return param_list

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

        param_list = self._complement_param_list(param_list)
        return param_list

    def _complement_param_list(self, param_list):
        diff = len(param_list) - self.length

        if diff < 0:
            param_list += [param_list[-1]] * -diff
        elif diff > 0:
            param_list = param_list[:self._length]

        return param_list
