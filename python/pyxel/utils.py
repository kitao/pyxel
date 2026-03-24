import ast
import os

# Keys for the import classification dict
SYSTEM = "system"
LOCAL = "local"


def _to_module_filename(module_path):
    filename = module_path + ".py"
    if os.path.isfile(filename):
        return filename
    init_file = os.path.join(module_path, "__init__.py")
    if os.path.isdir(module_path) and os.path.isfile(init_file):
        return init_file
    return None


def _resolve_module_path(dir_path, level, name):
    """Build a filesystem path from an import's directory, level, and name."""
    parts = [dir_path]
    if level > 1:
        parts.extend([".."] * (level - 1))
    parts.append(name.replace(".", os.sep))
    return os.path.join(*parts)


def _track_module(imports, filename, checked_files, dir_path, level, name):
    """Classify a single import as local or system and recurse into local ones."""
    module_path = _resolve_module_path(dir_path, level, name)
    module_filename = _to_module_filename(module_path)

    if module_filename:
        imports[LOCAL].add(os.path.abspath(module_filename))
        _list_imported_modules(imports, module_filename, checked_files)
    elif level == 0:
        # Only top-level imports are considered system modules
        imports[SYSTEM].add(name)


def _list_imported_modules(imports, filename, checked_files):
    if filename in checked_files:
        return
    checked_files.add(filename)

    dir_path = os.path.dirname(filename)
    with open(filename, encoding="utf8") as file:
        root = ast.parse(file.read())

    for node in ast.walk(root):
        if isinstance(node, ast.Import):
            for alias in node.names:
                _track_module(imports, filename, checked_files, dir_path, 0, alias.name)

        elif isinstance(node, ast.ImportFrom):
            if node.module:
                _track_module(
                    imports,
                    filename,
                    checked_files,
                    dir_path,
                    node.level,
                    node.module,
                )
            else:
                # Relative import without module name (e.g. "from . import foo")
                for alias in node.names:
                    _track_module(
                        imports,
                        filename,
                        checked_files,
                        dir_path,
                        node.level,
                        alias.name,
                    )


def list_imported_modules(filename):
    imports = {SYSTEM: set(), LOCAL: set()}
    checked_files = set()
    _list_imported_modules(imports, filename, checked_files)

    return {
        SYSTEM: sorted(imports[SYSTEM]),
        LOCAL: sorted(imports[LOCAL]),
    }
