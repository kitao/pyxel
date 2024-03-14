"""
Copyright (c) Cookie Yang. All right reserved.
"""
import pyxel

pyxel.init(160, 120)

# 定义一个简单的乐曲
melody = (
    "c4 c4 g4 g4 a4 a4 g4 "
    "f4 f4 e4 e4 d4 d4 c4 "
    "g4 g4 f4 f4 e4 e4 d4 "
    "g4 g4 f4 f4 e4 e4 d4 "
    "c4 c4 g4 g4 a4 a4 g4 "
    "f4 f4 e4 e4 d4 d4 c4 "
)

# 定义音乐
pyxel.sound(0).set(
    notes=melody,  # a -g , 0-4
    tones="s",  # s表示方波, t表示三角音, n表示噪音
    volumes="6",  # 0为静音, 音量
    effects="n",
    speed=44
)

pyxel.play(0, 0, loop=True)

def update():
    pass

def draw():
    pyxel.cls(0)
    pyxel.text(55, 41, "Custom Music Example", 7)

pyxel.run(update, draw)
