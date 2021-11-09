class WidgetVar:
    """
    Events:
        get (value) -> value
        set (value) -> value
        change (value)
    """

    def __init__(self, value):
        self._value = value
        self._event_listeners = {"get": [], "set": [], "change": []}

    def get(self):
        value = self._value
        for listener in self._event_listeners["get"]:
            value = listener(value)
        return value

    def set(self, value):
        for listener in self._event_listeners["set"]:
            value = listener(value)

        if self._value == value:
            return

        self._value = value
        for listener in self._event_listeners["change"]:
            listener(value)

    def add_event_listener(self, event, listener):
        self._event_listeners[event].append(listener)

    def remove_event_listener(self, event, listener):
        self._event_listeners[event].remove(listener)
