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
    classifiers=(
        "Development Status :: 4 - Beta",
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: MacOS",
        "Operating System :: Microsoft :: Windows",
        "Topic :: Games/Entertainment",
        "Topic :: Multimedia :: Graphics",
        "Topic :: Multimedia :: Sound/Audio",
    ),
    packages=[
        "pyxel",
        "pyxel.ui",
        "pyxel.editor",
        "pyxel.editor.assets",
        "pyxel.examples",
        "pyxel.examples.assets",
    ],
    package_data={"": ["*.png", "*.gif"]},
    install_requires=["numpy", "glfw", "PyOpenGL", "sounddevice", "Pillow"],
    python_requires=">=3",
    entry_points={
        "console_scripts": [
            "pyxel=pyxel.editor:run",
            "install_pyxel_examples=pyxel.examples:install",
        ]
    },
)
