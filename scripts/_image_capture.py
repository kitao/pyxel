# Write palette-preserving documentation GIFs at their intended playback rate.

from itertools import pairwise

from PIL import Image


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
