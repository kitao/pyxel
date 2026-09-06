const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const { loadNamedFunction } = require("./source-loader");

const WEB_DIR = path.join(__dirname, "..");
const read = (...parts) =>
  fs.readFileSync(path.join(WEB_DIR, ...parts), "utf8");

test("Code Maker controls and dialogs expose keyboard semantics", () => {
  const source = read("code-maker", "index.html");
  for (const id of [
    "load-button",
    "save-button",
    "code-tab-button",
    "resource-tab-button",
    "run-button",
  ]) {
    const openingTag = source.match(
      new RegExp(`<button[^>]+id="${id}"[^>]*>`, "s"),
    )?.[0];
    assert.ok(openingTag, id);
    assert.doesNotMatch(openingTag, /tabindex="-1"/, id);
  }
  const menuItems = [...source.matchAll(/<button[^>]+role="menuitem"[^>]*>/gs)];
  assert.ok(menuItems.length > 0);
  for (const [openingTag] of menuItems) {
    assert.match(openingTag, /tabindex="-1"/);
  }
  assert.match(source, /<input[^>]+id="load-file"[^>]+tabindex="-1"/s);
  const modals = [...source.matchAll(/<div[^>]+id="modal-[^"]+"[^>]*>/gs)];
  assert.ok(modals.length > 0);
  for (const [openingTag] of modals) {
    assert.match(openingTag, /role="dialog"/);
    assert.match(openingTag, /aria-modal="true"/);
    const labelId = openingTag.match(/aria-labelledby="([^"]+)"/)?.[1];
    assert.ok(labelId?.trim());
    assert.match(source, new RegExp(`<h[1-6][^>]+id="${labelId}"[^>]*>`));
  }
  for (const id of ["input-load-gist", "input-load-github", "input-load-url"]) {
    const openingTag = source.match(
      new RegExp(`<input[^>]+id="${id}"[^>]*>`, "s"),
    )?.[0];
    assert.ok(openingTag, id);
    assert.ok(openingTag.match(/aria-label="([^"]*)"/)?.[1]?.trim(), id);
  }
});

test("Code Maker resource tab starts disabled", () => {
  const source = read("code-maker", "index.html");
  const openingTag = source.match(
    /<button[^>]+id="resource-tab-button"[^>]*>/s,
  )?.[0];
  assert.ok(openingTag);
  assert.match(openingTag, /\sdisabled(?:\s|>)/);
});

test("Code Maker fixes focus-line widths on a 375px mobile-first layout", () => {
  const source = read("code-maker", "index.html");
  const focusLineLeft = { style: {} };
  const focusLineCenter = { style: {} };
  const focusLineRight = { style: {} };
  const updateFocusLineLayout = loadNamedFunction(
    source,
    "updateFocusLineLayout",
    {
      focusLineCenter,
      focusLineLeft,
      focusLineRight,
      leftPane: {
        getBoundingClientRect: () => {
          throw new Error("desktop layout must not be measured");
        },
      },
      mainSplitter: {
        getBoundingClientRect: () => ({ width: 375 }),
      },
      splitterHandle: {
        getBoundingClientRect: () => {
          throw new Error("desktop layout must not be measured");
        },
      },
      window: {
        matchMedia: () => ({ matches: true }),
      },
    },
  );

  updateFocusLineLayout();

  assert.deepEqual(focusLineLeft.style, {
    flex: "0 0 auto",
    width: "187.5px",
  });
  assert.deepEqual(focusLineCenter.style, {
    flex: "0 0 auto",
    width: "0px",
  });
  assert.deepEqual(focusLineRight.style, {
    flex: "0 0 auto",
    width: "187.5px",
  });
});

