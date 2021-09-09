import setuptools
from pyxel import VERSION

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="pyxel",
    version=VERSION,
    description="A retro game engine for Python",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/kitao/pyxel",
    author="Takashi Kitao",
    author_email="takashi.kitao@gmail.com",
    license="MIT",
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: MacOS",
        "Operating System :: Microsoft :: Windows",
        "Operating System :: POSIX :: Linux",
        "Topic :: Games/Entertainment",
        "Topic :: Multimedia :: Graphics",
        "Topic :: Multimedia :: Sound/Audio",
    ],
    packages=[
        "pyxel",
        "pyxel.ui",
        "pyxel.core",
        "pyxel.core.bin.macos",
        "pyxel.core.bin.win32",
        "pyxel.core.bin.win64",
        "pyxel.core.bin.linux",
        "pyxel.editor",
        "pyxel.editor.assets",
        "pyxel.examples",
        "pyxel.examples.assets",
    ],
    package_data={
        "": ["*.pyxres", "*.png", "*.gif", "*.dylib", "*.dll", "*.so", "*.exe"]
    },
    python_requires=">=3.6.8",
    entry_points={
        "console_scripts": [
            "pyxeleditor=pyxel.editor:run",
            "pyxelpackager=pyxel.packager:run",
            "install_pyxel_examples=pyxel.examples:install",
        ]
    },
)
