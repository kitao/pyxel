"""
def get_array2d_size(arr):
    if isinstance(arr, list):
        return len(arr[0]), len(arr)
    else:
        return 256, 256


def new_array2d(width, height):
    return [[0] * width for _ in range(height)]


def fill_array2d(arr, val):
    width, height = get_array2d_size(arr)

    for i in range(height):
        for j in range(width):
            arr[i][j] = val


def slice_array2d(arr, x, y, width, height):
    sliced_arr = new_array2d(width, height)

    for i in range(height):
        for j in range(width):
            sliced_arr[i][j] = arr.pget(x + j, y + i)

    return sliced_arr


def copy_array2d(dst, dx, dy, src, sx=0, sy=0, cw=None, ch=None):
    dw, dh = get_array2d_size(dst)
    sw, sh = get_array2d_size(src)

    cw = cw or sw
    ch = ch or sh

    rx1 = max(max(-dx, 0), max(-sx, 0))
    ry1 = max(max(-dy, 0), max(-sy, 0))
    rx2 = max(max(dx + cw - dw, 0), max(sx + cw - sw, 0))
    ry2 = max(max(dy + ch - dh, 0), max(sy + ch - sh, 0))

    cw -= rx1 + rx2
    ch -= ry1 + ry2

    if cw <= 0 or ch <= 0:
        return

    dx += rx1
    dy += ry1
    sx += rx1
    sy += ry1

    for i in range(ch):
        for j in range(cw):
            dst.pset(dx + j, dy + i, src[sy + i][sx + j])
"""
