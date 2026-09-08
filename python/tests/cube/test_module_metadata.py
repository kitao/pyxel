import importlib
import os
import pickle
import subprocess
import sys
from pathlib import Path

import pytest
from _assertions import raises_exact  # type: ignore[reportMissingImports]
from pyxel import cube

ROOT_DIR = Path(__file__).parents[3]
PUBLIC_CUBE_CLASSES = (
    "Camera",
    "Collider",
    "Contact",
    "Mat4",
    "Mesh",
    "Motion",
    "Node",
    "Primitive",
    "Quat",
    "RaycastHit",
    "Shading",
    "Vec3",
)


def test_fresh_from_pyxel_import_cube_resolves_the_package():
    _run_fresh_interpreter(
        "import importlib, sys\n"
        "import pyxel\n"
        "from pyxel import cube\n"
        "assert cube is pyxel.cube\n"
        "assert cube is importlib.import_module('pyxel.cube')\n"
        "assert cube is sys.modules['pyxel.cube']\n"
        "assert cube.__name__ == 'pyxel.cube'\n"
        "assert cube.__package__ == 'pyxel.cube'\n"
        "assert cube.__spec__ is not None\n"
    )


def test_fresh_explicit_cube_import_has_the_same_identity():
    _run_fresh_interpreter(
        "import importlib, sys\n"
        "import pyxel.cube as imported_cube\n"
        "from pyxel import cube\n"
        "assert cube is imported_cube\n"
        "assert cube is importlib.import_module('pyxel.cube')\n"
        "assert cube is sys.modules['pyxel.cube']\n"
    )


@pytest.mark.parametrize("name", PUBLIC_CUBE_CLASSES)
def test_public_cube_classes_use_their_importable_module(name):
    cls = getattr(cube, name)
    assert cls.__module__ == "pyxel.cube"
    assert getattr(importlib.import_module(cls.__module__), name) is cls
    assert pickle.loads(pickle.dumps(cls)) is cls


def test_cube_exports_the_public_classes():
    assert cube.__all__ == list(PUBLIC_CUBE_CLASSES)


def test_cube_instance_pickle_error_uses_the_public_type_name():
    with raises_exact(TypeError, "cannot pickle 'pyxel.cube.Vec3' object"):
        pickle.dumps(cube.Vec3.ZERO)


def _run_fresh_interpreter(source):
    env = {**os.environ}
    env["PYTHONPATH"] = os.pathsep.join(
        filter(None, (str(ROOT_DIR / "python"), env.get("PYTHONPATH")))
    )
    result = subprocess.run(
        [sys.executable, "-c", source],
        cwd=ROOT_DIR,
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode == 0, result.stdout + result.stderr
