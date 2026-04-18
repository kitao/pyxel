import os
from pathlib import Path
import pytest

ASSETS_DIR = os.path.join(
    os.path.dirname(__file__), os.pardir, "pyxel", "examples", "assets"
)
REFERENCES_DIR = Path(__file__).parent / "references"


def pytest_addoption(parser):
    parser.addoption(
        "--update-references",
        action="store_true",
        help="Update reference files instead of comparing",
    )


@pytest.fixture(scope="session")
def update_references(request):
    return request.config.getoption("--update-references")


def pytest_collection_modifyitems(items):
    # Ensure regression tests (test_examples, test_apps) run last
    regression = []
    others = []
    for item in items:
        if "test_examples" in str(item.fspath) or "test_apps" in str(item.fspath):
            regression.append(item)
        else:
            others.append(item)
    items[:] = others + regression


@pytest.fixture(scope="session", autouse=True)
def init_pyxel():
    import pyxel

    pyxel.init(160, 120, headless=True)


@pytest.fixture(autouse=True)
def reset_pyxel_state():
    import pyxel

    pyxel.clip()
    pyxel.camera()
    pyxel.pal()
    pyxel.dither(1.0)
    pyxel.rseed(0)
    pyxel.nseed(0)
    yield


@pytest.fixture
def assets_dir():
    return ASSETS_DIR
