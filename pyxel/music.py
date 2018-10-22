class Music:
    def __init__(self):
        self._ch0 = []
        self._ch1 = []
        self._ch2 = []
        self._ch3 = []

    @property
    def ch0(self):
        return self._ch0

    @property
    def ch1(self):
        return self._ch1

    @property
    def ch2(self):
        return self._ch2

    @property
    def ch3(self):
        return self._ch3

    def set(self, ch0, ch1, ch2, ch3):
        self._ch0[:] = ch0
        self._ch1[:] = ch1
        self._ch2[:] = ch2
        self._ch3[:] = ch3
