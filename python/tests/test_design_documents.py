import re
from pathlib import Path

DESIGN_DIR = Path(__file__).parents[2] / "docs" / "design"
DECISIONS_DIR = DESIGN_DIR / "design-decisions"


def test_every_decision_section_states_one_decision_and_one_reason():
    violations = []

    for path in sorted(DECISIONS_DIR.glob("*.md")):
        for section in re.split(r"^### ", path.read_text(), flags=re.MULTILINE)[1:]:
            counts = (section.count("**Decision:**"), section.count("**Reason:**"))
            if counts != (1, 1):
                violations.append(f"{path.name}: {section.splitlines()[0]} {counts}")

    assert violations == []


def test_local_links_and_anchors_resolve():
    anchors = {}
    for path in sorted(DESIGN_DIR.rglob("*.md")):
        headings = re.findall(r"^#{1,6}\s+(.*)$", path.read_text(), re.MULTILINE)
        anchors[path.resolve()] = {_slug(heading) for heading in headings}
    broken = []

    for path in sorted(DESIGN_DIR.rglob("*.md")):
        for target in re.findall(r"\]\(([^)]+)\)", path.read_text()):
            if target.startswith("http"):
                continue
            file_part, _, anchor = target.partition("#")
            resolved = (
                (path.parent / file_part).resolve() if file_part else path.resolve()
            )
            if (
                not resolved.exists()
                or anchor
                and resolved in anchors
                and anchor not in anchors[resolved]
            ):
                broken.append(f"{path.name}: {target}")

    assert broken == []


def test_governing_text_carries_no_dates_or_incident_narratives():
    offending = []

    for path in sorted(DESIGN_DIR.rglob("*.md")):
        for number, line in enumerate(path.read_text().splitlines(), 1):
            if re.search(r"\b20\d\d-\d\d-\d\d\b|\bthis audit\b", line):
                offending.append(f"{path.name}:{number}")

    assert offending == []


def _slug(heading):
    text = re.sub(r"[^\w\s-]", "", heading.lower()).strip().replace(" ", "-")
    return re.sub(r"-+", "-", text)
