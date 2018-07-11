from .image import Image
from .constants import (
    FONT_WIDTH,
    FONT_HEIGHT,
    FONT_IMAGE_WIDTH,
    FONT_IMAGE_HEIGHT,
    FONT_IMAGE_DATA,
)


def create_font_image():
    image = Image(FONT_IMAGE_WIDTH, FONT_IMAGE_HEIGHT)
    row_count = FONT_IMAGE_WIDTH // FONT_WIDTH

    for i, v in enumerate(FONT_IMAGE_DATA):
        left = (i % row_count) * FONT_WIDTH
        top = (i // row_count) * FONT_HEIGHT
        data = image.data

        for j in range(FONT_WIDTH * FONT_HEIGHT):
            x = left + j % FONT_WIDTH
            y = top + j // FONT_WIDTH
            data[y, x] = (v & 0x800000) and 1 or 0
            v <<= 1

    return image
