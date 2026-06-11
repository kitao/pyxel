import pyxel
from pyxel.cube import Camera, Mat4, MeshData, Node, PrimData, Shading, Vec3

W, H = 40, 30
pyxel.init(W, H)

verts = [
    -1.0,
    -1.0,
    -1.0,
    1.0,
    -1.0,
    -1.0,
    1.0,
    1.0,
    -1.0,
    -1.0,
    1.0,
    -1.0,
    -1.0,
    -1.0,
    1.0,
    1.0,
    -1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    -1.0,
    1.0,
    1.0,
]
indices = [
    0,
    2,
    1,
    0,
    3,
    2,
    4,
    5,
    6,
    4,
    6,
    7,
    0,
    1,
    5,
    0,
    5,
    4,
    3,
    6,
    2,
    3,
    7,
    6,
    0,
    4,
    7,
    0,
    7,
    3,
    1,
    2,
    6,
    1,
    6,
    5,
]
prim_data = PrimData(PrimData.MODE_TRIANGLES, verts, indices)
mesh_data = MeshData(
    primitives=[prim_data], transforms=[Mat4.IDENTITY], parents=[-1], col_img=7
)

scene = Node()
scene.shading = Shading([pyxel.colors[i] for i in range(16)])

camera = Camera()
camera.clear_color = 0
camera.transform = Mat4.look_at(Vec3(3, 2, 4), Vec3.ZERO, Vec3.UP)
scene.camera = camera


class MeshNode(Node):
    def on_draw(self):
        self.mesh(Mat4.IDENTITY, mesh_data)


class BoxNode(Node):
    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(2, 2, 2), 7)


def dump_screen(label, fh):
    fh.write(f"=== {label} ===\n")
    for y in range(H):
        row = ""
        for x in range(W):
            c = pyxel.screen.pget(x, y)
            row += f"{c:x}" if c < 16 else "?"
        fh.write(row + "\n")
    fh.write("\n")


with open("/tmp/cmp_path.txt", "w") as f:
    # Frame 1: mesh path
    for child in list(scene.children):
        scene.remove_child(child)
    scene.add_child(MeshNode())
    scene.draw(0, 0, W, H)
    pyxel.flip()
    dump_screen("mesh path", f)

    # Frame 2: box_solid path
    for child in list(scene.children):
        scene.remove_child(child)
    scene.add_child(BoxNode())
    scene.draw(0, 0, W, H)
    pyxel.flip()
    dump_screen("box_solid path", f)

pyxel.quit()
