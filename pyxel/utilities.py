import os
import platform
import subprocess

import PIL.Image

from .constants import DEFAULT_PALETTE, ICON_DATA


def get_pil_palette(palette):
    rgb_palette = []

    for color in palette:
        r = (color >> 16) & 0xFF
        g = (color >> 8) & 0xFF
        b = color & 0xFF
        rgb_palette.extend((r, g, b))

    rgb_palette += [0] * 240 * 3

    pil_palette = PIL.Image.new("P", (1, 1), 0)
    pil_palette.putpalette(rgb_palette)

    return pil_palette


def get_icon_image():
    width = len(ICON_DATA[0])
    height = len(ICON_DATA)
    color_list = list(map(lambda x: int(x, 16), "".join(ICON_DATA)))

    image = []
    for color in color_list:
        rgb = DEFAULT_PALETTE[color]
        image.append((rgb >> 16) & 0xFF)
        image.append((rgb >> 8) & 0xFF)
        image.append(rgb & 0xFF)

    icon = PIL.Image.frombuffer(
        "RGB", (width, height), bytes(image), "raw", "RGB", 0, 1
    ).convert("RGBA")

    pixels = icon.load()
    for x in range(width):
        for y in range(height):
            r, g, b, a = pixels[x, y]
            if (r, g, b) == (0, 0, 0):
                pixels[x, y] = (0, 0, 0, 0)

    return icon


def get_desktop_path():
    plat = platform.system()

    if plat == "Windows":
        path = os.path.join(os.path.join(os.environ["USERPROFILE"]), "Desktop")
    elif plat == "Darwin":
        path = os.path.join(os.path.join(os.path.expanduser("~")), "Desktop")
    else:
        path = os.path.join(os.path.join(os.path.expanduser("~")), "Desktop")
        if not os.path.exists(path):
            try:
                path = (
                    subprocess.check_output(["xdg-user-dir DESKTOP"], shell=True)
                    .decode("utf-8")
                    .split("\n")[0]
                )
                if not os.path.exists(path):
                    raise OSError
            except (subprocess.CalledProcessError, OSError):
                path = os.path.expanduser("~")

    return path


def get_copy_rect(sx, sy, sw, sh, dx, dy, dw, dh, cw, ch):
    over_sx = max(-sx, 0)
    over_sy = max(-sy, 0)
    over_dx = max(-dx, 0)
    over_dy = max(-dy, 0)

    if over_sx > 0 or over_dx > 0:
        cw -= max(over_sx, over_dx)
        if over_sx > 0:
            sx = 0
        if over_dx > 0:
            dx = 0

    if over_sy > 0 or over_dy > 0:
        ch -= max(over_sy, over_dy)
        if over_sy > 0:
            sy = 0
        if over_dy > 0:
            dy = 0

    over_sx = max(sx + cw - sw, 0)
    over_sy = max(sx + ch - sh, 0)
    over_dx = max(dx + cw - dw, 0)
    over_dy = max(dx + ch - dh, 0)

    if over_sx > 0 or over_dx > 0:
        cw -= max(over_sx, over_dx)

    if over_sy > 0 or over_dy > 0:
        ch -= max(over_sy, over_dy)

    if cw > 0 and ch > 0:
        return sx, sy, dx, dy, cw, ch
    else:
        return None
