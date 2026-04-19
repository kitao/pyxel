import ast
from pathlib import Path

# Keys for the import classification dict
_SYSTEM = "system"
_LOCAL = "local"


def _to_module_filename(module_path: str) -> str | None:
    filename = Path(f"{module_path}.py")
    if filename.is_file():
        return str(filename)
    module_dir = Path(module_path)
    init_file = module_dir / "__init__.py"
    if module_dir.is_dir() and init_file.is_file():
        return str(init_file)
    return None


def _resolve_module_path(dir_path: str, level: int, name: str) -> str:
    path = Path(dir_path)
    for _ in range(level - 1):
        path = path / ".."
    for part in name.split("."):
        path = path / part
    return str(path)


def _track_module(
    imports: dict[str, set[str]],
    checked_files: set[str],
    dir_path: str,
    level: int,
    name: str,
) -> None:
    module_path = _resolve_module_path(dir_path, level, name)
    module_filename = _to_module_filename(module_path)

    if module_filename:
        imports[_LOCAL].add(str(Path(module_filename).absolute()))
        _list_imported_modules(imports, module_filename, checked_files)
    elif level == 0:
        # Only top-level imports are considered system modules
        imports[_SYSTEM].add(name)


def _list_imported_modules(
    imports: dict[str, set[str]], filename: str, checked_files: set[str]
) -> None:
    if filename in checked_files:
        return
    checked_files.add(filename)

    dir_path = str(Path(filename).parent)
    try:
        root = ast.parse(Path(filename).read_text(encoding="utf-8"))
    except (SyntaxError, UnicodeDecodeError):
        return

    for node in ast.walk(root):
        if isinstance(node, ast.Import):
            for alias in node.names:
                _track_module(imports, checked_files, dir_path, 0, alias.name)

        elif isinstance(node, ast.ImportFrom):
            if node.module:
                _track_module(
                    imports,
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
                        checked_files,
                        dir_path,
                        node.level,
                        alias.name,
                    )


def list_imported_modules(filename: str) -> dict[str, list[str]]:
    imports: dict[str, set[str]] = {_SYSTEM: set(), _LOCAL: set()}
    checked_files: set[str] = set()
    _list_imported_modules(imports, filename, checked_files)

    return {
        _SYSTEM: sorted(imports[_SYSTEM]),
        _LOCAL: sorted(imports[_LOCAL]),
    }
