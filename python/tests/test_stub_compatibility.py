import os
import subprocess
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).parents[2]


def test_stub_accepts_none_reset_calls(tmp_path):
    source = tmp_path / "reset_calls.py"
    source.write_text(
        "import pyxel\n"
        "\n"
        "pyxel.clip()\n"
        "pyxel.clip(None)\n"
        "pyxel.clip(w=None)\n"
        "pyxel.clip(None, None, None, None)\n"
        "pyxel.camera()\n"
        "pyxel.camera(None)\n"
        "pyxel.camera(y=None)\n"
        "pyxel.camera(None, None)\n"
        "pyxel.pal()\n"
        "pyxel.pal(None)\n"
        "pyxel.pal(col2=None)\n"
        "pyxel.pal(None, None)\n"
        "\n"
        "image = pyxel.Image(1, 1)\n"
        "image.clip()\n"
        "image.clip(None)\n"
        "image.clip(w=None)\n"
        "image.clip(None, None, None, None)\n"
        "image.camera()\n"
        "image.camera(None)\n"
        "image.camera(y=None)\n"
        "image.camera(None, None)\n"
        "image.pal()\n"
        "image.pal(None)\n"
        "image.pal(col2=None)\n"
        "image.pal(None, None)\n"
        "\n"
        "tilemap = pyxel.Tilemap(1, 1, 0)\n"
        "tilemap.clip()\n"
        "tilemap.clip(None)\n"
        "tilemap.clip(w=None)\n"
        "tilemap.clip(None, None, None, None)\n"
        "tilemap.camera()\n"
        "tilemap.camera(None)\n"
        "tilemap.camera(y=None)\n"
        "tilemap.camera(None, None)\n",
        encoding="utf-8",
    )

    result = _run_mypy(source)
    assert result.returncode == 0, result.stdout + result.stderr


def test_stub_rejects_partial_numeric_or_mixed_none_reset_calls(tmp_path):
    source = tmp_path / "invalid_reset_calls.py"
    source.write_text(
        "import pyxel\n"
        "\n"
        "pyxel.clip(1.0)\n"
        "pyxel.clip(None, 2.0, None, None)\n"
        "pyxel.camera(1.0)\n"
        "pyxel.camera(None, 2.0)\n"
        "pyxel.pal(1)\n"
        "pyxel.pal(None, 2)\n"
        "\n"
        "image = pyxel.Image(1, 1)\n"
        "image.clip(1.0)\n"
        "image.clip(None, 2.0, None, None)\n"
        "image.camera(1.0)\n"
        "image.camera(None, 2.0)\n"
        "image.pal(1)\n"
        "image.pal(None, 2)\n"
        "\n"
        "tilemap = pyxel.Tilemap(1, 1, 0)\n"
        "tilemap.clip(1.0)\n"
        "tilemap.clip(None, 2.0, None, None)\n"
        "tilemap.camera(1.0)\n"
        "tilemap.camera(None, 2.0)\n",
        encoding="utf-8",
    )

    result = _run_mypy(source)
    assert result.returncode == 1
    assert result.stdout.count("[call-overload]") == 16, result.stdout


def test_cube_stub_exposes_primitive_lists(tmp_path):
    source = tmp_path / "primitive_lists.py"
    source.write_text(
        "import pyxel\n"
        "from pyxel.cube import Primitive, Shading\n"
        "primitive = Primitive(Primitive.MODE_TRIANGLES, [], [])\n"
        "positions: list[float] = primitive.positions\n"
        "indices: list[int] = primitive.indices\n"
        "normals: list[float] = primitive.normals\n"
        "uvs: list[float] = primitive.uvs\n"
        "Primitive(primitive.mode, positions, indices, normals, uvs)\n"
        "Shading(pyxel.colors).build(pyxel.colors)\n",
        encoding="utf-8",
    )

    result = _run_mypy(source)
    assert result.returncode == 0, result.stdout + result.stderr


def test_stub_exposes_resource_lists(tmp_path):
    source = tmp_path / "resource_lists.py"
    source.write_text(
        "import pyxel\n"
        "\n"
        "colors: list[int] = pyxel.colors\n"
        "images: list[pyxel.Image] = pyxel.images\n"
        "tilemaps: list[pyxel.Tilemap] = pyxel.tilemaps\n"
        "channels: list[pyxel.Channel] = pyxel.channels\n"
        "tones: list[pyxel.Tone] = pyxel.tones\n"
        "sounds: list[pyxel.Sound] = pyxel.sounds\n"
        "musics: list[pyxel.Music] = pyxel.musics\n"
        "\n"
        "sound = pyxel.Sound()\n"
        "notes: list[int] = sound.notes\n"
        "tone_values: list[int] = sound.tones\n"
        "volumes: list[int] = sound.volumes\n"
        "effects: list[int] = sound.effects\n"
        "wavetable: list[int] = pyxel.Tone().wavetable\n"
        "\n"
        "music = pyxel.Music()\n"
        "seqs: list[list[int]] = music.seqs\n"
        "music.set(*seqs)\n"
        "pyxel.play(0, seqs[0])\n"
        "pyxel.Channel().play(sounds)\n",
        encoding="utf-8",
    )

    result = _run_mypy(source)
    assert result.returncode == 0, result.stdout + result.stderr


def test_stub_rejects_resource_property_assignment(
    tmp_path,
):
    source = tmp_path / "invalid_resource_operations.py"
    source.write_text(
        "import pyxel\n"
        "\n"
        "image = pyxel.Image(1, 1)\n"
        "image.width = 2\n"
        "image.height = 2\n"
        "tilemap = pyxel.Tilemap(1, 1, 0)\n"
        "tilemap.width = 2\n"
        "tilemap.height = 2\n"
        "\n"
        "sound = pyxel.Sound()\n"
        "sound.notes = [1]\n"
        "sound.tones = [1]\n"
        "sound.volumes = [1]\n"
        "sound.effects = [1]\n"
        "pyxel.Tone().wavetable = [1]\n"
        "pyxel.Music().seqs = [[1]]\n"
        "sound.notes += [1]\n"
        "pyxel.Music().seqs += [[1]]\n",
        encoding="utf-8",
    )

    result = _run_mypy(source)
    assert result.returncode == 1
    assert result.stdout.count("is read-only") == 12, result.stdout


def test_cube_stub_rejects_read_only_property_assignment(tmp_path):
    source = tmp_path / "invalid_cube_operations.py"
    source.write_text(
        "from pyxel.cube import (\n"
        "    Mat4, Primitive, Quat, Vec3,\n"
        ")\n"
        "Vec3().x = 1.0\n"
        "Mat4().pos = Vec3.ZERO\n"
        "Quat().w = 0.0\n"
        "primitive = Primitive(Primitive.MODE_TRIANGLES, [], [])\n"
        "primitive.positions = [0.0]\n",
        encoding="utf-8",
    )

    result = _run_mypy(source)

    assert result.returncode == 1
    assert result.stdout.count("error:") == 4, result.stdout
    for message in (
        'Property "x" defined in "Vec3" is read-only',
        'Property "pos" defined in "Mat4" is read-only',
        'Property "w" defined in "Quat" is read-only',
        'Property "positions" defined in "Primitive" is read-only',
    ):
        assert message in result.stdout


def _run_mypy(source):
    env = {**os.environ, "MYPYPATH": str(ROOT_DIR / "python")}
    return subprocess.run(
        [sys.executable, "-m", "mypy", "--no-error-summary", str(source)],
        cwd=ROOT_DIR,
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )
