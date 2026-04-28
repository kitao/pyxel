import json
import shutil
import subprocess
import sys
from pathlib import Path

import pytest

# --- paths ---

REFERENCES_DIR = Path(__file__).parent / "references"
EXAMPLES_DIR = Path(__file__).parent.parent / "pyxel" / "examples"
APPS_DIR = EXAMPLES_DIR / "apps"
EXAMPLE_REFS_DIR = REFERENCES_DIR / "examples"
APP_REFS_DIR = REFERENCES_DIR / "apps"
EDITOR_REFS_DIR = REFERENCES_DIR / "editor"

_RUNNER = Path(__file__).parent / "_runner.py"


# --- subprocess wrappers ---


def _run_subprocess(*args):
    subprocess.run([sys.executable, str(_RUNNER), *args], check=True)


def run_example_subprocess(script_path, plan, out_dir):
    _run_subprocess("example", str(script_path), json.dumps(plan), str(out_dir))


def run_flip_example_subprocess(script_path, plan, out_dir):
    _run_subprocess("flip_example", str(script_path), json.dumps(plan), str(out_dir))


def run_app_subprocess(pyxapp_path, plan, out_dir):
    _run_subprocess("app", str(pyxapp_path), json.dumps(plan), str(out_dir))


def run_editor_subprocess(editor, resource_file, out_dir):
    _run_subprocess("editor", editor, str(resource_file), str(out_dir))


# --- result collection ---


def collect_plan_results(plan, out_dir):
    return [
        (step["frame"], out_dir / f"frame_{step['frame']}.png")
        for step in plan
        if step.get("capture", True)
    ]


def collect_editor_results(out_dir):
    return [
        (1, out_dir / "f1.png"),
        ("edit", out_dir / "fedit.png"),
    ]


# --- compare ---


def compare_or_update_all(name, results, refs_dir, update_references):
    updated = []
    failures = []
    for frame, actual_path in results:
        ref_path = refs_dir / f"{name}_f{frame}.png"
        if update_references:
            refs_dir.mkdir(parents=True, exist_ok=True)
            shutil.copy(actual_path, ref_path)
            updated.append(ref_path.name)
            continue
        if not ref_path.exists():
            failures.append(
                f"Reference missing: {ref_path.name}. Run with --update-references"
            )
            continue
        actual_bytes = Path(actual_path).read_bytes()
        ref_bytes = ref_path.read_bytes()
        if actual_bytes != ref_bytes:
            diff_path = ref_path.with_name(f"{name}_f{frame}_actual.png")
            shutil.copy(actual_path, diff_path)
            failures.append(
                f"Screenshot mismatch: {ref_path.name}. "
                f"Actual saved to {diff_path.name}"
            )
    if updated:
        pytest.skip(f"References updated: {', '.join(updated)}")
    if failures:
        pytest.fail("\n".join(failures))
