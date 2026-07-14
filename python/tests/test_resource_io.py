import zipfile
from pathlib import Path

import pytest

import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]


def _write_legacy_resource(path, entries):
    entries = dict(entries)
    with zipfile.ZipFile(path, "w") as zf:
        zf.writestr("pyxel_resource/version", entries.pop("version", "1.9.0"))
        for name, value in entries.items():
            zf.writestr(f"pyxel_resource/{name}", value)


def _write_resource(path, toml_text):
    with zipfile.ZipFile(path, "w") as zf:
        zf.writestr("pyxel_resource.toml", toml_text)


def _mark_zip_entry_encrypted(path, entry):
    data = bytearray(path.read_bytes())
    entry = entry.encode()

    local_patched = False
    offset = 0
    while (offset := data.find(b"PK\x03\x04", offset)) >= 0:
        name_length = int.from_bytes(data[offset + 26 : offset + 28], "little")
        name_start = offset + 30
        if data[name_start : name_start + name_length] == entry:
            flags = int.from_bytes(data[offset + 6 : offset + 8], "little") | 1
            data[offset + 6 : offset + 8] = flags.to_bytes(2, "little")
            local_patched = True
            break
        offset += 4

    central_patched = False
    offset = 0
    while (offset := data.find(b"PK\x01\x02", offset)) >= 0:
        name_length = int.from_bytes(data[offset + 28 : offset + 30], "little")
        name_start = offset + 46
        if data[name_start : name_start + name_length] == entry:
            flags = int.from_bytes(data[offset + 8 : offset + 10], "little") | 1
            data[offset + 8 : offset + 10] = flags.to_bytes(2, "little")
            central_patched = True
            break
        offset += 4

    assert local_patched and central_patched
    path.write_bytes(data)


