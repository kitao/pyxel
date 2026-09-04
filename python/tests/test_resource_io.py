import zipfile
from pathlib import Path
from uuid import uuid4

import PIL.Image
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
    # Set encryption flags in both ZIP headers without encrypting the payload.
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
    @pytest.mark.parametrize(("version", "tile"), [("1.4.0", "021"), ("1.9.0", "0101")])
    def test_load_old_format_pyxres(self, tmp_path, version, tile):
        # Legacy text format: hex grids per bank under pyxel_resource/.
        path = tmp_path / "legacy.pyxres"
        _write_legacy_resource(
            path,
            {
                "version": version,
                "image0": "78\n9a\n",
                "tilemap0": tile + "\n",
                "sound00": "000cff\n01\n73\n00\n20\n",
                "music0": "0001\nnone\nnone\nnone\n",
            },
        )

        pyxel.load(str(path))
        assert pyxel.images[0].pget(0, 0) == 7
        assert pyxel.images[0].pget(1, 0) == 8
        assert pyxel.images[0].pget(0, 1) == 9
        assert pyxel.images[0].pget(1, 1) == 10
        assert pyxel.tilemaps[0].pget(0, 0) == (1, 1)
        assert list(pyxel.sounds[0].notes) == [0, 12, -1]
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
                (
                    "invalid hexadecimal digit 'g' in 'pyxel_resource/image0' "
                    "at line 1, column 2"
                ),
            ),
            (
                {"image0": "0あ"},
                (
                    "invalid hexadecimal digit 'あ' in 'pyxel_resource/image0' "
                    "at line 1, column 2"
                ),
            ),
            (
                {"image0": "0\n" * 257},
                "too many image rows in 'pyxel_resource/image0': got 257, maximum 256",
            ),
            (
                {"tilemap0": "000"},
                (
                    "invalid tile width in 'pyxel_resource/tilemap0' at line 1: "
                    "expected groups of 4 hexadecimal digits"
                ),
            ),
            (
                {"tilemap0": "00z0"},
                (
                    "invalid hexadecimal digit 'z' in 'pyxel_resource/tilemap0' "
                    "at line 1, column 3"
                ),
            ),
            (
                {"tilemap0": "0000" * 257},
                (
                    "too many tiles in 'pyxel_resource/tilemap0' at line 1: "
                    "got 257, maximum 256"
                ),
            ),
            (
                {"tilemap0": "0000\n" * 256 + "bad\n"},
                "invalid decimal value 'bad' in 'pyxel_resource/tilemap0' at line 257",
            ),
            (
                {"tilemap0": "0000\n" * 256 + "9999\n"},
                (
                    "image index 9999 in 'pyxel_resource/tilemap0' at line 257 "
                    "is out of range 0..3"
                ),
            ),
            (
                {"sound00": "0g"},
                (
                    "invalid hexadecimal digit 'g' in 'pyxel_resource/sound00' "
                    "at line 1, column 2"
                ),
            ),
            (
                {"sound00": "0"},
                (
                    "invalid value width in 'pyxel_resource/sound00' at line 1: "
                    "expected groups of 2 hexadecimal digits"
                ),
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

        with pytest.raises(
            Exception, match="invalid tile width in 'pyxel_resource/tilemap0'"
        ):
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

        with pytest.raises(Exception, match="^Failed to parse line"):
            pyxel.load(str(path))

        assert pyxel.images[0].pget(0, 0) == 7

    @pytest.mark.parametrize(
        "header",
        [
            "format_version = 99\n",
            '"format_version" = 99 # future version\n',
            "format_version_label = 1\nformat_version = 99\n",
        ],
    )
    def test_load_unsupported_format_version(self, tmp_path, header):
        path = tmp_path / "future.pyxres"
        _write_resource(path, header)

        with raises_exact(Exception, "Unsupported resource format version '99'"):
            pyxel.load(str(path))

    @pytest.mark.parametrize(
        ("version", "message"),
        [
            (1, "Failed to parse resource data"),
            (99, "Unsupported resource format version '99'"),
        ],
    )
    def test_format_version_precedes_body_parse_error(self, tmp_path, version, message):
        path = tmp_path / "broken.pyxres"
        _write_resource(path, f"format_version = {version}\nimages = [\n")

        with raises_exact(Exception, message):
            pyxel.load(str(path))

    def test_nested_format_version_is_not_the_resource_version(self, tmp_path):
        path = tmp_path / "nested-version.pyxres"
        _write_resource(path, "[metadata]\nformat_version = 99\n")

        with raises_exact(Exception, "Failed to parse resource format version"):
            pyxel.load(str(path))

    @pytest.mark.parametrize(
        ("toml_text", "message"),
        [
            (
                (
                    "format_version = 1\n"
                    "tilemaps = []\nsounds = []\nmusics = []\n"
                    "[[images]]\nwidth = 1\nheight = 1\ndata = []\n"
                ),
                "Invalid resource data: images[0].data must not be empty",
            ),
            (
                (
                    "format_version = 1\n"
                    "images = []\nsounds = []\nmusics = []\n"
                    "[[tilemaps]]\nwidth = 1\nheight = 1\nimgsrc = 0\ndata = []\n"
                ),
                "Invalid resource data: tilemaps[0].data must not be empty",
            ),
            (
                (
                    "format_version = 1\n"
                    "images = []\nsounds = []\nmusics = []\n"
                    "[[tilemaps]]\nwidth = 1\nheight = 1\nimgsrc = 3\ndata = [[0, 0]]\n"
                ),
                "Invalid resource data: tilemaps[0].imgsrc 3 is out of range 0..3",
            ),
            (
                (
                    "format_version = 1\n"
                    "images = []\ntilemaps = []\nmusics = []\n"
                    "[[sounds]]\nnotes = []\ntones = []\nvolumes = []\neffects = []\nspeed = 0\n"
                ),
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

        with pytest.raises(Exception, match=r"tilemaps\[0\]\.data must not be empty"):
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

    def test_excl_aliases_deprecated(self, capfd, tmp_path):
        # Save and load share a once-per-session deprecation warning.
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        path = str(tmp_path / "test_excl_dep.pyxres")
        pyxel.save(path, exclude_images=False, excl_images=True)  # type: ignore[call-arg]
        out = capfd.readouterr().out
        assert out == "excl_* options are deprecated. Use exclude_* instead.\n"

        pyxel.images[0].cls(0)
        pyxel.load(path)
        assert pyxel.images[0].pget(0, 0) == 0

        pyxel.images[0].pset(0, 0, 7)
        full_path = str(tmp_path / "with_images.pyxres")
        pyxel.save(full_path)
        pyxel.images[0].cls(0)
        pyxel.load(full_path, exclude_images=False, excl_images=True)  # type: ignore[call-arg]
        assert pyxel.images[0].pget(0, 0) == 0
        pyxel.load(full_path)
        assert pyxel.images[0].pget(0, 0) == 7


class TestPalette:
    def test_load_pal(self, assets_dir):
        original_colors = list(pyxel.colors)
        try:
            pyxel.load_pal(str(assets_dir / "audio_bgm.pyxpal"))
            assert len(pyxel.colors) == 32
        finally:
            pyxel.colors[:] = original_colors

    def test_load_pal_skips_whitespace_only_lines(self, tmp_path):
        original_colors = list(pyxel.colors)
        try:
            pal_file = tmp_path / "test.pyxpal"
            pal_file.write_text("ff0000\n   \n00ff00\n")
            pyxel.load_pal(str(pal_file))
            assert list(pyxel.colors) == [0xFF0000, 0x00FF00]
        finally:
            pyxel.colors[:] = original_colors

    def test_save_load_pal_roundtrip(self, tmp_path):
        original_colors = list(pyxel.colors)
        path = str(tmp_path / "test.pyxpal")
        pyxel.save_pal(path)

        try:
            pyxel.colors[0] = 0xFFFFFF
            pyxel.load_pal(path)
            assert list(pyxel.colors) == original_colors
        finally:
            pyxel.colors[:] = original_colors


class TestScreenshot:
    def test_screenshot(self, tmp_path):
        pyxel.cls(7)
        pyxel.flip()
        path = str(tmp_path / "test_screenshot.png")
        pyxel.screenshot(path)
        with PIL.Image.open(path) as image:
            assert image.format == "PNG"
            assert image.size == (pyxel.width * 2, pyxel.height * 2)

    def test_screenshot_with_scale(self, tmp_path):
        pyxel.cls(7)
        pyxel.flip()
        path1 = str(tmp_path / "test_s1.png")
        path2 = str(tmp_path / "test_s2.png")
        pyxel.screenshot(path1, scale=1)
        pyxel.screenshot(path2, scale=2)
        with PIL.Image.open(path1) as image1, PIL.Image.open(path2) as image2:
            assert image1.size == (pyxel.width, pyxel.height)
            assert image2.size == (pyxel.width * 2, pyxel.height * 2)

    def test_screencast(self, tmp_path):
        pyxel.reset_screencast()
        pyxel.cls(5)
        pyxel.flip()
        path = str(tmp_path / "test_screencast.gif")
        pyxel.screencast(path)
        with PIL.Image.open(path) as image:
            assert image.format == "GIF"
            assert image.size == (pyxel.width * 2, pyxel.height * 2)
            assert image.n_frames == 1


class TestUserDataDir:
    def test_user_data_dir(self):
        vendor = f"PyxelTest-{uuid4().hex}"
        result = pyxel.user_data_dir(vendor, "TestApp")
        path = Path(result)
        try:
            assert isinstance(result, str)
            assert result
            assert path.is_dir()
        finally:
            if path.name == "TestApp" and path.parent.name == vendor:
                path.rmdir()
                path.parent.rmdir()
