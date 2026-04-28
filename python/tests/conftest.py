from pathlib import Path

import pytest

import pyxel

from _capture import compare_or_update_all  # type: ignore[reportMissingImports]

ASSETS_DIR = Path(__file__).parent.parent / "pyxel" / "examples" / "assets"


def pytest_addoption(parser):
    parser.addoption(
        "--update-references",
        action="store_true",
        help="Update reference files instead of comparing",
    )


def pytest_collection_modifyitems(items):
    # Run app/example regression tests last so cheap failures surface early
    regression = []
    others = []
    for item in items:
        if "test_examples" in str(item.fspath) or "test_apps" in str(item.fspath):
            regression.append(item)
        else:
            others.append(item)
    items[:] = others + regression


@pytest.fixture(scope="session")
def update_references(request):
    return request.config.getoption("--update-references")


@pytest.fixture(scope="session", autouse=True)
def init_pyxel():
    pyxel.init(160, 120, headless=True)


@pytest.fixture(autouse=True)
def reset_pyxel_state():
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


@pytest.fixture
def compare_screenshots(update_references):
    def _compare(name, results, refs_dir):
        compare_or_update_all(name, results, refs_dir, update_references)

    return _compare
