import inspect

import pytest

import pyxel
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


def palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]


class TestAttributes:
    def test_default_state(self):
        n = Node()
        assert n.name == ""
        assert n.active is True
        assert n.visible is True
        # transform reads as Mat4.IDENTITY by default.
        assert isinstance(n.transform, Mat4)
        # Cascade attributes are None on a freshly constructed Node so
        # they inherit from the closest non-None ancestor; the root node
        # provides the camera / shading (cube-design.md § 15).
        assert n.shading is None
        assert n.collider is None
        assert n.parent is None
        assert n.children == ()

    def test_constructor_rejects_unused_arguments(self):
        assert str(inspect.signature(Node)) == "()"
        with pytest.raises(TypeError):
            Node(1)
        with pytest.raises(TypeError):
            Node(name="player")

    def test_subclass_constructor_keeps_python_arguments(self):
        class Player(Node):
            def __init__(self, name):
                super().__init__()
                self.name = name

        player = Player("hero")
        assert player.name == "hero"

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
        # `pos` reads back as Vec3 of the translation column.
        pos = n.transform.pos
        assert pos.x == 1
        assert pos.y == 2
        assert pos.z == 3

    def test_set_shading(self):
        n = Node()
        shading = Shading(palette())
        n.shading = shading
        # Setter round-trips so reading via the node yields the same
        # entry as reading from the original shading directly.
        assert n.shading[0, 2] == shading[0, 2]
        n.shading = None
        assert n.shading is None

    def test_set_collider(self):
        n = Node()
        collider = Collider()
        n.collider = collider
        # Verify the round-trip through the cascade slot (set, read
        # back, reset to None).
        assert isinstance(n.collider, Collider)
        n.collider = None
        assert n.collider is None


# Collider / Contact construction and field round-trips (cube-design.md
# § 11 / § 12); geometric correctness lives in the Rust unit tests.
class TestColliderContactBasics:
    def test_collider_constructable(self):
        c = Collider()
        assert "Collider(" in repr(c)

    def test_contact_constructable(self):
        c = Contact()
        # Default point / normal are Vec3.ZERO.
        assert c.point == Vec3.ZERO
        assert c.normal == Vec3.ZERO

    def test_contact_round_trip(self):
        c = Contact()
        c.point = Vec3(1, 2, 3)
        c.normal = Vec3(0, 1, 0)
        assert c.point == Vec3(1, 2, 3)
        assert c.normal == Vec3(0, 1, 0)


# Node.BILLBOARD_* class constants were removed along with the
# billboard kwarg (use Mat4 to face the camera when needed).
class TestClassConstantsRemoved:
    def test_billboard_off_attribute_removed(self):
        assert not hasattr(Node, "BILLBOARD_OFF")

    def test_billboard_on_attribute_removed(self):
        assert not hasattr(Node, "BILLBOARD_ON")

    def test_billboard_fixed_y_attribute_removed(self):
        assert not hasattr(Node, "BILLBOARD_FIXED_Y")


