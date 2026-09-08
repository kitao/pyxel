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

test("Code Maker loads an empty main.py and rejects an absent one", async () => {
  const files = {
    "project/main.py": { async: async () => new Uint8Array() },
    "project/my_resource.pyxres": {
      async: async () => new Uint8Array([1, 2, 3]),
    },
  };
  const displayed = [];
  const window = {
    _projectLoad: 1,
    _project: {},
    _codeEditor: { setValue: (code) => displayed.push(code) },
  };
  const loadProject = loadNamedFunction(source, "loadProjectFromZip", {
    JSZip: { loadAsync: async () => ({ files }) },
    TextDecoder,
    window,
    uint8ToBase64: (bytes) => Buffer.from(bytes).toString("base64"),
    base64ToUint8: (value) => Buffer.from(value, "base64"),
    sanitizeProjectName: (name) => name,
    resetRuntimeScreen() {},
    resetResourceEditor() {},
  });

  await loadProject(new ArrayBuffer(0), "project");
  assert.equal(window._project.code, "");
  assert.deepEqual(displayed, [""]);

  delete files["project/main.py"];
  await assert.rejects(loadProject(new ArrayBuffer(0), "project"), {
    message: "Missing main.py",
  });
});

test("Code Maker share URLs replace existing queries and fragments", () => {
  for (const suffix of ["#editor", "?gist=old#editor"]) {
    const urls = [];
    const updateShareUrl = loadNamedFunction(source, "updateShareUrl", {
      URL,
      location: { href: `https://example.test/code-maker/${suffix}` },
      history: {
        replaceState: (_state, _title, url) => urls.push(String(url)),
      },
    });
    updateShareUrl("github", "kitao/pyxel", "feature/demo");
    updateShareUrl();
    assert.deepEqual(urls, [
      "https://example.test/code-maker/?github=kitao/pyxel&ref=feature%2Fdemo",
      "https://example.test/code-maker/",
    ]);
  }
});

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

test("Code Maker reads resources only after Python saving completes", async () => {
  const calls = [];
  const window = { _project: { resource: "old" }, _isCopyingResource: false };
  let fail = false;
  const pyodide = {
    runPython() {
      assert.equal(window._isCopyingResource, true);
      calls.push("save");
      if (fail) throw new Error("disk full");
    },
    FS: {
      readFile(filename) {
        assert.equal(filename, "/work/my_resource.pyxres");
        calls.push("read");
        return new Uint8Array([1, 2, 3]);
      },
    },
  };
  const copy = loadNamedFunction(source, "copyResourceToProject", {
    window,
    resourceEditor: {
      contentWindow: {
        pyxelContext: { pyodide },
        document: { dispatchEvent: (event) => calls.push(event.type) },
      },
    },
    KeyboardEvent: class {
      constructor(type) {
        this.type = type;
      }
    },
    setTimeout: (fn) => fn(),
    PYXEL_WORKING_DIRECTORY: "/work",
    uint8ToBase64: (bytes) => Buffer.from(bytes).toString("base64"),
  });
  const pending = copy();
  assert.deepEqual(calls, [], "unwind an existing Python save callback first");
  await pending;
  assert.deepEqual(calls, ["save", "read"]);
  assert.equal(window._project.resource, "AQID");
  assert.equal(window._isCopyingResource, false);

  calls.length = 0;
  fail = true;
  await assert.rejects(copy(), { message: "disk full" });
  assert.deepEqual(calls, ["save"]);
  assert.equal(window._project.resource, "AQID");
  assert.equal(window._isCopyingResource, false);
});

test("Code Maker stops Run, File and Gist when resource saving fails", async () => {
  for (const action of ["Run", "Save", "Copy"]) {
    const calls = [];
    const controls = new Map();
    const document = {
      getElementById(id) {
        if (!controls.has(id)) {
          controls.set(id, {
            addEventListener: (event, fn) => controls.set(`${id}:${event}`, fn),
          });
        }
        return controls.get(id);
      },
      addEventListener() {},
      querySelectorAll: () => [],
    };
    const context = {
      window: {
        _isCopyingResource: false,
        addEventListener() {},
        showSaveFilePicker: () => calls.push("picker"),
      },
      document,
      copyCodeToProject() {},
      copyResourceToProject: async () => {
        throw new Error("disk full");
      },
      buildArchiveBlob: () => calls.push("archive"),
      resetRuntimeScreen: () => calls.push("run"),
      showModal: () => calls.push("modal"),
      navigator: { clipboard: { writeText: () => calls.push("clipboard") } },
      alert: (message) => calls.push(message),
      animateButton() {},
      loadFromGist() {},
      loadFromGitHub() {},
      loadFromUrl() {},
    };
    let operation;
    if (action === "Run") {
      loadNamedFunction(source, "setupButtonHandlers", context)();
      operation = controls.get("run-button:click");
    } else {
      operation = loadNamedFunction(
        source,
        action === "Save" ? "saveToFile" : "saveToGist",
        context,
      );
    }
    await operation();
    assert.deepEqual(calls, [`${action} failed: disk full`]);
  }
});

test("Code Maker does not fall back to downloading after a file write fails", async () => {
  const calls = [];
  const save = loadNamedFunction(source, "saveToFile", {
    window: {
      _project: { name: "project" },
      showSaveFilePicker: async () => ({
        name: "project.zip",
        createWritable: async () => ({
          write: async () => {
            throw new Error("disk full");
          },
          close: () => calls.push("close"),
        }),
      }),
    },
    copyCodeToProject() {},
    copyResourceToProject: async () => {},
    sanitizeProjectName: (name) => name,
    buildArchiveBlob: async () => {
      calls.push("archive");
      return new Uint8Array();
    },
    alert: (message) => calls.push(message),
    document: {
      createElement: () => {
        calls.push("download");
        return { click() {} };
      },
      body: { appendChild() {}, removeChild() {} },
    },
    URL: { createObjectURL: () => "blob:test", revokeObjectURL() {} },
    setTimeout: (fn) => fn(),
  });
  await save();
  assert.deepEqual(calls, ["archive", "Save failed: disk full"]);
});
