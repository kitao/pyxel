import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name='pyxel',
    version='0.0.3',
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
    packages=['pyxel', 'pyxel.editor'],
    install_requires=['numpy', 'pygame', 'PyOpenGL'],
    python_requires='>=3',
    entry_points={
        'console_scripts': ['pyxel=pyxel.editor:run'],
    },
)