class TestHierarchy:
    def test_add_remove_child(self):
        p = Node()
        c = Node()
        p.add_child(c)
        assert p.children == (c,) or p.children[0] is c
        p.remove_child(c)
        assert p.children == ()

    def test_reparent_unlinks(self):
        p1 = Node()
        p2 = Node()
        c = Node()
        p1.add_child(c)
        p2.add_child(c)
        # add_child re-parents: child should no longer be in p1.
        assert len(p1.children) == 0
        assert len(p2.children) == 1

    def test_remove_child_rejects_non_child(self):
        p1 = Node()
        p2 = Node()
        c = Node()
        p2.add_child(c)
        with pytest.raises(ValueError):
            p1.remove_child(c)
        assert c.parent is p2
        assert p2.children == (c,)

    def test_add_child_rejects_self_parenting(self):
        n = Node()
        with pytest.raises(ValueError):
            n.add_child(n)
        assert n.parent is None
        assert n.children == ()

    def test_add_child_rejects_ancestor_cycle(self):
        root = Node()
        mid = Node()
        leaf = Node()
        root.add_child(mid)
        mid.add_child(leaf)
        with pytest.raises(ValueError):
            leaf.add_child(root)
        assert root.parent is None
        assert mid.parent is root
        assert leaf.parent is mid

    def test_find_by_name(self):
        root = Node()
        head = Node()
        head.name = "head"
        root.add_child(head)
        # Subtree DFS pre-order; self matches first when its name fits.
        root.name = "root"
        assert len(root.find_by_name("root")) == 1
        found = root.find_by_name("head")
        assert len(found) == 1
        assert found[0].name == "head"
        assert root.find_by_name("missing") == []

    def test_find_by_name_multiple_matches(self):
        # Pyxel cube does not enforce unique names; find_by_name returns
        # every match (e.g. multiple "zako" enemies under the same root).
        root = Node()
        a = Node()
        b = Node()
        a.name = "zako"
        b.name = "zako"
        root.add_child(a)
        root.add_child(b)
        assert len(root.find_by_name("zako")) == 2

    def test_find_by_tags(self):
        root = Node()
        a = Node()
        b = Node()
        a.tags = ["enemy"]
        b.tags = ["player"]
        root.add_child(a)
        root.add_child(b)
        found = root.find_by_tags(["enemy"])
        assert len(found) == 1
        assert found[0] is a
        # Multiple tags match any (OR).
        found2 = root.find_by_tags(["enemy", "player"])
        assert len(found2) == 2

    def test_destroy(self):
        p = Node()
        c = Node()
        p.add_child(c)
        c.destroy()
        # Deferred semantics (cube-design.md § 16 step 8): destroy()
        # sets the flag but the parent / child links survive until
        # Scene.update detaches the node at the end of the frame.
        assert c.destroyed is True
        assert len(p.children) == 1


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
        # A subclass __init__ taking extra positional args must work;
        # Node.__new__ accepts and ignores the extra args (the § 14
        # Player sample relies on this).
        class Tagged(Node):
            def __init__(self, label):
                super().__init__()
                self.name = label

        n = Tagged("hero")
        assert n.name == "hero"

    def test_node_subclass_chained_init_args(self):
        # Subclass __init__ taking extra positional args must work;
        # Node.__new__ accepts and ignores the extra args (mirrors the
        # simpler test_subclass_with_init_args; this variant exercises
        # the same chain through a deeper hierarchy).
        class Level(Node):
            def __init__(self, depth):
                super().__init__()
                self.name = f"level-{depth}"

        s = Level(3)
        assert s.name == "level-3"

    def test_lifecycle_hooks_default_noop(self):
        # Default implementations are no-op; they must be callable
        # directly as well as from the pipeline (§ 16).
        n = Node()
        other = Node()
        n.on_update()
        n.on_draw()
        n.on_collide(other, Contact())
        n.on_destroy()


# Calling draw methods outside an active draw context must be a safe
# no-op (the binding dispatches through with_draw_context, which skips
# when no context is active). Per-call state kwargs were removed in
# favor of Node.dither / depth_test / depth_write / shaded state-setters.
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

    def test_sprite_takes_image(self):
        # sprite needs an Image; the no-window environment cannot
        # construct one easily — verify the method is callable.
        assert callable(Node().sprite)

    def test_mesh_draw_method_removed(self):
        assert not hasattr(Node(), "mesh")

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

    def test_from_mesh_generated_nodes_draw_attached_primitives(self):
        prim = Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0, 1, 2],
        )
        mesh = Mesh(
            primitives=[prim],
            transforms=[Mat4.IDENTITY],
            parents=[-1],
            col_img=8,
        )
        root = Node()
        root.camera = Camera()
        root.add_child(Node.from_mesh(mesh))

        root.draw(0, 0, 64, 64)

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


# Setter methods on Node that mutate the active DrawContext state.
# Called outside on_draw, they are no-ops (no active draw context).
# Inside on_draw, they affect subsequent draws within the same body and
# reset at the entry of the next Node's on_draw.
class TestStateSetters:
    def test_setters_callable_outside_draw_are_noop(self):
        # Called with no active draw context — should not raise.
        n = Node()
        n.dither(0.5)
        n.depth_test(False)
        n.depth_write(False)
        n.shaded(False)

    def test_setters_inside_on_draw(self):
        # Subclass that exercises the setters inside on_draw and draws.
        class Probe(Node):
            def on_draw(self):
                self.dither(0.5)
                self.depth_test(False)
                self.depth_write(False)
                self.shaded(False)
                self.box(Mat4.IDENTITY, Vec3(1, 1, 1), 7)

        root = Node()
        root.camera = Camera()
        root.add_child(Probe())
        # No window — Node.draw composes the context and dispatches
        # on_draw without rasterizing. We only assert no error is raised.
        root.draw(0, 0, 64, 64)


# Smoke-test that Node exposes the frame-level draw API and queries.
class TestNodeIntegrationOfDrawAndQueries:
    def test_box_sphere_text_via_node(self):
        n = Node()
        n.box(Mat4.IDENTITY, Vec3(1, 1, 1), 4)
        n.boxb(Mat4.IDENTITY, Vec3(1, 1, 1), 5)
        n.sphere(Vec3.ZERO, 1.0, 7)
        n.sphereb(Vec3.ZERO, 1.0, 8)
        n.text(Vec3.ZERO, "ok", 9)

    def test_draw_api_signature_wiring(self):
        # Multi-angle rendering: building the tree once and rendering
        # via different cameras is part of the contract. The smoke test
        # here confirms the draw API and Camera type are reachable.
        n = Node()
        cam = Camera()
        assert hasattr(n, "draw")
        assert hasattr(cam, "transform")