class TestSaveLoad:
    def test_load_pyxres(self, assets_dir):
        pyxel.load(str(assets_dir / "sample.pyxres"))

    def test_load_old_format_pyxres(self, tmp_path):
        # Legacy text format: hex grids per bank under pyxel_resource/.
        path = tmp_path / "legacy.pyxres"
        with zipfile.ZipFile(path, "w") as zf:
            zf.writestr("pyxel_resource/version", "1.9.0")
            zf.writestr("pyxel_resource/image0", "78\n9a\n")
            zf.writestr("pyxel_resource/tilemap0", "0101\n")
            zf.writestr("pyxel_resource/sound00", "000c\n01\n73\n00\n20\n")
            zf.writestr("pyxel_resource/music0", "0001\nnone\nnone\nnone\n")

        pyxel.load(str(path))
        assert pyxel.images[0].pget(0, 0) == 7
        assert pyxel.images[0].pget(1, 0) == 8
        assert pyxel.images[0].pget(0, 1) == 9
        assert pyxel.images[0].pget(1, 1) == 10
        assert pyxel.tilemaps[0].pget(0, 0) == (1, 1)
        assert list(pyxel.sounds[0].notes) == [0, 12]
        assert list(pyxel.sounds[0].tones) == [0, 1]
        assert list(pyxel.sounds[0].volumes) == [7, 3]
        assert list(pyxel.sounds[0].effects) == [0, 0]
        assert pyxel.sounds[0].speed == 20
        assert list(pyxel.musics[0].seqs[0]) == [0, 1]

    @pytest.mark.parametrize(
        ("entries", "detail"),
        [
            (
                {"version": b"\xff"},
                "failed to read 'pyxel_resource/version' as UTF-8",
            ),
            ({"version": "not-a-version"}, "invalid version 'not-a-version'"),
            ({"version": "42949673.96"}, "invalid version '42949673.96'"),
            ({"version": "999.0"}, "unsupported version '999.0'"),
            (
                {"image0": b"\xff"},
                "failed to read 'pyxel_resource/image0' as UTF-8",
            ),
            (
                {"image0": "0g"},
                "invalid hexadecimal digit 'g' in 'pyxel_resource/image0' "
                "at line 1, column 2",
            ),
            (
                {"image0": "0あ"},
                "invalid hexadecimal digit 'あ' in 'pyxel_resource/image0' "
                "at line 1, column 2",
            ),
            (
                {"image0": "0\n" * 257},
                "too many image rows in 'pyxel_resource/image0': got 257, maximum 256",
            ),
            (
                {"tilemap0": "000"},
                "invalid tile width in 'pyxel_resource/tilemap0' at line 1: "
                "expected groups of 4 hexadecimal digits",
            ),
            (
                {"tilemap0": "00z0"},
                "invalid hexadecimal digit 'z' in 'pyxel_resource/tilemap0' "
                "at line 1, column 3",
            ),
            (
                {"tilemap0": "0000" * 257},
                "too many tiles in 'pyxel_resource/tilemap0' at line 1: "
                "got 257, maximum 256",
            ),
            (
                {"tilemap0": "0000\n" * 256 + "bad\n"},
                "invalid decimal value 'bad' in 'pyxel_resource/tilemap0' at line 257",
            ),
            (
                {"tilemap0": "0000\n" * 256 + "9999\n"},
                "image index 9999 in 'pyxel_resource/tilemap0' at line 257 "
                "is out of range 0..3",
            ),
            (
                {"sound00": "0g"},
                "invalid hexadecimal digit 'g' in 'pyxel_resource/sound00' "
                "at line 1, column 2",
            ),
            (
                {"sound00": "0"},
                "invalid value width in 'pyxel_resource/sound00' at line 1: "
                "expected groups of 2 hexadecimal digits",
            ),
            (
                {"sound00": "none\nnone\nnone\nnone\nfast\n"},
                "invalid decimal value 'fast' in 'pyxel_resource/sound00' at line 5",
            ),
            (
                {"music0": "00\n00\n00\n00\n00\n"},
                "too many music channels in 'pyxel_resource/music0': got 5, maximum 4",
            ),
        ],
    )
    def test_malformed_legacy_resource_has_exact_error(self, tmp_path, entries, detail):
        path = tmp_path / "malformed.pyxres"
        _write_legacy_resource(path, entries)

        with raises_exact(
            Exception, f"Failed to load legacy resource file '{path}': {detail}"
        ):
            pyxel.load(str(path))

    def test_malformed_legacy_resource_does_not_partially_commit(self, tmp_path):
        path = tmp_path / "partial.pyxres"
        _write_legacy_resource(path, {"image0": "1", "tilemap0": "g"})
        pyxel.images[0].cls(7)

        with pytest.raises(Exception):
            pyxel.load(str(path))

        assert pyxel.images[0].pget(0, 0) == 7

    @pytest.mark.parametrize("entry", ["version", "image0"])
    def test_unreadable_legacy_entry_is_not_treated_as_missing(self, tmp_path, entry):
        path = tmp_path / "encrypted-entry.pyxres"
        _write_legacy_resource(path, {"image0": "1"})
        archive_entry = f"pyxel_resource/{entry}"
        _mark_zip_entry_encrypted(path, archive_entry)

        with raises_exact(
            Exception,
            f"Failed to load legacy resource file '{path}': "
            f"failed to open '{archive_entry}'",
        ):
            pyxel.load(str(path))

    def test_invalid_sidecar_palette_does_not_partially_commit(self, tmp_path):
        path = tmp_path / "invalid-palette.pyxres"
        _write_legacy_resource(path, {"image0": "1"})
        path.with_suffix(".pyxpal").write_text("not-hex\n", encoding="utf-8")
        pyxel.images[0].cls(7)

        with pytest.raises(Exception):
            pyxel.load(str(path))

        assert pyxel.images[0].pget(0, 0) == 7

    def test_load_unsupported_format_version(self, tmp_path):
        # A pyxres written by a newer Pyxel must be rejected, not silently misread.
        path = tmp_path / "future.pyxres"
        with zipfile.ZipFile(path, "w") as zf:
            zf.writestr("pyxel_resource.toml", "format_version = 99\n")

        with raises_exact(Exception, "Unsupported resource format version '99'"):
            pyxel.load(str(path))

    @pytest.mark.parametrize(
        ("toml_text", "message"),
        [
            (
                "format_version = 1\n"
                "tilemaps = []\nsounds = []\nmusics = []\n"
                "[[images]]\nwidth = 1\nheight = 1\ndata = []\n",
                "Invalid resource data: images[0].data must not be empty",
            ),
            (
                "format_version = 1\n"
                "images = []\nsounds = []\nmusics = []\n"
                "[[tilemaps]]\nwidth = 1\nheight = 1\nimgsrc = 0\ndata = []\n",
                "Invalid resource data: tilemaps[0].data must not be empty",
            ),
            (
                "format_version = 1\n"
                "images = []\nsounds = []\nmusics = []\n"
                "[[tilemaps]]\nwidth = 1\nheight = 1\nimgsrc = 3\ndata = [[0, 0]]\n",
                "Invalid resource data: tilemaps[0].imgsrc 3 is out of range 0..3",
            ),
            (
                "format_version = 1\n"
                "images = []\ntilemaps = []\nmusics = []\n"
                "[[sounds]]\nnotes = []\ntones = []\nvolumes = []\neffects = []\nspeed = 0\n",
                "Invalid resource data: sounds[0].speed must be greater than 0",
            ),
        ],
    )
    def test_malformed_resource_has_exact_error(self, tmp_path, toml_text, message):
        path = tmp_path / "malformed-new.pyxres"
        _write_resource(path, toml_text)

        with raises_exact(Exception, message):
            pyxel.load(str(path))

    def test_malformed_resource_does_not_partially_commit(self, tmp_path):
        path = tmp_path / "partial-new.pyxres"
        _write_resource(
            path,
            "format_version = 1\n"
            "sounds = []\nmusics = []\n"
            "[[images]]\nwidth = 1\nheight = 1\ndata = [[1]]\n"
            "[[tilemaps]]\nwidth = 1\nheight = 1\nimgsrc = 0\ndata = []\n",
        )
        pyxel.images[0].cls(7)

        with pytest.raises(Exception):
            pyxel.load(str(path))

        assert pyxel.images[0].pget(0, 0) == 7

    def test_save_load_roundtrip(self, tmp_path):
        img = pyxel.images[0]
        img.cls(0)
        img.pset(0, 0, 7)
        img.pset(1, 0, 3)
        snd = pyxel.sounds[0]
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].pset(0, 0, (5, 5))
        pyxel.musics[0].set([0])

        path = str(tmp_path / "test.pyxres")
        pyxel.save(path)

        img.cls(0)
        snd.set("a2", "s", "7", "n", 5)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.musics[0].set([1, 2, 3])

        pyxel.load(path)
        assert pyxel.images[0].pget(0, 0) == 7
        assert pyxel.images[0].pget(1, 0) == 3
        assert list(pyxel.sounds[0].notes) == [24, 28, 31]
        assert pyxel.sounds[0].speed == 10
        assert pyxel.tilemaps[0].pget(0, 0) == (5, 5)
        assert list(pyxel.musics[0].seqs[0]) == [0]

    def test_save_exclude_images(self, tmp_path):
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 9)
        path = str(tmp_path / "test_excl_images.pyxres")
        pyxel.save(path, exclude_images=True)

        pyxel.images[0].cls(0)
        pyxel.load(path)
        assert pyxel.images[0].pget(0, 0) == 0

    def test_save_exclude_tilemaps(self, tmp_path):
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].pset(0, 0, (1, 1))
        path = str(tmp_path / "test_excl_tilemaps.pyxres")
        pyxel.save(path, exclude_tilemaps=True)

        pyxel.tilemaps[0].cls((0, 0))
        pyxel.load(path)
        assert pyxel.tilemaps[0].pget(0, 0) == (0, 0)

    def test_save_exclude_sounds(self, tmp_path):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        path = str(tmp_path / "test_excl_sounds.pyxres")
        pyxel.save(path, exclude_sounds=True)

        pyxel.sounds[0].set("a2", "s", "7", "n", 5)
        modified_notes = list(pyxel.sounds[0].notes)
        pyxel.load(path)
        assert list(pyxel.sounds[0].notes) == modified_notes

    def test_save_exclude_musics(self, tmp_path):
        pyxel.musics[0].set([0])
        path = str(tmp_path / "test_excl_musics.pyxres")
        pyxel.save(path, exclude_musics=True)

        pyxel.musics[0].set([0, 1, 2])
        modified_seq0 = list(pyxel.musics[0].seqs[0])
        pyxel.load(path)
        assert list(pyxel.musics[0].seqs[0]) == modified_seq0

    def test_save_exclude_multiple(self, tmp_path):
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 9)
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        path = str(tmp_path / "test_excl_multi.pyxres")
        pyxel.save(path, exclude_images=True, exclude_sounds=True)

        pyxel.images[0].cls(0)
        pyxel.sounds[0].set("a2", "s", "7", "n", 5)
        modified_notes = list(pyxel.sounds[0].notes)
        pyxel.load(path)
        assert pyxel.images[0].pget(0, 0) == 0
        assert list(pyxel.sounds[0].notes) == modified_notes

    def test_load_nonexistent_file_raises(self):
        with raises_exact(
            Exception, "Failed to open file '/nonexistent/path/file.pyxres'"
        ):
            pyxel.load("/nonexistent/path/file.pyxres")

    def test_save_creates_file(self, tmp_path):
        path = str(tmp_path / "new_file.pyxres")
        assert not Path(path).exists()
        pyxel.save(path)
        assert Path(path).exists()
        assert Path(path).stat().st_size > 0

    def test_excl_aliases_deprecated(self, capfd, tmp_path):
        # excl_* are the deprecated aliases; warning fires only once per session,
        # so test save and load in order.
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        path = str(tmp_path / "test_excl_dep.pyxres")
        pyxel.save(path, excl_images=True)  # type: ignore[call-arg]
        out = capfd.readouterr().out
        assert out == "excl_* options are deprecated. Use exclude_* instead.\n"

        pyxel.images[0].cls(0)
        pyxel.load(path, excl_images=True)  # type: ignore[call-arg]
        # excl_images=True excluded images on save, so load brings back nothing.
        assert pyxel.images[0].pget(0, 0) == 0


