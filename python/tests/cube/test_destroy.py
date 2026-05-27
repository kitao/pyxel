from pyxel.cube import Node, Scene


class DestroyTracker(Node):
    fire_log: list[str] = []  # class-level so siblings can write to a shared log

    # PyO3 Node.__new__ accepts no positional arguments; the override
    # below routes any subclass-construction args away from the base
    # __new__ so users can pass per-instance data through __init__.
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, label: str):
        super().__init__()
        self.label = label

    def on_destroy(self):
        DestroyTracker.fire_log.append(self.label)


def setup_scene_with_subtree() -> tuple[Scene, DestroyTracker, DestroyTracker, DestroyTracker]:
    DestroyTracker.fire_log = []
    scene = Scene()
    root = DestroyTracker("root")
    mid = DestroyTracker("mid")
    leaf = DestroyTracker("leaf")
    scene.add_child(root)
    root.add_child(mid)
    mid.add_child(leaf)
    return scene, root, mid, leaf


def test_destroyed_flag_set_immediately_but_not_detached():
    scene, root, mid, leaf = setup_scene_with_subtree()
    mid.destroy()
    assert mid.destroyed is True
    assert leaf.destroyed is True
    assert root.destroyed is False
    # Tree intact until Scene.update step 8.
    assert len(root.children) == 1
    assert len(mid.children) == 1


def test_scene_update_fires_on_destroy_post_order_then_detaches():
    scene, root, mid, _leaf = setup_scene_with_subtree()
    mid.destroy()
    scene.update()
    # Post-order: leaf first, then mid.
    assert DestroyTracker.fire_log == ["leaf", "mid"]
    assert len(root.children) == 0


def test_destroy_on_subtree_does_not_destroy_scene():
    scene, root, _mid, _leaf = setup_scene_with_subtree()
    root.destroy()
    scene.update()
    assert scene.destroyed is False
    assert len(scene.children) == 0
