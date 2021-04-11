proc mouseX(self: Pyxel): int = 0

proc mouseY(self: Pyxel): int = 0

proc mouseWheel(self: Pyxel): int = 0

proc btn(self: Pyxel, key: int): bool = false

proc btnp(self: Pyxel, key, hold, period: int): bool = false

proc btnr(self: Pyxel, key: int): bool = false

proc mouse(self: Pyxel, visible: bool) = discard
