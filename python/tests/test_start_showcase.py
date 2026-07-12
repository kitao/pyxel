from collections.abc import Iterator
from http.server import ThreadingHTTPServer
from importlib.machinery import SourceFileLoader
from importlib.util import module_from_spec, spec_from_loader
from pathlib import Path
import threading
from types import ModuleType
from urllib.error import HTTPError
from urllib.request import urlopen

import pytest


ROOT_DIR = Path(__file__).parents[2]
START_SHOWCASE_PATH = ROOT_DIR / "scripts" / "start_showcase"
CDN_RUNTIME_URL = "https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"


def _load_start_showcase() -> ModuleType:
    loader = SourceFileLoader("start_showcase", str(START_SHOWCASE_PATH))
    spec = spec_from_loader(loader.name, loader)
    assert spec is not None
    module = module_from_spec(spec)
    loader.exec_module(module)
    return module


@pytest.fixture
def showcase_url() -> Iterator[str]:
    module = _load_start_showcase()
    server = ThreadingHTTPServer(("127.0.0.1", 0), module.Handler)
    thread = threading.Thread(target=server.serve_forever)
    thread.start()
    try:
        host, port = server.server_address
        yield f"http://{host}:{port}"
    finally:
        server.shutdown()
        server.server_close()
        thread.join()


def _get(url: str) -> tuple[int, bytes]:
    try:
        with urlopen(url) as response:
            return response.status, response.read()
    except HTTPError as error:
        return error.code, error.read()


def test_start_showcase_preserves_public_paths(showcase_url: str):
    expected_paths = [
        "/wasm/pyxel.js",
        "/python/pyxel/examples/01_hello_pyxel.py",
        "/pyxel-sw.js",
    ]
    for path in expected_paths:
        status, _body = _get(showcase_url + path)
        assert status == 200, path

    status, body = _get(showcase_url + "/web/showcase/")
    assert status == 200

    status, body = _get(showcase_url + "/web/showcase/examples/01-hello-pyxel.html")
    assert status == 200
    assert CDN_RUNTIME_URL.encode() in body


@pytest.mark.parametrize(
    "path",
    [
        "/.git/HEAD",
        "/crates/Cargo.toml",
        "/%2e%2e/.git/HEAD",
    ],
)
def test_start_showcase_rejects_repository_internal_paths(showcase_url: str, path: str):
    status, _body = _get(showcase_url + path)
    assert status == 404
