<!-- This file is generated from web/user-guide/cube/index.html and web/user-guide/cube/user-guide.json. -->

# Pyxel Cube User Guide

*This document was auto-generated from the [Pyxel Cube User Guide](https://kitao.github.io/pyxel/web/user-guide/cube/) web page, which also offers multilingual support.*

## Overview

**Pyxel Cube** is the software-rendered 3D extension of Pyxel.

This guide is a work in progress; the Cube API is still being finalized.

## Getting Started

A minimal Cube program subclasses Node, sets up a camera, and draws inside the scene:

```python
import pyxel
from pyxel.cube import Camera, Mat4, Node, Vec3

class Scene(Node):
    def __init__(self):
        super().__init__()
        self.camera = Camera()
        self.camera.transform = Mat4.look_at(Vec3(0.0, 3.0, 4.0), Vec3.ZERO)

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(1.0, 1.0, 1.0), 11)

pyxel.init(200, 150)
scene = Scene()

def update():
    scene.update()

def draw():
    scene.draw(0, 0, pyxel.width, pyxel.height)

pyxel.run(update, draw)
```
