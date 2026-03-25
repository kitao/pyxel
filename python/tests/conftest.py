import os
import pytest

ASSETS_DIR = os.path.join(
    os.path.dirname(__file__), os.pardir, "pyxel", "examples", "assets"
)


@pytest.fixture(scope="session", autouse=True)
def init_pyxel():
    import pyxel

    pyxel.init(160, 120, headless=True)


@pytest.fixture
def assets_dir():
    return ASSETS_DIR
