import tempfile
from pathlib import Path

import pyxel

pyxel.init(16, 16, headless=True)
with tempfile.TemporaryDirectory() as directory:
    project = Path(directory) / "project.pyxres"
    pyxel.images[:] = [pyxel.Image(8, 8)]
    pyxel.tilemaps[:] = [pyxel.Tilemap(1, 1, 0)]
    pyxel.save(str(project))
    original = project.read_bytes()
    pyxel.tilemaps[0].imgsrc = 2
    try:
        pyxel.save(str(project))
    except Exception as error:
        assert str(error) == "Invalid resource data: tilemaps[0].imgsrc 2 is out of range 0..1"
    else:
        raise AssertionError("An unreadable resource was written")

    assert project.read_bytes() == original
    assert pyxel.tilemaps[0].imgsrc == 2
    pyxel.save(str(project), exclude_images=True)
    pyxel.images[:] = [pyxel.Image(8, 8) for _ in range(3)]
    pyxel.load(str(project))
    assert pyxel.tilemaps[0].imgsrc == 2
    pyxel.save(str(project))
    pyxel.load(str(project))
    assert len(pyxel.images) == 3 and pyxel.tilemaps[0].imgsrc == 2

print("The wheel preserved the previous file, loaded a tilemap-only export, and round-tripped the corrected project.")
