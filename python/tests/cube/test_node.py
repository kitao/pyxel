import inspect
import math

import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]
from pyxel import Image
from pyxel.cube import (
    Camera,
    Collider,
    Contact,
    Mat4,
    Mesh,
    Node,
    Primitive,
    Shading,
    Vec3,
)


class TestAttributes:
    def test_default_state(self):
        n = Node()
        assert n.name == ""
        assert n.active is True
        assert n.visible is True
        assert n.transform == Mat4.IDENTITY
        assert n.shading is None
        assert n.collider is None
        assert n.parent is None
        assert n.children == ()

    def test_constructor_rejects_unused_arguments(self):
        assert str(inspect.signature(Node)) == "()"
        with raises_exact(TypeError, "Node() takes no arguments"):
            Node(1)
        with raises_exact(TypeError, "Node() takes no arguments"):
            Node(name="player")

    def test_set_name(self):
        n = Node()
        n.name = "head"
        assert n.name == "head"

    def test_set_active_visible(self):
        n = Node()
        n.active = False
        n.visible = False
        assert n.active is False
        assert n.visible is False

    def test_set_transform(self):
        n = Node()
        n.transform = Mat4.from_translation(Vec3(1, 2, 3))
        pos = n.transform.pos
        assert pos.x == 1
        assert pos.y == 2
        assert pos.z == 3

    def test_set_shading(self):
        n = Node()
        shading = Shading(palette())
        n.shading = shading
        assert n.shading[0, 2] == shading[0, 2]
        n.shading = None
        assert n.shading is None

    def test_set_collider(self):
        n = Node()
        collider = Collider()
        n.collider = collider
        assert isinstance(n.collider, Collider)
        n.collider = None
        assert n.collider is None


class TestColliderContactBasics:
    def test_collider_constructable(self):
        c = Collider()
        assert repr(c) == "Collider(size=Vec3(0, 0, 0), radius=0, mass=1)"

    def test_collider_rejects_invalid_mass(self):
        message = "mass must be finite and greater than or equal to 0"
        for mass in (-1.0, math.nan, math.inf, -math.inf):
            with raises_exact(ValueError, message):
                Collider(mass=mass)

    def test_collider_mass_setter_rejects_invalid_value_without_mutation(self):
        collider = Collider(mass=2.0)
        message = "mass must be finite and greater than or equal to 0"
        for mass in (-1.0, math.nan, math.inf, -math.inf):
            with raises_exact(ValueError, message):
                collider.mass = mass
            assert collider.mass == 2.0

    def test_contact_not_user_constructible(self):
        with raises_exact(TypeError, "cannot create 'pyxel.cube.Contact' instances"):
            Contact()


class TestHierarchy:
    def test_add_child_and_remove_child_keep_canonical_signature(self):
        assert str(inspect.signature(Node.add_child)) == "(self, /, node)"
        assert str(inspect.signature(Node.remove_child)) == "(self, /, node)"

    def test_add_child_and_remove_child_take_node_keyword(self):
        parent, child = Node(), Node()
        parent.add_child(node=child)
        assert child.parent is parent
        parent.remove_child(node=child)
        assert child.parent is None

    def test_add_child_and_remove_child_reject_unknown_child_keyword(self):
        parent, child = Node(), Node()
        with raises_exact(
            TypeError,
            "Node.add_child() got an unexpected keyword argument 'child'",
        ):
            parent.add_child(child=child)

        parent.add_child(child)
        with raises_exact(
            TypeError,
            "Node.remove_child() got an unexpected keyword argument 'child'",
        ):
            parent.remove_child(child=child)

    def test_add_child_and_remove_child_require_node_argument(self):
        parent, child = Node(), Node()
        with raises_exact(
            TypeError,
            "Node.add_child() missing 1 required positional argument: 'node'",
        ):
            parent.add_child()

        parent.add_child(child)
        with raises_exact(
            TypeError,
            "Node.remove_child() missing 1 required positional argument: 'node'",
        ):
            parent.remove_child()

    def test_add_remove_child(self):
        p = Node()
        c = Node()
        p.add_child(c)
        assert p.children == (c,)
        p.remove_child(c)
        assert p.children == ()

    def test_reparent_unlinks(self):
        p1 = Node()
        p2 = Node()
        c = Node()
        p1.add_child(c)
        p2.add_child(c)
        assert p1.children == ()
        assert p2.children == (c,)

    def test_remove_child_rejects_non_child(self):
        p1 = Node()
        p2 = Node()
        c = Node()
        p2.add_child(c)
        with raises_exact(ValueError, "remove_child requires a direct child"):
            p1.remove_child(c)
        assert c.parent is p2
        assert p2.children == (c,)

    def test_add_child_rejects_self_parenting(self):
        n = Node()
        with raises_exact(ValueError, "add_child would create a cycle"):
            n.add_child(n)
        assert n.parent is None
        assert n.children == ()

    def test_add_child_rejects_ancestor_cycle(self):
        root = Node()
        mid = Node()
        leaf = Node()
        root.add_child(mid)
        mid.add_child(leaf)
        with raises_exact(ValueError, "add_child would create a cycle"):
            leaf.add_child(root)
        assert root.parent is None
        assert mid.parent is root
        assert leaf.parent is mid

    def test_find_by_name(self):
        root = Node()
        head = Node()
        head.name = "head"
        root.add_child(head)
        root.name = "root"
        assert root.find_by_name("root") == [root]
        assert root.find_by_name("head") == [head]
        assert root.find_by_name("missing") == []

    def test_find_by_name_multiple_matches(self):
        # Pyxel Cube does not enforce unique names; find_by_name returns
        # every match (e.g. multiple "zako" enemies under the same root).
        root = Node()
        a = Node()
        b = Node()
        a.name = "zako"
        b.name = "zako"
        root.add_child(a)
        root.add_child(b)
        assert root.find_by_name("zako") == [a, b]

    def test_find_by_tags(self):
        root = Node()
        a = Node()
        b = Node()
        a.tags = ["enemy"]
        b.tags = ["player"]
        root.add_child(a)
        root.add_child(b)
        assert root.find_by_tags(["enemy"]) == [a]
        # Multiple tags match any (OR).
        assert root.find_by_tags(["enemy", "player"]) == [a, b]


