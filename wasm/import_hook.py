import importlib.machinery
import importlib.util
import os.path
import sys

SKIP_MODULES = frozenset(("_hashlib", "_uuid", "javascript", "js", "ssl"))


class ImportHook:
    def __init__(self):
        self.imported_modules = set()

    def invalidate_caches(self):
        # Runtime resets remove downloaded modules and may change the main script.
        self.imported_modules.clear()

    def find_spec(self, fullname, path, target=None):
        if (
            fullname in self.imported_modules
            or fullname in sys.builtin_module_names
            or fullname in sys.modules
            or fullname in SKIP_MODULES
        ):
            return
        # Record the name before find_spec re-enters this hook.
        self.imported_modules.add(fullname)

        # Keep modules that Python can already load ahead of remote source files.
        spec = importlib.util.find_spec(fullname)
        if spec and spec.loader is not None:
            return

        # Follow Python's search path, including package __path__, instead of
        # inferring a directory from a frame whose filename may be relative.
        module_name = fullname.rsplit(".", 1)[-1]
        directories = set()
        for directory in sys.path if path is None else path:
            if not isinstance(directory, str):
                continue
            directory = os.path.abspath(directory)
            if directory in directories:
                continue
            directories.add(directory)
            package_path = os.path.join(directory, module_name, "__init__.py")
            module_path = os.path.join(directory, f"{module_name}.py")
            if os.path.exists(package_path) or os.path.exists(module_path):
                # A downloaded file may share the directory's cached timestamp.
                importlib.machinery.PathFinder.invalidate_caches()
                return


sys.meta_path.insert(0, ImportHook())
