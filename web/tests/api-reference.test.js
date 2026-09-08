const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const apiReferenceSource = fs.readFileSync(
  path.join(__dirname, "..", "api-reference", "api-reference.js"),
  "utf8",
);

test("API search keeps constants, descriptions and groups together", () => {
  const refs = {};
  const element = (attr, key) => {
    refs[attr] ??= new Map();
    const node = {
      style: { display: "" },
      tagName: ["cat-id", "cg-id"].includes(attr) ? "DETAILS" : "DIV",
      open: false,
    };
    refs[attr].set(key, node);
    return node;
  };
  const category = element("cat-id", "input");
  const groups = [];
  const chips = [];
  const descriptions = [];
  for (let i = 0; i < 2; i++) {
    const gid = `input-cg-${i}`;
    groups.push([
      element("cg-id", gid),
      element("const-grid", gid),
      element("const-section", `${gid}-s-0`),
    ]);
    chips.push(element("const-key", `${gid}-s-0-0`));
    descriptions.push({ style: { display: "" } });
    element("sec-details", `${gid}-s-0`).children = [descriptions[i]];
  }
  const noResults = new Set();
  const context = vm.createContext({
    refs,
    fixture: {
      categories: [
        {
          id: "input",
          collapsed: true,
          constant_groups: [
            { collapsed: true, sections: [{ constants: ["KEY_A"] }] },
            { collapsed: true, sections: [{ constants: ["MOUSE_LEFT"] }] },
          ],
        },
      ],
    },
    localStorage: { getItem: () => null },
    document: {
      getElementById: () => ({
        classList: {
          toggle: (name, enabled) =>
            enabled ? noResults.add(name) : noResults.delete(name),
        },
      }),
    },
  });
  vm.runInContext(apiReferenceSource, context);

  vm.runInContext(
    "data = fixture; Object.assign(domCache, refs); searchQuery = 'key_a'; updateVisibility();",
    context,
  );
  assert.equal(chips[0].style.display, "");
  assert.equal(descriptions[0].style.display, "");
  assert.equal(chips[1].style.display, "none");
  assert.equal(descriptions[1].style.display, "none");
  assert.ok(groups[0].every((node) => node.style.display === ""));
  assert.ok(groups[1].every((node) => node.style.display === "none"));
  assert.equal(groups[0][0].open, true);
  assert.equal(category.open, true);
  assert.equal(category.style.display, "");
  assert.ok(noResults.has("hidden"));

  vm.runInContext("searchQuery = 'missing'; updateVisibility();", context);
  assert.equal(category.style.display, "none");
  assert.ok(!noResults.has("hidden"));

  vm.runInContext("searchQuery = ''; updateVisibility();", context);
  assert.ok(
    [...chips, ...descriptions, ...groups.flat(), category].every(
      (node) => node.style.display === "",
    ),
  );
  assert.ok(noResults.has("hidden"));
});

test("API reference restores the Advanced preference after reload", () => {
  const storage = new Map();
  const loadToolbar = () => {
    const elements = [];
    const context = vm.createContext({
      document: {
        getElementById: () => ({ appendChild() {} }),
        createElement: (tagName) => {
          const node = {
            tagName,
            dataset: {},
            appendChild() {},
            setAttribute() {},
            addEventListener(event, handler) {
              this[event] = handler;
            },
          };
          elements.push(node);
          return node;
        },
      },
      localStorage: {
        getItem: (key) => storage.get(key) ?? null,
        setItem: (key, value) => storage.set(key, value),
      },
      buildPageHeader() {},
      buildVariantSwitch() {},
    });
    vm.runInContext(apiReferenceSource, context);
    vm.runInContext(
      `
      data = { categories: [{ id: "test", items: [{ advanced: true }] }] };
      cacheDomRefs = () => {};
      updateTexts = () => {};
      updateVisibility = () => {};
      buildPage();
    `,
      context,
    );
    return elements.find((node) => node.type === "checkbox");
  };

  const first = loadToolbar();
  assert.equal(first.checked, false);

  first.checked = true;
  first.change();
  const second = loadToolbar();
  assert.equal(second.checked, true);

  second.checked = false;
  second.change();
  assert.equal(loadToolbar().checked, false);
});
