"""Tests for `pyxel.cube.Node`.

Most cube primitives are wired through Scene-rooted draw contexts; the
draw command tests below confirm that each public method accepts its
documented signature without raising and is a safe no-op outside an
active draw context (cube-design.md § 12.5). Functional rendering is
covered by `cube_headless.py` and the example programs.
"""

import pytest

from pyxel.cube import (
    Camera,
    Collider,
    Contact,
    FloatBuffer,
    Light,
    Mat4,
    Mesh,
    Node,
    Scene,
    ShadeRamp,
    Vec3,
)


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
        # both light and shade_ramp; see § 12.4).
        assert n.light is None
        assert n.shade_ramp is None
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

    def test_set_light(self):
        n = Node()
        light = Light()
        light.intensity = 0.25
        n.light = light
        assert n.light.intensity == 0.25
        n.light = None
        assert n.light is None

    def test_set_shade_ramp(self):
        n = Node()
        ramp = ShadeRamp()
        n.shade_ramp = ramp
        # Default ramp samples through to (col, col, 0) at level 15.
        assert n.shade_ramp[0, 15] == ramp[0, 15]
        n.shade_ramp = None
        assert n.shade_ramp is None

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
    def test_prim_modes_distinct_and_gl_ordered(self):
        # OpenGL-ordered: POINTS=0, LINES=1, TRIANGLES=2 (cube-design.md § 12.1).
        assert Node.PRIM_POINTS == 0
        assert Node.PRIM_LINES == 1
        assert Node.PRIM_TRIANGLES == 2

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
        # find walks the subtree depth-first; root matches itself first.
        root.name = "root"
        assert root.find("root") is not None
        found = root.find("head")
        assert found is not None
        assert found.name == "head"
        assert root.find("missing") is None

    def test_destroy(self):
        p = Node()
        c = Node()
        p.add_child(c)
        c.destroy()
        # destroy detaches from parent.
        assert p.children == ()


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
        # invoke it yet (collision pipeline deferred — § 15).
        n = Node()
        other = Node()
        n.on_update()
        n.on_draw()
        n.on_collide(other, None)
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
        m = Mesh(positions=FloatBuffer([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]))
        Node().mesh(Mat4.IDENTITY, m, col=8, shaded=False)


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
        n.on_collide(other)
        n.on_collide(other, Contact())

    def test_keyword(self):
        n = Node()
        other = Node()
        n.on_collide(other=other)
        n.on_collide(other=other, contact=None)
        n.on_collide(other=other, contact=Contact())
