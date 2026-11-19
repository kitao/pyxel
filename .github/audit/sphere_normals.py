from pyxel.cube import Collider, Node, Vec3

for radius in (2.0**-90, 2.0**-44, 1.0, 2.0**45):
    target = Node()
    target.collider = Collider(radius=radius)
    for axis in range(3):
        for sign in (-1, 1):
            direction = [0, 0, 0]
            direction[axis] = sign
            ray = Vec3(*direction)
            for inside in (False, True):
                origin = Vec3.ZERO if inside else ray * (-4 * radius)
                expected = ray if inside else ray * -1
                hit = target.raycast(origin, ray)
                assert hit is not None and hit.normal == expected
                assert hit.point == expected * radius
print("Verified entry and exit normals for 48 axis-aligned sphere rays")
