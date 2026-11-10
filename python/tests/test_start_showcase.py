import sys
import threading
from collections.abc import Iterator
from http.server import ThreadingHTTPServer
from importlib.machinery import SourceFileLoader
from importlib.util import module_from_spec, spec_from_loader
from pathlib import Path
from types import ModuleType, SimpleNamespace
from urllib.error import HTTPError
from urllib.request import urlopen

import pytest

ROOT_DIR = Path(__file__).parents[2]
START_SHOWCASE_PATH = ROOT_DIR / "scripts" / "start_showcase"
CDN_RUNTIME_URL = "https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"


@pytest.fixture
def showcase_module() -> ModuleType:
    return _load_start_showcase()


@pytest.fixture
def showcase_url(showcase_module: ModuleType) -> Iterator[str]:
    server = ThreadingHTTPServer(("127.0.0.1", 0), showcase_module.Handler)
    thread = threading.Thread(target=server.serve_forever)
    thread.start()
    try:
        host, port = server.server_address
        yield f"http://{host}:{port}"
    finally:
        server.shutdown()
        server.server_close()
        thread.join()


def test_start_showcase_preserves_public_paths(showcase_url: str):
    expected_paths = [
        "/docs/images/pyxel_thanks.png",
        "/docs/pyxel.gpl",
        "/wasm/pyxel.js",
        "/python/pyxel/examples/01_hello_pyxel.py",
        "/pyxel-sw.js",
    ]
    for path in expected_paths:
        status, _body = _get(showcase_url + path)
        assert status == 200, path

    status, body = _get(showcase_url + "/web/showcase/")
    assert status == 200
    assert b"navigator.serviceWorker.register('/pyxel-sw.js'" in body

    status, body = _get(showcase_url + "/web/code-maker/")
    assert status == 200
    assert CDN_RUNTIME_URL.encode() not in body
    assert b'src="/wasm/pyxel.js"' in body

    status, body = _get(showcase_url + "/web/showcase/examples/01-hello-pyxel.html")
    assert status == 200
    assert CDN_RUNTIME_URL.encode() not in body
    assert b'src="/wasm/pyxel.js"' in body
    assert body.index(b"navigator.serviceWorker") < body.index(b"</head>")
    assert b'<pyxel-run\n      root="../../../python/pyxel/examples"' in body

    status, body = _get(showcase_url + "/web/web-usage/index.html")
    assert status == 200
    assert b"https://kitao.dev/pyxel/3.0.1/pyxel.js" in body


def test_main_opens_the_served_address(showcase_module, monkeypatch, capsys):
    opened = []
    monkeypatch.setattr(showcase_module, "_get_ip", lambda: "192.0.2.1")
    monkeypatch.setattr(
        showcase_module,
        "ThreadingHTTPServer",
        lambda *_args: SimpleNamespace(serve_forever=lambda: None),
    )
    monkeypatch.setattr(
        showcase_module.threading,
        "Timer",
        lambda _delay, callback: SimpleNamespace(start=callback),
    )
    monkeypatch.setattr(showcase_module.webbrowser, "open", opened.append)

    assert showcase_module.main() == 0
    assert "http://192.0.2.1:8000/" in capsys.readouterr().out
    assert opened == ["http://192.0.2.1:8000/web/showcase/"]


def test_local_launcher_preserves_hosted_module_bytes(
    showcase_url, showcase_module, monkeypatch
):
    data = b"\xef\xbb\xbf" + "# 日本語".encode()
    requests = []

    class Connection:
        def __init__(self, host, timeout):
            assert host == "cdn.jsdelivr.net"

        def request(self, method, path):
            requests.append((method, path))

        def getresponse(self):
            return SimpleNamespace(status=200, read=lambda: data)

        def close(self):
            pass

    monkeypatch.setattr(showcase_module, "HTTPSConnection", Connection)
    status, page = _get(showcase_url + "/web/launcher/")
    assert status == 200 and b"https://kitao.dev/pyxel/launcher/source/" not in page
    prefix = showcase_url + "/pyxel/launcher/source/"
    names = ["pkg/helper.py", ".py", "UPPER.PY", "a%0Ab.py", "a%09b.py", "a%5Cb.py"]
    for name in names:
        with urlopen(prefix + "owner/repo@main/" + name) as response:
            assert response.headers["X-Pyxel-Source"] == "1"
            assert response.read() == b"\0" + data
    for target in [
        "owner/repo/a%2f..%2fescape.py",
        "owner/repo/game.zip",
        "https://elsewhere.test/test.py",
    ]:
        assert _get(prefix + target)[0] == 404
    assert requests == [("GET", "/gh/owner/repo@main/" + name) for name in names]


def test_inject_html_uses_offsets_from_rewritten_document():
    module = _load_start_showcase()
    original = (
        '<head><script src="'
        + CDN_RUNTIME_URL
        + '"></script></head><body><pyxel-run root="."></pyxel-run></body>'
    )

    injected = module._inject_html(original)
    assert injected.index("navigator.serviceWorker") < injected.index("</head>")
    assert '<pyxel-run root=".">' in injected


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


@pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
@pytest.mark.parametrize("index_name", ["index.html", "index.htm"])
@pytest.mark.parametrize("public_target", [True, False])
def test_start_showcase_checks_index_symlink_targets(
    showcase_url, showcase_module, tmp_path, monkeypatch, index_name, public_target
):
    root = tmp_path.resolve()
    public = root / "web"
    public.mkdir()
    target = (public if public_target else root) / "page.html"
    target.write_text("INDEX_TARGET", encoding="utf-8")
    (public / index_name).symlink_to(target)
    monkeypatch.setattr(showcase_module, "ROOT_DIR", root)
    monkeypatch.setattr(showcase_module, "PUBLIC_ROOT_DIRS", (public,))

    for path in (f"/web/{index_name}", "/web/", "/web%2f", "/web"):
        status, body = _get(showcase_url + path)
        assert status == (200 if public_target else 404), path
        assert (b"INDEX_TARGET" in body) == public_target, path


def _load_start_showcase() -> ModuleType:
    loader = SourceFileLoader("start_showcase", str(START_SHOWCASE_PATH))
    spec = spec_from_loader(loader.name, loader)
    assert spec is not None
    module = module_from_spec(spec)
    loader.exec_module(module)
    return module


def _get(url: str) -> tuple[int, bytes]:
    try:
        with urlopen(url) as response:
            return response.status, response.read()
    except HTTPError as error:
        return error.code, error.read()
