const assert = require("node:assert/strict");
const test = require("node:test");
const {
  refresh,
  makeUnique,
  normalize,
  subtree,
  KEY,
  REF,
  OWNER,
  COLLISION,
  BASELINE,
  motionUnchanged,
  parsePayload,
  acceptCopiedMotion,
} = require("../../python/pyxel/examples/cube/tools/pyxel_cube_model_references.js");

function ids() {
  let next = 0;
  return () => `generated-${++next}`;
}

function source(color = "red") {
  return {
    meta: { model_format: "free", format_version: "5.0" },
    resolution: { width: 16, height: 16 },
    groups: [{ uuid: "body", name: "Same Name", origin: [1, 2, 3] }],
    elements: [
      {
        uuid: "cube",
        type: "cube",
        from: [0, 0, 0],
        to: [2, 4, 6],
        origin: [1, 2, 3],
        faces: { north: { texture: 0, uv: [0, 0, 16, 16] } },
      },
    ],
    outliner: [{ uuid: "body", children: ["cube"] }],
    textures: [{ uuid: "same-source-texture-id", source: color }],
    animations: [
      {
        uuid: "move",
        name: "move",
        animators: {
          body: {
            type: "bone",
            keyframes: [
              {
                uuid: "frame",
                channel: "position",
                time: 0,
                data_points: [{ x: 0, y: 0, z: 0 }],
              },
            ],
          },
        },
      },
    ],
  };
}

function scene() {
  return {
    meta: { model_format: "free", format_version: "5.0" },
    resolution: { width: 16, height: 16 },
    [KEY]: {
      tree: {
        visual: "tree.bbmodel",
        collision: "solid.bbmodel",
        pivot: [1, 2, 3],
      },
      rock: { visual: "rock.bbmodel", pivot: [1, 2, 3] },
    },
    groups: [
      {
        uuid: "a",
        name: "Same Name",
        origin: [10, 20, 30],
        rotation: [0, 45, 0],
        [REF]: "tree",
      },
      {
        uuid: "b",
        name: "Same Name",
        origin: [40, 50, 60],
        rotation: [0, -90, 0],
        [REF]: "tree",
      },
      { uuid: "c", name: "Same Name", origin: [70, 80, 90], [REF]: "rock" },
    ],
    elements: [
      {
        uuid: "custom",
        type: "cube",
        from: [0, 0, 0],
        to: [1, 1, 1],
        faces: {},
      },
    ],
    outliner: [
      { uuid: "a", children: [] },
      { uuid: "b", children: [] },
      { uuid: "c", children: [] },
      "custom",
    ],
    textures: [],
    animations: [],
  };
}

function sources() {
  const collision = source();
  collision.animations = [];
  return {
    tree: { visual: source("red"), collision },
    rock: { visual: source("blue") },
  };
}

test("references preserve placement and separate prototype textures and motion targets", () => {
  const input = scene();
  const original = JSON.stringify(input);
  const output = refresh(input, sources(), ids());
  assert.equal(JSON.stringify(input), original);
  for (let i = 0; i < 3; i++)
    assert.deepEqual(output.groups[i], input.groups[i]);
  assert.equal(output.elements.length, 6);
  assert.equal(output.textures.length, 3); // one visual per prototype, one collision
  assert.equal(output.animations.length, 2);
  for (const [asset, placements] of [
    ["tree", ["a", "b"]],
    ["rock", ["c"]],
  ]) {
    const members = new Set(
      placements.flatMap((id) =>
        subtree(output.outliner.find((node) => node.uuid === id)),
      ),
    );
    const animation = output.animations.find(
      (clip) => clip.name === `${asset}/move`,
    );
    assert(animation);
    assert.equal(Object.keys(animation.animators).length, placements.length);
    assert(Object.keys(animation.animators).every((id) => members.has(id)));
  }

  const first = output.groups.find(
    (group) => group.uuid === output.outliner[0].children[0].uuid,
  );
  assert.deepEqual(first.origin, [10, 20, 30]);
  assert.deepEqual(
    output.groups
      .filter((group) => group[COLLISION])
      .map((group) => group.origin),
    [
      [10, 20, 30],
      [40, 50, 60],
    ],
  );

  const visualElements = [0, 1, 2].map((index) => {
    const node = output.outliner[index].children[0];
    return output.elements.find((element) => element.uuid === node.children[0]);
  });
  assert.equal(
    visualElements[0].faces.north.texture,
    visualElements[1].faces.north.texture,
  );
  assert.notEqual(
    visualElements[0].faces.north.texture,
    visualElements[2].faces.north.texture,
  );
  assert.equal(
    output.textures[visualElements[2].faces.north.texture].source,
    "blue",
  );
});

