import zipfile
from pathlib import Path
from uuid import uuid4

import PIL.Image
import pytest
import pyxel
import tomllib
from _assertions import raises_exact  # type: ignore[reportMissingImports]


class TestSaveLoad:
    @pytest.mark.parametrize("version", [1, 2, 3, 4])
    def test_load_toml_resource_versions(self, tmp_path, version):
        path = tmp_path / "toml.pyxres"
        _write_resource(
            path,
            f"format_version = {version}\n"
            "images = []\ntilemaps = []\nmusics = []\n"
            "[[sounds]]\nnotes = [24, -1, 28]\ntones = [0, 1]\n"
            "volumes = [7, 3]\neffects = [0, 2]\nspeed = 20\n",
        )

        original_sounds = list(pyxel.sounds)
        try:
            pyxel.load(str(path))

            sound = pyxel.sounds[0]
            assert list(sound.notes) == [24, -1, 28]
            assert list(sound.tones) == [0, 1]
            assert list(sound.volumes) == [7, 3]
            assert list(sound.effects) == [0, 2]
            assert sound.speed == 20
        finally:
            pyxel.sounds[:] = original_sounds

    def test_pre_2_resource_format_is_rejected(self, tmp_path):
        path = tmp_path / "legacy.pyxres"
        with zipfile.ZipFile(path, "w") as archive:
            archive.writestr("pyxel_resource/version", "1.9.0")
            archive.writestr("pyxel_resource/image0", "1")

        with raises_exact(Exception, f"Failed to read file '{path}'"):
            pyxel.load(str(path))

    def test_invalid_sidecar_palette_does_not_partially_commit(self, tmp_path):
        path = tmp_path / "invalid-palette.pyxres"
        _write_resource(
            path,
            "format_version = 1\n"
            "tilemaps = []\nsounds = []\nmusics = []\n"
            "[[images]]\nwidth = 1\nheight = 1\ndata = [[1]]\n",
        )
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

    @pytest.mark.parametrize("bank_name", ["images", "tilemaps"])
    def test_save_ignores_excluded_graphics(self, tmp_path, bank_name):
        bank = getattr(pyxel, bank_name)
        original = bank[0]
        original_sound = pyxel.sounds[0]
        path = tmp_path / "excluded.pyxres"
        try:
            bank[0] = (
                pyxel.Image(0, 1) if bank_name == "images" else pyxel.Tilemap(0, 1, 0)
            )
            pyxel.sounds[0] = pyxel.Sound()
            pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
            pyxel.save(str(path), **{f"exclude_{bank_name}": True})
        finally:
            bank[0] = original
            pyxel.sounds[0] = original_sound

        with zipfile.ZipFile(path) as archive:
            data = tomllib.loads(archive.read("pyxel_resource.toml").decode("utf-8"))
        assert data[bank_name] == []
        assert data["sounds"][0]["notes"] == [24, 28, 31]

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
            if path.name == "testapp" and path.parent.name == vendor.lower():
                path.rmdir()
                path.parent.rmdir()


def _write_resource(path, toml_text):
    with zipfile.ZipFile(path, "w") as zf:
        zf.writestr("pyxel_resource.toml", toml_text)
