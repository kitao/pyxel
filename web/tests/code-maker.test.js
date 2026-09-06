const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const { loadArrowFunction, loadNamedFunction } = require("./source-loader");

const source = fs.readFileSync(
  path.join(__dirname, "..", "code-maker", "index.html"),
  "utf8",
);
const sharedSource = fs.readFileSync(
  path.join(__dirname, "..", "shared.js"),
  "utf8",
);

function createDropTarget() {
  const listeners = [];
  return {
    addEventListener(type, handler, options) {
      listeners.push({ type, handler, capture: options?.capture });
    },
    load() {
      for (const listener of listeners.filter((item) => item.type === "load")) {
        listener.handler();
      }
    },
    async drop(file) {
      const event = {
        dataTransfer: { files: [file] },
        prevented: false,
        stopped: false,
        preventDefault() {
          this.prevented = true;
        },
        stopPropagation() {
          this.stopped = true;
        },
      };
      for (const listener of listeners.filter((item) => item.type === "drop")) {
        await listener.handler(event);
        if (event.stopped) {
          assert.equal(listener.capture, true);
        }
      }
      return event;
    },
  };
}

for (const surface of ["page", "runtime", "resource"]) {
  test(`Code Maker routes dropped files by extension in the ${surface}`, async () => {
    const targets = Object.fromEntries(
      ["page", "runtime", "resource", "code"].map((name) => [
        name,
        createDropTarget(),
      ]),
    );
    const selectedTabs = [];
    targets.page.getElementById = (id) => ({
      click: () => selectedTabs.push(id),
    });
    const displayed = [];
    const resources = [];
    const timers = [];
    let unlocked = false;
    const runtimeScreen = {
      ...createDropTarget(),
      contentWindow: { document: targets.runtime },
    };
    const resourceEditor = {
      ...createDropTarget(),
      contentWindow: {
        document: targets.resource,
        pyxelContext: {
          initialized: false,
          resolveInput() {
            unlocked = true;
          },
        },
        dropFileToPyxel: (name, data) => resources.push({ name, data }),
      },
    };
    const window = {
      _project: { code: "old", resource: "old resource", files: {} },
      _codeEditor: { setValue: (code) => displayed.push(code) },
    };
    const setupFileDrop = loadNamedFunction(source, "setupFileDrop", {
      document: targets.page,
      runtimeScreen,
      resourceEditor,
      waitForPyxelReady: loadArrowFunction(sharedSource, "waitForPyxelReady", {
        setTimeout: (callback) => timers.push(callback),
      }),
      window,
      TextEncoder,
      uint8ToBase64: (bytes) => Buffer.from(bytes).toString("base64"),
    });
    setupFileDrop();

    const code = 'pyxel.load("assets/test.pyxres")\nprint("hello")';
    const expected = 'pyxel.load("my_resource.pyxres")\nprint("hello")';
    const codeEvent = await targets[surface].drop({
      name: "example.py",
      text: async () => code,
    });
    assert.deepEqual(displayed, [expected]);
    assert.equal(window._project.code, expected);
    assert.equal(
      Buffer.from(window._project.files["main.py"], "base64").toString(),
      expected,
    );
    assert.equal(codeEvent.prevented && codeEvent.stopped, true);
    assert.deepEqual(selectedTabs, ["code-tab-button"]);

    const data = new Uint8Array([1, 2, 3]).buffer;
    const resourceEvent = await targets[surface].drop({
      name: "assets.pyxres",
      arrayBuffer: async () => data,
    });
    assert.equal(unlocked, true);
    assert.deepEqual(resources, []);
    resourceEditor.contentWindow.pyxelContext.initialized = true;
    timers.shift()();
    assert.deepEqual(resources, [{ name: "assets.pyxres", data }]);
    assert.deepEqual(selectedTabs, ["code-tab-button", "resource-tab-button"]);
    assert.equal(window._project.resource, "old resource");
    assert.equal(resourceEvent.prevented && resourceEvent.stopped, true);

    // Other file types still reach the embedded application's own drop handler.
    const otherEvent = await targets[surface].drop({ name: "image.png" });
    assert.equal(otherEvent.stopped, false);
    assert.equal(resources.length, 1);
    assert.deepEqual(displayed, [expected]);

    if (surface !== "page") {
      const frame = surface === "runtime" ? runtimeScreen : resourceEditor;
      frame.contentWindow.document = createDropTarget();
      frame.load();
      await frame.contentWindow.document.drop({
        name: "replacement.py",
        text: async () => "print('after reset')",
      });
      assert.equal(window._project.code, "print('after reset')");
    }
  });
}