test("refresh propagates prototype geometry edits without accumulating previews, textures or clips", () => {
  const uuid = ids();
  const first = refresh(scene(), sources(), uuid);
  const updatedSources = sources();
  updatedSources.tree.visual.elements[0].to[1] = 8;
  const second = refresh(first, updatedSources, uuid);
  for (const key of ["elements", "groups", "textures", "animations"])
    assert.equal(second[key].length, first[key].length);
  for (const placement of ["a", "b"]) {
    const node = second.outliner.find((node) => node.uuid === placement)
      .children[0];
    const element = second.elements.find(
      (element) => element.uuid === node.children[0],
    );
    const origin = second.groups.find(
      (group) => group.uuid === placement,
    ).origin;
    assert.equal(element.to[1], origin[1] - 2 + 8);
  }
  assert.deepEqual(
    second.elements.find((element) => element.uuid === "custom"),
    first.elements.find((element) => element.uuid === "custom"),
  );
});

test("make unique detaches textures and keeps geometry, collisions and motion on future refresh", () => {
  const uuid = ids();
  const initial = refresh(scene(), sources(), uuid);
  const unique = makeUnique(initial, "a", uuid);
  const members = new Set(subtree(unique.outliner[0]));
  const uniqueElements = unique.elements.filter((element) =>
    members.has(element.uuid),
  );
  const secondElement = unique.elements.find(
    (element) => element.uuid === unique.outliner[1].children[0].children[0],
  );
  assert.notEqual(
    uniqueElements[0].faces.north.texture,
    secondElement.faces.north.texture,
  );
  assert.equal(
    unique.animations.find((animation) => animation.name === "Same Name/move")[
      OWNER
    ],
    "",
  );

  const changed = sources();
  changed.tree.visual.elements[0].to[1] = 100;
  const refreshed = refresh(unique, changed, uuid);
  for (const element of uniqueElements) {
    const actual = refreshed.elements.find(
      (item) => item.uuid === element.uuid,
    );
    assert.deepEqual(actual.from, element.from);
    assert.deepEqual(actual.to, element.to);
    assert.deepEqual(actual.origin, element.origin);
    assert.deepEqual(
      refreshed.textures[actual.faces.north.texture],
      unique.textures[element.faces.north.texture],
    );
  }
  assert(
    refreshed.groups.some(
      (group) => members.has(group.uuid) && group[COLLISION],
    ),
  );
  assert(
    refreshed.animations.some(
      (animation) => animation.name === "Same Name/move",
    ),
  );
});

test("duplicated references get independent animation targets after refresh", () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  // Blockbench duplicating the placement copies its model property; refresh must
  // derive ownership from the new placement UUID, not its name or old clips.
  model.groups.push({ ...model.groups[0], uuid: "duplicate" });
  model.outliner.push({ uuid: "duplicate", children: [] });
  const output = refresh(model, sources(), uuid);
  assert.equal(output.animations.length, 2);
  const shared = output.animations.find((clip) => clip.name === "tree/move");
  assert.equal(Object.keys(shared.animators).length, 3);
  for (const id of ["a", "b", "duplicate"]) {
    const members = new Set(
      subtree(output.outliner.find((node) => node.uuid === id)),
    );
    assert.equal(
      Object.keys(shared.animators).filter((target) => members.has(target))
        .length,
      1,
    );
  }
});

test("unsupported updates fail before mutation instead of losing authored data", () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  const target = model.outliner[0].children[0].uuid;
  model.animations.push({
    name: "Hand Edited",
    animators: { [target]: { keyframes: [{ channel: "position" }] } },
  });
  const original = JSON.stringify(model);
  assert.throws(() => refresh(model, sources(), uuid), /edits generated parts/);
  assert.equal(JSON.stringify(model), original);

  const bad = sources();
  bad.tree.collision.animations = source().animations;
  assert.throws(
    () => refresh(scene(), bad, ids()),
    /Collision sources must be static/,
  );
});

