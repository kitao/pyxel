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


def _parse_imports_recursively(imports, filename, parsed_files):
    if filename in parsed_files:
        return
    parsed_files.add(filename)
    dir_path = os.path.dirname(filename)
    with open(filename) as file:
        root = ast.parse(file.read())
    for node in ast.walk(root):
        if isinstance(node, ast.Import):
            for alias in node.names:
                module_path = os.path.join(dir_path, alias.name.replace(".", os.sep))
                module_filename = _to_module_filename(module_path)
                if module_filename:
                    imports["local"].add(os.path.relpath(module_filename))
                    _parse_imports_recursively(imports, module_filename, parsed_files)
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
                    imports["local"].add(os.path.relpath(module_filename))
                    _parse_imports_recursively(imports, module_filename, parsed_files)
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
                        _parse_imports_recursively(
                            imports, module_filename, parsed_files
                        )
                    else:
                        imports["system"].add(alias.name)


def parse_imports(filename):
    imports = {"system": set(), "local": set()}
    parsed_files = set()
    _parse_imports_recursively(imports, filename, parsed_files)
    return {
        "system": sorted(imports["system"]),
        "local": sorted(imports["local"]),
    }
