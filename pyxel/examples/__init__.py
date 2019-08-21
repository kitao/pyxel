import glob
import os
import shutil


def install():
    src_dir = os.path.dirname(__file__)
    dst_dir = os.path.join(os.getcwd(), "pyxel_examples")

    print("Install Pyxel examples to {} ...".format(dst_dir))

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


if __name__ == "__main__":
    install()