test("legacy nested groups normalize without retaining stale coordinate fields in outliner", () => {
  const model = source();
  model.meta.format_version = "4.10";
  model.animations = [];
  model.outliner[0] = { ...model.groups[0], children: ["cube"] };
  delete model.groups;
  const result = normalize(model);
  assert.deepEqual(result.groups[0].origin, [1, 2, 3]);
  assert.equal(result.outliner[0].origin, undefined);
  assert.equal(result.meta.format_version, "5.0");
});

test("source clip changes update references while two unique placements keep separate clips", () => {
  const uuid = ids();
  let model = refresh(scene(), sources(), uuid);
  model = makeUnique(model, "a", uuid);
  model = makeUnique(model, "b", uuid);
  const independent = model.animations.filter((animation) => !animation[OWNER]);
  assert.deepEqual(
    independent.map((animation) => animation.name),
    ["Same Name/move", "Same Name 2/move"],
  );
  assert.equal(
    model.animations.some((animation) => animation.name === "tree/move"),
    false,
  );

  const changed = sources();
  changed.rock.visual.animations[0].length = 3;
  changed.tree.visual.animations[0].length = 5;
  const updated = refresh(model, changed, uuid);
  assert.deepEqual(
    updated.animations.filter((animation) => !animation[OWNER]),
    independent,
  );
  assert.equal(
    updated.animations.find((animation) => animation.name === "rock/move")
      .length,
    3,
  );
});

test("imported clip names cannot overwrite scene-authored clips", () => {
  const model = scene();
  model.animations.push({ name: "tree/move", animators: {} });
  assert.throws(
    () => refresh(model, sources(), ids()),
    /Animation name already used: tree\/move/,
  );
  assert.equal(model.animations.length, 1);
});

test("empty auto-created animators are removed without blocking a reference conversion", () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  const target = model.outliner[0].children[0].uuid;
  model.animations.push({
    name: "Scene clip",
    animators: { [target]: { keyframes: [] } },
  });
  const result = refresh(model, sources(), uuid);
  assert.deepEqual(
    result.animations.find((animation) => animation.name === "Scene clip")
      .animators,
    {},
  );
});

test("edited generated channels block refresh and unique preserves edits without approving other targets", () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  const clip = model.animations.find(
    (animation) => animation.name === "tree/move",
  );
  const targets = Object.keys(clip.animators);
  for (const target of targets)
    clip.animators[target].keyframes[0].data_points[0].y = 123;
  assert.throws(() => refresh(model, sources(), uuid), /was edited/);

  const unique = makeUnique(model, "a", uuid);
  const copy = unique.animations.find(
    (animation) => animation.name === "Same Name/move",
  );
  assert.equal(copy.animators[targets[0]].keyframes[0].data_points[0].y, 123);
  assert.equal(copy[BASELINE], undefined);
  assert.throws(() => refresh(unique, sources(), uuid), /was edited/);

  const bothUnique = makeUnique(unique, "b", uuid);
  assert.doesNotThrow(() => refresh(bothUnique, sources(), uuid));
});

test("unique removes only its baseline targets so a clean remaining placement can refresh", () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  const clip = model.animations.find(
    (animation) => animation.name === "tree/move",
  );
  const target = model.outliner[0].children[0].uuid;
  clip.animators[target].keyframes[0].data_points[0].y = 123;

  const unique = makeUnique(model, "a", uuid);
  assert(
    motionUnchanged(
      unique.animations.find((animation) => animation.name === "tree/move"),
    ),
  );
  assert.doesNotThrow(() => refresh(unique, sources(), uuid));
});

test("clip settings and deleted channels are edits, while selection and empty animators are not", () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  const clip = model.animations[0];
  clip.selected = true;
  clip.animators.unused = { keyframes: [] };
  assert(motionUnchanged(clip));

  clip.length = 8;
  assert.throws(() => refresh(model, sources(), uuid), /was edited/);

  delete clip.length;
  delete clip.animators[Object.keys(clip.animators)[0]];
  assert.throws(() => refresh(model, sources(), uuid), /was edited/);
});

