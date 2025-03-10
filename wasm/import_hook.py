import importlib.util
import inspect
import os.path
import sys


class ImportHook:
    def __init__(self):
        self.imported_modules = set()
        self.main_dir = None

    def find_spec(self, fullname, path, target=None):
        # Skip imported modules if already processed, built-in, or in sys.modules
        if (
            fullname in self.imported_modules
            or fullname in sys.builtin_module_names
            or fullname in sys.modules
        ):
            return None
        self.imported_modules.add(fullname)

        # Skip imported modules from the standard library or installed packages
        spec = importlib.util.find_spec(fullname)
        if spec and spec.origin:
            origin = os.path.realpath(spec.origin)
            if (
                origin.startswith(os.path.realpath(sys.base_prefix))
                or "site-packages" in origin
                or "dist-packages" in origin
            ):
                return None

        # Find the script that triggered the import
        caller_file = next(
            (
                frame.filename
                for frame in inspect.stack()
                if not frame.filename.startswith("<")
            ),
            None,
        )
        if not caller_file:
            return None

        # Trigger file download for missing modules in the caller's directory
        print(f"Attempting to import '{fullname}'")
        caller_dir = os.path.dirname(os.path.abspath(caller_file))
        module_name = fullname.replace(".", os.sep)
        module_path = os.path.join(caller_dir, f"{module_name}.py")
        package_path = os.path.join(caller_dir, module_name, "__init__.py")
        is_found = (
            os.path.exists(module_name)
            or os.path.exists(module_path)
            or os.path.exists(package_path)
        )
        if is_found and self.main_dir is None:
            self.main_dir = caller_dir

        # Trigger file download from the main directory if needed
        if self.main_dir and self.main_dir != caller_dir:
            main_module_path = os.path.join(self.main_dir, f"{module_name}.py")
            main_package_path = os.path.join(self.main_dir, module_name, "__init__.py")
            os.path.exists(main_module_path) or os.path.exists(main_package_path)
        return None


sys.meta_path.insert(0, ImportHook())
