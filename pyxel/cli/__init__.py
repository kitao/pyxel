import sys

import pyxel
from pyxel.editor import App


def print_usage():
    print("Pyxel {}, a retro game engien for Python".format(pyxel.PYXEL_VERSION))


def main():
    if len(sys.argv) < 2:
        print_usage()
        exit(0)

    App(sys.argv[1])


# run(sys.argv[1] if len(sys.argv) >= 2 else none)


if __name__ == "__main__":
    main()
