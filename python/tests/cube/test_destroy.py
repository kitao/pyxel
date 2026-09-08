from typing import ClassVar

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
    root_node, root, mid, _leaf = _setup_root_with_subtree()
    mid.destroy()
    root_node.update()
    # Post-order: leaf first, then mid.
    assert _DestroyTracker.fire_log == ["leaf", "mid"]
    assert len(root.children) == 0


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
    assert root.destroyed is False


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
