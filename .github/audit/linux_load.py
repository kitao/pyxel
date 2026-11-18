"""Exercise the built wheel in a fresh Linux process for each SDL2 load path."""
import importlib.metadata
import os
import subprocess
import sys
from pathlib import Path

mode, expected_arch = sys.argv[1:]
wheels = list(Path("/wheels").glob("*.whl"))
assert len(wheels) == 1 and expected_arch in wheels[0].name, wheels
subprocess.run([sys.executable, "-m", "pip", "install", "--no-index", "--no-deps", "--disable-pip-version-check", str(wheels[0])], check=True)

# A different process checks the initial loader search, leaving this one pristine.
probe = subprocess.run([sys.executable, "-c", 'import ctypes; ctypes.CDLL("libSDL2-2.0.so.0")'], capture_output=True)
assert (probe.returncode == 0) == (mode == "system"), (mode, probe.returncode)
os.environ.update(SDL_AUDIODRIVER="dummy", SDL_VIDEODRIVER="dummy")
import pyxel

bundled = (Path(pyxel.__file__).parent / "libs/libSDL2-2.0.so.0").resolve()
assert bundled.is_file() and pyxel.VERSION == importlib.metadata.version("pyxel")
pyxel.init(16, 16, headless=True)
pyxel.cls(4)
assert pyxel.pget(3, 7) == 4
loaded = {Path(line.split(maxsplit=5)[-1].strip()).resolve() for line in Path("/proc/self/maps").read_text().splitlines() if "/libSDL2-2.0.so" in line}
assert len(loaded) == 1 and (loaded == {bundled}) == (mode == "fallback"), (mode, loaded)
print(f"PASS arch={expected_arch} mode={mode} loaded={loaded.pop()} version={pyxel.VERSION} pixel=4", flush=True)