test("legacy animated originals require native migration instead of silently changing coordinates", () => {
  const model = source();
  model.meta.format_version = "4.10";
  model.animations[0].animators.body.keyframes[0].data_points[0].x = 2;
  assert.throws(() => normalize(model), /Blockbench 5/);
});

test("parse payload uses embedded pixels and leaves unrelated project objects in place", () => {
  const model = scene();
  model.textures = [
    {
      uuid: "painted",
      source: "fresh pixels",
      path: "/old.png",
      relative_path: "old.png",
      saved: false,
    },
  ];
  for (const key of [
    "collections",
    "reference_images",
    "texture_groups",
    "animation_controllers",
  ])
    model[key] = [{ uuid: key }];

  const payload = parsePayload(model);
  assert.equal(payload.textures[0].source, "fresh pixels");
  assert.equal(payload.textures[0].path, undefined);
  assert.equal(payload.textures[0].relative_path, undefined);
  assert.equal(model.textures[0].path, "/old.png");
  for (const key of [
    "collections",
    "reference_images",
    "texture_groups",
    "animation_controllers",
  ])
    assert.equal(payload[key], undefined);
});

// Execute the registered action as well as the document helpers. This catches an
// accidental pre-refresh, which would overwrite edits or require missing originals.
function nativeHarness(document, selected = "a", deferTextures = false) {
  const vm = require("node:vm");
  const fs = require("node:fs");
  const script = fs.readFileSync(
    require.resolve("../../python/pyxel/examples/cube/tools/pyxel_cube_model_references.js"),
    "utf8",
  );
  const copy = (value) => JSON.parse(JSON.stringify(value));
  let parsed;
  const actions = {};
  const errors = [];
  const listeners = {};
  const pendingLoads = [];
  const finished = [];
  const exports = [];
  const sandbox = {
    require(name) {
      if (name === "fs")
        return {
          readFileSync() {
            throw new Error("Must not read missing originals");
          },
        };
      return require(name);
    },
    Plugin: {
      register(_id, plugin) {
        plugin.onload();
      },
    },
    Property: function () {},
    Action: function (id, definition) {
      actions[id] = definition.click;
    },
    ModelProject: {},
    Project: { save_path: "/missing/scene.bbmodel" },
    Format: { id: "free" },
    Texture: { all: [] },
    Group: {
      all: [],
      first_selected: document.groups.find((group) => group.uuid === selected),
    },
    Animation: { all: [] },
    Outliner: { elements: [], root: [] },
    Undo: {
      initEdit() {},
      finishEdit() {
        finished.push(
          sandbox.Texture.all.every((texture) => texture.canvasReady),
        );
      },
      cancelEdit() {},
    },
    Canvas: { updateAll() {} },
    MenuBar: { addAction() {} },
    Blockbench: {
      export(result) {
        exports.push(result);
      },
      on(name, callback) {
        listeners[name] = callback;
      },
      showMessageBox(box) {
        errors.push(box.message);
      },
    },
    guid: ids(),
    Codecs: {
      gltf: {
        async compile() {
          assert(sandbox.Texture.all.every((texture) => texture.canvasReady));
          return new ArrayBuffer(8);
        },
      },
      project: {
        compile() {
          return parsed
            ? { ...copy(parsed), animations: copy(sandbox.Animation.all) }
            : copy(document);
        },
        parse(payload) {
          parsed = copy(payload);
          sandbox.Texture.all = payload.textures.map((texture) => ({
            ...texture,
            internal: true,
            saved: false,
            startWatcher() {},
            img: {
              complete: true,
              addEventListener() {},
              removeEventListener() {},
            },
            load(callback) {
              const complete = () => {
                this.canvasReady = true;
                callback(this);
              };
              if (deferTextures) pendingLoads.push(complete);
              else complete();
            },
          }));
          sandbox.Animation.all = payload.animations.map((animation) => ({
            ...animation,
            loop: animation.loop || "once",
            getUndoCopy() {
              return copy(this);
            },
          }));
        },
      },
    },
  };

  sandbox.Texture.all = document.textures.map((texture) => ({
    ...texture,
    internal: false,
    remove(noUpdate) {
      assert.equal(noUpdate, true);
    },
  }));
  vm.runInNewContext(script, sandbox);
  return {
    actions,
    errors,
    sandbox,
    listeners,
    pendingLoads,
    finished,
    exports,
    parsed: () => parsed,
  };
}

