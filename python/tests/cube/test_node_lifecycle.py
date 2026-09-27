import pytest
from pyxel import Image
from pyxel.cube import Camera, Collider, Node, Vec3


class HookNode(Node):
    def __init__(self, name, events):
        self.name = name
        self.events = events
        self.action = None

    def visit(self):
        self.events.append(self.name)
        action, self.action = self.action, None
        if action:
            action()

    def on_update(self):
        self.visit()

    def on_draw(self):
        self.visit()


@pytest.fixture(params=["update", "draw"])
def phase(request):
    def run(root):
        if request.param == "update":
            root.update()
        else:
            root.camera = Camera()
            root.draw(0, 0, 8, 8, Image(8, 8))

    return run, "active" if request.param == "update" else "visible"


def test_reparented_node_runs_once_in_original_order(phase):
    run, _ = phase
    events = []
    root, a, b = [HookNode(name, events) for name in ["root", "a", "b"]]
    root.add_child(a)
    root.add_child(b)
    a.action = lambda: b.add_child(a)
    run(root)
    assert events == ["root", "a", "b"]
    assert a.parent is b

    events.clear()
    run(root)
    assert events == ["root", "b", "a"]


def test_detached_queued_node_is_skipped(phase):
    run, _ = phase
    events = []
    root, a, b = [HookNode(name, events) for name in ["root", "a", "b"]]
    root.add_child(a)
    root.add_child(b)
    a.action = lambda: root.remove_child(b)
    run(root)
    assert events == ["root", "a"]
    assert b.parent is None


def test_new_child_waits_until_next_matching_phase(phase):
    run, _ = phase
    events = []
    root, child = [HookNode(name, events) for name in ["root", "new"]]
    root.action = lambda: root.add_child(child)
    run(root)
    assert events == ["root"]

    events.clear()
    run(root)
    assert events == ["root", "new"]


def test_disabling_ancestor_stops_queued_descendants(phase):
    run, flag = phase
    events = []
    root, child = [HookNode(name, events) for name in ["root", "child"]]
    root.add_child(child)
    root.action = lambda: setattr(root, flag, False)
    run(root)
    assert events == ["root"]


def test_earlier_callback_can_enable_initially_disabled_branch(phase):
    run, flag = phase
    events = []
    root, a, branch, leaf = [
        HookNode(name, events) for name in ["root", "a", "branch", "leaf"]
    ]
    root.add_child(a)
    root.add_child(branch)
    branch.add_child(leaf)
    setattr(branch, flag, False)
    a.action = lambda: setattr(branch, flag, True)
    run(root)
    assert events == ["root", "a", "branch", "leaf"]


def test_new_update_child_can_join_later_physics_phase():
    events = []
    root, child = [HookNode(name, events) for name in ["root", "new"]]
    child.collider = Collider(velocity=Vec3.RIGHT, mass=0)
    root.action = lambda: root.add_child(child)
    root.update()
    assert events == ["root"]
    assert child.transform.pos == Vec3.RIGHT
