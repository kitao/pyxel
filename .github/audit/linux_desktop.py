"""Exercise a native Pyxel window and the PulseAudio output of a CI desktop."""
import array
from pathlib import Path
import subprocess
import sys
import tempfile
import time

TITLE = "Pyxel Linux desktop audit"


def run_game(markers):
    import pyxel

    pyxel.init(64, 64, title=TITLE, display_scale=3)
    pyxel.sounds[0].set("c3e3g3", "t", "6", "n", 6)
    color = 1

    def update():
        nonlocal color
        if pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT):
            color = 8
        if pyxel.btnp(pyxel.KEY_SPACE):
            color = 11
            pyxel.play(0, 0, loop=True)

    def draw():
        pyxel.cls(color)
        marker = markers / str(color)
        if pyxel.frame_count >= 5 and not marker.exists():
            marker.touch()

    pyxel.run(update, draw)


def wait_for(path, process):
    deadline = time.monotonic() + 20
    while not path.is_file():
        if process.poll() is not None or time.monotonic() >= deadline:
            raise RuntimeError(f"Did not reach frame marker {path.name}: status={process.poll()}")
        time.sleep(0.05)


def pixel(window):
    dump = subprocess.check_output(["xwd", "-silent", "-id", window])
    raw = subprocess.check_output(["convert", "xwd:-", "-gravity", "Center", "-crop",
                                   "1x1+0+0", "+repage", "-depth", "8", "rgb:-"], input=dump)
    assert len(raw) == 3
    return raw.hex()


def wait_color(window, expected):
    deadline = time.monotonic() + 10
    while (actual := pixel(window)) != expected:
        if time.monotonic() >= deadline:
            raise RuntimeError(f"Display pixel {actual}, expected {expected}")
        time.sleep(0.1)


def monitor_samples():
    command = ["timeout", "6", "parec", "--raw", "--device=pyxel_audit.monitor",
               "--format=s16le", "--rate=22050", "--channels=1", "--latency-msec=30"]
    with subprocess.Popen(command, stdout=subprocess.PIPE) as recorder:
        try:
            data = recorder.stdout.read(22050)
        finally:
            recorder.terminate()
    assert len(data) == 22050
    return array.array("h", data)


def verify(markers):
    process = subprocess.Popen([sys.executable, __file__, "game", str(markers)])
    try:
        wait_for(markers / "1", process)
        window = subprocess.check_output(["xdotool", "search", "--onlyvisible", "--name",
                                          f"^{TITLE}$"], text=True).strip()
        assert window.isdecimal()
        subprocess.run(["xdotool", "windowfocus", "--sync", window], check=True)
        wait_color(window, "2b335f")
        quiet = max(abs(value) for value in monitor_samples())
        subprocess.run(["xdotool", "mousemove", "--window", window, "60", "60",
                        "mousedown", "1"], check=True)
        wait_for(markers / "8", process)
        subprocess.run(["xdotool", "mouseup", "1"], check=True)
        wait_color(window, "d4186c")
        subprocess.run(["xdotool", "keydown", "space"], check=True)
        wait_for(markers / "11", process)
        subprocess.run(["xdotool", "keyup", "space"], check=True)
        wait_color(window, "70c6a9")
        audio = monitor_samples()
        peak = max(abs(value) for value in audio)
        assert quiet <= 16 and peak > 100 and len(set(audio)) > 8, (quiet, peak)
        subprocess.run(["xdotool", "keydown", "Escape"], check=True)
        assert process.wait(timeout=10) == 0
        print(f"PASS X11 window, mouse, Space, display pixels, PulseAudio output and Escape; quiet={quiet} peak={peak}", flush=True)
    finally:
        if process.poll() is None:
            process.kill()
            process.wait()


if __name__ == "__main__":
    if sys.argv[1:] and sys.argv[1] == "game":
        run_game(Path(sys.argv[2]))
    else:
        with tempfile.TemporaryDirectory() as directory:
            verify(Path(directory))