class TestSubclassing:
    def test_subclass_attribute_round_trip(self):
        class Actor(Node):
            def __init__(self):
                super().__init__()
                self.payload = 42

        a = Actor()
        assert a.payload == 42
        assert isinstance(a, Node)

    def test_subclass_with_init_args(self):
        # A subclass __init__ taking extra positional args must work because
        # Node.__new__ accepts and ignores them.
        class Tagged(Node):
            def __init__(self, label):
                super().__init__()
                self.name = label

        n = Tagged("hero")
        assert n.name == "hero"

    def test_node_subclass_chained_init_args(self):
        class Level(Node):
            def __init__(self, depth):
                super().__init__()
                self.name = f"level-{depth}"
                if depth > 0:
                    self.add_child(Level(depth - 1))

        s = Level(3)
        assert s.name == "level-3"
        assert s.children[0].name == "level-2"
        assert len(s.find_by_name("level-0")) == 1

    def test_default_lifecycle_hooks_are_callable(self):
        n = Node()
        n.on_update()
        n.on_draw()
        n.on_destroy()


class TestImmediateDrawSafety:
    def test_pset(self):
        Node().pset(Vec3.ZERO, 7)

    def test_line(self):
        Node().line(Vec3.ZERO, Vec3(1, 0, 0), 8)

    def test_tri_filled(self):
        Node().tri(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 9)

    def test_trib(self):
        Node().trib(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 10)

    def test_circ(self):
        Node().circ(Vec3.ZERO, 1.0, 11)

    def test_circb(self):
        Node().circb(Vec3.ZERO, 1.0, 12)

    def test_rect_family(self):
        m = Mat4.IDENTITY
        n = Node()
        n.rect(m, 2.0, 1.0, 7)
        n.rectb(m, 2.0, 1.0, 8)
        n.elli(m, 2.0, 1.0, 9)
        n.ellib(m, 2.0, 1.0, 10)

    def test_box_family(self):
        m = Mat4.IDENTITY
        n = Node()
        n.box(m, Vec3(1, 1, 1), 4)
        n.boxb(m, Vec3(1, 1, 1), 5)

    def test_sphere_family(self):
        n = Node()
        n.sphere(Vec3.ZERO, 0.5, 12)
        n.sphereb(Vec3.ZERO, 0.5, 13)

    def test_text(self):
        Node().text(Vec3.ZERO, "X", 7)
        Node().text(Vec3(0, 1, 0), "Hi", 6, font=None)

    def test_sprite(self):
        uvs = ((0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0))
        Node().sprite(Vec3.ZERO, pyxel.images[0], uvs, 1.0, 1.0, colkey=0)

    def test_from_mesh_builds_named_node_tree(self):
        prim = Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0, 1, 2],
        )
        m = Mesh(
            primitives=[None, prim, prim],
            transforms=[
                Mat4.IDENTITY,
                Mat4.from_translation(Vec3(1, 0, 0)),
                Mat4.from_translation(Vec3(0, 1, 0)),
            ],
            parents=[-1, 0, 0],
            names=["rig", "body", "arm"],
            col_img=8,
        )

        root = Node.from_mesh(m)

        assert root.name == "rig"
        assert [child.name for child in root.children] == ["body", "arm"]
        assert root.children[0].parent is root
        assert root.find_by_name("arm")[0].transform.pos == Vec3(0, 1, 0)

    def test_prim_with_primitive(self):
        prim = Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0, 1, 2],
            cull=Primitive.CULL_BACK,
        )
        Node().prim(Mat4.IDENTITY, primitive=prim, col_img=7)

    def test_prim_col_img_accepts_image(self):
        img = pyxel.images[0]
        prim = Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0, 1, 2],
            uvs=[0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        )
        Node().prim(Mat4.IDENTITY, prim, col_img=img, colkey=0)

    def test_mesh_col_img_rejects_other_types(self):
        with raises_exact(TypeError, "col_img must be int or Image"):
            Mesh(col_img="7")

    def test_mesh_col_img_accepts_image(self):
        img = pyxel.images[0]
        prim = Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0, 1, 2],
            uvs=[0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        )
        m = Mesh(
            primitives=[prim],
            transforms=[Mat4()],
            parents=[-1],
            col_img=img,
            colkey=0,
        )
        assert isinstance(m.col_img, Image)
        assert isinstance(Node.from_mesh(m), Node)


