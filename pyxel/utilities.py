import os
import platform
import subprocess

import PIL.Image

import pyxel

from .constants import DEFAULT_PALETTE, GIF_TRANSPARENCY_COLOR, ICON_DATA


def get_pyxel_image(img):
    return pyxel.image(img)


def get_pyxel_tilemap(tm):
    return pyxel.tilemap(tm)


def get_pyxel_sound(snd):
    return pyxel.sound(snd)


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


def copy_ndarray(dest, dx, dy, src, sx=0, sy=0, cw=None, ch=None):
    dh, dw = dest.shape
    sh, sw = src.shape
    cw = cw or sw
    ch = ch or sh

    rx1 = max(max(-dx, 0), max(-sx, 0))
    ry1 = max(max(-dy, 0), max(-sy, 0))
    rx2 = max(max(dx + cw - dw, 0), max(sx + cw - sw, 0))
    ry2 = max(max(dy + ch - dh, 0), max(sy + ch - sh, 0))

    cw -= rx1 + rx2
    ch -= ry1 + ry2

    if cw <= 0 or ch <= 0:
        return False

    dx += rx1
    dy += ry1
    sx += rx1
    sy += ry1

    dest[dy : dy + ch, dx : dx + cw] = src[sy : sy + ch, sx : sx + cw]

    return True


def palettize_pil_image(pil_image):
    global _pil_palette

    if not hasattr(palettize_pil_image, "pil_palette"):
        rgb_palette = get_palette()

        pil_palette = PIL.Image.new("P", (1, 1), 0)
        pil_palette.putpalette(rgb_palette)

        palettize_pil_image.pil_palette = pil_palette

    im = pil_image.im.convert("P", 0, palettize_pil_image.pil_palette.im)
    return pil_image._new(im)


def get_palette(fill=True):
    rgb_palette = []

    for color in pyxel._app._palette:
        r = (color >> 16) & 0xFF
        g = (color >> 8) & 0xFF
        b = color & 0xFF
        rgb_palette.extend((r, g, b))

    rgb_palette.extend(GIF_TRANSPARENCY_COLOR)

    if fill:
        rgb_palette += [0] * 237 * 3

    return rgb_palette