class TestPalette:
    def test_load_pal(self, assets_dir):
        original_colors = list(pyxel.colors)
        try:
            pyxel.load_pal(str(assets_dir / "audio_bgm.pyxpal"))
            # The bundled palette carries 32 colors, one hex value per line.
            assert len(pyxel.colors) == 32
        finally:
            pyxel.colors[:] = original_colors

    def test_load_pal_skips_whitespace_only_lines(self, tmp_path):
        backup_path = str(tmp_path / "backup.pyxpal")
        pyxel.save_pal(backup_path)
        try:
            pal_file = tmp_path / "test.pyxpal"
            pal_file.write_text("ff0000\n   \n00ff00\n")
            pyxel.load_pal(str(pal_file))
            assert pyxel.colors[0] == 0xFF0000
            assert pyxel.colors[1] == 0x00FF00
        finally:
            pyxel.load_pal(backup_path)

    def test_save_load_pal_roundtrip(self, tmp_path):
        original_colors = list(pyxel.colors)
        path = str(tmp_path / "test.pyxpal")
        pyxel.save_pal(path)

        pyxel.colors[0] = 0xFFFFFF
        pyxel.load_pal(path)
        assert list(pyxel.colors) == original_colors

    def test_save_pal_creates_file(self, tmp_path):
        path = str(tmp_path / "test_save_only.pyxpal")
        assert not Path(path).exists()
        pyxel.save_pal(path)
        assert Path(path).exists()
        assert Path(path).stat().st_size > 0


