import base64
import runpy
import sys
from html.parser import HTMLParser
from pathlib import Path
from textwrap import dedent
from types import SimpleNamespace

import pytest
import pyxel.cli

PAGE_PATH = Path(__file__).parents[2] / "web" / "code-maker" / "pyxel-screen.html"


class ScriptParser(HTMLParser):
    script = ""

    def handle_starttag(self, tag, attrs):
        if tag == "pyxel-run":
            self.script = dict(attrs)["script"]


@pytest.mark.parametrize(
    ("path", "copied"),
    [
        ("assets/version2.dat", True),
        ("assets/v1..v2.dat", True),
        ("../outside.dat", False),
    ],
)
def test_project_files_are_available_when_code_runs(
    monkeypatch, tmp_path, path, copied
):
    work_dir = tmp_path / "app"
    work_dir.mkdir()
    monkeypatch.chdir(work_dir)

    code = (
        "from pathlib import Path\n"
        f"assert Path({path!r}).exists() is {copied!r}\n"
        + (f"assert Path({path!r}).read_bytes() == b'asset'\n" if copied else "")
        + "Path('ran').touch()\n"
    )
    project = SimpleNamespace(
        code=code,
        resource=base64.b64encode(b"resource").decode(),
        files=SimpleNamespace(
            to_py=lambda: {path: base64.b64encode(b"asset").decode()}
        ),
    )

    monkeypatch.setitem(
        sys.modules, "js", SimpleNamespace(parent=SimpleNamespace(_project=project))
    )
    monkeypatch.setattr(pyxel.cli, "run_python_script", runpy.run_path)

    parser = ScriptParser()
    parser.feed(PAGE_PATH.read_text(encoding="utf-8"))

    script_path = tmp_path / "bootstrap.py"
    script_path.write_text(dedent(parser.script), encoding="utf-8")
    runpy.run_path(str(script_path))
    assert (work_dir / "ran").exists()