# box and sphere accept col_img: int | Image for textured fill. The
# smoke tests only verify the API surface and that the call does not
# raise; per-pixel correctness is covered by manual visual inspection.
class TestBoxSphereTexturing:
    def test_box_flat_col(self):
        # Existing positional-int path still works.
        Node().box(Mat4.IDENTITY, Vec3(1, 1, 1), 11)

    def test_box_textured(self):
        img = pyxel.images[0]
        Node().box(Mat4.IDENTITY, Vec3(1, 1, 1), img)

    def test_box_textured_with_colkey(self):
        img = pyxel.images[0]
        Node().box(Mat4.IDENTITY, Vec3(1, 1, 1), img, colkey=0)

    def test_sphere_flat_col(self):
        Node().sphere(Vec3.ZERO, 1.0, 11)

    def test_sphere_textured(self):
        img = pyxel.images[0]
        Node().sphere(Vec3.ZERO, 1.0, img)

    def test_sphere_textured_with_colkey(self):
        img = pyxel.images[0]
        Node().sphere(Vec3.ZERO, 1.0, img, colkey=0)


# `on_collide` is invoked by Node.update step 7 once per contact pair
# (cube-design.md § 16). The signature must accept both positional and
# keyword forms with the documented argument names so the engine call
# and direct user calls both work.
class TestOnCollideSignature:
    def test_positional(self):
        n = Node()
        other = Node()
        n.on_collide(other, Contact())

    def test_keyword(self):
        n = Node()
        other = Node()
        n.on_collide(other=other, contact=Contact())


# circ, circb, text, and sprite are always-billboard primitives.
# Pixel-level verification that the geometry faces the camera is
# covered by manual visual inspection of c01_hello_cube (the Label text
# must stay readable as the scene rotates). This unit test only
# confirms the plain positional shape continues to work.
class TestAlwaysBillboard:
    def test_circ_circb_text_plain_call(self):
        n = Node()
        n.circ(Vec3.ZERO, 1.0, 11)
        n.circb(Vec3.ZERO, 1.0, 12)
        n.text(Vec3.ZERO, "X", 7)


_TRIANGLE_PRIMITIVE = Primitive(
    Primitive.MODE_TRIANGLES,
    [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
    [0, 1, 2],
)
_UNIT_QUAD_UVS = ((0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0))


# billboard kwarg is removed from all primitives that previously had it.
class TestBillboardKwargRemoved:
    @pytest.mark.parametrize(
        "call",
        [
            lambda n: n.line(Vec3.ZERO, Vec3(1, 0, 0), 7, billboard=1),
            lambda n: n.tri(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 7, billboard=1),
            lambda n: n.trib(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 7, billboard=1),
            lambda n: n.rect(Mat4.IDENTITY, 1.0, 1.0, 7, billboard=1),
            lambda n: n.rectb(Mat4.IDENTITY, 1.0, 1.0, 7, billboard=1),
            lambda n: n.elli(Mat4.IDENTITY, 1.0, 1.0, 7, billboard=1),
            lambda n: n.ellib(Mat4.IDENTITY, 1.0, 1.0, 7, billboard=1),
            lambda n: n.box(Mat4.IDENTITY, Vec3(1, 1, 1), 7, billboard=1),
            lambda n: n.boxb(Mat4.IDENTITY, Vec3(1, 1, 1), 7, billboard=1),
            lambda n: n.plane(
                Mat4.IDENTITY, pyxel.images[0], _UNIT_QUAD_UVS, 1.0, 1.0, billboard=1
            ),
            lambda n: n.prim(Mat4.IDENTITY, _TRIANGLE_PRIMITIVE, billboard=1),
        ],
    )
    def test_billboard_kwarg_rejected(self, call):
        with pytest.raises(TypeError):
            call(Node())


class TestCameraCascade:
    def test_camera_default_none(self):
        n = Node()
        assert n.camera is None
        assert n.effective_camera is None

    def test_set_and_get_camera(self):
        n = Node()
        c = Camera()
        n.camera = c
        assert n.camera is not None
        assert n.effective_camera is not None

    def test_effective_camera_inherits_from_ancestor(self):
        root = Node()
        leaf = Node()
        root.add_child(leaf)
        c = Camera()
        root.camera = c
        # leaf has no own camera; resolves to root's.
        assert leaf.camera is None
        assert leaf.effective_camera is not None

    def test_draw_without_camera_raises(self):
        n = Node()
        with pytest.raises(ValueError):
            n.draw(0, 0, 64, 64)

    def test_camera_clear_color_roundtrip(self):
        c = Camera()
        assert c.clear_color is None
        c.clear_color = 5
        assert c.clear_color == 5

    def test_draw_accepts_no_clear_color_arg(self):
        n = Node()
        n.camera = Camera()
        # clear_color is no longer a draw parameter.
        n.draw(0, 0, 32, 32)
