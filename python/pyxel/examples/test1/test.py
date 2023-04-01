import pyxel

pyxel.init(80,80)
pyxel.load("test_res.pyxres")

x=0
y=0
def update():
    global x,y
    x = pyxel.mouse_x
    y = pyxel.mouse_y
    return

def draw():
    pyxel.cls(1)
    pyxel.blt(x,y,0,0,0,8,8)
    return

#right
#pyxel.blt(35,5,0,8,0,8,8)
#left
#pyxel.blt(50,5,0,0,8,8,8)
#enemy
#pyxel.blt(5,15,0,24,0,8,8,9)
#enemy missle
#pyxel.blt(5,30,0,24,8,8,8,9)

pyxel.run(update,draw)