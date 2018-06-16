import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name='pyxel',
    version='0.0.1',
    description='A fantasy retro gaming console in Python',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://github.com/kitao/pyxel',
    author='Takashi Kitao',
    author_email='takashi.kitao@gmail.com',
    license='MIT',
    classifiers=(
        'Development Status :: 1 - Planning',
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: MacOS',
        'Operating System :: Microsoft :: Windows',
        'Operating System :: POSIX :: Linux',
        'Topic :: Games/Entertainment',
    ),
    packages=['pyxel'],
    install_requires=['numpy', 'pyglet', 'PyOpenGL'],
    python_requires='>=3',
)
