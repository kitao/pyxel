<!-- This file is generated from web/user-guide/cube/index.html and web/user-guide/cube/user-guide.json. -->

# Pyxel Cube User Guide

*[Read online in other languages](https://kitao.dev/pyxel/user-guide/cube/).*

## What Is Pyxel Cube?

**Pyxel Cube** is a submodule providing the 3D features included in Pyxel 3.0 and later. Import `pyxel.cube` to combine them with the existing 2D features and create retro 3D games.

For installation and basic usage, see the [Pyxel User Guide](https://kitao.dev/pyxel/user-guide/).

## Running Examples

To run the examples locally, enter the following command in a terminal. It creates a `pyxel_examples` directory in the current directory and copies the examples into it. If a directory with that name already exists, it will be replaced, so move any edited files elsewhere first.

```sh
pyxel copy_examples
```

Change to the `cube` directory inside the copied examples and pass the filename of the example you want to run to `pyxel run`.

```sh
cd pyxel_examples/cube
pyxel run c01_hello_cube.py
```

The examples are listed under “Cube Examples” in [Pyxel Showcase](https://kitao.dev/pyxel/showcase/). Each example can also run in your browser.

## Basic Usage

The basic steps for using Pyxel Cube are as follows.

### 1. Import Pyxel Cube Classes

Import the classes you need from `pyxel.cube`.

```python
import pyxel
from pyxel.cube import Node, Vec3
```

### 2. Define Nodes

Subclass `Node` and define the node's update and drawing behavior.

### 3. Create a Node Tree

Create node instances from your classes and add them as children of a parent node. This structure of nodes connected by parent-child relationships is called a **node tree**.

### 4. Update and Draw the Node Tree

Update and draw the node tree from your app's `update` and `draw` methods. This runs each registered node's processing and renders the view from the camera onto Pyxel's 2D screen.

## Coordinates and Transforms

This section explains how to specify the position, orientation, and size of objects in 3D space.

### Vectors ([Vec3](https://kitao.dev/pyxel/api-reference/cube/#vec3) Class)

A **vector** is a quantity with a direction and length. The `Vec3` class stores the X, Y, and Z components of a 3D vector.

To specify a position or displacement, create a vector by passing values in X, Y, Z order, as in `Vec3(x, y, z)`. Read its components through the `x`, `y`, and `z` properties.

```python
from pyxel.cube import Vec3

pos = Vec3(30, 20, 5)
print(pos.x, pos.y, pos.z)  # 30.0 20.0 5.0
```

The components are read-only and cannot be changed with an expression such as `pos.x += 10`. To increase the X component by 10, use `pos += Vec3(10, 0, 0)`, which assigns the new `Vec3` produced by the operation.

Pyxel Cube normally uses **+X for right, +Y for up, and −Z for forward**, with X and Z for movement along the ground. You can choose different axis conventions to suit your camera settings and application.

The Cube examples in [Pyxel Showcase](https://kitao.dev/pyxel/showcase/) treat a length of 100 as 1 m and 1 as 1 cm. At this scale, `Vec3(30, 20, 5)` is a position 30 cm along X, 20 cm along Y, and 5 cm along Z from the origin.

![An oblique view of the coordinate axes in 3D space. The point (30, 20, 5) lies 30 along X and 5 along Z across the ground, then 20 up along Y, from the origin O.](https://kitao.dev/pyxel/user-guide/cube/images/coordinates.svg)

On-screen size depends on the camera's position and settings, so you can choose units of length in 3D space to suit your application.

To move an object, add a displacement to its position. The following example moves a bullet toward a target.

```python
pos = Vec3(0, 0, 0)
target = Vec3(40, 0, -30)
direction = (target - pos).normalize()
pos += direction * 2
```

`target - pos` is a vector giving the direction and distance from the current position to the target. The example uses `normalize` to set its length to 1 while preserving its direction, multiplies it by the movement distance of 2, and adds it to `pos`.

### Transform Matrices ([Mat4](https://kitao.dev/pyxel/api-reference/cube/#mat4) Class)

A **transform matrix** is a matrix that applies a **transform**, such as translation, rotation, or scaling, to coordinates. The `Mat4` class stores a transform matrix for 3D space.

The following example creates a matrix that rotates 45 degrees around the Y axis, then translates 30 along X.

```python
from pyxel.cube import Mat4, Vec3

rotation = Mat4.from_euler(Vec3(0, 45, 0))
translation = Mat4.from_translation(Vec3(30, 0, 0))
transform = translation * rotation
```

Pass rotation angles in degrees around the X, Y, and Z axes to `from_euler`, and a displacement to `from_translation`. Combine matrices with `*`: `translation * rotation` applies the rotation on the right first, followed by the translation on the left.

For scaling, use `Mat4.from_scale` and specify the X, Y, and Z scale factors with a `Vec3`. The following example creates a matrix that doubles the width, height, and depth.

```python
scale = Mat4.from_scale(Vec3(2, 2, 2))  # Double the size
```

The components of `Mat4` cannot be changed directly either. Assign the result of an operation or transform method to a variable or property.

### Quaternions ([Quat](https://kitao.dev/pyxel/api-reference/cube/#quat) Class)

A **quaternion** is a value representing a rotation in 3D space. The `Quat` class stores a rotation as a quaternion.

For **interpolation**—finding an intermediate orientation between two orientations—use `slerp`. Convert the resulting rotation to a transform matrix with `Mat4.from_quat` to combine it with translation or scaling.

The following example finds the halfway orientation between no rotation and a 90-degree rotation around the Y axis.

```python
from pyxel.cube import Mat4, Quat, Vec3

start = Quat.IDENTITY
target = Quat.from_euler(Vec3(0, 90, 0))
rotation = start.slerp(target, 0.5)
transform = Mat4.from_quat(rotation)
```

The second argument to `slerp` is the interpolation fraction: 0 gives the starting orientation and 1 gives the target orientation. This example uses 0.5 to obtain a 45-degree rotation around the Y axis. Gradually increasing the fraction from 0 to 1 changes the orientation smoothly.

## Nodes

In Pyxel Cube, an instance of the [`Node`](https://kitao.dev/pyxel/api-reference/cube/#node) class is called a **node**. A node lets you set an object's position, orientation, and size; connecting nodes through parent-child relationships lets you move multiple objects together.

### Defining Nodes

Subclass `Node`, putting update processing in `on_update` and drawing processing in `on_draw`. Set position, orientation, and size by assigning a matrix to `transform`.

The following example defines an `Item` class that draws a box-shaped item rotating around the Y axis.

```python
import pyxel
from pyxel.cube import Mat4, Node, Vec3


class Item(Node):
    def on_update(self):
        angle = pyxel.frame_count
        self.transform = Mat4.from_euler(Vec3(0, angle, 0))

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(16, 16, 16), 11)
```

In this example, `on_update` rotates the item using the elapsed frame count as the angle in degrees around the Y axis.

In this example, `on_draw` calls `box` with a `Vec3` specifying width, height, and depth, along with color index 11, to draw a box. The first argument is `Mat4.IDENTITY`, a matrix that applies no additional transform, so the box is drawn at the node's position and orientation.

To double the rotation speed, edit `Item.on_update`: change `angle = pyxel.frame_count` to `angle = pyxel.frame_count * 2`. In `Item.on_draw`, adjust the size with `Vec3(16, 16, 16)`.

In the [c01_hello_cube.py](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c01_hello_cube.py) example, `Cube.on_update` calculates a cube's rotation and movement around a circle and assigns the result to `transform`.

### Creating, Updating, and Drawing a Node Tree

The topmost node in a node tree is called the **root node**. First create a root node, then add the nodes for your objects as its children. To add a child node, call the parent node's `add_child` method with the child as its argument.

Calling the root node's `update` from your app's `update` runs `on_update` for each node, including its children. Similarly, calling the root node's `draw` from your app's `draw` runs each node's `on_draw` and renders the view from the camera onto Pyxel's 2D screen.

Add the following code after the `Item` definition in the previous section to display a rotating box.

```python
from pyxel.cube import Camera

# Other imports and the Item definition are omitted here; see above.


class App:
    def __init__(self):
        pyxel.init(160, 120)
        self.root = Node()
        self.root.add_child(Item())
        self.root.camera = Camera()
        self.root.camera.transform = Mat4.look_at(Vec3(0, 24, 48), Vec3.ZERO)
        pyxel.run(self.update, self.draw)

    def update(self):
        self.root.update()

    def draw(self):
        pyxel.cls(0)
        self.root.draw(0, 0, pyxel.width, pyxel.height)


App()
```

This example creates a root node named `self.root` that does not draw a shape, and adds an `Item` instance that draws a box as its child. The `Camera` uses the constant `Vec3.ZERO`, which denotes the origin, as its target position, viewing the box from slightly above.

In `draw(x, y, w, h)`, `x, y` specify the top-left corner of the drawing area, and `w, h` specify its width and height. All four values are in **2D screen pixels**.

The example above draws across the entire screen. Changing the call to `self.root.draw(16, 16, 80, 60)` draws in an area with its top-left corner at (16, 16), a width of 80 pixels, and a height of 60 pixels.

Set the `target` argument to a destination [`Image`](https://kitao.dev/pyxel/api-reference/#image) to draw into an image instead of the screen.

### Parent-Child Relationships and Transforms

Setting a child node's `transform` transforms it relative to its parent. You can move the parent and child together while also controlling the child's movement independently.

Coordinates relative to a node are called **local coordinates**; coordinates relative to the entire 3D space are called **world coordinates**. `node.world_transform` returns the transform matrix in world coordinates, including the parent's transform.

### Controlling Updates and Visibility

`node.active = True` enables updates and collision detection; `node.active = False` disables them. If updates are disabled for a parent, its children and all nodes below them also skip updates and collision detection. This does not change the children's `active` values or whether drawing is enabled.

`node.visible = True` makes a node visible; `node.visible = False` hides it. If a parent is hidden, its children and all nodes below them are not drawn either. This does not change the children's `visible` values or whether updates and collision detection are enabled.

### Destroying and Detaching Nodes

`node.destroy()` marks this node and all nodes below it as destroyed. At the end of update processing, it calls each node's `on_destroy` and removes their parent-child relationships. Python instances remain as long as references to them exist.

Calling the parent node's `remove_child(node)` detaches a node without destroying it. You can add it as a child again with `add_child(node)`.

## Cameras and Shading

This section explains how to set the viewpoint for displaying 3D space on the screen and how to shade objects.

### Cameras ([Camera](https://kitao.dev/pyxel/api-reference/cube/#camera) Class)

A **camera** is a viewpoint into 3D space. The `Camera` class stores the viewpoint's position, orientation, and viewing range settings.

Create a `Camera`, assign a matrix specifying its position and orientation to `transform`, and assign the camera to the root node's `camera` property.

The following example uses `Mat4.look_at(camera position, target position)` during app initialization to create a matrix for looking at the origin from (0, 78, 104). Both positions are specified in world coordinates.

```python
from pyxel.cube import Camera

camera = Camera()
camera.transform = Mat4.look_at(Vec3(0, 78, 104), Vec3.ZERO)
self.root.camera = camera
```

`Camera()` defaults to **perspective projection**: objects of the same size appear smaller farther from the camera. The camera's `fov` specifies the vertical viewing angle, which defaults to 60 degrees.

Setting the camera's `ortho_size` to a viewing height in **3D space units** switches to **orthographic projection**, where distance does not affect apparent size. The following example shows a height of 100.

```python
self.root.camera.ortho_size = 100
```

The viewing width is determined by the specified height and the drawing area's aspect ratio. Set `self.root.camera.ortho_size = None` to return to perspective projection.

Objects too close to or too far from the camera are not drawn. Set `near` and `far` to the distances from the camera to the near and far limits of the visible range.

Set the background color with `camera.clear_color`. When you specify a color index, the entire drawing area specified by the node's `draw` method is filled with that color before the 3D scene is drawn. Setting it to `None` leaves the background unfilled, allowing 3D to be drawn over an existing 2D background.

To make the viewpoint follow a target, calculate the camera position from the target's position each frame and assign `Mat4.look_at(camera position, target position)` to the camera's `transform`.

In the [c05_3d_collision.py](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c05_3d_collision.py) example, `App.update_camera` moves the camera to follow the player.

### Shading ([Shading](https://kitao.dev/pyxel/api-reference/cube/#shading) Class)

**Shading** adds light and shade based on the orientation of surfaces and the direction of light. The `Shading` class stores the light direction and color settings used for shading.

Passing Pyxel's drawing color list, `pyxel.colors`, when creating a `Shading` automatically generates a shading palette. The following example sets the light direction during app initialization and assigns this `Shading` to the root node's `shading` property.

```python
from pyxel.cube import Shading

shading = Shading(pyxel.colors)
shading.direction = Vec3(0.4, -1, -0.5)
self.root.shading = shading
```

`direction` is the direction in which light travels, in world coordinates. In the example above, the light shines diagonally from above.

Children are also drawn with their parent's `Shading`. Assign a different `Shading` to a child to draw it with those settings instead.

Without shading, objects are drawn in the specified color or the image's colors. To disable shading for some drawing operations, use the following calls within the node's `on_draw`.

```python
self.shaded(False)
self.box(Mat4.IDENTITY, Vec3(16, 16, 16), 11)
```

The `shaded` setting applies only to subsequent drawing by that node. Shading is enabled again at the start of the next `on_draw` call.

### Shading Palette

A **shading palette** is a two-dimensional table of drawing colors for each original color index and surface brightness level. Read and change its entries with `shading[color index, brightness]`.

Each color has eight levels, from 0 (darkest) to 7 (brightest). A level is selected from the surface orientation when drawing. Each level specifies two color indices: identical indices give a solid color, while different indices produce **dithering**, a checkerboard pattern of the two colors.

In an automatically generated palette, level 5 is set to the original color. For example, `shading[8, 5]` is `(8, 8)`. Levels 0-4 darken the color, while levels 6-7 brighten it.

The following example changes the colors used for dark surfaces with color index 8.

```python
shading = self.root.shading
shading[8, 0] = (2, 2)  # Set the darkest level to color 2
shading[8, 1] = (2, 8)  # Use two colors for dithering at the next level
```

After changing the colors in `pyxel.colors`, rebuild the shading palette with `shading.build(pyxel.colors)`. This also overwrites manual edits, so apply them after rebuilding.

## Drawing 3D Shapes

In Pyxel Cube, you draw shapes by calling 3D drawing methods inside a node's `on_draw`, much like drawing in 2D with Pyxel.

### Using Drawing Methods

Specify coordinates in 3D space instead of 2D screen coordinates. For example, the `line` method takes the positions of both endpoints as `Vec3` values.

```python
pyxel.line(0, 0, 30, 20, 8)  # 2D
self.line(Vec3(0, 0, 0), Vec3(30, 20, 0), 8)  # 3D
```

Specify shape coordinates and dimensions in 3D space units. Positions given as `Vec3` values are in the node's local coordinates, and transforms given as `Mat4` values are also applied relative to the node. On-screen position and size depend on the camera's position and settings. The following table lists the drawing methods and how to specify position and orientation.

| Content | Method | Position | Orientation |
| --- | --- | --- | --- |
| Point | `pset` | Point as a `Vec3` | — |
| Line | `line` | Endpoints as `Vec3` | Determined by endpoints |
| Triangle | `tri`, `trib` | Three vertices as `Vec3` | Determined by vertices |
| Rectangle | `rect`, `rectb` | `Mat4` | Specified by matrix |
| Circle | `circ`, `circb` | Center as a `Vec3` | Faces camera |
| Ellipse | `elli`, `ellib` | `Mat4` | Specified by matrix |
| Box | `box`, `boxb` | `Mat4` | Specified by matrix |
| Sphere | `sphere`, `sphereb` | Center as a `Vec3` | — |
| Plane with an image | `plane` | `Mat4` | Specified by matrix |
| Camera-facing image (billboard) | `sprite` | Position as a `Vec3` | Faces camera |
| Custom shape | `prim` | `Mat4` | Specified by matrix |
| Text | `text` | Position as a `Vec3` | Faces camera |

Methods whose names end in `b` draw outlines only. For example, `boxb` draws only the edges of a box.

### Drawing with Images

You can color 3D shapes with a color index or apply an image to their surfaces. An image applied to a surface is called a **texture**; specify one by passing an image created with Pyxel's [`Image`](https://kitao.dev/pyxel/api-reference/#image) class to a drawing method. For example, passing an image instead of a color index to `box` applies the entire image to each face.

The following example loads a prepared image, `assets/texture.png`, during node initialization and applies it to each face of a box in `on_draw`. Relative file paths are based on the directory of the Python file that calls `pyxel.init`.

```python
# In __init__
self.texture = pyxel.Image.from_image("assets/texture.png")
```
```python
# In on_draw
self.box(Mat4.IDENTITY, Vec3(30, 30, 30), self.texture)
```

`plane` suits signs on walls, while `sprite` suits pixel-art characters, explosions, and similar effects.

To apply the same image to a plane with a width and height of 30, use the following code in `on_draw` to call `plane`. `uvs` gives the image coordinates corresponding to the plane's top-left, top-right, bottom-left, and bottom-right corners, with (0, 0) at the image's top-left and (1, 1) at its bottom-right. Change this range to use only part of the image.

```python
uvs = ((0, 0), (1, 0), (0, 1), (1, 1))
self.plane(Mat4.IDENTITY, self.texture, uvs, 30, 30)
```

To keep the image facing the camera, replace the `plane` call above with the following `sprite` call. Images drawn with `sprite` are not shaded.

```python
self.sprite(Vec3.ZERO, self.texture, uvs, 30, 30)
```

To make a particular color transparent, pass its color index as the drawing method's `colkey` argument.

In the [c02_basic_shapes.py](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c02_basic_shapes.py) example, `PlaneShape` uses `plane` and `SpriteShape` uses `sprite`.

### Shape Data ([Primitive](https://kitao.dev/pyxel/api-reference/cube/#primitive) Class)

**Shape data** describes vertex positions and the lines or faces that connect them. The `Primitive` class stores shape data made up of points, lines, or triangles.

To define a custom shape, pass a drawing mode, a list of vertex coordinates, and a list of vertex indices to `Primitive(mode, positions, indices)`. The `mode` argument selects whether the vertices are drawn as points, lines, or triangles. `positions` is a flat list of vertex coordinates in the form `[x0, y0, z0, x1, y1, z1, ...]`. For triangles, for example, set `mode` to `Primitive.MODE_TRIANGLES` and supply three vertex indices per face in `indices`.

Helper methods such as `Primitive.plane` and `Primitive.box` generate vertex data for planes and boxes and return a `Primitive` instance.

The following example creates a plane with a width and height of 20 during node initialization, then increases `positions[1]`, the Y coordinate of the first (top-left) vertex, by 10.

```python
from pyxel.cube import Primitive

self.shape = Primitive.plane(20, 20)
self.shape.positions[1] += 10
self.shape.compute_normals()
```

After changing the shape, the example calls `compute_normals` to update the surface orientations used in shading calculations.

The code changes the plane's shape as shown below.

![The plane's top-left vertex moves from (-10, 10, 0) to (-10, 20, 0). Its displacement is (0, 10, 0); the other vertices do not move.](https://kitao.dev/pyxel/user-guide/cube/images/primitive.svg)

Inside the node's `on_draw`, pass the generated shape to `prim` to draw it. Specify the color or image here as well; the following example draws with color index 8 (red in the default palette).

```python
self.prim(Mat4.IDENTITY, self.shape, 8)
```

In the [c03_custom_shapes.py](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c03_custom_shapes.py) example, `on_draw` changes `positions` each frame to deform the shape.

### Drawing Text

To draw text at a position in 3D space from the node's `on_draw`, call the 3D `text` method. The following example draws “ENEMY” at the node's local coordinates (0, 20, 0). Text size does not change with distance from the camera, but other objects can hide parts of the text behind them.

```python
self.text(Vec3(0, 20, 0), "ENEMY", 7)
```

### Combining 3D and 2D

Use Pyxel's 2D drawing methods to overlay scores or instructions on a 3D scene. Positions are specified in screen coordinates, so they stay in place when the 3D camera moves. In your app's `draw`, call the 2D drawing methods after drawing the node tree.

```python
pyxel.cls(0)
self.root.draw(0, 0, pyxel.width, pyxel.height)
pyxel.text(8, 8, "SCORE: 0", 7)
```

The front-to-back order of 3D objects is determined by depth. In 2D drawing, later drawing operations appear on top of earlier ones.

## 3D Models and Motions

This section explains how to load 3D models and motions created in external tools, display the models, and play the motions.

### 3D Models ([Mesh](https://kitao.dev/pyxel/api-reference/cube/#mesh) Class)

A **3D model** describes a three-dimensional object, such as a character or terrain, through shapes, images, colors, and other data. The [`Mesh`](https://kitao.dev/pyxel/api-reference/cube/#mesh) class stores this data and the parent-child relationships between parts.

**GLB (binary glTF 2.0)** is a file format for storing 3D models and animations. Load a GLB file with `Mesh.from_glb` to create a `Mesh`.

To create a node tree for drawing a `Mesh`, pass that `Mesh` to `Node.from_mesh`. The generated nodes retain the parent-child relationships of the parts.

The following example runs during app initialization, after creating the root node `self.root`. It loads a prepared `assets/character.glb` file and adds the generated node tree as a child of `self.root`.

```python
from pyxel.cube import Mesh

mesh = Mesh.from_glb("assets/character.glb")
actor = Node.from_mesh(mesh)
self.root.add_child(actor)
```

Here, `actor` is the parent of all the parts, returned by `Node.from_mesh`. Assign a transform matrix to `actor.transform` to adjust the position, orientation, and size of the entire model. You can also create multiple node trees from the same `Mesh` and position each one independently.

GLB support and the authoring workflow are covered later in [Creating Resources for Cube](https://kitao.dev/pyxel/user-guide/cube/#s-resources).

### Motions ([Motion](https://kitao.dev/pyxel/api-reference/cube/#motion) Class)

A **motion** is the movement of a 3D model's parts. The `Motion` class stores changes in the parts' positions, orientations, and sizes over time.

Animations loaded from a GLB file are stored as `Motion` objects in `mesh.motions`. For a GLB without animations, `mesh.motions` is an empty list. To play a motion, take a `Motion` from the `Mesh` used to create the node and pass it to that node's `play_motion` method.

If the GLB above contains an animation named `walk`, add the following code after loading it to play that animation.

```python
for motion in mesh.motions:
    if motion.name == "walk":
        actor.play_motion(motion)
        break
```

Playback updates the `transform` of each part's node. It does not affect `actor.transform`, which positions the model as a whole.

`play_motion` loops by default; `loop=False` plays the motion once. To play at the original speed, set `Mesh.from_glb`'s `fps` argument to the same value as `pyxel.init`'s `fps` argument, because GLB times are converted from seconds to frames when loaded.

In the [c04_mesh_and_motion.py](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c04_mesh_and_motion.py) example, `play` switches which `Motion` is played.

## Physics Engine

This section explains how to use the physics engine for falling under gravity, bouncing on collision, and rolling.

### Colliders ([Collider](https://kitao.dev/pyxel/api-reference/cube/#collider) Class)

A **collider** is a shape used to detect collisions with an object. The `Collider` class stores this shape and physical properties such as mass and friction.

Create a collider, such as a sphere, capsule, or box, separately from the shape you draw, and assign it to the node's `collider` property.

The following example creates a box collider with a width, height, and depth of 26 and a mass of 1 during node initialization.

```python
from pyxel.cube import Collider

self.collider = Collider(size=Vec3(26, 26, 26), mass=1)
```

For sphere, capsule, and box colliders, calling the root node's `update` moves the node according to velocity and gravity. The collider's `mass` is its mass, and `velocity` is its displacement per update in world coordinates. `gravity` is the gravitational acceleration in m/s² and defaults to 9.8. Physics calculations treat a length of 100 in 3D space as 1 m, so adjust `gravity` if you choose different units of length.

Set `trigger=True` to use a collider only for detecting contact. Some example settings are:

- `mass=0`: floors that are not pushed by collisions, or platforms moved by code.
- `gravity=0`: characters whose movements, such as jumping, you control yourself.
- `mass=0, trigger=True`: areas that only detect contact, such as coins or goals.

To detect collisions against terrain geometry, set the collider's `mesh` property to a `Mesh`. A collider with a `Mesh` does not move under velocity or gravity; move or rotate it by changing the node's `transform`.

For objects driven by velocity or gravity, use spheres, capsules, or boxes instead. Apply only translation and rotation to nodes with these colliders and to their parents. Apply visual scaling to child nodes.

### Moving Characters

To move a character with keyboard input, use the node's `on_update` to change its collider's `velocity`.

The following example moves a node with a box collider along X using the left and right arrow keys. It leaves the Y velocity unchanged so the node continues falling under gravity.

```python
vx = 0
if pyxel.btn(pyxel.KEY_RIGHT):
    vx += 2
if pyxel.btn(pyxel.KEY_LEFT):
    vx -= 2
self.collider.velocity = Vec3(vx, self.collider.velocity.y, 0)
```

Handling landings and collisions with walls is covered later in [Responding to Collisions](https://kitao.dev/pyxel/user-guide/cube/#s-physics-response).

In the [c05_3d_collision.py](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c05_3d_collision.py) example, `Player` implements movement, jumping, and coin collection.

### Responding to Collisions

When colliders touch, the node's `on_collide(other, contact)` is called with the other node and contact information ([`Contact`](https://kitao.dev/pyxel/api-reference/cube/#contact)). In the contact information, `normal` is the direction of separation in world coordinates, and `depth` is the amount needed to correct the overlap.

Separation and bouncing are not applied automatically. In `on_collide`, adjust the node's position and the collider's velocity and angular velocity. The contact's `delta_velocity` and `delta_angular_velocity` give the collision corrections to velocity and angular velocity. The following example assumes that the parent node's `world_transform` is `Mat4.IDENTITY`.

```python
def on_collide(self, other, contact):
    if self.collider.mass > 0:
        shift = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = shift * self.transform
        self.collider.velocity += contact.delta_velocity
        self.collider.angular_velocity += contact.delta_angular_velocity
```

Use the collider's `restitution` to adjust bounciness and `friction` to adjust resistance to sliding. Set `rolls=True` to calculate rotation caused by collisions as well.

In the [c06_3d_physics.py](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c06_3d_physics.py) example, `on_collide` implements bouncing and rolling.

### Raycasts and Overlap Queries

To find objects, use a **raycast** to find what a line extending through space hits and where it hits, or an **overlap query** to find objects overlapping a sphere or box. For example, casting a line downward from a character's position locates the ground where its shadow should be drawn.

Filter the search with **tags**, strings attached to nodes to classify them. First tag the ground nodes with `node.tags.add("ground")`. The following example calls the parent node's `raycast` to search downward for the ground from the object's position. It passes the object's world position, `self.world_transform.pos`, as the starting point.

```python
hit = self.parent.raycast(self.world_transform.pos, Vec3.DOWN, tags={"ground"})
```

This example searches the colliders of nodes tagged `ground` within the parent node and its descendants.

A hit returns a [`RaycastHit`](https://kitao.dev/pyxel/api-reference/cube/#raycasthit), and `hit.point` gives the hit position. If nothing is hit, the result is `None`. `raycast_all` returns a result for each collider hit, ordered by distance.

Overlap queries take a search region in world coordinates. `overlap_sphere` takes a center position and radius, while `overlap_box` takes a transform matrix and size. They return a list of nodes whose colliders overlap that region.

## Creating Resources for Cube

Pyxel Cube loads source data for `Mesh` and `Motion` in GLB (binary glTF 2.0) format. This section covers GLB support and how to create and export resources with the recommended tool, [Blockbench](https://www.blockbench.net/).

### GLB Support

GLB loading supports the following features and has the following limitations.

- **Geometry and hierarchy**

  Triangle geometry, parent-child relationships between parts, and translation, rotation, and scaling are supported. Loading fails if the geometry uses a drawing mode other than triangles.
- **Colors and textures**

  GLB stores surface colors, textures, and related settings as **materials**. Pyxel Cube can load multiple materials and use each material's base color (the basic surface color) and image. If there are multiple sets of image coordinates, only the first set (`TEXCOORD_0`) is loaded.

  Colors absent from `pyxel.colors` are converted to similar colors when loaded. To use a custom palette, assign a list of RGB values for 1-256 colors to `pyxel.colors[:]` before loading.

  To cut out transparent areas of an image, set the material's `alphaMode` to `MASK`. However, cutout transparency is disabled if no unused index can be reserved for transparency after the loaded GLB's colors are converted to Pyxel color indices. Translucency with `BLEND`, per-vertex colors (vertex colors), normal maps that use images to represent surface bumps, and material metallic and roughness settings are not applied.
- **Motions**

  Keyframe animations that record parts' positions, orientations, and sizes at specific times are supported. GLB's `LINEAR`, `STEP`, and `CUBICSPLINE` interpolation modes can be used between keyframes.

  Skinning (deformation driven by a skeleton) and morphing (deformation by moving vertices) are not applied, so create arms, legs, and similar elements as separate parts and move those parts. GLB files containing morph animations cannot be loaded. For files using the glTF extension `KHR_animation_pointer`, all animations are excluded from loading.
- **Cameras and lights**

  Camera and lighting settings in GLB files are not used; set them in Cube.
- **File requirements**

  GLB files that reference external files cannot be loaded. Embed geometry, animation data, and images in the GLB file.

### Creating Resources in Blockbench

Create a project in **Generic Model** format and build the shapes for your character or terrain. Put moving parts in separate groups.

To paint textures with Pyxel's default colors, set up the palette as follows.

1. Save the [Pyxel palette (pyxel.gpl)](https://kitao.dev/pyxel/assets/pyxel.gpl).
2. In **Paint** mode, select **Import Palette** in the **Palette** panel and import it with **Replace old palette** enabled.
3. Right-click an empty area of the palette and enable **Lock Palette** to prevent adding, deleting, or dragging colors to reorder them.
4. Select colors from the palette and paint with 100% opacity and 0 softness.

**Lock Palette** does not restrict the colors you can paint with. For a custom palette, configure Blockbench with the same colors as `pyxel.colors`.

To create motions, set keyframes for each part's translation, rotation, and scaling in **Animate** mode.

For details on the authoring tools, see the [official Blockbench guide](https://www.blockbench.net/wiki/guides/blockbench-overview-tips/).

### Exporting to GLB

Save the editable project as .bbmodel, and export the data for Pyxel Cube as GLB.

```text
Blockbench (.bbmodel) → Export (.glb) → Mesh.from_glb()
```

Choose **File → Export → Export glTF Model** and export as **Binary (glb)**.

- **Embed Textures**: enabled
- **Export Groups as Armature**: disabled
- **Export Animations**: enabled when including motions

These settings embed textures and export each part's translation, rotation, and scaling as motions.

If a GLB material is exported with `BLEND`, transparent areas are drawn as opaque. To use cutout transparency, install the glTF Tools extension in VS Code and edit the material as follows.

1. Open the GLB, right-click its tab, and select `glTF: Import from GLB` to convert it to an editable glTF.
2. In the relevant material under `materials`, change `alphaMode` to `"MASK"` and save.
3. Right-click the glTF tab and select `glTF: Export to GLB (Binary file)` to export a GLB.

For details on conversion and editing, see [Using glTF Tools in VS Code](https://github.com/KhronosGroup/glTF-Tutorials/blob/main/AddingMaterialExtensions/AddingMaterialExtensions_002_UsingVisualStudioCode.md).

### Terrain for Rendering and Collisions

For complex terrain, creating a separate collision `Mesh` with simplified walkable surfaces and walls makes it easier to adjust collisions independently of the visible geometry.

When assembling terrain by placing and duplicating parts, the [Cube Model References](https://github.com/kitao/pyxel/tree/main/python/pyxel/examples/cube/tools) plugin for Blockbench can export separate GLB files for rendering and collisions.

### Images, Sounds, and Music

Image, sound, and music resources are shared with Pyxel. For creation and loading instructions, see [How to Create Resources in the Pyxel User Guide](https://kitao.dev/pyxel/user-guide/#s-resources).

## Distributing Applications

Pyxel Cube applications are distributed in the same way as Pyxel applications. See [How to Distribute Applications in the Pyxel User Guide](https://kitao.dev/pyxel/user-guide/#s-dist) for instructions, and [Using Pyxel on the Web](https://kitao.dev/pyxel/web-usage/) for embedding them in web pages.

## API Reference

See the [API Reference](https://kitao.dev/pyxel/api-reference/) for details on classes and arguments. Select **Cube** at the top of the page to switch to the Pyxel Cube API.

## Links

- [Pyxel User Guide](https://kitao.dev/pyxel/user-guide/#s-links): related manuals, tools, and communities, plus licensing and contribution information.
- [Frequently Asked Questions](https://github.com/kitao/pyxel/blob/main/docs/faq-ja.md): questions about usage and environments.