class TestScreenshot:
    def test_screenshot(self, tmp_path):
        pyxel.cls(7)
        pyxel.flip()
        path = str(tmp_path / "test_screenshot.png")
        pyxel.screenshot(path)
        assert Path(path).exists()
        assert Path(path).stat().st_size > 0

    def test_screenshot_with_scale(self, tmp_path):
        pyxel.cls(7)
        pyxel.flip()
        path1 = str(tmp_path / "test_s1.png")
        path2 = str(tmp_path / "test_s2.png")
        pyxel.screenshot(path1, scale=1)
        pyxel.screenshot(path2, scale=2)
        assert Path(path1).exists()
        assert Path(path2).exists()
        assert Path(path2).stat().st_size > Path(path1).stat().st_size

    def test_screencast(self, tmp_path):
        # In headless mode, flip() doesn't capture frames,
        # so screencast produces no GIF. Verify it doesn't raise.
        pyxel.reset_screencast()
        pyxel.cls(5)
        pyxel.flip()
        path = str(tmp_path / "test_screencast.gif")
        pyxel.screencast(path)

    def test_reset_screencast(self):
        pyxel.reset_screencast()


class TestUserDataDir:
    def test_user_data_dir(self):
        result = pyxel.user_data_dir("TestVendor", "TestApp")
        assert isinstance(result, str)
        assert len(result) > 0
