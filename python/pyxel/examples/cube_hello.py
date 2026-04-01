import math
import os

import pyxel
from pyxel import cube

pyxel.init(256, 200, title="Pyxel3D Hello", fps=30)

assets = os.path.join(os.path.dirname(__file__), "assets", "sample.pyxres")
pyxel.load(assets)

scene = cube.Scene()
scene.set_light(0, cube.Light(cube.Vec3(-1.0, -0.5, -1.0)))

S = 0.0625

shapes = [
    cube.Model.tex_cube(0, 0, 0, S, S),
    cube.Model.tex_pyramid(0, S, 0, S, S),
    cube.Model.tex_sphere(0, S * 2, 0, S, S),
    cube.Model.tex_cube(0, S * 3, 0, S, S),
    cube.Model.tex_pyramid(0, 0, S, S, S),
    cube.Model.tex_sphere(0, S, S, S, S),
    cube.Model.cube(8),
    cube.Model.tex_cube(0, S * 2, S, S, S),
    cube.Model.pyramid(10),
    cube.Model.tex_sphere(0, S * 3, S, S, S),
    cube.Model.sphere(11),
    cube.Model.tex_pyramid(0, 0, S * 2, S, S),
]

cam = cube.Camera(
    cube.Vec3(0.0, -10.0, 7.0),
    cube.Vec3(0.0, 0.0, -1.0),
    fov=50.0,
)
t = 0.0
LOOP_FRAMES = 210
frame = 0


def update():
    global t, frame
    t += 0.03
    frame += 1
    scene.remove_all()
    for i, shape in enumerate(shapes):
        a = (2.0 * math.pi / len(shapes)) * i + t
        r = 4.0 + math.sin(t * 2.0 + i) * 1.2
        x = math.cos(a) * r
        y = math.sin(a) * r
        z = math.sin(t * 3.0 + i * 0.7) * 1.5
        spin = t * 120.0 + i * 30.0
        scene.add(
            shape,
            pos=cube.Vec3(x, y, z),
            rot=cube.Vec3(spin * 0.7, spin * 0.5, spin),
            scale=cube.Vec3(1.5, 1.5, 1.5),
        )
    if frame == LOOP_FRAMES:
        pyxel.screencast()
        pyxel.quit()


def draw():
    pyxel.cls(1)
    scene.draw(0, 0, 256, 200, cam)
    msg = "Hello, Pyxel!"
    pyxel.text(128 - len(msg) * 2 + 2, 97, msg, 7)


pyxel.run(update, draw)
