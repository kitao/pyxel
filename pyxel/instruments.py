def triangle(x):
    return (abs((x % 1) * 4 - 2) - 1) * 0.7


def tilted_saw(x):
    x = x % 1
    return (((x < 0.875) and (x * 16 / 7) or ((1 - x) * 16)) - 1) * 0.7


def saw(x):
    return (x % 1 - 0.5) * 0.9


def square(x):
    return (x % 1 < 0.5 and 1 or -1) / 3


def pulse(x):
    return (x % 1 < 0.3125 and 1 or -1) / 3


def organ(x):
    x *= 4
    return (abs((x % 2) - 1) - 0.5 + (abs((
        (x * 0.5) % 2) - 1) - 0.5) / 2 - 0.1) * 0.7


def noise(x):
    noise._reg >>= 1
    noise._reg |= ((noise._reg ^ (noise._reg >> 1)) & 1) << 15
    return noise._reg & 1


noise._reg = 0x8000


def phaser(x):
    x = x * 2
    return abs((x % 2) - 1.5 +
               (abs((x * 127 / 128) % 2 - 1) - 0.5) / 2) - (1 / 4)


INSTRUMENTS = [triangle, tilted_saw, saw, square, pulse, organ, noise, phaser]
