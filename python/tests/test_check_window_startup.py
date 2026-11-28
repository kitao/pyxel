import hashlib
import os
import subprocess
import sys
from pathlib import Path

import pytest

SCRIPT = Path(__file__).resolve().parents[2] / "scripts/check_window_startup"


@pytest.mark.parametrize(("frames", "expected_status"), [(0, 1), (6, 0)])
def test_startup_reports_the_binding_and_requires_drawn_frames(
    tmp_path, frames, expected_status
):
    package = tmp_path / "pyxel"
    package.mkdir()
    (package / "__init__.py").write_text("from .pyxel_binding import *\n")
    (package / "pyxel_binding.pyi").write_text("VERSION: str\n")
    binding = package / "pyxel_binding.py"
    binding.write_text(
        f"""\
VERSION = "test-build"

def init(*args, **kwargs):
    pass

screen_mode = cls = circ = text = init

def quit():
    raise SystemExit

def run(update, draw):
    for _ in range({frames}):
        update()
        draw()
"""
    )
    digest = hashlib.sha256(binding.read_bytes()).hexdigest()[:12]

    result = subprocess.run(
        [sys.executable, SCRIPT],
        env={**os.environ, "PYTHONPATH": str(tmp_path)},
        capture_output=True,
        text=True,
        timeout=15,
        check=False,
    )

    assert result.returncode == expected_status, result.stdout + result.stderr
    if frames == 0:
        assert "window closed before the startup frames were drawn" in result.stderr
    for mode in range(3):
        assert (
            f"screen mode {mode}: pyxel test-build at {binding} (binding sha256 {digest},"
            in result.stdout
        )
