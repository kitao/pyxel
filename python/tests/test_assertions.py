import subprocess
import sys
from pathlib import Path

import pytest


@pytest.mark.parametrize(
    ("exception_type", "message", "failure"),
    [
        ("ValueError", "speed must be greater than 0", ""),
        ("Exception", "speed must be greater than 0", "AssertionError"),
        ("ValueError", "different", "AssertionError"),
        ("TypeError", "speed must be greater than 0", "ValueError"),
    ],
)
def test_raises_exact_releases_caller_resources(exception_type, message, failure):
    code = """
import builtins
import gc
import sys
import threading
import weakref

import pyxel

sys.path.insert(0, sys.argv[1])
from _assertions import raises_exact

class Probe:
    pass

gc.collect()
gc.disable()
errors = []
sys.unraisablehook = lambda error: errors.append(str(error.exc_value))

def check():
    probe = Probe()
    reference = weakref.ref(probe)
    sound = pyxel.Sound()
    caught = ""
    try:
        with raises_exact(getattr(builtins, sys.argv[2]), sys.argv[3]):
            sound.speed = 0
    except BaseException as error:
        caught = type(error).__name__
    assert caught == sys.argv[4], caught
    return reference

reference = check()
released_before_collection = reference() is None
worker = threading.Thread(target=gc.collect)
worker.start()
worker.join()
assert errors == [], errors
assert released_before_collection, "ExceptionInfo retained the caller's locals"
"""
    # Use a real native exception while keeping unrelated pytest garbage out of GC.
    result = subprocess.run(
        [
            sys.executable,
            "-c",
            code,
            str(Path(__file__).parent),
            exception_type,
            message,
            failure,
        ],
        capture_output=True,
        text=True,
        timeout=15,
        check=False,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    assert result.stderr == ""