test("native Unique action never refreshes and retains authored geometry, texture metadata and custom clips", async () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  const target = model.outliner[0].children[0];
  model.elements.find((element) => element.uuid === target.children[0]).to[1] =
    123;
  model.textures[0].source = "fresh pixels";
  model.textures[0].path = "/old.png";
  model.textures[0].relative_path = "old.png";
  model.textures[0].saved = false;
  model.animations.push({
    name: "Hand Edited",
    animators: { [target.uuid]: { keyframes: [{ time: 0 }] } },
  });

  const harness = nativeHarness(model);
  await harness.actions.pyxel_cube_unique();
  assert.deepEqual(harness.errors, []);
  const result = harness.parsed();
  assert.equal(
    result.elements.find((element) => element.uuid === target.children[0])
      .to[1],
    123,
  );
  assert(
    result.animations.some((animation) => animation.name === "Hand Edited"),
  );
  const texture = harness.sandbox.Texture.all.find(
    (item) => item.uuid === model.textures[0].uuid,
  );
  assert.equal(texture.source, "fresh pixels");
  assert.equal(texture.path, "/old.png");
  assert.equal(texture.relative_path, "old.png");
  assert.equal(texture.internal, false);
  assert.equal(texture.saved, false);

  const uniqueElement = result.elements.find(
    (element) => element.uuid === target.children[0],
  );
  const uniqueTexture =
    harness.sandbox.Texture.all[uniqueElement.faces.north.texture];
  assert.equal(uniqueTexture.path, undefined);
  assert.equal(uniqueTexture.relative_path, undefined);
  assert.equal(uniqueTexture.internal, true);
});

test("native apply normalizes clean baselines without blessing edits remaining after partial unique", async () => {
  for (const dirty of [false, true]) {
    const model = refresh(scene(), sources(), ids());
    const clip = model.animations.find(
      (animation) => animation.name === "tree/move",
    );
    const target = model.outliner[1].children[0].uuid;
    if (dirty) clip.animators[target].keyframes[0].data_points[0].y = 123;
    const harness = nativeHarness(model);
    await harness.actions.pyxel_cube_unique();
    assert.deepEqual(harness.errors, []);
    const remaining = harness.sandbox.Animation.all
      .find((animation) => animation.name === "tree/move")
      .getUndoCopy();
    assert.equal(motionUnchanged(remaining), !dirty);
  }
});

test("collision leaves stay hidden through save, unique and refresh without removing export geometry", () => {
  const uuid = ids();
  const originals = sources();
  const collision = originals.tree.collision;
  collision.elements.push({
    uuid: "collision-mesh",
    type: "mesh",
    visibility: true,
    origin: [0, 0, 0],
    vertices: { a: [0, 0, 0], b: [1, 0, 0], c: [0, 1, 0] },
    faces: { triangle: { vertices: ["a", "b", "c"], texture: 0 } },
  });
  collision.groups.push({ uuid: "nested", name: "Nested", visibility: true });
  collision.outliner[0].children.push({
    uuid: "nested",
    children: ["collision-mesh"],
  });

  const saved = JSON.parse(JSON.stringify(refresh(scene(), originals, uuid)));
  const output = refresh(makeUnique(saved, "a", uuid), originals, uuid);
  for (const placement of output.outliner.slice(0, 2)) {
    const collisionRoot = placement.children.find(
      (node) =>
        output.groups.find((group) => group.uuid === node.uuid)?.[COLLISION],
    );
    const members = new Set(subtree(collisionRoot));
    const elements = output.elements.filter((element) =>
      members.has(element.uuid),
    );
    assert.deepEqual(
      elements.map((element) => element.type),
      ["cube", "mesh"],
    );
    for (const item of [...output.groups, ...elements].filter((item) =>
      members.has(item.uuid),
    )) {
      assert.equal(item.visibility, false);
      assert.notEqual(item.export, false);
    }
    const visual = output.elements.find(
      (element) => element.uuid === placement.children[0].children[0],
    );
    assert.notEqual(visual.visibility, false);
  }
});

