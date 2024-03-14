import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()

#flip函数可以更新一次画面。

import pyxel

pyxel.init(120, 80)


pyxel.cls(3)
pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
pyxel.flip()