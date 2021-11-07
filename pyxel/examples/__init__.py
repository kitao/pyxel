import glob
import os
import shutil


def copy_pyxel_examples(dirname):
    src_dir = os.path.dirname(__file__)
    dst_dir = os.path.join(dirname, "pyxel_examples")

    print("Copy Pyxel examples to {} ...".format(dst_dir))

    shutil.rmtree(dst_dir, ignore_errors=True)
    os.makedirs(os.path.join(dst_dir, "assets"))

    patterns = ["[0-9]*.py", "assets/*.pyxres", "assets/*.png", "assets/*.gif"]

    for pattern in patterns:
        srcs = glob.glob(os.path.join(src_dir, pattern))

        for src in srcs:
            relpath = os.path.relpath(src, src_dir)
            dst = os.path.join(dst_dir, relpath)

            print("    {}".format(relpath))
            shutil.copyfile(src, dst)
