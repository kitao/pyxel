import os
import subprocess
from pathlib import Path

SCRIPT_PATH = Path(__file__).parents[2] / "scripts" / "run_examples"


def test_first_child_failure_stops_later_launches(tmp_path):
    bin_dir = tmp_path / "bin"
    bin_dir.mkdir()
    launch_log = tmp_path / "launches.txt"
    _write_executable(
        bin_dir / "pyxel",
        """#!/usr/bin/env bash
if [[ ! -e "$PYXEL_TEST_LAUNCH_LOG" ]]; then
    printf 'launch\\n' > "$PYXEL_TEST_LAUNCH_LOG"
    exit 23
fi
printf 'launch\\n' >> "$PYXEL_TEST_LAUNCH_LOG"
exit 0
""",
    )
    env = os.environ.copy()
    env["PATH"] = f"{bin_dir}{os.pathsep}{env['PATH']}"
    env["PYXEL_TEST_LAUNCH_LOG"] = launch_log.as_posix()

    result = subprocess.run(
        ["bash", str(SCRIPT_PATH)],
        cwd=SCRIPT_PATH.parents[1],
        env=env,
        capture_output=True,
        text=True,
        timeout=10,
        check=False,
    )
    assert result.returncode == 23, result.stdout + result.stderr
    assert launch_log.read_text(encoding="utf-8") == "launch\n"


def _write_executable(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")
    path.chmod(0o755)
