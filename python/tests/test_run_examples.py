import os
import subprocess
from pathlib import Path


SCRIPT_PATH = Path(__file__).parents[2] / "scripts" / "run_examples"


def _write_executable(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")
    path.chmod(0o755)


def test_child_failure_is_returned_by_entrypoint(tmp_path):
    bin_dir = tmp_path / "bin"
    bin_dir.mkdir()
    _write_executable(bin_dir / "pyxel", "#!/usr/bin/env bash\nexit 23\n")
    _write_executable(bin_dir / "sleep", "#!/usr/bin/env bash\nexit 0\n")
    env = os.environ.copy()
    env["PATH"] = f"{bin_dir}{os.pathsep}{env['PATH']}"

    result = subprocess.run(
        ["bash", str(SCRIPT_PATH)],
        cwd=SCRIPT_PATH.parents[1],
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )

    assert result.returncode == 23, result.stdout + result.stderr
