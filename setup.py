import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name='pyxel',
    version='0.1.0',
    description='A fantasy retro gaming console in Python',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://github.com/kitao/pyxel',
    author='Takashi Kitao',
    author_email='takashi.kitao@gmail.com',
    license='MIT',
    classifiers=(
        'Development Status :: 3 - Alpha',
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: MacOS',
        'Operating System :: Microsoft :: Windows',
        'Topic :: Games/Entertainment',
        'Topic :: Multimedia :: Graphics',
        'Topic :: Multimedia :: Sound/Audio',
    ),
    packages=['pyxel', 'pyxel.editor'],
    install_requires=['numpy', 'glfw', 'PyOpenGL', 'sounddevice'],
    python_requires='>=3',
    entry_points={
        'console_scripts': ['pyxel=pyxel.editor:run'],
    },
)
