import sys

from . import run

run(sys.argv[1] if len(sys.argv) >= 2 else None)
