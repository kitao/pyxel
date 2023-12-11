import pyxel


class App:
    def __init__(self):
        pyxel.init(64, 64, title="Synthesizer")
        channels = pyxel.channels.to_list()
        channels.append(pyxel.Channel())
        print(channels)
        tones = pyxel.tones.to_list()
        tones.append(pyxel.Tone())
        pyxel.tones.from_list(tones)
        wave = pyxel.tones[0].waveform.to_list()
        print(wave)
        pyxel.run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pass


App()
