# Cube Model References

This local tool for **Blockbench 5 or later on desktop** keeps a scene linked to its original models. Place a model, duplicate and arrange its group, edit the original, then refresh every placement. Exported files are ordinary GLBs; the game uses the existing `Mesh` and `Node` APIs.

## Install and place a model

1. In Blockbench, open **File → Plugins**, choose **Load Plugin from File**, and select `pyxel_cube_model_references.js` from this directory.
2. Create a Generic Model scene and save its `.bbmodel` file. Keep the original model files with your project so their relative paths remain valid.
3. Deselect groups and choose **Tools → Cube: Place / Link Model Reference…**. Give the original a short model ID, such as `tree`, and select its saved `.bbmodel`. An optional second `.bbmodel` supplies its static collision shape. The source pivot is the point of the original model that meets the placement group's origin; it defaults to `(0, 0, 0)`.
4. Move, rotate, rename, and duplicate the resulting group with ordinary Blockbench controls. To place another registered model, use the same dialog and select its ID. Different models may use different texture images and the same internal group names.

The scene stores its model registrations and reference IDs inside the `.bbmodel`. There is no separate registry to maintain in Python. The generated children are previews of the original: edit the original file rather than these children. Keep the plugin enabled when saving reference scenes.

## Change originals and update placements

Save the original `.bbmodel`, then choose **Cube: Refresh Model References** in the scene. Every linked placement receives its new geometry, textures, and motions. Placement origins, rotations, nesting, and display names remain unchanged. Refresh can be undone as one edit; ordinary scene geometry outside references remains editable.

Select an existing copied group and use **Cube: Place / Link Model Reference…** to replace its contents with a chosen original. The selected group itself keeps its placement and pivot. Set the source pivot to match the original. This does not infer a model type from a name, or recover an instance scale that was previously baked into individually edited vertices. Undo restores the previous contents.

To change only one placement, select its outer group and choose **Cube: Make Selected Reference Unique**. It keeps its current edits and becomes ordinary editable geometry, with independent textures and copies of its motion channels. The original files are not needed for this action. Subsequent refreshes leave it unchanged. You can convert only some copied groups, keeping custom scenery beside references.

## Export the scene

Choose **Cube: Refresh and Export Scene…**. This refreshes the previews, then saves:

- `level.glb`: visible scenery and motions, with embedded textures, scale 1, and armature disabled.
- `level_collision.glb`: the static collision shapes from the same placements, when any exist.

Generated collision shapes are hidden in the editor and retained in the collision export. Use this action for scenes with collision groups. The standard GLTF export includes hidden editor groups, so an eye icon alone does not exclude collision geometry. **Cube: Toggle Selected Collision Group** can also mark an ordinary group as collision-only.

Load the collision GLB as one mesh collider rather than one collider per placed tile:

```python
mesh = Mesh.from_glb("level.glb")
level = Node.from_mesh(mesh)
level.collider = Collider(mesh=Mesh.from_glb("level_collision.glb"), mass=0, gravity=0)
scene.add_child(level)
```

Both files use the same authored placements. Collision boxes are collected into one GLB; the tool does not perform a geometric union or remove internal faces. Use suitable simplified collision originals and check movement across their boundaries. Animation of visible parts does not animate this static terrain collider; moving platforms and characters still use game-controlled placement nodes and colliders.

## Add motions later

Original transform animations are retained. Save animated originals in Blockbench 5 or later before linking them; older animation coordinates require Blockbench's own conversion. Clips are named `model_id/original_clip`, such as `tree/sway`. One clip includes the corresponding channels of all placements of that original. Play it on a placement subtree to animate that placement independently, or on the entire level to synchronize them:

```python
motions = {motion.name: motion for motion in mesh.motions}
trees = level.find_by_name("Tree")
trees[0].play_motion(motions["tree/sway"])
trees[1].play_motion(motions["tree/sway"], start_frame=15)
```

Use a placement name such as `Tree` that identifies the outer groups you want to control. The tool preserves display names; `find_by_name` returns every match, including any identically named inner parts. Making a placement unique copies its channels to `group_name/original_clip`, adding a number if that name is already used.

Sources support Generic Model cubes, meshes, groups, textures, and node transform animations. Sources containing further model references, animation effects, or animated collision definitions are rejected before changing the scene. To animate generated model parts by hand, edit their original or make the placement unique first. Refresh refuses scene-authored keyframes targeting generated parts and edits to generated clips. Undo those edits, or make the affected placements unique to keep them. Changes to a shared clip's settings affect all its placements, so make all of those placements unique before refreshing.

Save and reopen the scene to continue working. Keep its relative layout to source files when moving the project; changing the source file's name or moving the scene to a different directory also requires updating that registered path. Select a linked group and reopen **Place / Link Model Reference…** to change its registered source files or pivot for all placements. The runtime does not need Blockbench, this plugin, or `.bbmodel` files.
