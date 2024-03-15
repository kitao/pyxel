"""
Copyright (c) Cookie Yang. All right reserved.
"""
import pyxel


pyxel.init(400, 400)
pyxel.cls(3)

pyxel.load("my_resource.pyxres")

"""
blt(x, y, img, u, v, w, h, [colkey])
x：显示图像的 x 坐标
y：显示图像的 y 坐标
img：图像编号（图像库编号）
u：图像库的位置 (x)
v：图像库的位置 (y)
w：要获取的图像 宽度
h : 获取图像的高度
"""
pyxel.blt(0, 0, 0, 0, 0, 16, 16, 0)

pyxel.show()

