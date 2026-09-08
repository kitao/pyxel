import ast
from importlib.machinery import PathFinder
from pathlib import Path

_SYSTEM = "system"
_LOCAL = "local"


def list_imported_modules(filename: str) -> dict[str, list[str]]:
    imports: dict[str, set[str]] = {_SYSTEM: set(), _LOCAL: set()}
    checked_files: set[str] = set()
    _list_imported_modules(imports, filename, checked_files)
    return {
        _SYSTEM: sorted(imports[_SYSTEM]),
        _LOCAL: sorted(imports[_LOCAL]),
    }


# Recursive import discovery


def _list_imported_modules(
    imports: dict[str, set[str]], filename: str, checked_files: set[str]
) -> None:
    # Keep import paths lexical; resolve only the visitation identity.
    resolved_filename = str(Path(filename).resolve())
    if resolved_filename in checked_files:
        return
    checked_files.add(resolved_filename)

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
                is_local = _track_module(
                    imports,
                    checked_files,
                    dir_path,
                    node.level,
                    node.module,
                )

                # Track from-import targets that resolve as modules.
                for alias in node.names:
                    target = f"{node.module}.{alias.name}"
                    _track_module(
                        imports,
                        checked_files,
                        dir_path,
                        node.level,
                        target,
                        allow_system=(
                            node.level == 0
                            and not is_local
                            and _is_importable_module(target)
                        ),
                    )
            else:
                # Track relative imports without module names, such as "from . import foo".
                for alias in node.names:
                    _track_module(
                        imports,
                        checked_files,
                        dir_path,
                        node.level,
                        alias.name,
                    )


def _track_module(
    imports: dict[str, set[str]],
    checked_files: set[str],
    dir_path: str,
    level: int,
    name: str,
    *,
    allow_system: bool = True,
) -> bool:
    module_path = _resolve_module_path(dir_path, level, name)
    module_filename = _to_module_filename(module_path)

    if module_filename:
        parts = name.split(".")
        module_files = []
        for index in range(1, len(parts)):
            parent_path = _resolve_module_path(dir_path, level, ".".join(parts[:index]))
            init_file = Path(parent_path) / "__init__.py"
            if init_file.is_file():
                module_files.append(str(init_file))
        module_files.append(module_filename)

        for filename in module_files:
            imports[_LOCAL].add(str(Path(filename).absolute()))
            _list_imported_modules(imports, filename, checked_files)
        return True
    elif allow_system and level == 0:
        # Only absolute imports can resolve as system modules.
        imports[_SYSTEM].add(name)
    return False


# Module path resolution


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


def _is_importable_module(name: str) -> bool:
    search_path = None
    parts = name.split(".")
    for index in range(len(parts)):
        fullname = ".".join(parts[: index + 1])
        spec = PathFinder.find_spec(fullname, search_path)
        if spec is None:
            return False
        if index < len(parts) - 1 and spec.submodule_search_locations is None:
            return False
        search_path = spec.submodule_search_locations
    return True
