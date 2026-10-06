<!-- This file is generated from web/user-guide/cube/index.html and web/user-guide/cube/user-guide.json. -->

# Pyxel Cube User Guide

*[Read online in other languages](https://kitao.github.io/pyxel/web/user-guide/cube/).*

## Overview

**Pyxel Cube** is a submodule included in Pyxel 3.0 and later. Import `pyxel.cube` to use 3D features alongside the existing 2D features and easily create retro 3D games.

If you are new to Pyxel, first read the [Pyxel User Guide](https://kitao.github.io/pyxel/web/user-guide/) to set up your environment and learn the basics.

## Run the examples

The Pyxel Cube examples are in the `cube` directory inside `pyxel_examples`.

Run an example locally with the following commands.

```sh
cd pyxel_examples/cube
pyxel run c01_hello_cube.py
```

See “Cube Examples” in [Pyxel Showcase](https://kitao.github.io/pyxel/web/showcase/) for the example list. You can also run the examples in your browser.

## Basic usage

### Import Pyxel Cube classes

Import the classes you need from `pyxel.cube`.

```python
import pyxel
from pyxel.cube import Node, Vec3
```

### Define a node

Subclass `Node` and define how the node updates and draws.

### Create a node tree

Create node instances from your classes and add them to a node that groups them into a **node tree**.

### Update and draw the node tree

Update and draw the node tree from your application's `update()` and `draw()`. Each registered node runs its own processing, and the camera's view is drawn on Pyxel's 2D screen.

## Coordinates and transforms

### Vectors ([Vec3](https://kitao.github.io/pyxel/web/api-reference/cube/#vec3) class)

```python
from pyxel.cube import Mat4, Quat, Vec3
```

A **vector** represents a position or displacement with three values: X, Y, and Z. Pyxel Cube uses `Vec3`. The standard directions are **+X right, +Y up, −Z forward, and +Z backward**; ground movement uses X and Z. `Vec3.UP` is (0, 1, 0), and `Vec3.FORWARD` is (0, 0, −1). You can choose another up axis to suit your camera and game.

![Oblique view of the X, Y, Z axes and origin O. The point (30, 20, 5) is 30 along X and 5 along Z on the ground, then 20 upward along Y.](https://kitao.github.io/pyxel/web/user-guide/cube/images/coordinates.svg)

The examples use 100 world units per meter, so 1 unit is 1 centimeter and `Vec3(100, 0, 0)` is 1 meter along X. You can choose your own scale. A world length of 100 does not necessarily occupy 100 pixels: its screen size depends on the camera.

Add a displacement to a position to move an object. For example, move a projectile toward a target as follows.

```python
pos = Vec3(0, 0, 0)
target = Vec3(40, 0, -30)
direction = (target - pos).normalize()
pos += direction * 2
```

Subtract the current position from the target to find the direction, then use `normalize()` to make its length 1. Multiply by the speed to get the displacement.

- `pos.x`, `pos.y`, `pos.z`: the coordinates. To change them, assign a new `Vec3`.
- `Vec3.ZERO`: `Vec3(0, 0, 0)`.

### Transform matrices ([Mat4](https://kitao.github.io/pyxel/web/api-reference/cube/#mat4) class)

Changing position, orientation, or size by translation, rotation, or scaling is called a **transform**. A `Mat4` matrix combines these operations. This example rotates 45 degrees around Y, then moves 30 along X from the origin. You can pass this matrix to nodes and drawing methods later.

```python
rotation = Mat4.from_euler(Vec3(0, 45, 0))
translation = Mat4.from_translation(Vec3(30, 0, 0))
transform = translation * rotation
```

`from_euler()` specifies rotation angles around X, Y, and Z in degrees; `from_translation()` specifies displacement along each axis. Multiplication with `*` combines transforms. In `translation * rotation`, the right-hand rotation happens first, in place, then the left-hand translation changes the position.

```python
Mat4.from_scale(Vec3(2, 2, 2))  # Double the size
Mat4.IDENTITY  # No translation, rotation, or scaling
```

The components of `Vec3` and `Mat4` cannot be modified. Their methods return new values; assign the result to a variable or property.

### Rotations ([Quat](https://kitao.github.io/pyxel/web/api-reference/cube/#quat) class)

`Quat` represents only rotation, using a **quaternion**. For simple angles, use `Mat4.from_euler()`. To rotate smoothly from the current orientation toward a target, interpolate with `Quat`, then convert the result to a matrix.

```python
start = Quat.IDENTITY
target = Quat.from_euler(Vec3(0, 90, 0))
rotation = start.slerp(target, 0.5)
transform = Mat4.from_quat(rotation)
```

The second argument of `slerp()` controls progress: 0 is the start, 1 is the target, and 0.5 is halfway. You do not need to calculate the rotation components yourself.

## Nodes

In Pyxel Cube, an object placed in 3D space is a **node**. Nodes can also group several objects.

### Define a node ([Node](https://kitao.github.io/pyxel/web/api-reference/cube/#node) class)

Subclass `Node`, put movement and other update processing in `on_update()`, and put drawing processing in `on_draw()`. Setting a matrix on the node's `transform` affects its drawn position, orientation, and size.

The following example defines a box node that rotates around the Y axis.

```python
import pyxel
from pyxel.cube import Mat4, Node, Vec3


class Box(Node):
    def on_update(self):
        angle = pyxel.frame_count
        self.transform = Mat4.from_euler(Vec3(0, angle, 0))

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(16, 16, 16), 11)
```

This `on_update()` uses the elapsed frame count as a rotation angle in degrees around the Y axis to change the box's orientation.

`on_draw()` draws a box with side length 16 using color number 11. `Mat4.IDENTITY` draws using the node's position and orientation; the three `Vec3` values are width, height, and depth.

### Create and add nodes

Create a box node from the `Box` class and add it as a child of a parent node. Call the parent's `add_child()` method with the child node as its argument.

A structure of nodes connected through parent-child relationships is a **node tree**; its topmost node is the **root node**. The application below adds the box to a node that groups its children.

### Update and draw the node tree

Creating and adding nodes alone does not advance their processing. Call the root node's `update()` from the application's `update()` to update each node every frame. From the application's `draw()`, call the root node's `draw(x, y, w, h)` to draw the node tree's camera view on the 2D screen.

Append the following code to the `Box` definition above to display the box. `Camera` defines the viewpoint. Here it views the box at the origin from slightly above; the next chapter explains its settings.

```python
from pyxel.cube import Camera


class App:
    def __init__(self):
        pyxel.init(160, 120)
        self.root = Node()
        self.root.add_child(Box())
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

`root` is a variable name chosen for this example, so you can change it freely. A node used for grouping draws no shape of its own and groups its children's processing.

The arguments to `draw(x, y, w, h)` specify **where on the 2D screen to draw and how large the drawing area is**. `x, y` are the area's top-left coordinates; `w, h` are its width and height. All four values are in pixels.

The example above draws across the entire 2D screen, starting at its top-left corner (0, 0) and using `pyxel.width` and `pyxel.height`. For example, changing the call to `self.root.draw(16, 16, 80, 60)` draws in an area whose top-left corner is (16, 16), width is 80 pixels, and height is 60 pixels.

To change the box's size, change `Vec3(16, 16, 16)` in `Box.on_draw()`. To make it rotate faster, change `angle = pyxel.frame_count` in `on_update()` to `angle = pyxel.frame_count * 2`.

### Parent-child relationships and transforms

A child node's `transform` is relative to its parent. Moving or rotating the parent also moves the child. For example, adding tires as children of a car body lets you rotate the tires while moving the entire car.

Coordinates relative to the parent node are **local coordinates**; coordinates relative to the entire 3D space are **world coordinates**. `node.transform` is the matrix representing the transform relative to the parent. `node.world_transform` returns a matrix representing the transform in the entire 3D space, including the parent's transform.

### Visibility, updates, and deletion

| What you want to do | Node operation |
| --- | --- |
| Hide temporarily | `node.visible = False`. Drawing stops; updates and collisions continue. |
| Stop updates | `node.active = False`. Updates and collisions stop; drawing continues. |
| Delete bullets or enemies when finished with them | `node.destroy()`. At the end of update(), `on_destroy()` is called and the node is detached from the tree. |

Visibility and update settings, as well as deletion, also affect descendants. To detach a node from the tree for reuse, call its parent's `remove_child(node)`.

The `on_update()` in the [Pyxel Cube introduction](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c01_hello_cube.py) example shows how to move nodes ([run](https://kitao.github.io/pyxel/web/showcase/examples/c01-hello-cube.html)).

## Camera and shading

### Camera ([Camera](https://kitao.github.io/pyxel/web/api-reference/cube/#camera) class)

A **camera** chooses a viewpoint and direction and converts 3D positions to a 2D view. Assign a `Camera` to `root.camera`. `Mat4.look_at(camera_position, target_position)` creates its world transform.

```python
from pyxel.cube import Camera

self.root.camera = Camera()
self.root.camera.transform = Mat4.look_at(Vec3(0, 78, 104), Vec3.ZERO)
```

`Camera()` uses **perspective projection**: distant objects appear smaller. `fov` sets the vertical field of view in degrees; its default is 60.

Set a number for `ortho_size` to use **orthographic projection**, where distance does not change apparent size. This is useful for an overview of a model.

```python
self.root.camera.ortho_size = 100
```

`ortho_size=100` shows 100 world units vertically. The visible width follows the viewport aspect ratio. This value is separate from pixel dimensions. Use `None` for perspective projection.

To follow a player, update the camera position and target each frame. `near` and `far` hide objects that are too close or too distant; adjust them to your scene.

### Shading ([Shading](https://kitao.github.io/pyxel/web/api-reference/cube/#shading) class)

**Shading** changes brightness according to face orientation and light direction. Assign a `Shading` to `root.shading`; child nodes inherit it.

```python
from pyxel.cube import Shading

self.root.shading = Shading(pyxel.colors)
self.root.shading.direction = Vec3(0.4, -1, -0.5)
```

`direction` is the world-space direction in which light travels. Here it points mainly downward. Without shading, surfaces use their original colors or textures.

### Shading palette

The display palette maps color numbers to RGB values. The **shading palette** maps a color number and brightness to drawing color numbers; it does not add display colors. `Shading(pyxel.colors)` builds it automatically from the display palette.

Brightness has eight levels, from 0 (dark) to 7 (bright), selected by face orientation. Each entry holds two color numbers. Equal numbers give a solid color; different numbers form a checkerboard dither. `Shading.BASE_LEVEL` (5) retains the original color.

For example, change how dark faces of color number 8 are drawn:

```python
shading = self.root.shading
shading[8, 0] = (2, 2)  # Set the darkest level to color 2
shading[8, 1] = (2, 8)  # Use two colors for dithering at the next level
```

After changing the display palette, use `shading.build(pyxel.colors)` to rebuild the shading palette. This replaces manual edits, so customize entries after rebuilding.

To draw only some shapes without shading, call `shaded(False)` inside the node's `on_draw()`.

```python
self.shaded(False)
self.box(Mat4.IDENTITY, Vec3(16, 16, 16), 11)
```

`shaded()` affects subsequent drawing in this `on_draw()`. Drawing state resets when the next node's `on_draw()` starts.

## 3D drawing

### Choose a drawing method

From a node's `on_draw()`, call the method for what you want to draw: points, lines, flat shapes, solids, images, or text in 3D space. Positions and dimensions use values in 3D space; the camera determines their screen positions and apparent sizes.

```python
pyxel.line(0, 0, 30, 20, 8)  # 2D
self.line(Vec3(0, 0, 0), Vec3(30, 20, 0), 8)  # 3D
```

Drawing positions are relative to the node's position and orientation. Methods taking a `Mat4` let you translate, rotate, or scale the drawn shape in addition to the node's transform.

| What to draw | Main methods and uses |
| --- | --- |
| Points, lines, triangles | `pset()`, `line()`, `tri()`: specify positions directly. |
| Flat shapes | `rect()` and `elli()` use a matrix for orientation. `circ()` always faces the camera. |
| Solids | `box()` and `sphere()` draw boxes and spheres. |
| Textured planes | `plane()` uses a matrix for orientation; `sprite()` always faces the camera. |
| Custom or deformed shapes | Pass `Primitive` shape data to `prim()`. |
| Text | `text()` places text at a 3D position with a constant size on screen. |

Triangles, rectangles, circles, ellipses, boxes, and spheres also have outline methods with a `b` suffix. For example, `boxb()` draws only a box's edges.

### Draw with images

A **texture** is an image applied to a surface. Use Pyxel's [Image](https://kitao.github.io/pyxel/web/api-reference/#image). For example, passing an image to `box()` instead of a color number applies the whole image to each face.

This example uses a prepared image file, `assets/texture.png`. Load it when initializing the node after `pyxel.init()`, then draw in `on_draw()`.

```python
self.texture = pyxel.Image.from_image("assets/texture.png")
```
```python
self.box(Mat4.IDENTITY, Vec3(30, 30, 30), self.texture)
```

Use `plane()` to orient a textured plane yourself, or `sprite()` to keep it facing the camera. The former suits signs on walls; the latter suits pixel-art characters or explosions. `sprite()` images are not shaded.

Inside `on_draw()`, this example applies the whole image to a plane 30 wide and 30 high. `uvs` gives image positions for the top-left, top-right, bottom-left, and bottom-right corners, with (0, 0) at the image's top-left and (1, 1) at its bottom-right.

```python
uvs = ((0, 0), (1, 0), (0, 1), (1, 1))
self.plane(Mat4.IDENTITY, self.texture, uvs, 30, 30)
# Replace the drawing above with this to always face the camera
# self.sprite(Vec3.ZERO, self.texture, uvs, 30, 30)
```

Change the `uvs` range to use part of an image. Set `colkey` to a color number to make that color transparent.

See `PlaneShape` and `SpriteShape` in the [basic 3D shapes](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c02_basic_shapes.py) example ([run](https://kitao.github.io/pyxel/web/showcase/examples/c02-basic-shapes.html)).

### Create shape data ([Primitive](https://kitao.github.io/pyxel/web/api-reference/cube/#primitive) class)

Methods such as `box()` draw basic shapes immediately. To create or deform a shape, or reuse it, **define its vertices and faces as data and pass them when drawing**. This shape data is a `Primitive`.

`Primitive.plane()`, `Primitive.box()`, and similar factories create basic shape data. Images or colors are specified separately when drawing, so one shape can have different appearances. A `Mesh`, covered next, combines multiple shapes and a hierarchy of parts.

When initializing the node, create a plane 20 wide and 20 high, then increase its top-left vertex's Y coordinate by 10.

```python
from pyxel.cube import Primitive

self.shape = Primitive.plane(20, 20)
self.shape.positions[1] += 10
self.shape.compute_normals()
```

`positions` lists each vertex's X, Y, and Z coordinates in order. The top-left vertex of `Primitive.plane()` uses `positions[0]`, `positions[1]`, and `positions[2]`.

`positions[1] += 10` changes only Y. `compute_normals()` recalculates surface directions after the vertices move.

This code changes the plane as follows.

![Move the plane's top-left vertex from (-10, 10, 0) to (-10, 20, 0), a displacement of (0, 10, 0). The other vertices stay in place.](https://kitao.github.io/pyxel/web/user-guide/cube/images/primitive.svg)

Pass the shape to `prim()` inside `on_draw()` to draw it. `8` is the standard palette's red color number.

```python
self.prim(Mat4.IDENTITY, self.shape, 8)
```

Changing `positions` every frame can produce a rippling body. `indices` stores the vertex numbers forming each face; `uvs` maps vertices to image positions. Use a `Primitive` with drawing methods and textures.

See `on_draw()` in the [custom 3D shapes](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c03_custom_shapes.py) example for vertex deformation ([run](https://kitao.github.io/pyxel/web/showcase/examples/c03-custom-shapes.html)).

### Combine 3D and 2D

Use Pyxel's 2D drawing for screen-fixed content such as scores and controls. Overlay it after drawing the node tree in the app's `draw()`.

```python
pyxel.cls(0)
self.root.draw(0, 0, pyxel.width, pyxel.height)
pyxel.text(8, 8, "SCORE: 0", 7)
```

This example draws 3D across the screen, then overlays a score. 3D objects overlap by depth; 2D content overlaps in drawing order.

- `x, y, w, h`: the 3D drawing region (viewport). You can draw in part of the screen.
- `Camera.clear_color`: the background color filling the entire drawing target. Use `None` to retain a 2D background.
- `target`: pass an `Image` to draw into it instead of the screen.

Use the node's `text()` for names attached to enemies or other 3D positions. Text has a constant screen size regardless of distance and is occluded according to 3D depth. Use `pyxel.text()` for a screen-fixed score and node `text()` for a name attached to an object.

```python
self.text(Vec3(0, 20, 0), "ENEMY", 7)
```

## Meshes and motions

[`Mesh`](https://kitao.github.io/pyxel/web/api-reference/cube/#mesh) handles three-dimensional data (**3D models**) for characters and terrain placed in a game. It brings together shapes such as heads, bodies and legs, relationships between parts, and surface images and colors.

`Mesh.from_glb()` reads **GLB (binary glTF 2.0)** files. They package shape, hierarchy and textures in one file and can also contain animation.

The supported GLB features and production steps are explained later in [Create resources](https://kitao.github.io/pyxel/web/user-guide/cube/#s-resources).

### Loading a Mesh from GLB

Load a GLB with `Mesh.from_glb()`, then create nodes for the scene with `Node.from_mesh()`. You can create multiple nodes from the same `Mesh`, for example to place several instances of a character.

The following example loads a prepared GLB file, `assets/character.glb`.

```python
from pyxel.cube import Mesh

mesh = Mesh.from_glb("assets/character.glb")
actor = Node.from_mesh(mesh)
self.root.add_child(actor)
```

The example above belongs in app initialization after `pyxel.init()`. After initialization, relative paths are based on the directory of the Python file that called `pyxel.init()`. Use the placement node's `actor.transform` to adjust position, orientation and size.

### Playing animation (class [`Motion`](https://kitao.github.io/pyxel/web/api-reference/cube/#motion))

`Motion` handles animation such as walking or running. Animations in the GLB are loaded with the `Mesh` and put in the `mesh.motions` list. Play them with `play_motion()` on the display node.

The next example assumes the GLB contains an animation named `walk`. Select the `Motion` by that name and play it.

```python
walk = next(motion for motion in mesh.motions if motion.name == "walk")
actor.play_motion(walk)
```

`mesh.motions` is a list, so it is empty for a GLB without animation. `play_motion()` loops by default; use `loop=False` to play once.

To play at the original animation speed, match the `fps` in `Mesh.from_glb(..., fps=30)` to the app's `pyxel.init(..., fps=...)`. GLB times in seconds are converted to frames using this value.

`Motion` updates the transforms of the imported parts. Use the placement root returned by `Node.from_mesh()` for overall placement and a further parent node for character movement and collision handling; this division makes them easier to combine.

For an example of switching Motion clips, see `play()` in the [GLB model animation](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c04_mesh_and_motion.py) sample ([run](https://kitao.github.io/pyxel/web/showcase/examples/c04-mesh-and-motion.html)).

## Physics engine

Cube's physics engine calculates motion from velocity and gravity, and detects collisions. Implement `on_collide()` to receive collision information and handle position and velocity corrections, landing, or collecting items as your game needs.

### Colliders ([Collider](https://kitao.github.io/pyxel/web/api-reference/cube/#collider) class)

A **collider** defines a collision shape and physical properties. Create a `Collider` and assign it to a node's `collider`. Use a sphere, capsule, box, or other shape separately from the visible `Mesh`. For complex terrain, assign its `Mesh` to the collider's `mesh`.

Mesh colliders are for fixed terrain. Use spheres, capsules, or boxes for moving objects, keeping their visible `Mesh` separate from the collision shape. Translate or rotate moving colliders and their parents; apply visual scaling to child nodes instead.

```python
from pyxel.cube import Collider

self.collider = Collider(size=Vec3(26, 26, 26), mass=1)
```

`mass` is mass; `velocity` is world-space displacement per update. `gravity` is gravitational acceleration in m/s², defaulting to 9.8. Gravity calculations convert 100 world units to 1 meter, so adjust gravity when using a different length scale from the examples.

- `mass=0`: a floor that collisions do not push, or a platform moved by code.
- `gravity=0`: a character whose movement, such as jumping, you control yourself.
- `mass=0, trigger=True`: a region that only detects contact, such as a coin or goal.

### Character movement

Inside `on_update()` of a node with the collider above, set X velocity from the left and right keys. Retain Y velocity so gravity continues to make it fall.

```python
vx = 2 * (pyxel.btn(pyxel.KEY_RIGHT) - pyxel.btn(pyxel.KEY_LEFT))
self.collider.velocity = Vec3(vx, self.collider.velocity.y, 0)
```

Set velocity to 2 in the held direction; releasing the keys sets X velocity to 0. Handle landing and walls in `on_collide()`, described next.

See `Player` in the [3D collision action game](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c05_3d_collision.py) example for movement, jumping, and coin collection ([run](https://kitao.github.io/pyxel/web/showcase/examples/c05-3d-collision.html)).

### Respond to collisions

Cube calculates motion and collisions, then passes the other node and contact information ([Contact](https://kitao.github.io/pyxel/web/api-reference/cube/#contact)) to `on_collide(other, contact)`. `normal` is the world-space push-out direction; `depth` is the penetration correction distance.

This example applies the reported position, velocity, and angular-velocity corrections when the parent node's transform is `Mat4.IDENTITY`. Without an `on_collide()` implementation, push-out and bounce corrections are not applied.

```python
def on_collide(self, other, contact):
    if self.collider.mass:
        shift = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = shift * self.transform
        self.collider.velocity += contact.delta_velocity
        self.collider.angular_velocity += contact.delta_angular_velocity
```

Change bounce strength with `restitution` and resistance to sliding with `friction`. Set `rolls=True` to calculate rotation caused by collisions too.

See `on_collide()` in the [3D physics simulation](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/cube/c06_3d_physics.py) example for bouncing and rolling ([run](https://kitao.github.io/pyxel/web/showcase/examples/c06-3d-physics.html)).

### Raycasting

**Raycasting** extends a line through space to find hit objects and positions. For example, cast downward from a character to locate the ground for drawing a shadow.

```python
hit = self.parent.raycast(self.world_transform.pos, Vec3.DOWN, tags={"ground"})
```

The origin uses world coordinates, so get the node's world position with `world_transform.pos`. This example searches colliders with the `ground` tag below the parent node. Tag ground nodes beforehand, for example with `ground.tags.add("ground")`.

A hit returns [RaycastHit](https://kitao.github.io/pyxel/web/api-reference/cube/#raycasthit), with the contact position in `hit.point`; a miss returns `None`. `raycast_all()` returns multiple contacts sorted by distance. Use `overlap_sphere()` or `overlap_box()` to find enemies or other objects within a region rather than along a line.

## Create resources

Create character and terrain shapes, images, and motions in a GLB, then load it as a `Mesh`. Before starting, check the features Cube supports and the required export settings.

### Supported GLB features

| Element | Main supported features |
| --- | --- |
| Shapes and hierarchy | Triangle meshes, parent-child relationships, translation, rotation, and scaling. |
| Colors and textures | Base colors and embedded images, including multiple materials. Colors are converted to `pyxel.colors` at load time. Texture coordinates use `TEXCOORD_0`. |
| Motion | Keyframes for part position, orientation, and size over time. LINEAR, STEP, and CUBICSPLINE interpolation are supported. |

Use `alphaMode=MASK` for image cutouts. An unused color number is needed for transparency; without one, cutouts are disabled. `BLEND` translucency, vertex colors, normal maps, and PBR features such as metallic and roughness are not applied.

**Skinning**, which bends vertices with bones, and **morphing**, which changes vertex shapes, are unsupported. Animate separate parts such as arms and legs by translation, rotation, and scaling. Set cameras and lights in Cube too.

Embed buffers and images in the GLB. Files referencing external resources, drawing modes other than triangles, and morph animations cannot be loaded. For files using `KHR_animation_pointer`, all animations are excluded from loading.

Colors are not limited to the standard 16. Before loading a GLB, assign a list of RGB values to `pyxel.colors[:]` to use a custom palette of 1-256 colors. Colors absent from the palette are converted to the closest available color at load time.

### Create Mesh resources

For these shapes, textures, and motions, we recommend [Blockbench](https://www.blockbench.net/). Choose the **Generic Model** project format.

To animate, group the moving parts and set translation, rotation, and scaling keyframes in **Animate** mode. After export to GLB, Cube loads them as `Motion` objects.

Blockbench's editable .bbmodel file and the GLB loaded by the game serve different purposes. Save the editable file and export a GLB for the game.

```text
Blockbench (.bbmodel) → Export (.glb) → Mesh.from_glb()
```

To paint textures with Pyxel's standard colors, set up the palette as follows.

1. Save the [Pyxel palette (pyxel.gpl)](https://kitao.github.io/pyxel/docs/pyxel.gpl).
2. In **Paint** mode's **Palette** panel, choose **Import Palette** and load the saved file. Enable **Replace old palette** to replace the existing palette.
3. Right-click empty space in the palette and enable **Lock Palette** to prevent adding, deleting, or dragging colors to reorder them.
4. Choose colors from the palette; paint with 100% opacity and 0 blur.

**Lock Palette** prevents these editing operations; it does not automatically restrict paint colors. For a custom palette, load the same colors that the game displays.

### Export to GLB

Use **File → Export → Export glTF Model** and export as **Binary (glb)**.

- **Embed Textures**: enabled
- **Export Groups as Armature**: disabled
- **Export Animations**: enabled when including motions

These settings embed textures in the GLB and animate part transforms. Cube does not use Armature skinning.

If an export with transparent images uses `BLEND`, Cube draws the transparent parts as opaque. To use cutouts, edit the exported GLB's materials to set `alphaMode=MASK`.

For detailed authoring instructions, see the [official Blockbench guide](https://www.blockbench.net/wiki/guides/blockbench-overview-tips/).

### Create terrain Meshes

For complex-looking terrain, create a collision `Mesh` with simplified walkable surfaces and walls, separate from the display `Mesh`. This makes collision boundaries easier to adjust. Load its GLB and assign it to the collider's `mesh`.

[Cube Model References](https://github.com/kitao/pyxel/tree/main/python/pyxel/examples/cube/tools) places and duplicates parts, then exports display and collision GLBs.

### Images, sound, and music

Image, sound, and music resources are shared with Pyxel. See [Create resources in the Pyxel User Guide](https://kitao.github.io/pyxel/web/user-guide/#s-resources) for creation and loading steps.

## Distribute applications

Pyxel Cube applications use the same distribution process as Pyxel. See [Distribute applications in the Pyxel User Guide](https://kitao.github.io/pyxel/web/user-guide/#s-dist) for the steps, and [Using Pyxel on the web](https://kitao.github.io/pyxel/web/web-usage/) for embedding in web pages.

Include the GLBs, images, sounds, and other resources used by the game, preserving the file layout referenced by the code.

## API reference

See the [API Reference](https://kitao.github.io/pyxel/web/api-reference/) for class and argument details. Select **Cube** at the top to view Pyxel Cube APIs. Select **Base** for shared APIs such as input, sound, and 2D drawing.

## Links

- [Pyxel User Guide](https://kitao.github.io/pyxel/web/user-guide/#s-links): related manuals, tools, and community links, plus license and contribution information.
- [Frequently asked questions](https://github.com/kitao/pyxel/blob/main/docs/faq-ja.md): questions about operation and the environment.
