import os.path

import pyxel

from .app import App


def run(filename):
    if filename:
        if os.path.splitext(filename)[1] == "." + pyxel.RESOURCE_FILE_EXTENSION:
            App(os.path.abspath(filename))
        else:
            print("invalid resource file type")
            exit(1)
    else:
        print("filename is not specified")
        exit(1)
