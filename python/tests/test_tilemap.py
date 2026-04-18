from pathlib import Path

import pyxel


# Tilemap class
class TestTilemap:
    def test_new_with_int(self):
        tm = pyxel.Tilemap(32, 32, 0)
        assert tm.width == 32
        assert tm.height == 32

    def test_new_with_image_instance(self):
        img = pyxel.Image(256, 256)
        tm = pyxel.Tilemap(16, 16, img)
        assert tm.width == 16
        assert tm.height == 16

    def test_imgsrc_read_write_int(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.imgsrc = 1
        assert tm.imgsrc == 1
        tm.imgsrc = 0
        assert tm.imgsrc == 0

    def test_imgsrc_read_write_image(self):
        img = pyxel.Image(256, 256)
        tm = pyxel.Tilemap(8, 8, 0)
        tm.imgsrc = img
        assert isinstance(tm.imgsrc, pyxel.Image)

    def test_pset_pget_returns_tuple(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.pset(0, 0, (1, 2))
        result = tm.pget(0, 0)
        assert result == (1, 2)
        assert isinstance(result, tuple)
        assert len(result) == 2

    def test_pset_pget_multiple(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.pset(0, 0, (1, 2))
        tm.pset(1, 0, (3, 4))
        tm.pset(0, 1, (5, 6))
        assert tm.pget(0, 0) == (1, 2)
        assert tm.pget(1, 0) == (3, 4)
        assert tm.pget(0, 1) == (5, 6)

    def test_clear(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.pset(0, 0, (5, 5))
        tm.cls((0, 0))
        assert tm.pget(0, 0) == (0, 0)

    def test_blt_with_int(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.blt(0, 0, 0, 0, 0, 8, 8)

    def test_blt_with_tilemap_instance(self):
        src = pyxel.Tilemap(8, 8, 0)
        src.cls((0, 0))
        src.pset(0, 0, (3, 3))
        dst = pyxel.Tilemap(8, 8, 0)
        dst.cls((0, 0))
        dst.blt(0, 0, src, 0, 0, 8, 8)
        assert dst.pget(0, 0) == (3, 3)

    def test_blt_with_tilekey(self):
        src = pyxel.Tilemap(8, 8, 0)
        src.cls((0, 0))
        src.pset(1, 0, (3, 3))
        dst = pyxel.Tilemap(8, 8, 0)
        dst.cls((1, 1))
        dst.blt(0, 0, src, 0, 0, 8, 8, tilekey=(0, 0))
        # (0,0) tiles in src are transparent, dst retains original
        assert dst.pget(0, 0) == (1, 1)
        # Non-tilekey tiles are copied
        assert dst.pget(1, 0) == (3, 3)

    def test_from_tmx(self, assets_dir):
        tm = pyxel.Tilemap.from_tmx(str(Path(assets_dir) / "urban_rpg.tmx"), 0)
        assert tm.width > 0
        assert tm.height > 0
        # Verify the tilemap has some non-zero content
        has_content = any(tm.pget(x, 0) != (0, 0) for x in range(tm.width))
        assert has_content

    def test_set_data(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.set(0, 0, ["0001 0002", "0003 0004"])
        assert tm.pget(0, 0) == (0, 1)
        assert tm.pget(1, 0) == (0, 2)
        assert tm.pget(0, 1) == (0, 3)
        assert tm.pget(1, 1) == (0, 4)

    def test_load_tmx(self, assets_dir):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.load(0, 0, str(Path(assets_dir) / "urban_rpg.tmx"), 0)
        has_nonzero = any(tm.pget(x, 0) != (0, 0) for x in range(32))
        assert has_nonzero

    def test_line(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.line(0, 0, 15, 0, (1, 1))
        assert tm.pget(0, 0) == (1, 1)
        assert tm.pget(8, 0) == (1, 1)
        assert tm.pget(15, 0) == (1, 1)
        assert tm.pget(0, 1) == (0, 0)  # Below line

    def test_rect(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.rect(2, 2, 4, 4, (1, 2))
        assert tm.pget(3, 3) == (1, 2)
        assert tm.pget(0, 0) == (0, 0)

    def test_rectb(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.rectb(2, 2, 6, 6, (3, 3))
        assert tm.pget(2, 2) == (3, 3)
        assert tm.pget(4, 4) == (0, 0)  # Inside hollow

    def test_circ(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.circ(16, 16, 5, (2, 2))
        assert tm.pget(16, 16) == (2, 2)

    def test_circb(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.circb(16, 16, 5, (2, 2))
        assert tm.pget(16, 16) == (0, 0)

    def test_elli(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.elli(8, 8, 16, 8, (1, 1))
        assert tm.pget(16, 12) == (1, 1)

    def test_ellib(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.ellib(8, 8, 16, 8, (1, 1))
        assert tm.pget(16, 12) == (0, 0)

    def test_tri(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.tri(8, 0, 0, 15, 15, 15, (4, 4))
        assert tm.pget(8, 8) == (4, 4)

    def test_trib(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.trib(8, 0, 0, 15, 15, 15, (4, 4))
        assert tm.pget(8, 8) == (0, 0)

    def test_fill(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.rect(2, 2, 8, 8, (5, 5))
        tm.fill(4, 4, (9, 9))
        assert tm.pget(4, 4) == (9, 9)

    def test_clip_restricts_drawing(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.clip(4, 4, 8, 8)
        tm.rect(0, 0, 16, 16, (1, 1))
        tm.clip()
        assert tm.pget(0, 0) == (0, 0)  # Outside clip
        assert tm.pget(6, 6) == (1, 1)  # Inside clip

    def test_camera_offsets_drawing(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.camera(10, 10)
        tm.pset(10, 10, (3, 3))
        tm.camera()
        assert tm.pget(0, 0) == (3, 3)

    def test_data_ptr_read(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        tm.pset(0, 0, (1, 2))
        ptr = tm.data_ptr()
        assert ptr[0] == 1
        assert ptr[1] == 2

    def test_data_ptr_write(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        ptr = tm.data_ptr()
        ptr[0] = 5
        ptr[1] = 6
        assert tm.pget(0, 0) == (5, 6)

    def test_data_ptr_row_stride(self):
        tm = pyxel.Tilemap(4, 4, 0)
        tm.cls((0, 0))
        tm.pset(0, 1, (7, 8))
        ptr = tm.data_ptr()
        # Each tile is 2 u16 values, row stride = width * 2
        offset = 4 * 2  # width=4, each tile=2 entries
        assert ptr[offset] == 7
        assert ptr[offset + 1] == 8

    def test_blt_with_rotate(self):
        src = pyxel.Tilemap(8, 8, 0)
        dst = pyxel.Tilemap(8, 8, 0)
        dst.blt(0, 0, src, 0, 0, 8, 8, rotate=45)

    def test_blt_with_scale(self):
        src = pyxel.Tilemap(8, 8, 0)
        dst = pyxel.Tilemap(8, 8, 0)
        dst.blt(0, 0, src, 0, 0, 8, 8, scale=2)


class TestTilemapCollide:
    def test_collide_no_walls(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        dx, dy = tm.collide(0, 0, 8, 8, 5.0, 5.0, [])
        assert dx == 5.0
        assert dy == 5.0

    def test_collide_returns_tuple_of_floats(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        result = tm.collide(0, 0, 8, 8, 1.0, 1.0, [])
        assert isinstance(result, tuple)
        assert len(result) == 2
        assert isinstance(result[0], float)
        assert isinstance(result[1], float)

    def test_collide_horizontal_wall(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall_tile = (1, 0)
        tm.pset(2, 0, wall_tile)  # Wall at tile (2, 0) = pixel x=16
        dx, dy = tm.collide(0, 0, 8, 8, 100.0, 0.0, [wall_tile])
        # Should stop before the wall
        assert 0 < dx < 100.0
        # Precise: entity width=8, wall at x=16, so dx should be 8.0
        assert dx == 8.0

    def test_collide_vertical_wall(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall_tile = (1, 0)
        tm.pset(0, 2, wall_tile)  # Wall at tile (0, 2) = pixel y=16
        dx, dy = tm.collide(0, 0, 8, 8, 0.0, 100.0, [wall_tile])
        assert 0 < dy < 100.0
        assert dy == 8.0

    def test_collide_no_movement(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        dx, dy = tm.collide(0, 0, 8, 8, 0.0, 0.0, [(1, 0)])
        assert dx == 0.0
        assert dy == 0.0

    def test_collide_multiple_wall_tiles(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall1 = (1, 0)
        wall2 = (2, 0)
        tm.pset(2, 0, wall1)
        tm.pset(0, 2, wall2)
        dx, _ = tm.collide(0, 0, 8, 8, 100.0, 0.0, [wall1, wall2])
        assert dx < 100.0
        _, dy = tm.collide(0, 0, 8, 8, 0.0, 100.0, [wall1, wall2])
        assert dy < 100.0

    def test_collide_negative_direction(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall_tile = (1, 0)
        tm.pset(0, 0, wall_tile)  # Wall at origin
        # Moving left into wall from position (24, 0)
        dx, dy = tm.collide(24, 0, 8, 8, -100.0, 0.0, [wall_tile])
        assert dx > -100.0  # Stopped before reaching full distance
        assert dy == 0.0

    def test_collide_empty_walls_list(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        tm.pset(1, 0, (1, 0))  # Tile exists but not in walls list
        dx, dy = tm.collide(0, 0, 8, 8, 100.0, 0.0, [])
        assert dx == 100.0  # No collision
