const assert = require("node:assert/strict");
const { getEventListeners } = require("node:events");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const { loadArrowFunction, loadNamedFunction } = require("./source-loader");

const pyxelSource = fs.readFileSync(
  path.join(__dirname, "..", "..", "wasm", "pyxel.js"),
  "utf8",
);
const sharedSource = fs.readFileSync(
  path.join(__dirname, "..", "shared.js"),
  "utf8",
);

test("unknown errors retain their summary without duplicating stack headers", () => {
  const format = loadArrowFunction(pyxelSource, "_formatUnknownError", {});
  const summary = "TypeError: invalid value";
  const frames = "    at run (app.js:1:2)";
  for (const stack of ["", frames, `${summary}\n${frames}`, summary]) {
    assert.equal(
      format({ name: "TypeError", message: "invalid value", stack }),
      stack.includes(frames) ? `${summary}\n${frames}` : summary,
    );
  }
});

test("optional showcase registration failures stay inside its warning handler", async () => {
  const source = fs.readFileSync(
    path.join(__dirname, "..", "..", "scripts", "start_showcase"),
    "utf8",
  );
  const script = source.match(
    /INJECT_SNIPPET = """\s*<script>([\s\S]*?)<\/script>/,
  )[1];
  for (const asynchronous of [false, true]) {
    const failure = new Error("registration denied");
    const warnings = [];
    const context = {
      location: { hostname: "localhost" },
      window: { isSecureContext: true },
      navigator: {
        serviceWorker: {
          register() {
            if (asynchronous) return Promise.reject(failure);
            throw failure;
          },
        },
      },
      console: { warn: (...args) => warnings.push(args) },
    };
    await vm.runInNewContext(script, context);
    assert.deepEqual(warnings, [["SW register failed", failure]]);
  }
});

const loadVirtualGamepad = () => {
  const screen = { style: {}, appendChild() {} };
  const rects = {
    "pyxel-gamepad-cross": { left: 0, right: 100, bottom: 100, width: 100 },
    "pyxel-gamepad-button": { left: 200, right: 300, bottom: 100, width: 100 },
    "pyxel-gamepad-menu": { left: 100, right: 200, bottom: 100, width: 100 },
  };
  const context = {
    window: Object.assign(new EventTarget(), {
      matchMedia: () => ({ matches: true }),
      customElements: { define() {} },
    }),
    document: Object.assign(new EventTarget(), {
      head: { appendChild() {} },
      getElementsByTagName: () => [],
      getElementById: () => null,
      querySelector: (selector) =>
        selector === "canvas#canvas" ? { style: {} } : screen,
      createElement: () => ({
        getBoundingClientRect() {
          return rects[this.id];
        },
      }),
    }),
    navigator: { userAgent: "test" },
    HTMLElement: class {},
  };
  vm.runInNewContext(
    `${pyxelSource}\n;globalThis.__test = {
      add: _addVirtualGamepad,
      read: _readVirtualGamepadBitmask,
    };`,
    context,
  );
  context.__test.add("enabled");
  return {
    document: context.document,
    read: context.__test.read,
    rects,
    resize: loadArrowFunction(pyxelSource, "_updateScreenElementsSize", {
      document: {
        querySelector: () => ({
          getBoundingClientRect: () => ({ width: 600, height: 200 }),
        }),
      },
      _setMinWidthFromRatio() {},
      _addVirtualGamepad: context.__test.add,
    }),
    addAgain() {
      context.__test.add("enabled");
    },
  };
};

const dispatchTouches = (target, type, touches) => {
  const event = new Event(type, { cancelable: true });
  event.touches = touches;
  target.dispatchEvent(event);
};

for (const order of [
  "frame-before-decode",
  "decode-before-frame",
  "input-before-poll",
]) {
  test(`MML Studio synchronizes current state with ${order}`, async () => {
    const source = fs.readFileSync(
      path.join(__dirname, "..", "mml-studio", "index.html"),
      "utf8",
    );
    const buttons = ["play", "stop", "loop"].map((action) => ({
      dataset: { action, state: String(action === "loop") },
      disabled: true,
    }));
    const textareas = Array.from({ length: 4 }, (_, i) => ({
      id: `ch${i + 1}`,
      value: "",
      addEventListener() {},
    }));
    const runtimeScreen = Object.assign(new EventTarget(), {
      contentWindow: {},
    });
    const polls = [];
    let initialize;
    let finishDecode;
    const decoded = new Promise((resolve) => {
      finishDecode = resolve;
    });
    const context = {
      URL,
      runtimeScreen,
      window: {
        addEventListener(name, callback) {
          if (name === "DOMContentLoaded") initialize = callback;
        },
      },
      document: {
        location: "https://example.test/?mml=shared",
        getElementById: (id) =>
          textareas.find((textarea) => textarea.id === id),
        querySelectorAll: (selector) => {
          if (selector === "[data-click]") return [];
          if (
            selector === "[data-input]" ||
            selector === "textarea[data-input]"
          ) {
            return textareas;
          }
          assert.ok(selector.startsWith("button[data-action"));
          return buttons;
        },
      },
      decodeMmlFromUrl: () => decoded,
      resolvePyxelInput() {},
      scheduleShareUrlUpdate() {},
      updateShareUrl() {},
      setTimeout: (callback) => polls.push(callback),
    };
    context.handleMmlInput = loadNamedFunction(
      source,
      "handleMmlInput",
      context,
    );
    context.waitForPyxelReady = loadArrowFunction(
      sharedSource,
      "waitForPyxelReady",
      context,
    );
    const start = source.lastIndexOf(
      'window.addEventListener("DOMContentLoaded",',
    );
    assert.notEqual(start, -1);
    vm.runInNewContext(
      source.slice(start, source.indexOf("</script>", start)),
      context,
    );

    const pending = initialize();
    const frameWindow = {
      pyxelContext: { resolveInput() {}, initialized: false },
    };
    if (order === "frame-before-decode") {
      runtimeScreen.contentWindow = frameWindow;
      runtimeScreen.dispatchEvent(new Event("load"));
    }

    finishDecode("t120cdef;;;");
    await pending;

    if (order !== "frame-before-decode") {
      assert.equal(buttons[0].disabled, true);
      textareas[0].value = "t90gab";
      buttons[2].dataset.state = "false";
      runtimeScreen.contentWindow = frameWindow;
      runtimeScreen.dispatchEvent(new Event("load"));
      if (order === "input-before-poll") {
        frameWindow.pyxelContext.resolveInput = null;
        frameWindow.pyxelContext.initialized = true;
      }
      polls.shift()();
    }

    assert.equal(buttons[0].disabled, false);
    assert.equal(frameWindow.js_ch1_mml, textareas[0].value);
    assert.equal(frameWindow.js_loop, buttons[2].dataset.state === "true");
  });
}

for (const state of ["both initialized", "one awaiting input"]) {
  test(`Code Maker enables controls with ${state}`, () => {
    const source = fs.readFileSync(
      path.join(__dirname, "..", "code-maker", "index.html"),
      "utf8",
    );
    const controls = {
      "run-button": { disabled: true },
      "resource-tab-button": { disabled: true, classList: { remove() {} } },
      "splitter-handle": { style: { pointerEvents: "none" } },
    };
    const runtimeScreen = { contentWindow: {} };
    const resourceEditor = { contentWindow: {} };
    const polls = [];
    const context = {
      runtimeScreen,
      resourceEditor,
      document: { getElementById: (id) => controls[id] },
      saveToFile() {},
      setTimeout: (callback) => polls.push(callback),
    };
    context.waitForPyxelReady = loadArrowFunction(
      sharedSource,
      "waitForPyxelReady",
      context,
    );

    loadNamedFunction(source, "onPyxelReady", context)();
    assert.equal(controls["run-button"].disabled, true);

    runtimeScreen.contentWindow.pyxelContext = {
      initialized: true,
      resolveInput: null,
    };
    resourceEditor.contentWindow.pyxelContext =
      state === "one awaiting input"
        ? { initialized: false, resolveInput() {} }
        : { initialized: true, resolveInput: null };
    polls.shift()();
    assert.equal(controls["run-button"].disabled, false);
    assert.equal(controls["resource-tab-button"].disabled, false);
    assert.equal(controls["splitter-handle"].style.pointerEvents, "auto");
    assert.equal(
      typeof resourceEditor.contentWindow._savePyxelFile,
      "function",
    );
  });
}

test("virtual gamepad releases canceled touches", () => {
  const gamepad = loadVirtualGamepad();
  dispatchTouches(gamepad.document, "touchstart", [
    { clientX: 250, clientY: 75 },
  ]);
  assert.equal(gamepad.read(), 1 << 4);

  const event = new Event("touchcancel");
  event.touches = [];
  let preventDefaultCalls = 0;
  event.preventDefault = () => preventDefaultCalls++;
  gamepad.document.dispatchEvent(event);
  assert.equal(gamepad.read(), 0);
  assert.equal(preventDefaultCalls, 0);
});

test("virtual gamepad keeps remaining touches when one touch is canceled", () => {
  const gamepad = loadVirtualGamepad();
  const up = { clientX: 50, clientY: 25 };
  const a = { clientX: 250, clientY: 75 };
  dispatchTouches(gamepad.document, "touchstart", [up, a]);
  assert.equal(gamepad.read(), (1 << 0) | (1 << 4));

  dispatchTouches(gamepad.document, "touchcancel", [up]);
  assert.equal(gamepad.read(), 1 << 0);

  dispatchTouches(gamepad.document, "touchend", []);
  assert.equal(gamepad.read(), 0);
});

test("virtual gamepad replaces touch handlers before controls load", () => {
  const gamepad = loadVirtualGamepad();
  gamepad.addAgain();
  dispatchTouches(gamepad.document, "touchstart", [
    { clientX: 250, clientY: 75 },
  ]);
  assert.equal(gamepad.read(), 1 << 4);
  for (const type of ["touchstart", "touchmove", "touchend", "touchcancel"]) {
    assert.equal(
      getEventListeners(gamepad.document, type).length,
      1,
      `${type} must have one handler`,
    );
  }

  dispatchTouches(gamepad.document, "touchcancel", []);
  assert.equal(gamepad.read(), 0);
});

test("virtual gamepad refreshes touch bounds after screen sizing", () => {
  const gamepad = loadVirtualGamepad();
  dispatchTouches(gamepad.document, "touchstart", [
    { clientX: 50, clientY: 25 },
  ]);
  assert.equal(gamepad.read(), 1 << 0);

  for (const [name, rect] of Object.entries(gamepad.rects)) {
    gamepad.rects[name] = Object.fromEntries(
      Object.entries(rect).map(([key, value]) => [key, value * 2]),
    );
  }
  gamepad.resize();
  dispatchTouches(gamepad.document, "touchmove", [
    { clientX: 100, clientY: 50 },
  ]);
  assert.equal(gamepad.read(), 1 << 0);
});

test("runtime asset base accepts query and fragment suffixes", () => {
  const source = pyxelSource.match(
    /const _scriptDir = \(\(\) => \{[\s\S]*?\}\)\(\);/,
  )[0];
  for (const suffix of ["", "?v=3", "#runtime"]) {
    const directory = vm.runInNewContext(`${source}\n_scriptDir;`, {
      document: {
        getElementsByTagName: () => [
          { src: `https://example.test/wasm/pyxel.js${suffix}` },
        ],
      },
    });
    assert.equal(directory, "https://example.test/wasm/", suffix);
  }
});

test("early runtime errors are logged without a secondary error", () => {
  const errors = [];
  const displayError = loadArrowFunction(pyxelSource, "_displayErrorOverlay", {
    console: { error: (message) => errors.push(message) },
    document: {
      getElementById: () => null,
      createElement: () => ({ style: {} }),
    },
  });

  assert.doesNotThrow(() => displayError("startup failed"));
  assert.deepEqual(errors, ["startup failed"]);
});

test("launchPyxel preserves initialization failures for external callers", async () => {
  const failure = new Error("wheel fetch failed");
  let reportedError = null;
  const context = {
    PYODIDE_URL: "https://cdn.example.test/pyodide/v1.2.3/full/pyodide.js",
    PYXEL_WHEEL_PATH: "pyxel-3.0.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl",
    console: { log: () => {} },
    _allowGamepadConnection: () => {},
    _createScreenElements: async () => ({}),
    _displayFatalErrorOverlay: (error) => {
      reportedError = error;
    },
    _suppressTouchZoomGestures: () => {},
    _loadPyodideAndPyxel: async () => {
      throw failure;
    },
  };
  const launchPyxel = loadNamedFunction(pyxelSource, "launchPyxel", context);

  await assert.rejects(
    launchPyxel({ command: "run" }),
    (error) => error === failure,
  );
  assert.equal(reportedError, null);
});

test("launchPyxel reports command failures and resolves for external callers", async () => {
  const failure = new Error("command failed");
  const canvas = {};
  const pyodide = {};
  const window = { pyxelContext: {} };
  let reportedError = null;
  const context = {
    PYODIDE_URL: "https://cdn.example.test/pyodide/v1.2.3/full/pyodide.js",
    PYXEL_WHEEL_PATH: "pyxel-3.0.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl",
    console: { log: () => {} },
    window,
    _allowGamepadConnection: () => {},
    _createScreenElements: async () => canvas,
    _displayFatalErrorOverlay: (error) => {
      reportedError = error;
    },
    _suppressTouchZoomGestures: () => {},
    _executePyxelCommand: async () => {
      throw failure;
    },
    _hookFileOperations: () => {},
    _hookPythonError: () => {},
    _loadPyodideAndPyxel: async () => pyodide,
    _waitForInput: async () => {},
  };
  const launchPyxel = loadNamedFunction(pyxelSource, "launchPyxel", context);

  await launchPyxel({ command: "run" });
  assert.equal(window.pyxelContext.initialized, true);
  assert.equal(reportedError, failure);
});

test("simultaneous bootstrap failures have one fatal path and no unhandled rejection", async () => {
  const scriptFailure = new Error("script failed");
  const fetchFailures = [
    new Error("wheel failed"),
    new Error("import hook failed"),
  ];
  let fetchIndex = 0;
  const loadPyodideAndPyxel = loadArrowFunction(
    pyxelSource,
    "_loadPyodideAndPyxel",
    {
      IMPORT_HOOK_PATH: "import_hook.py",
      PYODIDE_URL: "https://cdn.example.test/pyodide.js",
      PYXEL_WHEEL_PATH: "pyxel-test.whl",
      _fetchAsset: async () => {
        throw fetchFailures[fetchIndex++];
      },
      _loadScript: async () => {
        throw scriptFailure;
      },
      _scriptDir: "/wasm/",
      loadPyodide: async () => {
        throw new Error("loadPyodide must not run");
      },
    },
  );
  const fatalErrors = [];
  const launchPyxel = loadNamedFunction(pyxelSource, "launchPyxel", {
    PYODIDE_URL: "https://cdn.example.test/pyodide/v1.2.3/full/pyodide.js",
    PYXEL_WHEEL_PATH: "pyxel-3.0.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl",
    console: { log: () => {} },
    _allowGamepadConnection: () => {},
    _createScreenElements: async () => ({}),
    _displayFatalErrorOverlay: (error) => fatalErrors.push(error),
    _suppressTouchZoomGestures: () => {},
    _loadPyodideAndPyxel: loadPyodideAndPyxel,
  });
  const launchFromElement = loadArrowFunction(
    pyxelSource,
    "_launchPyxelFromElement",
    {
      launchPyxel,
      _displayFatalErrorOverlay: (error) => fatalErrors.push(error),
    },
  );

  const unhandled = [];
  const recordUnhandled = (error) => unhandled.push(error);
  process.on("unhandledRejection", recordUnhandled);
  try {
    launchFromElement({ command: "run" });
    await new Promise((resolve) => setTimeout(resolve, 20));
    assert.deepEqual(fatalErrors, [scriptFailure]);
    assert.deepEqual(unhandled, []);
  } finally {
    process.off("unhandledRejection", recordUnhandled);
  }
});

test("_loadImage rejects immediately when an image fails to load", async () => {
  const listeners = new Map();
  const image = {
    addEventListener: (name, listener) => listeners.set(name, listener),
    removeEventListener: (name) => listeners.delete(name),
    src: "",
  };
  const loadImage = loadArrowFunction(pyxelSource, "_loadImage", {});

  const pending = loadImage(image, "missing.png");
  listeners.get("error")();
  await assert.rejects(pending, {
    name: "Error",
    message: "Failed to load image: missing.png",
  });
  assert.equal(image.src, "missing.png");
  assert.equal(listeners.size, 0);
});

test("the startup prompt is keyboard accessible", async () => {
  const makeEventTarget = () => {
    const listeners = new Map();
    return {
      listeners,
      addEventListener(name, listener) {
        listeners.set(name, listener);
      },
      removeEventListener(name, listener) {
        if (listeners.get(name) === listener) listeners.delete(name);
      },
      dispatch(name, event = {}) {
        listeners.get(name)?.(event);
      },
    };
  };
  const body = makeEventTarget();
  const prompt = {
    ...makeEventTarget(),
    remove() {
      this.removed = true;
    },
    setAttribute(name, value) {
      this[name] = value;
    },
  };
  const logo = { remove: () => {} };
  const screen = { appendChild: () => {} };
  const window = { pyxelContext: {} };
  const document = {
    body,
    createElement: () => prompt,
    querySelector: (selector) =>
      selector === "div#pyxel-screen" ? screen : logo,
  };
  const waitForInput = loadArrowFunction(pyxelSource, "_waitForInput", {
    CLICK_TO_START_PATH: "click.png",
    TOUCH_TO_START_PATH: "touch.png",
    document,
    setTimeout,
    window,
    _isTouchDevice: () => false,
    _loadImage: async () => {},
    _scriptDir: "/wasm/",
    _updateScreenElementsSize: () => {},
  });

  const pending = waitForInput();
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(prompt.alt, "Start Pyxel");
  assert.equal(prompt.role, "button");
  assert.equal(prompt.tabIndex, 0);
  assert.ok(prompt.listeners.has("keydown"));

  let resolved = false;
  pending.then(() => {
    resolved = true;
  });
  prompt.dispatch("keydown", { key: "x", preventDefault: () => {} });
  await Promise.resolve();
  assert.equal(resolved, false);

  let defaultPrevented = false;
  prompt.dispatch("keydown", {
    key: " ",
    preventDefault: () => {
      defaultPrevented = true;
    },
  });
  await pending;
  assert.equal(defaultPrevented, true);
  assert.equal(prompt.removed, true);
  assert.equal(window.pyxelContext.resolveInput, null);
  assert.equal(prompt.listeners.size, 0);
  assert.equal(body.listeners.size, 0);
});

test("runtime-generated images have intentional text alternatives", () => {
  assert.match(pyxelSource, /logoImage\.alt = "Pyxel"/);
  assert.match(
    pyxelSource,
    /const createGamepadElement = \(id, path\) => \{[\s\S]*?img\.alt = "";/,
  );
});

test("custom-element launch callbacks consume rejected promises", async () => {
  const failure = new Error("unexpected launch failure");
  let reportedError = null;
  const launchFromElement = loadArrowFunction(
    pyxelSource,
    "_launchPyxelFromElement",
    {
      launchPyxel: async () => {
        throw failure;
      },
      _displayFatalErrorOverlay: (error) => {
        reportedError = error;
      },
    },
  );

  assert.equal(launchFromElement({ command: "run" }), undefined);
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(reportedError, failure);
});

test("Web runtime escapes reserved characters in fetched file paths", () => {
  const encodeUrlPath = loadArrowFunction(pyxelSource, "_encodeUrlPath", {});
  const requests = [];
  const context = {
    Blob,
    document: { body: { appendChild: () => {} }, createElement: () => ({}) },
    console: { log: () => {} },
    PYXEL_WATCH_INFO_FILE: ".pyxel-watch",
    PYXEL_WORKING_DIRECTORY: "/work",
    Uint8Array,
    URL,
    window: {},
    _encodeUrlPath: encodeUrlPath,
    XMLHttpRequest: class {
      status = 404;
      overrideMimeType() {}
      open(_method, url) {
        requests.push(url);
      }
      send() {}
    },
  };
  const fs = {
    analyzePath: () => ({ exists: false }),
    cwd: () => "/work",
    mkdir: () => {},
    open: () => {},
    readFile: () => new Uint8Array(),
    stat: () => {},
    writeFile: () => {},
  };
  const hookFileOperations = loadArrowFunction(
    pyxelSource,
    "_hookFileOperations",
    context,
  );
  hookFileOperations({ FS: fs }, "https://example.test/root");

  fs.open("/work/apps/demo#preview?.py", 557056);
  for (const name of [
    "/work-other/outside.py",
    "/work/../outside.py",
    "../work-other/outside.py",
    "/work",
  ]) {
    fs.open(name, 557056);
    fs.stat(name);
  }
  fs.open("apps/../inside.py", 557056);
  assert.deepEqual(requests, [
    "https://example.test/root/apps/demo%23preview%3F.py",
    "https://example.test/root/inside.py",
  ]);
});

test("file hooks preserve stat's symlink option and result", () => {
  const result = {};
  const fs = {
    open() {},
    stat(path, dontFollow) {
      assert.equal(path, "/tmp/link");
      assert.equal(dontFollow, true);
      return result;
    },
  };
  const hookFileOperations = loadArrowFunction(
    pyxelSource,
    "_hookFileOperations",
    {
      window: {},
      PYXEL_WATCH_INFO_FILE: ".pyxel-watch",
      PYXEL_WORKING_DIRECTORY: "/work",
    },
  );
  hookFileOperations({ FS: fs }, ".");

  assert.equal(fs.stat("/tmp/link", true), result);
});

test("legacy page redirects preserve encoded queries and fragments", () => {
  for (const route of [
    "api-reference",
    "api-reference/cube",
    "code-maker",
    "editor-manual",
    "launcher",
    "mml-studio",
    "showcase",
    "user-guide",
    "user-guide/cube",
  ]) {
    const source = fs.readFileSync(
      path.join(__dirname, "..", "..", "wasm", route, "index.html"),
      "utf8",
    );
    const destinations = [];
    vm.runInNewContext(source.match(/<script>([\s\S]*?)<\/script>/)[1], {
      location: {
        search: "?lang=ja&ref=a%2Fb",
        hash: "#entry",
        replace: (value) => destinations.push(value),
      },
    });
    assert.deepEqual(
      destinations.map(
        (value) =>
          new URL(value, `https://example.test/pyxel/wasm/${route}/`).href,
      ),
      [`https://example.test/pyxel/web/${route}/?lang=ja&ref=a%2Fb#entry`],
      route,
    );
  }
});

test("MML URL codec roundtrips UTF-8 and dispatches legacy URL forms", async () => {
  const source = fs.readFileSync(
    path.join(__dirname, "..", "mml-studio", "index.html"),
    "utf8",
  );
  const legacyCalls = [];
  const context = {
    atob,
    btoa,
    Blob,
    CompressionStream,
    DecompressionStream,
    Response,
    Uint8Array,
    // lz-string owns decompression; these saved values pin the page's dispatch.
    LZString: {
      decompressFromEncodedURIComponent: (value) => {
        legacyCalls.push(value);
        return "legacy MML";
      },
    },
  };
  vm.runInNewContext(sharedSource, context);
  for (const name of [
    "b64urlEncode",
    "b64urlDecode",
    "compressDeflateRaw",
    "decompressDeflateRaw",
    "encodeMmlToUrl",
    "decodeMmlFromUrl",
  ])
    loadNamedFunction(source, name, context);

  const text = ["t120cdef", "", "t90gab\n", "日本語"].join(";");
  const encoded = await context.encodeMmlToUrl(text);
  assert.equal(await context.decodeMmlFromUrl(encoded), text);
  assert.deepEqual(legacyCalls, []);

  const plus =
    "C4RgTADABA9gLFAZgagKaIMYHNkbVgQ2UQBMAjE5XM1E1MrAPgsMeQPsctQG4+g";
  for (const value of [
    plus,
    plus.replaceAll("+", " "),
    "C4RgTADAxgJgpgMwNwqA",
  ]) {
    assert.equal(await context.decodeMmlFromUrl(value), "legacy MML");
    assert.equal(legacyCalls.at(-1), value);
  }
  assert.equal(legacyCalls.length, 3);
});

test("MML share updates keep the newest content when compression finishes out of order", async () => {
  const source = fs.readFileSync(
    path.join(__dirname, "..", "mml-studio", "index.html"),
    "utf8",
  );
  const textareas = Array.from({ length: 4 }, () => ({ value: "" }));
  const shareUrl = {};
  const qrCodeImage = {};
  const replacedUrls = [];
  const context = {
    document: {
      getElementById: (id) => {
        if (id === "share-url") return shareUrl;
        if (id === "qr-code-image") return qrCodeImage;
        return textareas[Number(id.slice(2)) - 1];
      },
    },
    window: {
      addEventListener() {},
      history: {
        replaceState: (_state, _title, url) => replacedUrls.push(url),
      },
    },
    location: { origin: "https://example.test", pathname: "/mml-studio/" },
    clearTimeout() {},
  };
  vm.runInNewContext(source.match(/<script>([\s\S]*?)<\/script>/)[1], context);
  const pending = [];
  context.encodeMmlToUrl = (text) =>
    new Promise((resolve) => pending.push({ text, resolve }));

  textareas[0].value = "cdef";
  const old = context.updateShareUrl();
  textareas[0].value = "gab";
  const newer = context.updateShareUrl();
  assert.deepEqual(
    pending.map(({ text }) => text),
    ["cdef;;;", "gab;;;"],
  );

  pending[1].resolve("newer");
  await newer;
  const newestUrl = "https://example.test/mml-studio/?mml=newer";
  assert.equal(shareUrl.href, newestUrl);
  const newestQr = qrCodeImage.src;

  pending[0].resolve("older");
  await old;
  assert.equal(shareUrl.href, newestUrl);
  assert.equal(shareUrl.textContent, newestUrl);
  assert.equal(qrCodeImage.src, newestQr);
  assert.deepEqual(replacedUrls, [newestUrl]);

  const stale = context.updateShareUrl();
  textareas[0].value = "";
  await context.updateShareUrl();
  const emptyUrl = "https://example.test/mml-studio/";
  assert.equal(shareUrl.href, emptyUrl);
  const emptyQr = qrCodeImage.src;

  pending[2].resolve("stale");
  await stale;
  assert.equal(shareUrl.href, emptyUrl);
  assert.equal(qrCodeImage.src, emptyQr);
  assert.deepEqual(replacedUrls, [newestUrl, emptyUrl]);
});
