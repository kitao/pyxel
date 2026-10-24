from typing import ClassVar

import pytest
from pyxel.cube import Node


class _DestroyTracker(Node):
    fire_log: ClassVar[list[str]] = []  # shared so siblings write one log

    def __init__(self, label: str):
        super().__init__()
        self.label = label

    def on_destroy(self):
        _DestroyTracker.fire_log.append(self.label)


def test_destroyed_flag_set_immediately_but_not_detached():
    _root_node, root, mid, leaf = _setup_root_with_subtree()
    mid.destroy()
    assert mid.destroyed is True
    assert leaf.destroyed is True
    assert root.destroyed is False
    # Parent and child links remain intact until deferred destruction runs.
    assert len(root.children) == 1
    assert len(mid.children) == 1


def test_update_fires_on_destroy_post_order_then_detaches():
    root_node, root, mid, leaf = _setup_root_with_subtree()
    mid.destroy()
    root_node.update()
    # Post-order: leaf first, then mid.
    assert _DestroyTracker.fire_log == ["leaf", "mid"]
    assert len(root.children) == 0
    assert mid.destroyed is True
    assert leaf.destroyed is True
    mid.update()
    leaf.update()
    assert _DestroyTracker.fire_log == ["leaf", "mid"]


def test_destroy_on_subtree_does_not_destroy_root():
    root_node, root, _mid, _leaf = _setup_root_with_subtree()
    root.destroy()
    root_node.update()
    assert root_node.destroyed is False
    assert len(root_node.children) == 0


def test_destroying_update_root_fires_once():
    _DestroyTracker.fire_log = []
    root = _DestroyTracker("root")
    root.destroy()
    root.update()
    root.update()
    assert _DestroyTracker.fire_log == ["root"]
    assert root.destroyed is True
    root.destroy()
    root.update()
    assert _DestroyTracker.fire_log == ["root"]


def test_direct_subtree_update_cleans_up_under_inactive_ancestor():
    root_node, root, _mid, _leaf = _setup_root_with_subtree()
    root_node.active = False
    root.destroy()
    root.update()
    assert _DestroyTracker.fire_log == ["leaf", "mid", "root"]
    assert root.parent is None
    assert root_node.children == ()
    assert root.destroyed


def test_on_destroy_reentering_update_is_not_notified_again():
    class Reentrant(Node):
        calls = 0

        def on_destroy(self):
            self.calls += 1
            if self.calls == 1:
                self.update()

    node = Reentrant()
    node.destroy()
    node.update()
    assert node.calls == 1
    assert node.destroyed


def test_callback_failure_does_not_repeat_already_delivered_notifications():
    log = []

    class Child(Node):
        def on_destroy(self):
            log.append(self.name)
            if self.name == "second":
                raise RuntimeError("on_destroy failed")

    root, first, second = Node(), Child(), Child()
    first.name, second.name = "first", "second"
    root.add_child(first)
    root.add_child(second)
    first.destroy()
    second.destroy()
    with pytest.raises(RuntimeError, match="on_destroy failed"):
        root.update()
    root.update()
    assert log == ["first", "second"]
    assert first.destroyed and second.destroyed
    assert root.children == ()


def _setup_root_with_subtree() -> tuple[
    Node, _DestroyTracker, _DestroyTracker, _DestroyTracker
]:
    _DestroyTracker.fire_log = []
    root_node = Node()
    root = _DestroyTracker("root")
    mid = _DestroyTracker("mid")
    leaf = _DestroyTracker("leaf")

    root_node.add_child(root)
    root.add_child(mid)
    mid.add_child(leaf)
    return root_node, root, mid, leaf
