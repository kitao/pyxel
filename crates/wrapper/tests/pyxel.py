import inspect
import os
import sys

sys.path.append(os.path.join(os.path.dirname(__file__), "../../target/debug"))

from pyxel_wrapper import *  # type: ignore  # noqa: E402,F401,F403

os.chdir(os.path.dirname(inspect.stack()[-1].filename))