function duplicatePlacement(model) {
  const original = model.outliner[0];
  const mapping = new Map(subtree(original).map((id) => [id, `copy-${id}`]));
  for (const collection of [model.groups, model.elements])
    for (const item of [...collection])
      if (mapping.has(item.uuid))
        collection.push({
          ...JSON.parse(JSON.stringify(item)),
          uuid: mapping.get(item.uuid),
        });

  function copyTree(node) {
    return typeof node === "string"
      ? mapping.get(node)
      : {
          ...node,
          uuid: mapping.get(node.uuid),
          children: node.children.map(copyTree),
        };
  }
  model.outliner.push(copyTree(original));
  for (const clip of model.animations)
    for (const [id, animator] of Object.entries(clip.animators))
      if (mapping.has(id))
        clip.animators[mapping.get(id)] = {
          ...JSON.parse(JSON.stringify(animator)),
          name: "Duplicated name",
        };
  return [...mapping];
}

test("native duplicate animation event accepts unchanged copied channels and survives saved forms", () => {
  const model = refresh(scene(), sources(), ids());
  const copies = duplicatePlacement(model);

  const harness = nativeHarness(model);
  harness.sandbox.Clipbench = {
    duplicate_map: new Map(
      copies.map(([original, copy]) => [{ uuid: original }, { uuid: copy }]),
    ),
  };
  harness.sandbox.Animation.all = model.animations.map((clip) => ({
    ...clip,
    getUndoCopy() {
      return JSON.parse(JSON.stringify(this));
    },
  }));

  harness.listeners.finish_edit({
    message: "Copy animations of duplicated bones",
  });
  model.animations = JSON.parse(JSON.stringify(harness.sandbox.Animation.all));
  assert(model.animations.every((clip) => motionUnchanged(clip)));

  const copiedClip = model.animations[0];
  const copiedTarget = Object.keys(copiedClip.animators).find((id) =>
    id.startsWith("copy-"),
  );
  copiedClip.animators[copiedTarget].keyframes[0].data_points[0].y = 123;
  assert.throws(() => refresh(model, sources(), ids()), /was edited/);

  copiedClip.animators[copiedTarget].keyframes[0].data_points[0].y = 0;
  const updated = refresh(JSON.parse(JSON.stringify(model)), sources(), ids());
  assert.equal(
    Object.keys(
      updated.animations.find((clip) => clip.name === "tree/move").animators,
    ).length,
    3,
  );
});

test("duplicating an edited channel does not approve original or copied edits", () => {
  const model = refresh(scene(), sources(), ids());
  const clip = model.animations[0];
  Object.values(clip.animators)[0].keyframes[0].data_points[0].y = 123;
  const copies = duplicatePlacement(model);
  acceptCopiedMotion(clip, copies);
  assert.throws(() => refresh(model, sources(), ids()), /was edited/);
});

test("ordinary target renames and placement deletion preserve motion edit protection", () => {
  const uuid = ids();
  const model = refresh(scene(), sources(), uuid);
  const clip = model.animations[0];
  Object.values(clip.animators)[0].name = "Renamed model part";
  assert(motionUnchanged(clip));

  const removed = new Set(subtree(model.outliner[0]));
  model.outliner.shift();
  model.groups = model.groups.filter((item) => !removed.has(item.uuid));
  model.elements = model.elements.filter((item) => !removed.has(item.uuid));
  for (const target of Object.keys(clip.animators))
    if (removed.has(target)) delete clip.animators[target];
  assert.doesNotThrow(() => refresh(model, sources(), uuid));

  const remaining = Object.keys(clip.animators)[0];
  delete clip.animators[remaining];
  assert.throws(() => refresh(model, sources(), uuid), /was edited/);
});

