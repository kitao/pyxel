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


def decode_editor_source(source):
    text = pyxel.utils._decode_python_source(source)
    return pyxel.utils._normalize_python_source_encoding(text)


@pytest.mark.parametrize(
    ("source", "expected"),
    [
        (b"", ""),
        (b"\xef\xbb\xbf# Japanese\n", "# Japanese\n"),
        (
            '# coding: latin-1\nvalue = "café\x80"\n'.encode("latin-1"),
            '# coding: utf-8\nvalue = "café\x80"\n',
        ),
        (
            '# coding: cp932\nvalue = "日本語①"\n'.encode("cp932"),
            '# coding: utf-8\nvalue = "日本語①"\n',
        ),
        (
            '# coding: shift_jis\nvalue = "波〜"\n'.encode("shift_jis"),
            '# coding: utf-8\nvalue = "波〜"\n',
        ),
        (
            '#!/usr/bin/python\r\n# coding=cp932; author: A\r\nvalue="日本語"\r\n'.encode(
                "cp932"
            ),
            '#!/usr/bin/python\r\n# coding=utf-8; author: A\r\nvalue="日本語"\r\n',
        ),
        (
            b'# coding: utf8\nvalue = "coding: cp932"\n# coding: latin-1\n',
            '# coding: utf-8\nvalue = "coding: cp932"\n# coding: latin-1\n',
        ),
        (
            b'value = "coding: cp932"\n# coding: fake_codec\n',
            'value = "coding: cp932"\n# coding: fake_codec\n',
        ),
        (
            '#!/usr/bin/python\r# coding: cp932\rvalue="日本語"\r'.encode("cp932"),
            '#!/usr/bin/python\r# coding: utf-8\rvalue="日本語"\r',
        ),
        (
            b"# first\r# second\r# coding: fake_codec\rvalue=1\r",
            "# first\r# second\r# coding: fake_codec\rvalue=1\r",
        ),
        (
            b"# ignored+AAo-value = '+ACM coding: utf-7'\n",
            "# ignored\nvalue = '# coding: utf-7'\n",
        ),
    ],
)
def test_source_decode_preserves_python_characters_and_utf8_save(
    tmp_path, source, expected
):
    text = decode_editor_source(source)
    assert text == expected
    original_path = tmp_path / "original.py"
    saved_path = tmp_path / "saved.py"
    original_path.write_bytes(source)
    saved_path.write_text(text, encoding="utf-8")
    original = runpy.run_path(str(original_path))
    saved = runpy.run_path(str(saved_path))
    assert original.get("value") == saved.get("value")


@pytest.mark.parametrize(
    "source",
    [
        b"# coding: fake_codec\nvalue = 1\n",
        b'# coding: utf-8\nvalue = "\xff"\n',
        b"\xef\xbb\xbf# coding: latin-1\nvalue = 1\n",
    ],
)
def test_source_decode_rejects_invalid_encoding(source):
    with pytest.raises((SyntaxError, UnicodeError)):
        decode_editor_source(source)


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
