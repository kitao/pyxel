from .constants import (
    SOUND_NOTE_TABLE,
    SOUND_TONE_TABLE,
    SOUND_EFFECT_TABLE,
)


class Sound:
    def __init__(self):
        self.note = [0]
        self.tone = [0]
        self.volume = [0]
        self.effect = [0]
        self.speed = 1

    def set(self, note, tone, volume, effect, speed):
        self.set_note(note)
        self.set_tone(tone)
        self.set_volume(volume)
        self.set_effect(effect)
        self.speed = speed

    def set_note(self, data):
        param_list = []
        last_param = 0
        data = data.replace(' ', '').replace('\n', '').replace('\t',
                                                               '').lower()

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

        self.note = param_list

    def set_tone(self, data):
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

        self.tone = param_list

    def set_volume(self, data):
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

        self.volume = param_list

    def set_effect(self, data):
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

        self.effect = param_list
