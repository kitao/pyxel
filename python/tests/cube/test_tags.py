import gc

import pytest
from pyxel.cube import Collider, Mat4, Node, Vec3


@pytest.mark.parametrize("tag_set", [set, frozenset])
def test_tag_edits_change_search_and_every_spatial_filter(tag_set):
    root, target = Node(), Node()
    target.collider = Collider(radius=1)
    root.add_child(target)
    tags = target.tags
    tags.add("enemy")
    tags.add("enemy")
    assert len(tags) == 1
    assert tags == {"enemy"}
    search_tags = tag_set({"enemy"})
    assert root.find_by_tags(search_tags) == [target]
    assert root.find_by_tags(tags) == [target]
    assert root.raycast(Vec3(0, 0, 3), Vec3.FORWARD, tags=search_tags).node is target
    assert [
        hit.node
        for hit in root.raycast_all(Vec3(0, 0, 3), Vec3.FORWARD, tags=search_tags)
    ] == [target]
    assert root.overlap_sphere(Vec3.ZERO, 2, tags=tags) == [target]
    assert root.overlap_box(Mat4.IDENTITY, Vec3.ONE, tags=search_tags) == [target]
    assert root.overlap_sphere(Vec3.ZERO, 2, tags=tag_set()) == []

    tags.discard("enemy")
    assert root.find_by_tags(search_tags) == []
    assert root.overlap_sphere(Vec3.ZERO, 2, tags=search_tags) == []


def test_set_views_follow_reassignment_and_copies_are_independent():
    node = Node()
    node.tags = {"enemy"}
    view = node.tags
    copied = view.copy()
    node.tags = {"boss"}
    assert view == {"boss"}
    assert copied == {"enemy"}
    copied.add("copy-only")
    assert "copy-only" not in view
    node.tags = view
    assert node.tags == {"boss"}
    other = Node()
    other.tags = view
    view.add("source-only")
    assert other.tags == {"boss"}
    del node
    gc.collect()
    view.add("still-live")
    assert "still-live" in view


def test_set_operations_and_comparisons():
    node, other = Node(), Node()
    node.tags = {"a", "b"}
    other.tags = {"b", "c"}
    assert node.tags | other.tags == {"a", "b", "c"}
    assert {"b", "c"} | node.tags == {"a", "b", "c"}
    assert node.tags & other.tags == {"b"}
    assert {"b", "c"} & node.tags == {"b"}
    assert node.tags - other.tags == {"a"}
    assert {"b", "c"} - node.tags == {"c"}
    assert node.tags ^ other.tags == {"a", "c"}
    assert {"b", "c"} ^ node.tags == {"a", "c"}
    assert isinstance(frozenset({"c"}) | node.tags, frozenset)
    assert node.tags == {"a", "b"}
    assert {"a", "b"} == node.tags
    assert node.tags != other.tags
    assert node.tags > {"a"}
    assert node.tags >= {"a", "b"}
    assert node.tags < {"a", "b", "c"}
    assert node.tags <= {"a", "b"}
    assert node.tags != ["a", "b"]
    assert node.tags.isdisjoint(["c"])
    assert node.tags.issubset(["a", "b", "c"])
    assert node.tags.issuperset(["a"])
    assert node.tags.union(["c"], ["d"]) == {"a", "b", "c", "d"}
    assert node.tags.intersection(["b", "c"], ["b"]) == {"b"}
    assert node.tags.difference(["b"], ["c"]) == {"a"}
    assert node.tags.symmetric_difference(["b", "c"]) == {"a", "c"}
    assert node.tags.union() == node.tags.intersection() == node.tags.difference()
    assert set(node.tags) == {"a", "b"}
    with pytest.raises(TypeError):
        hash(node.tags)


def test_binary_operation_allows_the_other_operands_reflected_method():
    class Other:
        def __ror__(self, other):
            return ("reflected", set(other))

    node = Node()
    node.tags = {"enemy"}
    assert node.tags | Other() == ("reflected", {"enemy"})


def test_set_mutation_methods_and_augmented_assignment():
    node = Node()
    assert not node.tags
    assert repr(node.tags) == "set()"
    node.tags.update(["a", "a"], ("b",))
    node.tags |= {"c"}
    assert node.tags == {"a", "b", "c"}
    node.tags &= {"b", "c", "d"}
    node.tags -= {"b"}
    node.tags ^= {"c", "d"}
    assert node.tags == {"d"}
    node.tags.update(["a", "b", "c"])
    node.tags.difference_update(["a"], ["d"])
    node.tags.intersection_update(["b", "c"], ["b"])
    node.tags.symmetric_difference_update(["b", "b", "c"])
    assert node.tags == {"c"}
    assert node.tags.pop() == "c"
    with pytest.raises(KeyError):
        node.tags.pop()
    with pytest.raises(KeyError):
        node.tags.remove("missing")
    node.tags.discard("missing")
    node.tags.add("present")
    node.tags.remove("present")
    node.tags.add("a")
    node.tags.clear()
    assert node.tags == set()


def test_update_resolves_python_iterators_before_borrowing_node():
    node = Node()

    class Tags:
        def __iter__(self):
            node.tags.add("side-effect")
            yield "new"

    node.tags.update(Tags())
    assert node.tags == {"side-effect", "new"}
