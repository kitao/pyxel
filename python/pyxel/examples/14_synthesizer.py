import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150, title="Synthesizer")

        # Extend two channels
        channels = pyxel.channels.to_list()
        channels.append(pyxel.Channel())
        channels.append(pyxel.Channel())
        for channel in channels:
            channel.gain = 0.1  # Prevent the total volume from becoming too loud
        pyxel.channels.from_list(channels)

        # Extend two tones
        tones = pyxel.tones.to_list()
        tones.append(pyxel.Tone())
        tones.append(pyxel.Tone())
        pyxel.tones.from_list(tones)

        pyxel.tones[4].waveform.from_list([15] + [0] * 31)
        # Set the waveform for all 32 steps using values from 0 to 15.
        pyxel.tones[4].gain = 0.3

        pyxel.tones[5].noise = 1
        # 0: Disable noise (use waveform), 1: Short-period noise, 2: Long-period noise
        pyxel.tones[5].gain = 0.6

        pyxel.channels[1].detune = 20
        # The unit is cents, where a change of 100 represents a semitone shift.

        pyxel.sounds[0].set("c2c2c2c2c2c2", "0", "5", "n", 30)
        pyxel.sounds[1].set("r c2c2c2c2c2", "0", "5", "n", 30)
        pyxel.sounds[2].set("rr e2e2e2e2", "4", "5", "n", 30)
        pyxel.sounds[3].set("rrr g2g2g2", "0", "5", "n", 30)
        pyxel.sounds[4].set("rrrr b2b2", "0", "5", "n", 30)
        pyxel.sounds[5].set("rrrrr d3", "5", "5", "n", 30)
        pyxel.musics[0].set([0], [1], [2], [3], [4], [5])
        pyxel.playm(0, loop=True)

        pyxel.run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pyxel.text(20, 72, "This example is still under development!", 7)


App()
