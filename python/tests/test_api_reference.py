import ast
import json
import re
from pathlib import Path

import pytest

ROOT_DIR = Path(__file__).parents[2]

REFERENCE_STUBS = (
    ("web/api-reference/api-reference.json", "python/pyxel/__init__.pyi"),
    ("web/api-reference/cube/api-reference.json", "python/pyxel/cube/__init__.pyi"),
)


@pytest.mark.parametrize(("reference_path", "stub_path"), REFERENCE_STUBS)
def test_reference_signatures_are_callable_stub_signatures(reference_path, stub_path):
    stub_signatures = load_stub_signatures(ROOT_DIR / stub_path)
    reference = json.loads((ROOT_DIR / reference_path).read_text(encoding="utf-8"))
    mismatches = []

    for category in reference["categories"]:
        class_name = next(
            (
                item["signature"].partition("(")[0]
                for item in category["items"]
                if item["type"] == "class"
                and "." not in item["signature"].partition("(")[0]
            ),
            None,
        )
        for item in category["items"]:
            if item["type"] not in ("class", "function"):
                continue
            name, parenthesis, parameters = item["signature"].rstrip(")").partition("(")
            if "{" in name:
                owner, _, members = name.partition(".")
                for member in members.strip("{}").split(","):
                    member = member.strip()
                    if (
                        member not in stub_signatures
                        or f"{owner}.{member}" not in stub_signatures
                    ):
                        mismatches.append(f"{owner}.{member}")
                continue
            if "." not in name and class_name is not None:
                name = (
                    f"{class_name}.__init__"
                    if name == class_name
                    else f"{class_name}.{name}"
                )
            if not parenthesis:
                if name.removesuffix(".__init__") not in stub_signatures:
                    mismatches.append(item["signature"])
                continue
            overloads = stub_signatures.get(name, ())
            if not any(
                is_callable_as(parameters, overload, name.partition(".")[0])
                for overload in overloads
            ):
                mismatches.append((item["signature"], sorted(overloads)))

    assert mismatches == []


def load_stub_signatures(path):
    signatures = {}

    for node in ast.parse(path.read_text(encoding="utf-8")).body:
        if isinstance(node, ast.FunctionDef):
            signatures.setdefault(node.name, set()).add(format_parameters(node.args))
        elif isinstance(node, ast.ClassDef):
            signatures.setdefault(node.name, set())
            for member in node.body:
                if isinstance(member, ast.FunctionDef):
                    key = f"{node.name}.{member.name}"
                    signatures.setdefault(key, set()).add(
                        format_parameters(member.args)
                    )

    return signatures


def format_parameters(arguments):
    positional = arguments.posonlyargs + arguments.args
    defaults = [None] * (len(positional) - len(arguments.defaults)) + arguments.defaults
    parts = [
        format_parameter(argument, default)
        for argument, default in zip(positional, defaults)
        if argument.arg not in ("self", "cls")
    ]
    if arguments.vararg:
        parts.append(f"*{arguments.vararg.arg}")
    elif arguments.kwonlyargs:
        parts.append("*")
    parts.extend(
        format_parameter(argument, default)
        for argument, default in zip(arguments.kwonlyargs, arguments.kw_defaults)
    )

    return ",".join(parts)


def format_parameter(argument, default):
    if default is None:
        return argument.arg
    return f"{argument.arg}={ast.unparse(default)}"


def is_callable_as(parameters, overload, owner):
    # The reference writes variadic parameters as "seq0, seq1, seq2, ...", lists
    # a parameter without its default when a separate no-argument form follows,
    # omits the parameters whose defaults that shorter form relies on, and
    # qualifies a class attribute default that the stub names at class scope.
    expected = split_parameters(parameters)
    actual = split_parameters(overload)
    if expected and expected[-1] == "...":
        return any(part.startswith("*") and part != "*" for part in actual)
    if len(expected) > len(actual):
        return False

    for expected_part, actual_part in zip(expected, actual):
        expected_name, _, expected_default = expected_part.partition("=")
        actual_name, _, actual_default = actual_part.partition("=")
        accepted_defaults = ("", actual_default, f"{owner}.{actual_default}")
        if expected_name != actual_name or expected_default not in accepted_defaults:
            return False

    return all("=" in part or part == "*" for part in actual[len(expected) :])


def split_parameters(parameters):
    normalized = re.sub(r"\s+", "", parameters).replace("'", '"')
    return re.split(r",(?![^\[\]()]*[\]\)])", normalized) if normalized else []
