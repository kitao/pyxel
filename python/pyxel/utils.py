import ast
import os


def _to_module_filename(module_path):
    filename = module_path + ".py"
    if os.path.isfile(filename):
        return filename
    elif os.path.isdir(module_path):
        filename = os.path.join(module_path, "__init__.py")
        if os.path.isfile(filename):
            return filename
    return None


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
                module_path = os.path.join(dir_path, alias.name.replace(".", os.sep))
                module_filename = _to_module_filename(module_path)

                if module_filename:
                    imports["local"].add(os.path.abspath(module_filename))
                    _list_imported_modules(imports, module_filename, checked_files)
                else:
                    imports["system"].add(alias.name)

        elif isinstance(node, ast.ImportFrom):
            if node.module:
                module_path = os.path.join(
                    dir_path,
                    *([".."] * (node.level - 1)),
                    node.module.replace(".", os.sep),
                )
                module_filename = _to_module_filename(module_path)

                if module_filename:
                    imports["local"].add(os.path.abspath(module_filename))
                    _list_imported_modules(imports, module_filename, checked_files)
                elif node.level == 0:
                    imports["system"].add(node.module)
            else:
                for alias in node.names:
                    module_path = os.path.join(
                        dir_path,
                        *([".."] * (node.level - 1)),
                        alias.name.replace(".", os.sep),
                    )
                    module_filename = _to_module_filename(module_path)

                    if module_filename:
                        imports["local"].add(module_filename)
                        _list_imported_modules(imports, module_filename, checked_files)
                    else:
                        imports["system"].add(alias.name)


def list_imported_modules(filename):
    imports = {"system": set(), "local": set()}
    checked_files = set()
    _list_imported_modules(imports, filename, checked_files)

    return {
        "system": sorted(imports["system"]),
        "local": sorted(imports["local"]),
    }