test("Code Maker menu keyboard navigation wraps and closes predictably", () => {
  const source = read("code-maker", "index.html");
  let closeCount = 0;
  let preventCount = 0;
  const context = {
    closeAllDropdowns: () => {
      closeCount += 1;
    },
    document: { activeElement: null },
  };
  const items = Array.from({ length: 4 }, () => ({
    focus() {
      context.document.activeElement = this;
    },
  }));
  const trigger = {
    focus() {
      context.document.activeElement = this;
    },
  };
  const menu = {
    parentElement: { querySelector: () => trigger },
    querySelectorAll: () => items,
  };
  const handleMenuKeydown = loadNamedFunction(
    source,
    "handleMenuKeydown",
    context,
  );
  const press = (key) =>
    handleMenuKeydown(menu, {
      key,
      preventDefault: () => {
        preventCount += 1;
      },
    });

  items[0].focus();
  press("ArrowUp");
  assert.equal(context.document.activeElement, items[3]);
  press("ArrowDown");
  assert.equal(context.document.activeElement, items[0]);
  press("End");
  assert.equal(context.document.activeElement, items[3]);
  press("Home");
  assert.equal(context.document.activeElement, items[0]);
  assert.equal(preventCount, 4);

  press("Tab");
  assert.equal(closeCount, 1);
  assert.equal(preventCount, 4, "Tab must retain native focus movement");

  press("Escape");
  assert.equal(closeCount, 2);
  assert.equal(context.document.activeElement, trigger);
  assert.equal(preventCount, 5);
});

test("Code Maker restores focus before dispatching a menu action", () => {
  const source = read("code-maker", "index.html");
  const calls = [];
  const trigger = {
    focus() {
      calls.push("focus");
    },
  };
  const item = {
    dataset: { action: "save-local" },
    closest: () => ({ querySelector: () => trigger }),
  };
  const activateMenuItem = loadNamedFunction(source, "activateMenuItem", {
    closeAllDropdowns: () => calls.push("close"),
    saveToFile: () => calls.push("save"),
  });

  activateMenuItem(item, {});

  assert.deepEqual(calls, ["close", "focus", "save"]);
});

test("embedded runtime frames have accessible names", () => {
  for (const app of ["code-maker", "mml-studio"]) {
    const frames = [...read(app, "index.html").matchAll(/<iframe\b[^>]*>/gs)];
    assert.ok(frames.length > 0, app);
    for (const [openingTag] of frames) {
      assert.ok(openingTag.match(/title="([^"]*)"/)?.[1]?.trim(), app);
    }
  }
});

test("MML Studio toggles expose their state and channel names", () => {
  const source = read("mml-studio", "index.html");
  const actions = [
    "loop",
    "solo1",
    "mute1",
    "solo2",
    "mute2",
    "solo3",
    "mute3",
    "solo4",
    "mute4",
  ];
  for (const action of actions) {
    const openingTag = source.match(
      new RegExp(`<button[^>]+data-action="${action}"[^>]*>`, "s"),
    )?.[0];
    assert.ok(openingTag, action);
    const initialState = action === "loop" ? "true" : "false";
    assert.match(openingTag, new RegExp(`aria-pressed="${initialState}"`));
    if (action !== "loop") {
      assert.match(
        openingTag,
        new RegExp(`aria-label="(?:Solo|Mute) channel ${action.at(-1)}"`),
      );
    }
  }

  const attributes = {};
  const button = {
    dataset: new Proxy(
      { action: "solo1", state: "false" },
      {
        set(target, name, value) {
          target[name] = String(value);
          return true;
        },
      },
    ),
    classList: { toggle() {} },
    setAttribute: (name, value) => {
      attributes[name] = String(value);
    },
  };
  const context = { runtimeScreen: { contentWindow: {} } };
  context.setButtonStyle = loadNamedFunction(source, "setButtonStyle", context);
  const handleButtonPress = loadNamedFunction(
    source,
    "handleButtonPress",
    context,
  );

  handleButtonPress(button);
  assert.equal(attributes["aria-pressed"], "true");
  assert.equal(context.runtimeScreen.contentWindow.js_solo1, true);
  handleButtonPress(button);
  assert.equal(attributes["aria-pressed"], "false");
  assert.equal(context.runtimeScreen.contentWindow.js_solo1, false);
});
