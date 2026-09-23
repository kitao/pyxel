<!-- This file is generated from web/user-guide/cube/index.html and web/user-guide/cube/user-guide.json. -->

# Pyxel Cube User Guide

*[Read online in other languages](https://kitao.github.io/pyxel/web/user-guide/cube/).*

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
        self.camera.transform = Mat4.look_at(Vec3(0.0, 78.0, 104.0), Vec3.ZERO)

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(26.0, 26.0, 26.0), 11)


pyxel.init(200, 150)
scene = Scene()


def update():
    scene.update()


def draw():
    scene.draw(0, 0, pyxel.width, pyxel.height)


pyxel.run(update, draw)
```

## Creating Models in Blockbench

First, download the Pyxel GIMP palette file [pyxel.gpl](https://kitao.github.io/pyxel/docs/pyxel.gpl). Use this palette when creating or editing models for the Cube examples.

- Create a Generic Model or open a sample .bbmodel, then switch to Paint. Choose Import Palette in the Palette panel, select pyxel.gpl, and enable Replace Palette. Check that the usual 16 Pyxel colors appear in their original order before painting.
- Edit with these 16 colors. The .bbmodel files are the editable projects; .glb files are for loading in Pyxel Cube. For c05, extract garden_models.zip in cube/assets and open a .bbmodel inside. The palette is a Blockbench setting, so import pyxel.gpl again when using a different installation.
- Use File > Export > Export glTF Model. Select Binary (glb), enable Embed Textures, and disable Armature. Enable Export Animations for animated models. Keep the saved scale when re-exporting a sample project.

## Building the c05 Stage

Extract garden_models.zip from cube/assets and open garden_parts.bbmodel. Build Example is an L-shaped island assembled from the same parts as garden_stage.bbmodel. Duplicate the part groups to build your own stage.

- Grass parts are 48 by 48 units and 32 units high, with their origin at the walking surface center. Stack Cliff parts below in 32-unit steps. Stacking instead of stretching preserves the texture dot size. A/B share the same shape but differ in pattern placement.
- N/E/S/W identify exposed sides (-Z/+X/+Z/-X). Choose the matching orientation rather than rotating the painted lighting. Place the matching Inner part at inside corners. Ramp rises 24 units over a 48-unit run; Ramp Tall has a deeper foundation. Bridge spans 48 units. Use Joint parts to fill the 1.5-unit side gaps where ramps meet islands.
- Copy visual parts into garden_stage.bbmodel and a matching-height box from Collision Columns into garden_stage_collision.bbmodel. Align their walking surfaces. Collision boxes cover whole columns without per-part divisions or decorative bevels. Export the two assembled models under their existing .glb names. The parts library itself is not used at runtime.
