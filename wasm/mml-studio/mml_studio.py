import pyxel
from js import window  # type: ignore

NUM_CHANNELS = 4


def js_var(name, default):
    return getattr(window, name) if hasattr(window, name) else default


class App:
    def __init__(self):
        pyxel.init(100, 20, title="Pyxel MML Studio", quit_key=pyxel.KEY_NONE)

        self.default_gain = pyxel.channels[0].gain

        loop = js_var("js_loop", False)
        self.sounds = []
        for i in range(NUM_CHANNELS):
            sound = pyxel.Sound()
            sound.mml(js_var(f"js_ch{i + 1}_mml", ""))
            self.sounds.append(sound)
            pyxel.play(i, sound, loop=loop)

        pyxel.run(self.update, self.draw)

    def update(self):
        if js_var("js_stop", False):
            pyxel.stop()

        solo_enabled = any(
            js_var(f"js_solo{i + 1}", False) for i in range(NUM_CHANNELS)
        )
        for i in range(NUM_CHANNELS):
            pyxel.channels[i].gain = (
                self.default_gain
                if not solo_enabled or js_var(f"js_solo{i + 1}", False)
                else 0.0
            )

        for i in range(NUM_CHANNELS):
            if js_var(f"js_mute{i + 1}", False):
                pyxel.channels[i].gain = 0.0

    def draw(self):
        pyxel.cls(1)

        pyxel.rectb(0, -1, pyxel.width, pyxel.height + 2, 5)

        for i in range(NUM_CHANNELS):
            total_sec = self.sounds[i].total_sec()
            (_, play_sec) = pyxel.play_pos(i) or (None, None)

            if total_sec is None or total_sec == 0 or play_sec is None:
                continue

            x = pyxel.width * play_sec / total_sec
            y = i * 5 + 2
            if pyxel.channels[i].gain > 0:
                pyxel.circb(x, y, 2, i + 8)
            else:
                pyxel.rect(x - 1, y - 1, 3, 3, 5)


App()
