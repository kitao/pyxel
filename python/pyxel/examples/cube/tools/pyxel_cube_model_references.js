/* Local Blockbench tool: editable model references, refreshed before GLB export. */
(() => {
  const KEY = "pyxel_cube_models";
  const REF = "pyxel_cube_model";
  const COLLISION = "pyxel_cube_collision";
  const OWNER = "pyxel_cube_owner";
  const BASELINE = "pyxel_cube_motion_baseline";
  const { createHash } = require("crypto");
  const clone = (value) => JSON.parse(JSON.stringify(value));

  // Model documents

  function normalize(input) {
    const model = clone(input);
    if (model.meta?.model_format !== "free") {
      throw new Error("Use a Blockbench Generic Model (.bbmodel).");
    }
    if (parseFloat(model.meta.format_version) < 5 && input.animations?.length)
      throw new Error(
        "Open animated originals in Blockbench 5 and save them again before linking.",
      );

    model.groups ||= [];
    model.elements ||= [];
    model.textures ||= [];
    model.animations ||= [];
    model.outliner ||= [];
    const groups = new Map(model.groups.map((group) => [group.uuid, group]));
    function visit(list) {
      for (const node of list) {
        if (typeof node === "string") continue;
        if (!groups.has(node.uuid)) {
          const group = { ...node };
          delete group.children;
          groups.set(node.uuid, group);
          model.groups.push(group);
        }
        visit(node.children || []);
        for (const key of Object.keys(node))
          if (!["uuid", "children", "isOpen"].includes(key)) delete node[key];
      }
    }
    visit(model.outliner);

    model.meta.format_version = "5.0";
    return model;
  }

  function subtree(node) {
    return typeof node === "string"
      ? [node]
      : [node.uuid, ...(node.children || []).flatMap(subtree)];
  }

  // Ignore editor selection and generated IDs, but retain every authored value.
  function signature(value) {
    function canonical(item) {
      if (Array.isArray(item)) return item.map(canonical);
      if (!item || typeof item !== "object") return item;
      return Object.fromEntries(
        Object.keys(item)
          .sort()
          .filter((key) => !["uuid", "selected", OWNER, BASELINE].includes(key))
          .map((key) => [key, canonical(item[key])]),
      );
    }
    return createHash("sha256")
      .update(JSON.stringify(canonical(value)))
      .digest("hex");
  }

  function motionTargetSignature(animator) {
    // The target UUID identifies a bone; its display name follows ordinary renames.
    const { name, ...content } = animator;
    return signature(content);
  }

  function motionBaseline(animation) {
    const { animators, ...settings } = animation;
    return {
      settings: signature(settings),
      targets: Object.fromEntries(
        Object.entries(animators || {})
          .filter(([, animator]) => animator.keyframes?.length)
          .map(([id, animator]) => [id, motionTargetSignature(animator)]),
      ),
    };
  }

  function motionUnchanged(animation, liveIds) {
    const baseline = clone(animation[BASELINE] || {});
    // Deleting a placement removes its animation channels too. Removing channels
    // from a bone that still exists remains an authored edit and must be protected.
    if (liveIds && baseline.targets)
      for (const id of Object.keys(baseline.targets))
        if (!liveIds.has(id)) delete baseline.targets[id];
    return signature(baseline) === signature(motionBaseline(animation));
  }

  function acceptCopiedMotion(animation, copies) {
    if (!animation[OWNER] || !animation[BASELINE]?.targets) return;
    const baseline = clone(animation[BASELINE]);
    for (const [original, copy] of copies) {
      const expected = baseline.targets[original];
      const before = animation.animators?.[original];
      const after = animation.animators?.[copy];
      if (
        expected &&
        !baseline.targets[copy] &&
        before &&
        after &&
        motionTargetSignature(before) === expected &&
        motionTargetSignature(after) === expected
      )
        baseline.targets[copy] = expected;
    }
    animation[BASELINE] = baseline;
  }

  function parsePayload(model) {
    // These are the only objects replaced by apply. Native parse appends other
    // project objects such as collections and reference images instead of replacing.
    const payload = {};
    for (const key of [
      "meta",
      "resolution",
      "elements",
      "groups",
      "outliner",
      "textures",
      "animations",
      KEY,
    ])
      if (model[key] !== undefined) payload[key] = clone(model[key]);
    for (const texture of payload.textures || []) {
      delete texture.path;
      delete texture.relative_path;
    }
    return payload;
  }

  // Build the complete candidate before touching the open project. Geometry and
  // animation IDs are remapped together; texture slots stay local to each source.
  function refresh(input, sources, uuid) {
    const model = normalize(input);
    const groups = new Map(model.groups.map((group) => [group.uuid, group]));
    const refs = [];
    function collect(list, inside = false) {
      for (const node of list) {
        if (typeof node === "string") continue;
        const group = groups.get(node.uuid);
        if (group[REF]) {
          if (inside)
            throw new Error(
              "Move nested references outside their generated model, or make it unique first.",
            );
          refs.push({ node, group });
        }
        collect(node.children || [], inside || !!group[REF]);
      }
    }
    collect(model.outliner);

    const removed = new Set(
      refs.flatMap(({ node }) => (node.children || []).flatMap(subtree)),
    );
    const liveIds = new Set(
      [...model.groups, ...model.elements].map((item) => item.uuid),
    );
    for (const animation of model.animations) {
      if (animation[OWNER] && !motionUnchanged(animation, liveIds))
        throw new Error(
          `Animation "${animation.name}" was edited. Make its placements unique to keep these edits, or undo them before refreshing.`,
        );
      if (
        !animation[OWNER] &&
        Object.entries(animation.animators || {}).some(
          ([id, animator]) => removed.has(id) && animator.keyframes?.length,
        )
      ) {
        throw new Error(
          `Animation "${animation.name}" edits generated parts. Edit the original model or make this placement unique first.`,
        );
      }
    }

    for (const animation of model.animations) {
      if (!animation[OWNER])
        for (const id of Object.keys(animation.animators || {})) {
          if (removed.has(id)) delete animation.animators[id];
        }
    }
    model.elements = model.elements.filter(
      (element) => !removed.has(element.uuid),
    );
    model.groups = model.groups.filter((group) => !removed.has(group.uuid));

    // Keep only channels outside refreshed branches. Unique placements have their
    // own clips; shared prototype clips will be assembled again below.
    const survivingIds = new Set(
      [...model.groups, ...model.elements].map((item) => item.uuid),
    );
    // Undo restores clips before removing obsolete ones. Reuse each logical clip's
    // UUID so native add() does not temporarily collide with its own previous name.
    const clipIds = new Map(
      model.animations
        .filter((animation) => animation[OWNER])
        .map((animation) => [animation.name, animation.uuid]),
    );
    model.animations = model.animations.filter((animation) => {
      if (!animation[OWNER]) return true;
      for (const id of Object.keys(animation.animators || {})) {
        if (!survivingIds.has(id)) delete animation.animators[id];
      }
      return Object.keys(animation.animators || {}).length > 0;
    });

    const used = new Set(
      model.elements.flatMap((element) =>
        Object.values(element.faces || {}).map((face) => face.texture),
      ),
    );
    const oldSlots = new Map();
    model.textures = model.textures.filter((texture, index) => {
      if (texture[OWNER] && !used.has(index)) return false;
      oldSlots.set(index, oldSlots.size);
      return true;
    });
    for (const element of model.elements) {
      for (const face of Object.values(element.faces || {})) {
        if (typeof face.texture === "number")
          face.texture = oldSlots.get(face.texture);
      }
    }

    const textureSlots = new Map();
    for (const { node, group } of refs) {
      const definition = model[KEY]?.[group[REF]];
      if (!definition)
        throw new Error(`Missing model registration: ${group[REF]}`);
      node.children = [];
      for (const role of ["visual", "collision"]) {
        const source = sources[group[REF]]?.[role];
        if (!source) {
          if (role === "visual" || definition[role])
            throw new Error(`Missing ${role} source for ${group[REF]}`);
          continue;
        }
        const part = normalize(source);
        if (part.groups.some((item) => item[REF]))
          throw new Error(
            "A source model cannot itself contain model references.",
          );
        if (
          part.elements.some((item) => !["cube", "mesh"].includes(item.type))
        ) {
          throw new Error(
            "Model references currently support cubes, meshes, and groups.",
          );
        }
        if (role === "collision" && part.animations.length)
          throw new Error(
            "Collision sources must be static; animate gameplay colliders in Python.",
          );

        const ids = new Map(
          [...part.groups, ...part.elements].map((item) => [item.uuid, uuid()]),
        );
        const pivot = definition.pivot || [0, 0, 0];
        const offset = (group.origin || [0, 0, 0]).map(
          (value, axis) => value - pivot[axis],
        );
        const shift = (value) => value.map((v, axis) => v + offset[axis]);
        const slots = part.textures.map((texture, index) => {
          const key = `${group[REF]}/${role}/${index}`;
          if (!textureSlots.has(key)) {
            const copy = clone(texture);
            copy.uuid = uuid();
            copy[OWNER] = key;
            copy.uv_width ||= part.resolution.width;
            copy.uv_height ||= part.resolution.height;
            textureSlots.set(key, model.textures.length);
            model.textures.push(copy);
          }
          return textureSlots.get(key);
        });

        for (const item of [...part.groups, ...part.elements]) {
          // Blockbench group visibility does not hide its Three.js descendants.
          // Hide each collision leaf; native GLTF exports hidden objects too.
          if (role === "collision") item.visibility = false;
          item.uuid = ids.get(item.uuid);
          for (const field of ["origin", "from", "to"])
            if (item[field]) item[field] = shift(item[field]);
          for (const face of Object.values(item.faces || {})) {
            if (typeof face.texture === "number")
              face.texture = slots[face.texture];
          }
        }

        function mapTree(list) {
          return list.map((entry) =>
            typeof entry === "string"
              ? ids.get(entry)
              : {
                  ...entry,
                  uuid: ids.get(entry.uuid),
                  children: mapTree(entry.children || []),
                },
          );
        }
        let children = mapTree(part.outliner);
        if (role === "collision") {
          const collision = {
            uuid: uuid(),
            name: "Collision",
            origin: group.origin || [0, 0, 0],
            [COLLISION]: true,
            visibility: false,
          };
          model.groups.push(collision);
          children = [{ uuid: collision.uuid, children }];
        }
        node.children.push(...children);
        model.groups.push(...part.groups);
        model.elements.push(...part.elements);

        for (const animation of part.animations) {
          const animators = {};
          for (const [id, animator] of Object.entries(
            animation.animators || {},
          )) {
            if (!ids.has(id)) {
              if (animator.keyframes?.length)
                throw new Error(
                  "Model references support node transform animation, not animation effects.",
                );
              continue;
            }
            for (const keyframe of animator.keyframes || [])
              keyframe.uuid = uuid();
            animators[ids.get(id)] = animator;
          }

          animation.name = `${group[REF]}/${animation.name}`;
          const existing = model.animations.find(
            (clip) => clip.name === animation.name,
          );
          if (existing) {
            if (existing[OWNER] !== group[REF])
              throw new Error(`Animation name already used: ${animation.name}`);
            Object.assign(existing.animators, animators);
          } else {
            animation.uuid = clipIds.get(animation.name) || uuid();
            animation[OWNER] = group[REF];
            animation.animators = animators;
            model.animations.push(animation);
          }
        }
      }
    }

    for (const animation of model.animations)
      if (animation[OWNER]) animation[BASELINE] = motionBaseline(animation);
    return model;
  }

  function makeUnique(input, id, uuid) {
    const model = normalize(input);
    const group = model.groups.find((item) => item.uuid === id);
    if (!group?.[REF]) throw new Error("Select a model reference group.");

    function find(list) {
      for (const node of list) {
        if (typeof node === "string") continue;
        if (node.uuid === id) return node;
        const child = find(node.children || []);
        if (child) return child;
      }
    }
    const members = new Set(subtree(find(model.outliner)));

    const slots = new Map();
    for (const element of model.elements) {
      if (!members.has(element.uuid)) continue;
      for (const face of Object.values(element.faces || {})) {
        if (typeof face.texture !== "number") continue;
        if (!slots.has(face.texture)) {
          const texture = clone(model.textures[face.texture]);
          texture.uuid = uuid();
          texture[OWNER] = "";
          delete texture.path;
          delete texture.relative_path;
          texture.internal = true;
          texture.saved = false;
          slots.set(face.texture, model.textures.length);
          model.textures.push(texture);
        }
        face.texture = slots.get(face.texture);
      }
    }

    group[REF] = "";
    for (const animation of [...model.animations]) {
      if (!animation[OWNER]) continue;
      const targeted = Object.keys(animation.animators || {}).filter((key) =>
        members.has(key),
      );
      if (!targeted.length) continue;
      const copy = clone(animation);
      copy.uuid = uuid();
      copy[OWNER] = "";
      delete copy[BASELINE];
      copy.animators = {};
      const clip = animation.name.slice(animation.name.indexOf("/") + 1);
      let prefix = group.name;
      for (
        let index = 2;
        model.animations.some((item) => item.name === `${prefix}/${clip}`);
        index++
      )
        prefix = `${group.name} ${index}`;
      copy.name = `${prefix}/${clip}`;

      for (const key of targeted) {
        copy.animators[key] = animation.animators[key];
        delete animation.animators[key];
        if (animation[BASELINE]?.targets)
          delete animation[BASELINE].targets[key];
      }
      model.animations.push(copy);
    }

    model.animations = model.animations.filter(
      (animation) =>
        !animation[OWNER] || Object.keys(animation.animators || {}).length,
    );
    return model;
  }

  if (typeof Plugin === "undefined") {
    module.exports = {
      normalize,
      refresh,
      makeUnique,
      subtree,
      KEY,
      REF,
      COLLISION,
      OWNER,
      BASELINE,
      motionBaseline,
      motionUnchanged,
      acceptCopiedMotion,
      parsePayload,
    };
    return;
  }

  // Blockbench integration

  const actions = [];
  const properties = [];
  const listeners = [];
  let busy = false;
  const fs = require("fs");
  const path = require("path");

  function on(name, callback) {
    Blockbench.on(name, callback);
    listeners.push([name, callback]);
  }

  function snapshot() {
    const model = Codecs.project.compile({ raw: true, bitmaps: true });
    // bitmaps:true changes internal to true in the native save copy.
    for (const texture of model.textures) {
      const original = Texture.all.find((item) => item.uuid === texture.uuid);
      texture.internal = original.internal;
      texture.path = original.path;
    }
    return model;
  }

  function aspects() {
    return {
      elements: [...Outliner.elements],
      groups: [...Group.all],
      textures: [...Texture.all],
      bitmap: true,
      animations: [...Animation.all],
      outliner: true,
      selection: true,
    };
  }

  async function apply(model, label) {
    const liveIds = new Set(
      [...model.groups, ...model.elements].map((item) => item.uuid),
    );
    const cleanClips = new Set(
      model.animations
        .filter(
          (animation) =>
            animation[OWNER] && motionUnchanged(animation, liveIds),
        )
        .map((animation) => animation.uuid),
    );

    Undo.initEdit(aspects());
    try {
      for (const element of [...Outliner.elements]) element.remove();
      for (const group of [...Group.all].reverse()) group.remove();
      for (const texture of [...Texture.all]) texture.remove(true);
      for (const animation of [...Animation.all]) animation.remove();
      Codecs.project.parse(parsePayload(model));
      for (const texture of Texture.all) {
        const original = model.textures.find(
          (item) => item.uuid === texture.uuid,
        );
        for (const key of ["path", "relative_path", "internal", "saved"])
          if (original[key] !== undefined) texture[key] = original[key];
        texture.startWatcher();
      }

      // Native parse supplies animation defaults. Baseline its saved form only for
      // clips already proven clean; Unique must not approve edits on other branches.
      const normalized = Codecs.project.compile({ raw: true, bitmaps: false });
      for (const animation of Animation.all)
        if (cleanClips.has(animation.uuid))
          animation[BASELINE] = motionBaseline(
            normalized.animations.find((item) => item.uuid === animation.uuid),
          );

      // img.complete can become true before native onload copies pixels to the
      // texture canvas. Its load callback runs after that copy and material update.
      await Promise.all(
        Texture.all.map(
          (texture) =>
            new Promise((resolve, reject) => {
              const failed = () =>
                reject(new Error(`Could not load texture: ${texture.name}`));
              texture.img.addEventListener("error", failed, { once: true });
              texture.load(() => {
                texture.img.removeEventListener("error", failed);
                resolve();
              });
            }),
        ),
      );
      Canvas.updateAll();
      Undo.finishEdit(label, aspects());
    } catch (error) {
      Undo.cancelEdit();
      throw error;
    }
  }

  function projectDirectory() {
    if (!Project.save_path)
      throw new Error(
        "Save this scene as a .bbmodel before adding model references.",
      );
    return path.dirname(Project.save_path);
  }

  function readModel(filename) {
    const document = JSON.parse(fs.readFileSync(filename, "utf8"));
    for (const texture of document.textures || []) {
      if (!texture.source?.startsWith("data:")) {
        const imagePath = texture.relative_path
          ? path.resolve(path.dirname(filename), texture.relative_path)
          : texture.path;
        const extension = path
          .extname(imagePath || "")
          .slice(1)
          .replace("jpg", "jpeg");
        texture.source = `data:image/${extension};base64,${fs.readFileSync(imagePath).toString("base64")}`;
      }
      // Embedded source is authoritative; an old external path must not override it.
      delete texture.path;
      delete texture.relative_path;
    }
    return document;
  }

  function refreshed(model) {
    const base = projectDirectory();
    const sources = {};
    for (const id of new Set(
      (model.groups || []).map((group) => group[REF]).filter(Boolean),
    )) {
      const definition = model[KEY]?.[id];
      if (!definition) throw new Error(`Missing model registration: ${id}`);
      sources[id] = {};
      for (const role of ["visual", "collision"]) {
        if (definition[role])
          sources[id][role] = readModel(path.resolve(base, definition[role]));
      }
    }
    return refresh(model, sources, guid);
  }

  async function run(operation) {
    if (busy) return;
    busy = true;
    try {
      await operation();
    } catch (error) {
      Blockbench.showMessageBox({
        title: "Cube Model References",
        message: error.message,
      });
    } finally {
      busy = false;
    }
  }

  function link() {
    const base = projectDirectory();
    const selected = Group.first_selected;
    const registry = Project[KEY] || {};
    const options = {
      "": "Register a model…",
      ...Object.fromEntries(Object.keys(registry).map((id) => [id, id])),
    };
    let shownId = selected?.[REF] || "";
    const initial = registry[shownId] || {};
    const absolute = (filename) =>
      filename ? path.resolve(base, filename) : "";

    new Dialog({
      id: "pyxel_cube_reference",
      title: selected ? "Link Selected Group" : "Place Model Reference",
      lines: selected
        ? [
            "Replaces this group's contents, preserving its position, rotation and pivot. Changing a registered source updates every linked placement.",
          ]
        : [],
      form: {
        registered: { type: "select", label: "Model", options, value: shownId },
        id: {
          type: "text",
          label: "New model ID",
          condition: (form) => !form.registered,
        },
        visual: {
          type: "file",
          label: "Model (.bbmodel)",
          value: absolute(initial.visual),
          extensions: ["bbmodel"],
          readtype: "text",
        },
        collision: {
          type: "file",
          label: "Collision (.bbmodel, optional)",
          value: absolute(initial.collision),
          extensions: ["bbmodel"],
          readtype: "text",
        },
        pivot: {
          type: "vector",
          label: "Source pivot",
          dimensions: 3,
          value: initial.pivot || [0, 0, 0],
        },
      },
      onFormChange(values) {
        if (values.registered === shownId) return;
        shownId = values.registered;
        const definition = registry[shownId] || {};
        this.setFormValues({
          visual: absolute(definition.visual),
          collision: absolute(definition.collision),
          pivot: definition.pivot || [0, 0, 0],
        });
      },
      onConfirm(values) {
        this.hide();
        run(async () => {
          const model = normalize(snapshot());
          const id = values.registered || values.id.trim();
          if (!id || id.includes("/"))
            throw new Error("Give the original model an ID without a slash.");
          model[KEY] ||= {};
          if (!values.registered && model[KEY][id])
            throw new Error(
              "That model ID already exists. Choose it from the Model list.",
            );
          if (!values.visual)
            throw new Error("Choose the original .bbmodel file.");

          model[KEY][id] = {
            visual: path.relative(base, values.visual),
            collision: values.collision
              ? path.relative(base, values.collision)
              : "",
            pivot: values.pivot,
          };

          let group = model.groups.find((item) => item.uuid === selected?.uuid);
          if (!group) {
            group = {
              uuid: guid(),
              name: id,
              origin: [0, 0, 0],
              rotation: [0, 0, 0],
            };
            model.groups.push(group);
            model.outliner.push({ uuid: group.uuid, children: [] });
          }
          group[REF] = id;

          await apply(refreshed(model), "Link Cube model reference");
          Group.all.find((item) => item.uuid === group.uuid)?.select();
        });
      },
    }).show();
  }

  async function compileRole(collision) {
    const detached = [];
    const roots = [];
    function visit(node, insideCollision = false) {
      const inside = insideCollision || !!node[COLLISION];
      const contains =
        node instanceof Group &&
        node.children.some(
          (child) =>
            child[COLLISION] || (child instanceof Group && hasCollision(child)),
        );
      const keep = collision ? inside || contains : !inside;
      if (!keep) {
        const object = node.mesh;
        if (node.parent === "root") {
          roots.push([object, object.no_export]);
          object.no_export = true;
        } else if (object?.parent) {
          detached.push([
            object,
            object.parent,
            object.parent.children.indexOf(object),
          ]);
          object.parent.remove(object);
        }
        return;
      }
      if (node instanceof Group)
        for (const child of node.children) visit(child, inside);
    }

    function hasCollision(group) {
      return group.children.some(
        (child) =>
          child[COLLISION] || (child instanceof Group && hasCollision(child)),
      );
    }

    for (const node of Outliner.root) visit(node);
    try {
      return await Codecs.gltf.compile({
        encoding: "binary",
        scale: 1,
        embed_textures: true,
        armature: false,
        animations: !collision,
      });
    } finally {
      for (const [object, parent, index] of detached.reverse()) {
        parent.add(object);
        parent.children.splice(parent.children.indexOf(object), 1);
        parent.children.splice(index, 0, object);
      }
      for (const [object, value] of roots) object.no_export = value;
    }
  }

  async function exportScene() {
    await apply(refreshed(snapshot()), "Refresh Cube model references");
    const visual = await compileRole(false);
    const collision = Group.all.some((group) => group[COLLISION])
      ? await compileRole(true)
      : null;

    Blockbench.export({
      type: "Cube scene",
      extensions: ["glb"],
      name: Project.name,
      content: visual,
      custom_writer(content, filename) {
        fs.writeFileSync(filename, Buffer.from(content));
        if (collision)
          fs.writeFileSync(
            filename.replace(/\.glb$/i, "_collision.glb"),
            Buffer.from(collision),
          );
      },
    });
  }

  Plugin.register("pyxel_cube_model_references", {
    title: "Pyxel Cube Model References",
    author: "Pyxel",
    version: "1.0.0",
    description:
      "Place editable model references, refresh their previews and export visual/collision GLBs.",
    variant: "desktop",
    min_version: "5.0.0",
    onload() {
      properties.push(
        new Property(ModelProject, "object", KEY, { default: {} }),
      );
      properties.push(new Property(Group, "string", REF));
      properties.push(new Property(Group, "boolean", COLLISION));
      properties.push(new Property(Texture, "string", OWNER));
      properties.push(new Property(Animation, "string", OWNER));
      properties.push(
        new Property(Animation, "object", BASELINE, { default: {} }),
      );

      on("finish_edit", ({ message }) => {
        if (message !== "Copy animations of duplicated bones") return;
        const copies = [...Clipbench.duplicate_map].map(([original, copy]) => [
          original.uuid,
          copy.uuid,
        ]);
        for (const animation of Animation.all) {
          const saved = animation.getUndoCopy({});
          acceptCopiedMotion(saved, copies);
          if (saved[BASELINE]) animation[BASELINE] = saved[BASELINE];
        }
      });
      on("create_undo_save", ({ save }) => {
        save[KEY] = clone(Project[KEY] || {});
      });
      on("load_undo_save", ({ save }) => {
        if (save[KEY]) Project[KEY] = clone(save[KEY]);
        // Native Animation.extend restores saved targets but retains extra ones.
        // Restore generated clips exactly; do not approve the leftover channels.
        for (const [id, saved] of Object.entries(save.animations || {})) {
          if (!saved[OWNER]) continue;
          const animation = Animation.all.find((item) => item.uuid === id);
          if (!animation) continue;
          for (const target of Object.keys(animation.animators))
            if (!Object.hasOwn(saved.animators || {}, target))
              animation.removeAnimator(target);
        }
      });

      function action(id, name, click) {
        const item = new Action(id, {
          name,
          icon: "view_in_ar",
          condition: () => Project && Format.id === "free",
          click: () => run(click),
        });
        actions.push(item);
        MenuBar.addAction(item, "tools");
      }
      action("pyxel_cube_link", "Cube: Place / Link Model Reference…", link);
      action("pyxel_cube_refresh", "Cube: Refresh Model References", () =>
        apply(refreshed(snapshot()), "Refresh Cube model references"),
      );
      action(
        "pyxel_cube_unique",
        "Cube: Make Selected Reference Unique",
        () => {
          const selected = Group.first_selected;
          if (!selected?.[REF])
            throw new Error("Select a model reference group.");
          return apply(
            makeUnique(snapshot(), selected.uuid, guid),
            "Make Cube model unique",
          );
        },
      );
      action(
        "pyxel_cube_collision",
        "Cube: Toggle Selected Collision Group",
        () => {
          const group = Group.first_selected;
          if (!group) throw new Error("Select a collision group.");
          Undo.initEdit({ groups: [group] });
          group[COLLISION] = !group[COLLISION];
          Undo.finishEdit("Set Cube collision group");
        },
      );
      action(
        "pyxel_cube_export",
        "Cube: Refresh and Export Scene…",
        exportScene,
      );
    },
    onunload() {
      for (const action of actions) action.delete();
      for (const property of properties) property.delete();
      for (const [name, callback] of listeners)
        Blockbench.removeListener(name, callback);
    },
  });
})();