test("refresh preserves logical clip identity so native add-before-remove Undo keeps names", () => {
  const uuid = ids();
  const initialSources = sources();
  initialSources.tree.visual.animations[0].length = 1;
  const before = refresh(scene(), initialSources, uuid);

  const changed = sources();
  changed.tree.visual.animations[0].length = 3;
  const after = refresh(before, changed, uuid);
  assert.deepEqual(
    after.animations.map((clip) => clip.uuid),
    before.animations.map((clip) => clip.uuid),
  );

  // Match native Undo's animation lookup, add/createUniqueName, then remove order.
  function restore(current, saved, reference) {
    const clips = JSON.parse(JSON.stringify(current.animations));
    for (const clip of saved.animations) {
      let existing = reference.animations.some(
        (item) => item.uuid === clip.uuid,
      )
        ? clips.find((item) => item.uuid === clip.uuid)
        : null;
      if (!existing) {
        existing = {};
        clips.push(existing);
      }
      Object.assign(existing, JSON.parse(JSON.stringify(clip)));
      const name = existing.name.replace(/\d+$/, "");
      for (
        let suffix = 2;
        clips.some((item) => item !== existing && item.name === existing.name);
        suffix++
      )
        existing.name = `${name}${suffix}`;
    }
    return {
      animations: clips.filter(
        (clip) =>
          !reference.animations.some((item) => item.uuid === clip.uuid) ||
          saved.animations.some((item) => item.uuid === clip.uuid),
      ),
    };
  }

  const undone = restore(after, before, after);
  assert.deepEqual(undone.animations, before.animations);

  const redone = restore(undone, after, before);
  assert.deepEqual(redone.animations, after.animations);
});

test("Undo restores generated target membership exactly without approving keyframe edits", () => {
  const uuid = ids();
  const before = refresh(scene(), sources(), uuid);
  const after = JSON.parse(JSON.stringify(before));
  const copies = duplicatePlacement(after);
  for (const clip of after.animations) acceptCopiedMotion(clip, copies);
  const harness = nativeHarness(after);
  const userClip = {
    uuid: "user-clip",
    name: "Hand-authored",
    animators: { original: {}, extra: {} },
  };
  harness.sandbox.Animation.all = [
    ...JSON.parse(JSON.stringify(after.animations)),
    userClip,
  ].map((clip) => ({
    ...clip,
    removeAnimator(id) {
      delete this.animators[id];
    },
  }));
  const save = {
    animations: Object.fromEntries(
      before.animations.map((clip) => [clip.uuid, clip]),
    ),
  };
  save.animations[userClip.uuid] = { ...userClip, animators: { original: {} } };

  // Native extend restores each saved channel and baseline but does not remove
  // added copied channels. The plugin hook runs after that native restore.
  for (const clip of harness.sandbox.Animation.all) {
    const saved = save.animations[clip.uuid];
    Object.assign(clip.animators, JSON.parse(JSON.stringify(saved.animators)));
    if (saved[BASELINE])
      clip[BASELINE] = JSON.parse(JSON.stringify(saved[BASELINE]));
  }
  assert.equal(motionUnchanged(harness.sandbox.Animation.all[0]), false);
  harness.listeners.load_undo_save({ save });
  assert(
    harness.sandbox.Animation.all
      .filter((clip) => clip[OWNER])
      .every((clip) => motionUnchanged(clip)),
  );
  assert.equal(
    Object.hasOwn(harness.sandbox.Animation.all.at(-1).animators, "extra"),
    true,
  );

  const restored = {
    ...after,
    animations: JSON.parse(JSON.stringify(harness.sandbox.Animation.all)),
  };
  assert.doesNotThrow(() => refresh(restored, sources(), uuid));

  const target = Object.values(harness.sandbox.Animation.all[0].animators)[0];
  target.keyframes[0].data_points[0].y = 123;
  harness.listeners.load_undo_save({ save });
  assert.equal(target.keyframes[0].data_points[0].y, 123);
  assert.equal(motionUnchanged(harness.sandbox.Animation.all[0]), false);
});

test("native texture canvas callbacks complete before Undo snapshot and GLB export even when images report complete", async () => {
  const model = scene();
  for (const group of model.groups) delete group[REF];
  model.textures = [
    {
      uuid: "painted",
      name: "Painted",
      source: "fresh pixels",
      internal: true,
    },
  ];

  const harness = nativeHarness(model, "a", true);
  const operation = harness.actions.pyxel_cube_export();
  assert.equal(harness.pendingLoads.length, 1);
  assert(harness.sandbox.Texture.all[0].img.complete);
  assert.equal(harness.finished.length, 0);
  assert.equal(harness.exports.length, 0);

  harness.pendingLoads[0]();
  await operation;
  assert.deepEqual(harness.errors, []);
  assert.deepEqual(harness.finished, [true]);
  assert.equal(harness.exports.length, 1);
});
