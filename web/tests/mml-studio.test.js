const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const { loadArrowFunction, loadNamedFunction } = require("./source-loader");

const source = fs.readFileSync(
  path.join(__dirname, "..", "mml-studio", "index.html"),
  "utf8",
);
const wasmSource = fs.readFileSync(
  path.join(__dirname, "..", "..", "wasm", "pyxel.js"),
  "utf8",
);

const controller = (runtime) => {
  const timers = [];
  const errors = [];
  const ready = [];
  const press = loadNamedFunction(source, "handleButtonPress", {
    runtimeScreen: { contentWindow: runtime },
    setButtonStyle() {},
    setTimeout: (callback, delay) => timers.push({ callback, delay }),
    console: { error: (...args) => errors.push(args) },
    waitForPyxelReady: (condition, callback) =>
      ready.push({ condition, callback }),
  });
  return {
    press: (action) => press({ dataset: { action } }),
    timers,
    errors,
    ready,
  };
};

const deferredRuntime = () => {
  const resets = [];
  const runtime = {
    pyxelContext: { initialized: true },
    js_startup_complete: true,
    resetPyxel: () =>
      new Promise((resolve, reject) => resets.push({ resolve, reject })),
  };
  return { runtime, resets, ...controller(runtime) };
};

const settle = () => new Promise((resolve) => setImmediate(resolve));

test("first PLAY stays pending until initial startup consumes it", async () => {
  const { runtime, press, timers, resets, ready } = deferredRuntime();
  runtime.pyxelContext.initialized = false;
  runtime.js_startup_complete = false;
  press("play");
  timers[0].callback();
  await settle();
  assert.equal(runtime.js_play, true);
  assert.equal(resets.length, 0);
  assert.equal(ready[0].condition(), false);

  runtime.pyxelContext.initialized = true;
  assert.equal(ready[0].condition(), false);
  assert.equal(runtime.js_play, true);

  runtime.js_startup_complete = true;
  assert.equal(ready[0].condition(), true);
  ready[0].callback();
  assert.equal(runtime.js_play, false);
});

test("STOP cancels the first PLAY before runtime startup", () => {
  const { runtime, press, ready } = deferredRuntime();
  runtime.pyxelContext.initialized = false;
  runtime.js_startup_complete = false;
  press("play");
  press("stop");
  assert.equal(runtime.js_play, false);
  assert.equal(runtime.js_stop, true);
  ready[0].callback();
  assert.equal(runtime.js_stop, true);
});

for (const actions of [
  ["play", "play"],
  ["play", "stop"],
  ["play", "stop", "play"],
]) {
  test(`${actions.join(" then ")} during initial source loading respects the latest action`, () => {
    const { runtime, press, resets, ready } = deferredRuntime();
    runtime.js_startup_complete = false;
    actions.forEach(press);
    assert.equal(resets.length, 0);
    assert.equal(runtime.js_play, actions.at(-1) === "play");
    assert.equal(runtime.js_stop, actions.at(-1) === "stop");

    runtime.js_startup_complete = true;
    ready.forEach(({ condition, callback }) => {
      assert.equal(condition(), true);
      callback();
    });
    assert.equal(runtime.js_play, false);
    assert.equal(runtime.js_stop, actions.at(-1) === "stop");
  });
}

test("PLAY outlives button feedback and completes only after its reset", async () => {
  const { runtime, resets, press, timers } = deferredRuntime();
  press("play");
  timers[0].callback();
  assert.equal(runtime.js_play, true);

  resets[0].resolve();
  await settle();
  assert.equal(runtime.js_play, false);
});

test("older PLAY completion cannot clear the latest queued request", async () => {
  const { runtime, resets, press, timers } = deferredRuntime();
  press("play");
  press("play");
  timers.forEach(({ callback }) => callback());

  resets[0].resolve();
  await settle();
  assert.equal(runtime.js_play, true);

  resets[1].resolve();
  await settle();
  assert.equal(runtime.js_play, false);
});

test("STOP cancels pending PLAY and survives feedback until App consumption", async () => {
  const { runtime, resets, press, timers } = deferredRuntime();
  press("play");
  press("stop");
  timers.forEach(({ callback }) => callback());
  assert.equal(runtime.js_play, false);
  assert.equal(runtime.js_stop, true);

  resets[0].resolve();
  await settle();
  assert.equal(runtime.js_play, false);
  assert.equal(runtime.js_stop, true);

  press("play");
  assert.equal(runtime.js_stop, false);

  resets[1].resolve();
  await settle();
  assert.equal(runtime.js_play, false);
});

test("reset rejection cancels only its matching PLAY and reports failure", async () => {
  const { runtime, resets, press, errors } = deferredRuntime();
  press("play");
  press("play");
  const failure = new Error("reset failed");

  resets[0].reject(failure);
  await settle();
  assert.equal(runtime.js_play, true);

  resets[1].reject(failure);
  await settle();
  assert.equal(runtime.js_play, false);
  assert.equal(errors.length, 2);
  assert.equal(errors[0][1], failure);
});

test("actual serial reset queue retains PLAY through delayed audio suspension", async () => {
  let release;
  const suspended = new Promise((resolve) => {
    release = resolve;
  });
  const timers = [];
  const launches = [];
  const runtime = {
    js_startup_complete: true,
    pyxelContext: {
      initialized: true,
      clearPythonError() {},
      hasFatalError: false,
      params: {},
      resetPromise: null,
    },
  };
  const audioContext = {
    state: "running",
    async suspend() {
      await suspended;
      this.state = "suspended";
    },
    resume() {
      this.state = "running";
    },
  };
  runtime.pyxelContext.pyodide = {
    _module: { SDL2: { audioContext }, _emscripten_cancel_main_loop() {} },
    runPython() {},
  };
  const context = {
    window: runtime,
    document: { getElementById: () => null },
    setTimeout: (callback, delay) => timers.push({ callback, delay }),
    PYXEL_WORKING_DIRECTORY: "/work",
    _executePyxelCommand: async () =>
      launches.push([runtime.js_play, runtime.js_stop]),
    _displayFatalErrorOverlay: (error) => {
      throw error;
    },
  };
  context._resetPyxel = loadArrowFunction(wasmSource, "_resetPyxel", context);
  runtime.resetPyxel = loadNamedFunction(wasmSource, "resetPyxel", context);
  const { press, timers: feedback } = controller(runtime);

  press("play");
  press("play");
  feedback.forEach(({ callback }) => callback());
  await settle();
  assert.deepEqual(launches, []);
  assert.equal(runtime.js_play, true);

  timers.find(({ delay }) => delay === 50).callback();
  await settle();
  assert.deepEqual(launches, []);

  release();
  await settle();
  assert.deepEqual(launches, [
    [true, false],
    [true, false],
  ]);
  assert.equal(runtime.js_play, false);
  assert.equal(runtime.pyxelContext.resetPromise, null);
});
