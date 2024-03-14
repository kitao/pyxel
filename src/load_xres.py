"""
Copyright (c) Cookie Yang. All right reserved.
"""
import pyxel


pyxel.init(160, 120)
pyxel.load("my_resource.pyxres", excl_images=True)
pyxel.show()  # show会直到按下Esc按键
