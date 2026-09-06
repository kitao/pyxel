import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]


class TestTilemapCreation:
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

    def test_set_data(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.set(0, 0, ["0001 0002", "0003 0004"])
        assert tm.pget(0, 0) == (0, 1)
        assert tm.pget(1, 0) == (0, 2)
        assert tm.pget(0, 1) == (0, 3)
        assert tm.pget(1, 1) == (0, 4)


class TestTilemapDrawing:
    @pytest.mark.parametrize(
        "operation", ["bltm", "bltm3d", "image_bltm", "image_bltm3d"]
    )
    def test_invalid_imgsrc_raises(self, operation):
        tilemap = pyxel.Tilemap(1, 1, 999)
        image = pyxel.Image(1, 1)
        with raises_exact(ValueError, "imgsrc references an invalid image index"):
            if operation == "bltm":
                pyxel.bltm(0, 0, tilemap, 0, 0, 1, 1)
            elif operation == "bltm3d":
                pyxel.bltm3d(0, 0, 1, 1, tilemap, (0, 0, 1), (0, 0, 0))
            elif operation == "image_bltm":
                image.bltm(0, 0, tilemap, 0, 0, 1, 1)
            else:
                image.bltm3d(0, 0, 1, 1, tilemap, (0, 0, 1), (0, 0, 0))

    def test_line(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.line(0, 0, 15, 0, (1, 1))
        assert tm.pget(0, 0) == (1, 1)
        assert tm.pget(8, 0) == (1, 1)
        assert tm.pget(15, 0) == (1, 1)
        assert tm.pget(0, 1) == (0, 0)

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
        assert tm.pget(4, 4) == (0, 0)

    def test_circ(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.circ(16, 16, 5, (2, 2))
        assert tm.pget(16, 16) == (2, 2)

    def test_circb(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.circb(16, 16, 5, (2, 2))
        assert tm.pget(16, 11) == (2, 2)
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
        assert tm.pget(16, 8) == (1, 1)
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
        assert tm.pget(8, 0) == (4, 4)
        assert tm.pget(0, 15) == (4, 4)
        assert tm.pget(15, 15) == (4, 4)
        assert tm.pget(8, 8) == (0, 0)


class TestTilemapBlt:
    @pytest.mark.parametrize("rotate", [0, 30])
    @pytest.mark.parametrize("source_pos", [0, -1])
    def test_self_blit_matches_source_snapshot(self, rotate, source_pos):
        actual, expected, snapshot = [pyxel.Tilemap(8, 8, 0) for _ in range(3)]
        for tm in (actual, expected, snapshot):
            for y in range(8):
                for x in range(8):
                    tm.pset(x, y, (x, y))
        for dst, src in ((expected, snapshot), (actual, actual)):
            dst.blt(2, 2, src, source_pos, source_pos, -4, 4, rotate=rotate)
        assert list(expected.data_ptr()) != list(snapshot.data_ptr())
        assert list(actual.data_ptr()) == list(expected.data_ptr())

    def test_blt_with_int(self):
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].pset(0, 0, (3, 3))
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        tm.blt(0, 0, 0, 0, 0, 8, 8)
        assert tm.pget(0, 0) == (3, 3)

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
        assert dst.pget(1, 0) == (3, 3)

    def test_blt_with_rotate(self):
        src = pyxel.Tilemap(8, 8, 0)
        src.cls((0, 0))
        src.rect(0, 0, 8, 8, (1, 1))
        dst = pyxel.Tilemap(16, 16, 0)
        dst.cls((0, 0))
        dst.blt(4, 4, src, 0, 0, 8, 8, rotate=45)
        # (3, 7) is outside the unrotated 8x8 box and painted only when rotated
        assert dst.pget(3, 7) == (1, 1)

    def test_blt_with_scale(self):
        src = pyxel.Tilemap(8, 8, 0)
        src.cls((0, 0))
        src.pset(0, 0, (5, 5))
        dst = pyxel.Tilemap(16, 16, 0)
        dst.cls((0, 0))
        dst.blt(0, 0, src, 0, 0, 1, 1, scale=4)
        # At the origin, clipping leaves four painted tiles.
        drawn = sum(1 for x in range(8) for y in range(8) if dst.pget(x, y) == (5, 5))
        assert drawn == 4


class TestTilemapState:
    def test_clip_restricts_drawing(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.clip(4, 4, 8, 8)
        tm.rect(0, 0, 16, 16, (1, 1))
        tm.clip()
        assert tm.pget(0, 0) == (0, 0)
        assert tm.pget(6, 6) == (1, 1)

    def test_camera_offsets_drawing(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.camera(10, 10)
        tm.pset(10, 10, (3, 3))
        tm.camera()
        assert tm.pget(0, 0) == (3, 3)


class TestTilemapIO:
    def test_from_tmx(self, assets_dir):
        tm = pyxel.Tilemap.from_tmx(str(assets_dir / "urban_rpg.tmx"), 0)
        assert tm.width == 58
        assert tm.height == 32
        has_nonzero = any(tm.pget(x, 0) != (0, 0) for x in range(tm.width))
        assert has_nonzero

    def test_from_tmx_masks_tile_flip_flags(self, tmp_path):
        # 2147483650 combines the horizontal flip bit (1 << 31) with tile GID 2.
        tmx_path = tmp_path / "flipped.tmx"
        tmx_path.write_text(
            """<?xml version="1.0" encoding="UTF-8"?>
<map tilewidth="8" tileheight="8">
  <tileset firstgid="1" columns="3"/>
  <layer width="1" height="1">
    <data encoding="csv">2147483650</data>
  </layer>
</map>
""",
            encoding="utf-8",
        )

        tm = pyxel.Tilemap.from_tmx(str(tmx_path), 0)

        assert tm.pget(0, 0) == (1, 0)

    @pytest.mark.parametrize(
        ("columns", "encoding", "layer", "message"),
        [
            (0, "csv", 0, "Invalid tileset columns in file"),
            (1, "base64", 0, "Unsupported encoding in file"),
            (1, "csv", 1, "Layer 1 not found in file"),
        ],
    )
    def test_from_tmx_rejects_invalid_configuration(
        self, tmp_path, columns, encoding, layer, message
    ):
        tmx_path = tmp_path / "invalid.tmx"
        tmx_path.write_text(
            f"""<?xml version="1.0" encoding="UTF-8"?>
<map tilewidth="8" tileheight="8">
  <tileset firstgid="1" columns="{columns}"/>
  <layer width="1" height="1">
    <data encoding="{encoding}">1</data>
  </layer>
</map>
""",
            encoding="utf-8",
        )

        with raises_exact(Exception, f"{message} '{tmx_path}'"):
            pyxel.Tilemap.from_tmx(str(tmx_path), layer)

    def test_from_tmx_rejects_zero_layer_width(self, tmp_path):
        tmx_path = tmp_path / "zero_width.tmx"
        tmx_path.write_text(
            """<?xml version="1.0" encoding="UTF-8"?>
<map tilewidth="8" tileheight="8">
  <tileset firstgid="1" columns="1"/>
  <layer width="0" height="1">
    <data encoding="csv">1</data>
  </layer>
</map>
""",
            encoding="utf-8",
        )

        with raises_exact(Exception, f"Invalid layer dimensions in file '{tmx_path}'"):
            pyxel.Tilemap.from_tmx(str(tmx_path), 0)

    def test_from_tmx_rejects_tile_count_mismatch(self, tmp_path):
        tmx_path = tmp_path / "wrong_tile_count.tmx"
        tmx_path.write_text(
            """<?xml version="1.0" encoding="UTF-8"?>
<map tilewidth="8" tileheight="8">
  <tileset firstgid="1" columns="1"/>
  <layer width="2" height="1">
    <data encoding="csv">1</data>
  </layer>
</map>
""",
            encoding="utf-8",
        )

        with raises_exact(
            Exception,
            f"Layer data size does not match dimensions in file '{tmx_path}'",
        ):
            pyxel.Tilemap.from_tmx(str(tmx_path), 0)

    def test_load_tmx(self, assets_dir):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.load(0, 0, str(assets_dir / "urban_rpg.tmx"), 0)
        has_nonzero = any(tm.pget(x, 0) != (0, 0) for x in range(32))
        assert has_nonzero


class TestTilemapDataPtr:
    def test_data_ptr_keeps_tilemap_alive(self):
        tm = pyxel.Tilemap(2, 2, 0)
        ptr = tm.data_ptr()

        assert ptr._pyxel_owner is tm

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
        # Each tile is 2 u16 values, row stride = width * 2.
        offset = 4 * 2
        assert ptr[offset] == 7
        assert ptr[offset + 1] == 8


class TestTilemapCollide:
    def test_collide_no_walls(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        result = tm.collide(0, 0, 8, 8, 5.0, 5.0, [])
        assert isinstance(result, tuple)
        assert result == (5.0, 5.0)
        assert isinstance(result[0], float)
        assert isinstance(result[1], float)

    def test_collide_horizontal_wall(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall_tile = (1, 0)
        tm.pset(2, 0, wall_tile)
        dx, dy = tm.collide(0, 0, 8, 8, 100.0, 0.0, [wall_tile])
        # The entity ends at x=8; the wall starts at x=16.
        assert dx == 8.0
        assert dy == 0.0

    def test_collide_vertical_wall(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall_tile = (1, 0)
        tm.pset(0, 2, wall_tile)
        dx, dy = tm.collide(0, 0, 8, 8, 0.0, 100.0, [wall_tile])
        assert dy == 8.0
        assert dx == 0.0

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
        # The entity's leading edge starts at 8 and each wall starts at 16
        dx, _ = tm.collide(0, 0, 8, 8, 100.0, 0.0, [wall1, wall2])
        assert dx == 8.0
        _, dy = tm.collide(0, 0, 8, 8, 0.0, 100.0, [wall1, wall2])
        assert dy == 8.0

    def test_collide_negative_direction(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall_tile = (1, 0)
        tm.pset(0, 0, wall_tile)  # Wall at origin
        # Moving left into the wall starts from position (24, 0).
        dx, dy = tm.collide(24, 0, 8, 8, -100.0, 0.0, [wall_tile])
        assert dx == -16.0  # Stopped at the wall's right edge (8)
        assert dy == 0.0

    def test_collide_empty_walls_list(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        tm.pset(1, 0, (1, 0))  # Tile exists but not in walls list
        dx, dy = tm.collide(0, 0, 8, 8, 100.0, 0.0, [])
        assert dx == 100.0
        assert dy == 0.0


class TestTilemapDeprecatedProperties:
    def test_image_getter_returns_image_deprecated(self, capfd):
        tm = pyxel.Tilemap(8, 8, 0)
        result = tm.image  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Image)
        out = capfd.readouterr().out
        assert out == "Tilemap.image is deprecated. Use Tilemap.imgsrc instead.\n"

    def test_image_setter_deprecated(self, capfd):
        tm = pyxel.Tilemap(8, 8, 0)
        new_img = pyxel.Image(256, 256)
        tm.image = new_img  # type: ignore[attr-defined]
        assert isinstance(tm.imgsrc, pyxel.Image)
        new_img.pset(0, 0, 7)
        assert tm.imgsrc.pget(0, 0) == 7
        out = capfd.readouterr().out
        assert out == "Tilemap.image is deprecated. Use Tilemap.imgsrc instead.\n"

    def test_refimg_getter_returns_index_or_none_deprecated(self, capfd):
        tm = pyxel.Tilemap(8, 8, 0)
        result = tm.refimg  # type: ignore[attr-defined]
        assert result == 0
        tm.imgsrc = pyxel.Image(8, 8)
        assert tm.refimg is None  # type: ignore[attr-defined]
        out = capfd.readouterr().out
        assert out == "Tilemap.refimg is deprecated. Use Tilemap.imgsrc instead.\n"

    def test_refimg_setter_deprecated(self, capfd):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.refimg = 1  # type: ignore[attr-defined]
        assert tm.imgsrc == 1
        out = capfd.readouterr().out
        assert out == "Tilemap.refimg is deprecated. Use Tilemap.imgsrc instead.\n"
