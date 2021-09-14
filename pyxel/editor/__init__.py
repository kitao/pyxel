from os.path import abspath

from pyxel.editor.app import App


def run(filename):
    App(abspath(filename))
