import re

import setuptools

from pyxel import VERSION

with open("README.md", "r") as fh:
    long_description = re.sub(
        r'(src=")',
        r"\1https://raw.githubusercontent.com/kitao/pyxel/main/",
        re.sub(
            r'(\]\(|href=")(?!http)',
            r"\1https://github.com/kitao/pyxel/blob/main/",
            fh.read(),
        ),
    )

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
        "License :: OSI Approved :: MIT License",
        "Operating System :: MacOS",
        "Operating System :: Microsoft :: Windows",
        "Operating System :: POSIX :: Linux",
        "Programming Language :: Python :: 3",
        "Topic :: Games/Entertainment",
        "Topic :: Multimedia :: Graphics",
        "Topic :: Multimedia :: Sound/Audio",
    ],
    packages=[
        "pyxel",
        "pyxel.cli",
        "pyxel.editor",
        "pyxel.editor.assets",
        "pyxel.editor.widgets",
        "pyxel.examples",
        "pyxel.examples.assets",
        "pyxel.lib.linux",
        "pyxel.lib.macos",
        "pyxel.lib.windows",
    ],
    package_data={
        "": [
            "*.png",
            "*.pyd",
            "*.pyi",
            "*.pyxres",
            "*.so",
            "py.typed",
        ]
    },
    python_requires=">=3.7",
    entry_points={
        "console_scripts": [
            "pyxel=pyxel.cli:main",
        ]
    },
)
