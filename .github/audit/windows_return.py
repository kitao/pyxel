"""Check return to a calling cmd.exe after closing the test's own Pyxel window."""
import ctypes
import os
import subprocess
import sys
import tempfile
import time
from ctypes import wintypes
from pathlib import Path

if sys.argv[1:] and sys.argv[1] == "game":
    driver = Path(os.environ["SDL_OPENGL_LIBRARY"])
    dll_directory = os.add_dll_directory(str(driver.parent))
    gallium = ctypes.WinDLL(str(driver.parent / "libgallium_wgl.dll"))
    opengl = ctypes.WinDLL(str(driver))
    import pyxel

    title, marker = sys.argv[2:]
    pyxel.init(96, 64, title=title)
    opengl.glGetString.argtypes = [ctypes.c_uint]
    opengl.glGetString.restype = ctypes.c_char_p
    renderer = opengl.glGetString(0x1F01)
    requested = os.environ["GALLIUM_DRIVER"]
    if not renderer or requested.encode() not in renderer.lower():
        raise RuntimeError("Pyxel did not acquire the software OpenGL context")
    print(f"Software OpenGL renderer: {requested}", flush=True)

    def draw():
        pyxel.cls(4)
        if pyxel.frame_count == 0:
            print("First Pyxel draw callback reached", flush=True)
        if pyxel.frame_count >= 5 and not Path(marker).exists():
            Path(marker).touch()

    pyxel.run(lambda: None, draw)
    raise RuntimeError("The native frame loop returned unexpectedly")


def exercise(mode, folder):
    title = f"Pyxel CI {os.getpid()} {mode}"
    ready = folder / f"{mode}.ready"
    returned = folder / f"{mode}.returned"
    batch = folder / f"{mode}.cmd"
    python_line = subprocess.list2cmdline([sys.executable, str(Path(__file__).resolve()), "game", title, str(ready)])
    batch.write_text(
        "@echo off\n" + python_line + "\n"
        + 'set "pyxel_status=%errorlevel%"\n'
        + "echo Game exit code %pyxel_status%\n"
        + 'if not "%pyxel_status%"=="0" exit /b %pyxel_status%\n'
        + 'echo returned>"' + str(returned) + '"\n',
        encoding="utf-8",
    )
    process = subprocess.Popen(["cmd.exe", "/d", "/c", str(batch)])
    try:
        timeout = time.monotonic() + 45
        while not ready.exists():
            if process.poll() is not None or time.monotonic() >= timeout:
                raise RuntimeError(f"{mode}: window did not draw five frames; status={process.poll()}")
            time.sleep(0.1)
        user32 = ctypes.WinDLL("user32", use_last_error=True)
        user32.FindWindowW.argtypes = [wintypes.LPCWSTR, wintypes.LPCWSTR]
        user32.FindWindowW.restype = wintypes.HWND
        user32.PostMessageW.argtypes = [wintypes.HWND, wintypes.UINT, wintypes.WPARAM, wintypes.LPARAM]
        user32.PostMessageW.restype = wintypes.BOOL
        hwnd = user32.FindWindowW(None, title)
        if not hwnd:
            raise RuntimeError(f"{mode}: own window was not found")
        if mode == "close":
            if not user32.PostMessageW(hwnd, 0x0010, 0, 0):
                raise ctypes.WinError(ctypes.get_last_error())
        else:
            if not user32.PostMessageW(hwnd, 0x0100, 0x1B, 0x00010001):
                raise ctypes.WinError(ctypes.get_last_error())
        assert process.wait(timeout=30) == 0 and returned.is_file(), mode
        print(f"PASS {mode}: five frames drawn, window exited and cmd.exe executed the next command", flush=True)
    finally:
        if process.poll() is None:
            subprocess.run(["taskkill", "/PID", str(process.pid), "/T", "/F"], check=False)

with tempfile.TemporaryDirectory() as d:
    for method in ("close", "escape"):
        exercise(method, Path(d))
