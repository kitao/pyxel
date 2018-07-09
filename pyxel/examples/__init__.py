import os
import glob
import shutil


def copy():
    src_dir = os.path.dirname(__file__)
    dest_dir = os.path.join(os.getcwd(), 'pyxel_examples')

    print('copy Pyxel examples to {} ...'.format(dest_dir))

    shutil.rmtree(dest_dir, ignore_errors=True)
    os.makedirs(os.path.join(dest_dir, 'assets'))

    patterns = ['[0-9]*.py', 'assets/*.png', 'assets/*.gif']

    for pattern in patterns:
        srcs = glob.glob(os.path.join(src_dir, pattern))

        for src in srcs:
            relpath = os.path.relpath(src, src_dir)
            dest = os.path.join(dest_dir, relpath)

            print('    {}'.format(relpath))
            shutil.copyfile(src, dest)
