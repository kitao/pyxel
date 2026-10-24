# Gazelle

Gazelle by **monkus**, licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).

Original model: https://blockbenchworkshop.com/model/monkus/gazelle

The model, texture, and animations retain this attribution license. They are
third-party assets, separate from Pyxel's MIT-licensed sample code.

- `gazelle.bbmodel`: editable Blockbench project with all 13 source animations.
- `gazelle.png`: 64 × 64 texture mapped to Pyxel's default palette.
- `gazelle.glb`: embedded-texture model used by `cube/c04_mesh_and_motion.py`.

Changes from the original: texture colors mapped to the nearest RGB colors in
Pyxel's palette, project/texture renamed, and exported transparent materials
changed from glTF `BLEND` to `MASK` with an alpha cutoff of 0.05. Geometry and
animation tracks are unchanged. The sample scales and centers the model at
runtime, and presents the `move`, `run`, and `eat` animations.

Open the project in Blockbench. In Paint mode, import `docs/pyxel.gpl` from the
repository as the palette. The project embeds the same texture as the PNG and
GLB. Export with **Binary (glb)**, **Scale 16**, **Embed Textures** on,
**Armature** off, and **Export Animations** on. These settings are saved in the
project. Apply the material change described above after export to reproduce
the supplied GLB's transparency settings.
