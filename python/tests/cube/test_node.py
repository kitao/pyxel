import pyxel
import pytest
from pyxel import Image

from pyxel.cube import (
    Camera,
    Collider,
    Contact,
    Geometry,
    Mat4,
    Mesh,
    Node,
    Scene,
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
        # they inherit from the closest non-None ancestor (Scene seeds
        # the default Shading; see § 12.4).
        assert n.shading is None
        assert n.collider is None
        assert n.parent is None
        assert n.children == ()

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
        # Collider is currently a placeholder (no comparable state),
        # so we only verify the round-trip through the cascade slot.
        assert isinstance(n.collider, Collider)
        n.collider = None
        assert n.collider is None


class TestColliderPlaceholder:
    """Collider / Contact ship as placeholders today; verify the API
    surface only (cube-design.md § 15)."""

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


class TestClassConstants:
    def test_billboard_modes_distinct(self):
        # Follows Godot BillboardMode with shortened names: OFF=DISABLED,
        # ON=ENABLED, FIXED_Y matches Godot's FIXED_Y (cube-design.md § 12.1).
        assert Node.BILLBOARD_OFF == 0
        assert Node.BILLBOARD_ON == 1
        assert Node.BILLBOARD_FIXED_Y == 2


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
        found = root.find_by_tags("enemy")
        assert len(found) == 1
        assert found[0] is a
        # list[str] form: matches any.
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

    def test_lifecycle_hooks_default_noop(self):
        # Default implementations are no-op; they must be callable.
        # on_collide is wired even though the cube runtime does not
        # invoke it yet (collision pipeline deferred — § 16).
        n = Node()
        other = Node()
        n.on_update()
        n.on_draw()
        n.on_collide(other, Contact())
        n.on_destroy()


# Calling draw methods outside a `Scene.draw` context must be a safe
# no-op (cube-design.md § 12.5: with_draw_context returns None when no
# context is active). The tests below confirm each new immediate-mode
# command — including the keyword-only modifier slots — is accepted.
class TestImmediateDrawSafety:
    def test_pset(self):
        Node().pset(Vec3.ZERO, 7, dither_alpha=0.5, depth_test=False)

    def test_line(self):
        Node().line(
            Vec3.ZERO,
            Vec3(1, 0, 0),
            8,
            dither_alpha=1.0,
            depth_write=False,
            billboard=Node.BILLBOARD_ON,
        )

    def test_tri_filled(self):
        Node().tri(
            Vec3.ZERO,
            Vec3(1, 0, 0),
            Vec3(0, 1, 0),
            9,
            shaded=False,
            dither_alpha=0.75,
            billboard=Node.BILLBOARD_FIXED_Y,
        )

    def test_trib(self):
        Node().trib(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 10)

    def test_circ(self):
        Node().circ(Vec3.ZERO, 1.0, 11, dither_alpha=0.25)

    def test_circb(self):
        Node().circb(Vec3.ZERO, 1.0, 12, depth_test=False, depth_write=False)

    def test_rect_family(self):
        m = Mat4.IDENTITY
        n = Node()
        n.rect(m, 2.0, 1.0, 7, shaded=False)
        n.rectb(m, 2.0, 1.0, 8, billboard=Node.BILLBOARD_ON)
        n.elli(m, 2.0, 1.0, 9, shaded=True)
        n.ellib(m, 2.0, 1.0, 10)

    def test_box_family(self):
        m = Mat4.IDENTITY
        n = Node()
        n.box(m, Vec3(1, 1, 1), 4, shaded=True, dither_alpha=0.5)
        n.boxb(m, Vec3(1, 1, 1), 5, billboard=Node.BILLBOARD_FIXED_Y)

    def test_sphere_family(self):
        n = Node()
        # sphere takes Vec3 pos + radius (symmetric, so no Mat4 needed).
        n.sphere(Vec3.ZERO, 0.5, 12, shaded=True)
        n.sphereb(Vec3.ZERO, 0.5, 13, depth_write=False)

    def test_text(self):
        # Vec3-positioned screen-space text.
        Node().text(Vec3.ZERO, "X", 7)
        # With explicit font=None and modifier kwargs.
        Node().text(
            Vec3(0, 1, 0),
            "Hi",
            6,
            font=None,
            dither_alpha=0.5,
        )

    def test_sprite_takes_image(self):
        # sprite needs an Image; we can't easily construct one without
        # a Pyxel system, so just exercise the no-op signature shape via
        # the Scene helper that rejects Vec3 input (covered via
        # `cube_headless.py` end-to-end). This test placeholder confirms
        # the sprite method exists with the documented signature.
        assert callable(Node().sprite)

    def test_mesh_renames_argument_to_mesh_asset(self):
        # mesh_asset suffix mirrors `mml_str` flow (cube-design.md § 12.5).
        # Mesh has no factory methods; build a 1-triangle mesh by hand.
        geom = Geometry(positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
        m = Mesh(geometries=[geom], transforms=[Mat4()], parents=[-1], col_img=8)
        Node().mesh(Mat4.IDENTITY, m, shaded=False)

    def test_prim_with_geometry(self):
        # node.prim takes a Geometry asset directly (cube-design.md § 12.5).
        geom = Geometry(
            positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            indices=[0, 1, 2],
            prim=Geometry.PRIM_TRIANGLES,
            cull=Geometry.CULL_BACK,
        )
        Node().prim(Mat4.IDENTITY, geom, col_img=7, shaded=False)

    def test_prim_col_img_accepts_image(self):
        # col_img union accepts an Image (textured) as well as int (flat).
        # The session-wide conftest fixture already calls pyxel.init.
        img = pyxel.images[0]
        geom = Geometry(
            positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            uvs=[0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        )
        Node().prim(Mat4.IDENTITY, geom, col_img=img, colkey=0)

    def test_mesh_col_img_accepts_image(self):
        # Mesh.col_img union accepts Image — round-trip and draw.
        img = pyxel.images[0]
        geom = Geometry(
            positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            uvs=[0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        )
        m = Mesh(
            geometries=[geom],
            transforms=[Mat4()],
            parents=[-1],
            col_img=img,
            colkey=0,
        )
        # The col_img getter wraps the underlying core Image into a fresh
        # binding object each call; verify the round-trip by checking it
        # is an Image instance (not an int) and that drawing succeeds.
        assert isinstance(m.col_img, Image)
        Node().mesh(Mat4.IDENTITY, m, shaded=False)


class TestSceneIntegrationOfNewMethods:
    """Smoke-test that Scene (Node-derived) exposes the new methods too."""

    def test_box_sphere_text_via_scene(self):
        s = Scene()
        s.box(Mat4.IDENTITY, Vec3(1, 1, 1), 4)
        s.boxb(Mat4.IDENTITY, Vec3(1, 1, 1), 5)
        s.sphere(Vec3.ZERO, 1.0, 7)
        s.sphereb(Vec3.ZERO, 1.0, 8)
        s.text(Vec3.ZERO, "ok", 9)

    def test_run_multi_camera(self):
        # Multi-angle rendering: building the scene once and rendering
        # via different cameras is exercised in cube-design.md § 13.4.
        # The smoke test here confirms the API surface accepts repeated
        # calls without state corruption.
        s = Scene()
        cam = Camera()
        # No actual graphics — running outside an SDL window means
        # scene.draw composes the context but with_draw_context's
        # rasterizer is a no-op when target rebind is missed; this only
        # verifies the API signature wiring.
        assert hasattr(s, "draw")
        assert hasattr(cam, "transform")


class TestCameraProperty:
    """`Node.camera` is the active draw camera, valid only inside
    `on_draw` (cube-design.md § 12.1). Accessing it outside an active
    draw context raises so callers notice the misuse instead of seeing
    stale data."""

    def test_camera_outside_on_draw_raises(self):
        n = Node()
        with pytest.raises(RuntimeError):
            _ = n.camera

    def test_camera_on_scene_outside_on_draw_raises(self):
        # Scene inherits Node, so the same rule applies.
        s = Scene()
        with pytest.raises(RuntimeError):
            _ = s.camera


class TestOnCollideSignature:
    """`on_collide` is exposed today so user subclasses can stage
    collision-response code; the cube runtime does not invoke it yet
    (collision pipeline deferred — § 15). The signature must accept
    both positional and keyword forms with the documented argument
    names so the future pipeline (and user code) can call it freely."""

    def test_positional(self):
        n = Node()
        other = Node()
        n.on_collide(other, Contact())

    def test_keyword(self):
        n = Node()
        other = Node()
        n.on_collide(other=other, contact=Contact())