class TestStateSetters:
    @staticmethod
    def _draw(probe, shading=None):
        root = Node()
        camera = Camera()
        camera.clear_color = 0
        camera.transform = Mat4.look_at(Vec3(0, 0, 4), Vec3.ZERO, Vec3.UP)
        root.camera = camera
        root.shading = shading
        root.add_child(probe)
        pyxel.cls(0)
        root.draw(0, 0, 160, 120)
        return pyxel.pget(80, 60)

    def test_setters_callable_outside_draw(self):
        n = Node()
        n.dither(0.5)
        n.depth_test(False)
        n.depth_write(False)
        n.shaded(False)

    def test_dither_inside_on_draw_affects_subsequent_draws(self):
        class Probe(Node):
            def on_draw(self):
                self.dither(0.0)
                self.pset(Vec3.ZERO, 7)

        assert self._draw(Probe()) == 0

    def test_depth_test_inside_on_draw_affects_subsequent_draws(self):
        class Probe(Node):
            def on_draw(self):
                self.pset(Vec3.ZERO, 7)
                self.depth_test(False)
                self.pset(Vec3(0, 0, -1), 8)

        assert self._draw(Probe()) == 8

    def test_depth_write_inside_on_draw_affects_subsequent_draws(self):
        class Probe(Node):
            def on_draw(self):
                self.depth_write(False)
                self.pset(Vec3.ZERO, 7)
                self.depth_write(True)
                self.pset(Vec3(0, 0, -1), 8)

        assert self._draw(Probe()) == 8

    def test_shaded_inside_on_draw_affects_subsequent_draws(self):
        class Probe(Node):
            def on_draw(self):
                self.shaded(False)
                self.rect(Mat4.IDENTITY, 2, 2, 7)

        shading = Shading(palette())
        for level in range(4):
            shading[7, level] = (3, 3)
        assert self._draw(Probe(), shading) == 7

    @pytest.mark.parametrize("shaded", [False, True])
    @pytest.mark.parametrize("size", [(0, 0), (0, 1), (1, 0), (1, 1)])
    def test_textured_box_zero_dimensions(self, shaded, size):
        img = Image(*size)
        img.cls(7)

        class Probe(Node):
            def on_draw(self):
                self.shaded(shaded)
                self.box(Mat4.IDENTITY, Vec3.ONE, img)

        shading = Shading(palette())
        for level in range(4):
            shading[7, level] = (7, 7)
        assert self._draw(Probe(), shading) == (7 if all(size) else 0)


class TestBoxSphereTexturing:
    def test_box_textured(self):
        img = pyxel.images[0]
        Node().box(Mat4.IDENTITY, Vec3(1, 1, 1), img)

    def test_box_textured_with_colkey(self):
        img = pyxel.images[0]
        Node().box(Mat4.IDENTITY, Vec3(1, 1, 1), img, colkey=0)

    def test_box_col_img_rejects_other_types(self):
        with raises_exact(TypeError, "col_img must be int or Image"):
            Node().box(Mat4.IDENTITY, Vec3(1, 1, 1), "7")

    def test_sphere_textured(self):
        img = pyxel.images[0]
        Node().sphere(Vec3.ZERO, 1.0, img)

    def test_sphere_textured_with_colkey(self):
        img = pyxel.images[0]
        Node().sphere(Vec3.ZERO, 1.0, img, colkey=0)


class TestOnCollideSignature:
    def test_signature_param_names(self):
        assert str(inspect.signature(Node.on_collide)) == "(self, /, other, contact)"


class TestCameraCascade:
    def test_camera_default_none(self):
        n = Node()
        assert n.camera is None
        assert n.effective_camera is None

    def test_set_and_get_camera(self):
        n = Node()
        c = Camera()
        c.fov = 37
        n.camera = c
        assert n.camera.fov == 37
        assert n.effective_camera.fov == 37

    def test_effective_camera_inherits_from_ancestor(self):
        root = Node()
        branch = Node()
        leaf = Node()
        root.add_child(branch)
        branch.add_child(leaf)
        c = Camera()
        c.fov = 37
        root.camera = c
        assert leaf.camera is None
        assert leaf.effective_camera.fov == 37

        branch.camera = Camera()
        branch.camera.fov = 73
        assert leaf.effective_camera.fov == 73
        branch.camera = None
        assert leaf.effective_camera.fov == 37

    def test_draw_without_camera_raises(self):
        n = Node()
        with raises_exact(
            ValueError, "draw requires a camera on this node or an ancestor"
        ):
            n.draw(0, 0, 64, 64)

    def test_camera_clear_color_roundtrip(self):
        c = Camera()
        assert c.clear_color is None
        c.clear_color = 5
        assert c.clear_color == 5


def palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]
