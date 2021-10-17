class WidgetVariable:
    def __init__(self, value, *, on_get=None, on_set=None, on_change=None):
        self._value = value
        self._on_get = on_get
        self._on_set = on_set
        self._on_change = on_change

    @property
    def v(self):
        if self._on_get:
            return self._on_get(self._value)
        else:
            return self._value

    @v.setter
    def v(self, value):
        if self._on_set:
            value = self._on_set(value)

        if self._value == value:
            return

        self._value = value

        if self._on_change:
            self._on_change(value)
