const PYXEL_POCKET_CORE_PATH = "pyxel-pocket-core.js";

const _scanCorrection = {};
const _readVirtualGamepadBitmask = () => 0;

window.pyxelPocketContext = {
  initialized: false,
  canvas: null,
  module: null,
  params: null,
  hasFatalError: false,
  lastError: "",
};

async function launchPyxelPocket(params) {
  console.log("Launch Pyxel with PocketPy");
  console.log(params);

  window.pyxelPocketContext.params = params;
  window.pyxelPocketContext.hasFatalError = false;
  window.pyxelPocketContext.lastError = "";

  try {
    const canvas = _createPocketCanvas();
    window.pyxelPocketContext.canvas = canvas;

    _validatePocketParams(params);
    await _loadScript(`${_pocketScriptDir}${PYXEL_POCKET_CORE_PATH}`);

    const module = await createPyxelPocketModule({
      canvas,
      locateFile: (path) => `${_pocketScriptDir}${path}`,
      noInitialRun: true,
    });

    window.pyxelPocketContext.initialized = true;
    window.pyxelPocketContext.module = module;

    await _waitForPocketInput();
    _runPocketScript(
      module,
      params.script,
      params.name || "<pyxel-pocket-web>",
    );
  } catch (error) {
    window.pyxelPocketContext.hasFatalError = true;
    _displayPocketError(error);
  }

  return window.pyxelPocketContext;
}

function resetPyxel() {
  location.reload();
}

const _pocketScriptDir = (() => {
  const script = document.currentScript;
  if (!script?.src) {
    return "";
  }
  return script.src.substring(0, script.src.lastIndexOf("/") + 1);
})();

const _validatePocketParams = (params) => {
  if (!params || params.command !== "run") {
    throw new Error(`Unsupported PocketPy command: ${params?.command}`);
  }
  if (typeof params.script !== "string") {
    throw new Error("launchPyxelPocket requires params.script");
  }
};

const _loadScript = async (scriptSrc) => {
  if (window.createPyxelPocketModule) {
    return;
  }

  const script = document.createElement("script");
  script.src = scriptSrc;
  const firstScript = document.getElementsByTagName("script")[0];
  firstScript.parentNode.insertBefore(script, firstScript);

  await new Promise((resolve, reject) => {
    script.addEventListener("load", resolve, { once: true });
    script.addEventListener(
      "error",
      () => reject(new Error(`Failed to load ${scriptSrc}`)),
      { once: true },
    );
  });
};

const _createPocketCanvas = () => {
  if (!document.body) {
    document.documentElement.appendChild(document.createElement("body"));
  }

  document.body.style.margin = "0";
  document.body.style.background = "#000";
  document.getElementById("pyxel-screen")?.remove();

  const screen = document.createElement("div");
  screen.id = "pyxel-screen";
  Object.assign(screen.style, {
    position: "relative",
    width: "100vw",
    height: "100vh",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    overflow: "hidden",
    background: "#000",
  });
  screen.oncontextmenu = (event) => event.preventDefault();

  const canvas = document.createElement("canvas");
  canvas.id = "canvas";
  canvas.tabIndex = 0;
  Object.assign(canvas.style, {
    width: "100%",
    height: "100%",
    objectFit: "contain",
    imageRendering: "pixelated",
  });

  screen.appendChild(canvas);
  document.body.appendChild(screen);
  return canvas;
};

const _waitForPocketInput = async () => {
  const screen = document.getElementById("pyxel-screen");
  const prompt = document.createElement("button");
  prompt.id = "pyxel-start-prompt";
  prompt.textContent = "Click to start";
  Object.assign(prompt.style, {
    position: "absolute",
    zIndex: 10,
    padding: "8px 12px",
    border: "0",
    background: "#fff",
    color: "#000",
    font: "14px sans-serif",
  });
  screen.appendChild(prompt);

  await new Promise((resolve) => {
    prompt.addEventListener("click", resolve, { once: true });
  });
  prompt.remove();
  document.getElementById("canvas").focus();
};

const _runPocketScript = (module, source, filename) => {
  for (const name of [
    "_pyxel_pocket_run_script",
    "_pyxel_pocket_last_error",
    "_malloc",
    "_free",
    "UTF8ToString",
    "stringToUTF8",
    "lengthBytesUTF8",
  ]) {
    if (typeof module[name] !== "function") {
      throw new Error(`PocketPy module is missing ${name}`);
    }
  }

  const sourcePtr = _writePocketString(module, source);
  const filenamePtr = _writePocketString(module, filename);
  try {
    const status = module._pyxel_pocket_run_script(sourcePtr, filenamePtr);
    if (status !== 0) {
      const errorPtr = module._pyxel_pocket_last_error();
      const message = errorPtr
        ? module.UTF8ToString(errorPtr)
        : "PocketPy failed";
      throw new Error(message);
    }
  } catch (error) {
    if (!_isPocketMainLoopUnwind(error)) {
      throw error;
    }
  } finally {
    module._free(sourcePtr);
    module._free(filenamePtr);
  }
};

const _isPocketMainLoopUnwind = (error) =>
  error === "unwind" || error?.message === "unwind";

const _writePocketString = (module, value) => {
  const length = module.lengthBytesUTF8(value) + 1;
  const ptr = module._malloc(length);
  if (!ptr) {
    throw new Error("PocketPy Web allocation failed");
  }
  module.stringToUTF8(value, ptr, length);
  return ptr;
};

const _displayPocketError = (error) => {
  console.error(error);
  const message =
    error && error.stack
      ? error.stack
      : error
        ? String(error)
        : "Unknown error";
  window.pyxelPocketContext.lastError = message;

  const screen = document.getElementById("pyxel-screen") || document.body;
  let overlay = document.getElementById("pyxel-error-overlay");
  if (!overlay) {
    overlay = document.createElement("pre");
    overlay.id = "pyxel-error-overlay";
    Object.assign(overlay.style, {
      position: "absolute",
      inset: "10px",
      zIndex: 1000,
      margin: "0",
      padding: "8px",
      boxSizing: "border-box",
      overflow: "auto",
      background: "rgba(0,0,0,0.7)",
      color: "#fff",
      fontSize: "12px",
    });
    screen.appendChild(overlay);
  }
  overlay.textContent = message;
};
