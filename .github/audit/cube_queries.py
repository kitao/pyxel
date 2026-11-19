from pyxel.cube import Collider, Mat4, Node, Vec3

sphere = Node()
sphere.collider = Collider(radius=1)
limit = 2**24
origin = Vec3(-limit, 0, 0)
sphere.transform = Mat4.from_translation(Vec3(0.75, 0.75, 0))
for query in (sphere.raycast, sphere.raycast_all):
    assert not query(origin, Vec3.RIGHT, max_distance=limit - 1)
    result = query(origin, Vec3.RIGHT, max_distance=limit)
    hit = result[0] if isinstance(result, list) else result
    assert hit.node is sphere and hit.distance == limit

sphere.transform = Mat4.from_translation(Vec3(3, 0, 0))
assert sphere.raycast(origin, Vec3.RIGHT, max_distance=limit) is None

sphere.transform = Mat4.IDENTITY
origin = Vec3(-(2**18), 1 + 2**-23, 0)
assert sphere.raycast(origin, Vec3.RIGHT) is None
direction = Vec3(1, -(2**-40), 0)
assert sphere.raycast(origin, direction).node is sphere
assert [hit.node for hit in sphere.raycast_all(origin, direction)] == [sphere]
print("The candidate wheel preserves the distance boundary and detects shallow rays.")
