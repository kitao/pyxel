import json
import os
import subprocess
import sys
import time
from pathlib import Path

import pyxel
import pyxel.cli


def _python_environment(**values: str) -> dict[str, str]:
    env = os.environ.copy()
    package_root = str(Path(pyxel.__file__).resolve().parent.parent)
    env["PYTHONPATH"] = os.pathsep.join(
        value for value in (package_root, env.get("PYTHONPATH")) if value
    )
    env.update(values)
    return env


def _package_app(tmp_path: Path, monkeypatch, source: str) -> Path:
    app_dir = tmp_path / "project"
    script_dir = app_dir / "src"
    script_dir.mkdir(parents=True)
    (script_dir / "helper.py").write_text("VALUE = 42\n", encoding="utf-8")
    (script_dir / "data.txt").write_text("resource data\n", encoding="utf-8")
    (script_dir / "main.py").write_text(source, encoding="utf-8")
    monkeypatch.chdir(tmp_path)
    pyxel.cli.package_pyxel_app("project", "project/src/main.py")
    return tmp_path / "project.pyxapp"


def test_play_uses_packaged_application_context(tmp_path, monkeypatch):
    result_file = tmp_path / "result.json"
    app_file = _package_app(
        tmp_path,
        monkeypatch,
        "import json\n"
        "import os\n"
        "import sys\n"
        "from pathlib import Path\n"
        "import helper\n"
        "import pyxel\n"
        "cwd_before = str(Path.cwd().resolve())\n"
        "pyxel.init(8, 8, headless=True)\n"
        "script_dir = Path(__file__).resolve().parent\n"
        "payload = {\n"
        "    'argv': sys.argv[1:],\n"
        "    'cwd_before': cwd_before,\n"
        "    'cwd_after': str(Path.cwd().resolve()),\n"
        "    'file': str(Path(__file__).resolve()),\n"
        "    'helper': helper.VALUE,\n"
        "    'name': __name__,\n"
        "    'resource': Path('data.txt').read_text(encoding='utf-8'),\n"
        "    'sys_path_0': str(Path(sys.path[0]).resolve()),\n"
        "    'token': os.environ['PYXAPP_TOKEN'],\n"
        "}\n"
        "Path(os.environ['PYXAPP_RESULT']).write_text(\n"
        "    json.dumps(payload), encoding='utf-8'\n"
        ")\n"
        "pyxel.quit()\n",
    )
    launch_dir = tmp_path / "launch"
    launch_dir.mkdir()
    app_arg = os.path.relpath(app_file, launch_dir)

    result = subprocess.run(
        [sys.executable, "-m", "pyxel", "play", app_arg],
        cwd=launch_dir,
        env=_python_environment(
            PYXAPP_RESULT=str(result_file), PYXAPP_TOKEN="preserved"
        ),
        capture_output=True,
        text=True,
        timeout=10,
        check=False,
    )

    assert result.returncode == 0, result.stderr
    payload = json.loads(result_file.read_text(encoding="utf-8"))
    script_dir = Path(payload["file"]).parent
    assert payload == {
        "argv": ["play", app_arg],
        "cwd_before": str(launch_dir.resolve()),
        "cwd_after": str(script_dir),
        "file": str(script_dir / "main.py"),
        "helper": 42,
        "name": "__main__",
        "resource": "resource data\n",
        "sys_path_0": str(script_dir),
        "token": "preserved",
    }


def test_reset_restarts_relative_pyxapp_command(tmp_path, monkeypatch):
    result_file = tmp_path / "reset-result.json"
    app_file = _package_app(
        tmp_path,
        monkeypatch,
        "import json\n"
        "import os\n"
        "import sys\n"
        "from pathlib import Path\n"
        "import helper\n"
        "import pyxel\n"
        "pyxel.init(8, 8, headless=True)\n"
        "if os.environ.get('PYXAPP_RESET_STAGE') == 'second':\n"
        "    payload = {\n"
        "        'argv': sys.argv[1:],\n"
        "        'cwd': str(Path.cwd().resolve()),\n"
        "        'file': str(Path(__file__).resolve()),\n"
        "        'helper': helper.VALUE,\n"
        "        'resource': Path('data.txt').read_text(encoding='utf-8'),\n"
        "    }\n"
        "    Path(os.environ['PYXAPP_RESULT']).write_text(\n"
        "        json.dumps(payload), encoding='utf-8'\n"
        "    )\n"
        "    pyxel.quit()\n"
        "else:\n"
        "    os.environ['PYXAPP_RESET_STAGE'] = 'second'\n"
        "    pyxel.reset()\n",
    )
    launch_dir = tmp_path / "launch"
    launch_dir.mkdir()
    app_arg = os.path.relpath(app_file, launch_dir)

    result = subprocess.run(
        [sys.executable, "-m", "pyxel", "play", app_arg],
        cwd=launch_dir,
        env=_python_environment(
            PYXAPP_RESULT=str(result_file), PYXAPP_RESET_STAGE="first"
        ),
        capture_output=True,
        text=True,
        timeout=15,
        check=False,
    )
    deadline = time.monotonic() + 10
    while not result_file.is_file() and time.monotonic() < deadline:
        time.sleep(0.05)

    assert result.returncode == 0, result.stderr
    assert result_file.is_file()
    payload = json.loads(result_file.read_text(encoding="utf-8"))
    script_dir = Path(payload["file"]).parent
    assert payload == {
        "argv": ["play", app_arg],
        "cwd": str(script_dir),
        "file": str(script_dir / "main.py"),
        "helper": 42,
        "resource": "resource data\n",
    }
