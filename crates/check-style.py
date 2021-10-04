import glob
import os.path
import re
import sys

STYLE_RULES = {
    (
        r"^([ ]*)(?!\/\/)\S.*$\n^\1(for|while|return|continue|break)",
        "no blank line before control structure",
    ),
    (
        r"^([ ]*)(?!\/\/|.*\,$)\S.*$\n^\1(if)",
        "no blank line before if statement",
    ),
    (r"^([ ]*)\}$\n^\1(?!.*(\=\>))\S.*$", "no blank line after block"),
    (r"^([ ]*)let.*$\n^\1(?!let )\S.*", "no blank line after let statement"),
    (r"^([ ]*)(?!\/\/|let)\S+.*$\n\1let", "no blank line before let statement"),
}


def make_line_table(text):
    line_table = {-1: 1}

    for i, m in enumerate(re.finditer(r"\n", text), 2):
        line_table[m.start()] = i

    return line_table


def check_rule(rule, file_text, line_table):
    (pattern, rule_desc) = rule
    matches = list(re.finditer(pattern, file_text, flags=re.MULTILINE))
    results = []

    for m in matches:
        line_offset = file_text.rfind("\n", 0, m.start())
        line_number = line_table[line_offset]

        line_end = file_text.find("\n", m.end())
        line_text = file_text[line_offset + 1 : line_end]

        results.append((line_number, rule_desc, line_text))

    return results


def check_style(file, rules):
    with open(file, "r") as f:
        file_text = f.read()

    line_table = make_line_table(file_text)
    results = []

    for rule in rules:
        results += check_rule(rule, file_text, line_table)

    results.sort(key=lambda x: x[0])

    for result in results:
        (line_number, rule_desc, line_text) = result
        print(
            "\nwarning: {}\n{}:{}\n{}".format(rule_desc, file, line_number, line_text)
        )


def main():
    if len(sys.argv) < 2:
        print("{} <rust-dir>".format(sys.argv[0]))
        exit()

    files = glob.glob(os.path.join(sys.argv[1], "**/*.rs"), recursive=True)

    for file in files:
        if "/target/" in file:
            continue

        check_style(file, STYLE_RULES)

    print("")


if __name__ == "__main__":
    main()
