# Run documentation capture workers and write palette-preserving GIFs.

import os
import subprocess
import sys
from itertools import pairwise

from PIL import Image


def run_worker(command):
    # Unattended capture must not depend on the host's audio device.
    try:
        result = subprocess.run(
            command,
            env={**os.environ, "SDL_AUDIODRIVER": "dummy"},
            capture_output=True,
            text=True,
            timeout=120,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        # TimeoutExpired retains captured bytes even with text=True.
        print((error.stdout or b"").decode(errors="replace"), end="")
        print((error.stderr or b"").decode(errors="replace"), end="", file=sys.stderr)
        raise

    print(result.stdout, end="")
    print(result.stderr, end="", file=sys.stderr)
    result.check_returncode()
    if "Failed to initialize audio device" in result.stdout + result.stderr:
        raise RuntimeError("Failed to initialize audio device in capture worker")


def save_gif(frames, destination, fps=30, *, durations=None):
    colors = sorted({color for frame in frames for _, color in frame.getcolors(256)})
    palette = Image.new("P", (1, 1))
    palette.putpalette(
        [channel for color in colors for channel in color]
        + [0] * (768 - 3 * len(colors))
    )
    frames = [
        frame.quantize(palette=palette, dither=Image.Dither.NONE) for frame in frames
    ]

    if durations is None:
        # GIF stores hundredths of a second. Round cumulative time so a rate
        # such as 30 FPS does not become 33.3 FPS by truncating every frame.
        ticks = [round(index * 100 / fps) for index in range(len(frames) + 1)]
        durations = [10 * (end - start) for start, end in pairwise(ticks)]

    frames[0].save(
        destination,
        save_all=True,
        append_images=frames[1:],
        duration=durations,
        loop=0,
        disposal=1,
        optimize=True,
    )
