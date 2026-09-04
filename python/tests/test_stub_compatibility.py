import os
import subprocess
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).parents[2]


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


def test_stub_accepts_none_reset_calls(tmp_path):
    source = tmp_path / "reset_calls.py"
    source.write_text(
        "import pyxel\n"
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
        "pyxel.clip(1.0)\n"
        "pyxel.clip(None, 2.0, None, None)\n"
        "pyxel.camera(1.0)\n"
        "pyxel.camera(None, 2.0)\n"
        "pyxel.pal(1)\n"
        "pyxel.pal(None, 2)\n"
        "image = pyxel.Image(1, 1)\n"
        "image.clip(1.0)\n"
        "image.clip(None, 2.0, None, None)\n"
        "image.camera(1.0)\n"
        "image.camera(None, 2.0)\n"
        "image.pal(1)\n"
        "image.pal(None, 2)\n"
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


def test_cube_stub_accepts_live_primitive_sequence_operations(tmp_path):
    source = tmp_path / "valid_cube_sequence_operations.py"
    source.write_text(
        "from pyxel.cube import Primitive\n"
        "primitive = Primitive(Primitive.MODE_TRIANGLES, [0.0], [0])\n"
        "positions = primitive.positions\n"
        "value: float = positions[0]\n"
        "values: list[float] = positions[:]\n"
        "positions[0] = value\n"
        "positions[:] = values\n"
        "del positions[0]\n"
        "del positions[:]\n"
        "positions.append(1.0)\n"
        "positions.extend((2.0, 3.0))\n"
        "positions += [1.0]\n"
        "positions.insert(0, 0.0)\n"
        "popped: float = positions.pop()\n"
        "combined: list[float] = positions + [popped]\n"
        "repeated: list[float] = positions * 2\n"
        "for item in reversed(positions):\n"
        "    value = item\n"
        "positions.clear()\n",
        encoding="utf-8",
    )
    result = _run_mypy(source)

    assert result.returncode == 0, result.stdout + result.stderr


def test_cube_stub_rejects_runtime_invalid_operations(tmp_path):
    source = tmp_path / "invalid_cube_operations.py"
    source.write_text(
        "from pyxel.cube import (\n"
        "    Contact, Mat4, Motion, Primitive, Quat, RaycastHit, Vec3,\n"
        ")\n"
        "Vec3().x = 1.0\n"
        "Mat4().pos = Vec3.ZERO\n"
        "Quat().w = 0.0\n"
        "primitive = Primitive(Primitive.MODE_TRIANGLES, [], [])\n"
        "primitive.positions = [0.0]\n"
        "primitive.positions.sort()\n"
        "Motion()\n"
        "Contact()\n"
        "RaycastHit()\n",
        encoding="utf-8",
    )
    result = _run_mypy(source)

    assert result.returncode == 1
    for message in (
        'Property "x" defined in "Vec3" is read-only',
        'Property "pos" defined in "Mat4" is read-only',
        'Property "w" defined in "Quat" is read-only',
        'Property "positions" defined in "Primitive" is read-only',
        '"_PrimitiveSequence[float]" has no attribute "sort"',
        'Too few arguments for "Motion"',
        'Too few arguments for "Contact"',
        'Too few arguments for "RaycastHit"',
    ):
        assert message in result.stdout
