const PYODIDE_URL = "https://cdn.jsdelivr.net/pyodide/v314.0.2/full/pyodide.js";
const PYXEL_WHEEL_PATH = "pyxel-2.9.6-cp311-abi3-emscripten_5_0_3_wasm32.whl";
const PYXEL_LOGO_PATH = "images/pyxel_logo_76x32.png";
const TOUCH_TO_START_PATH = "images/touch_to_start_114x14.png";
const CLICK_TO_START_PATH = "images/click_to_start_114x14.png";
const GAMEPAD_CROSS_PATH = "images/gamepad_cross_98x98.png";
const GAMEPAD_BUTTON_PATH = "images/gamepad_button_98x98.png";
const GAMEPAD_MENU_PATH = "images/gamepad_menu_92x26.png";
const PYXEL_WORKING_DIRECTORY = "/pyxel_working_directory";
const PYXEL_WATCH_INFO_FILE = ".pyxel_watch_info";
const IMPORT_HOOK_PATH = "import_hook.py";
const VIRTUAL_GAMEPAD_UP = 0;
const VIRTUAL_GAMEPAD_DOWN = 1;
const VIRTUAL_GAMEPAD_LEFT = 2;
const VIRTUAL_GAMEPAD_RIGHT = 3;
const VIRTUAL_GAMEPAD_A = 4;
const VIRTUAL_GAMEPAD_B = 5;
const VIRTUAL_GAMEPAD_X = 6;
const VIRTUAL_GAMEPAD_Y = 7;
const VIRTUAL_GAMEPAD_START = 8;
const VIRTUAL_GAMEPAD_BACK = 9;
const VIRTUAL_GAMEPAD_BUTTON_COUNT = 10;

const _escapePythonString = (s) => JSON.stringify(s).slice(1, -1);

window.pyxelContext = {
  resolveInput: null,
  initialized: false,
  canvas: null,
  pyodide: null,
  params: null,
  hasFatalError: false,
};

const _virtualGamepadStates = Array(VIRTUAL_GAMEPAD_BUTTON_COUNT).fill(false);

// Pack the 10 button states into a single int so Rust can fetch them in
// one emscripten_run_script_int call instead of ten per frame.
const _readVirtualGamepadBitmask = () => {
  let bits = 0;
  for (let i = 0; i < VIRTUAL_GAMEPAD_BUTTON_COUNT; i++) {
    if (_virtualGamepadStates[i]) bits |= 1 << i;
  }
  return bits;
};

// Safari emits Arrow key events with location=3 (numpad), which Emscripten
// does not recognize. Re-dispatch them with location=0 (standard).
if (
  /safari/i.test(navigator.userAgent) &&
  !/chrome/i.test(navigator.userAgent)
) {
  // Normalize Safari's numpad arrow events before SDL2 handles them.
  const fixArrowEvent = (e) => {
    if (e.isTrusted && e.location === 3 && e.key.startsWith("Arrow")) {
      e.stopImmediatePropagation();
      e.preventDefault();
      document.dispatchEvent(
        new KeyboardEvent(e.type, {
          key: e.key,
          code: e.code,
          location: 0,
          keyCode: e.keyCode,
          repeat: e.repeat,
          ctrlKey: e.ctrlKey,
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          metaKey: e.metaKey,
          bubbles: true,
          cancelable: true,
        }),
      );
    }
  };
  document.addEventListener("keydown", fixArrowEvent, true);
  document.addEventListener("keyup", fixArrowEvent, true);
}

// Keyboard key correction for non-US layouts (e.g. JIS, AZERTY)
// Emscripten's SDL2 maps physical keys through a US-layout table, producing
// incorrect keycodes for non-US keyboards. This builds a persistent per-key
// correction map from SDL scancode to the actual character, using the browser's
// KeyboardEvent.key (which reflects the true layout).
// Map SDL scancodes to unshifted character codes.
const _scanCorrection = {};

// SDL scancodes for printable ASCII keys (USB HID usage codes)
const _CODE_TO_SCANCODE = {
  KeyA: 4,
  KeyB: 5,
  KeyC: 6,
  KeyD: 7,
  KeyE: 8,
  KeyF: 9,
  KeyG: 10,
  KeyH: 11,
  KeyI: 12,
  KeyJ: 13,
  KeyK: 14,
  KeyL: 15,
  KeyM: 16,
  KeyN: 17,
  KeyO: 18,
  KeyP: 19,
  KeyQ: 20,
  KeyR: 21,
  KeyS: 22,
  KeyT: 23,
  KeyU: 24,
  KeyV: 25,
  KeyW: 26,
  KeyX: 27,
  KeyY: 28,
  KeyZ: 29,
  Digit1: 30,
  Digit2: 31,
  Digit3: 32,
  Digit4: 33,
  Digit5: 34,
  Digit6: 35,
  Digit7: 36,
  Digit8: 37,
  Digit9: 38,
  Digit0: 39,
  Space: 44,
  Minus: 45,
  Equal: 46,
  BracketLeft: 47,
  BracketRight: 48,
  Backslash: 49,
  Semicolon: 51,
  Quote: 52,
  Backquote: 53,
  Comma: 54,
  Period: 55,
  Slash: 56,
};
document.addEventListener(
  "keydown",
  (e) => {
    if (
      e.key.length === 1 &&
      !e.shiftKey &&
      !e.ctrlKey &&
      !e.altKey &&
      !e.metaKey
    ) {
      const scancode = _CODE_TO_SCANCODE[e.code];
      if (scancode !== undefined) {
        _scanCorrection[scancode] = e.key.charCodeAt(0);
      }
    }
  },
  true,
);

// Public API

async function launchPyxel(params) {
  const pyxelVersion = PYXEL_WHEEL_PATH.match(/pyxel-([\d.]+)-/)[1];
  const pyodideVersion = PYODIDE_URL.match(/v([\d.]+)\//)[1];
  console.log(`Launch Pyxel ${pyxelVersion} with Pyodide ${pyodideVersion}`);
  console.log(params);

  _suppressTouchZoomGestures();
  _allowGamepadConnection();

  const canvas = await _createScreenElements();
  const pyodide = await _loadPyodideAndPyxel(canvas);

  _hookPythonError(pyodide);
  _hookFileOperations(pyodide, params.root || ".");
  await _waitForInput();

  window.pyxelContext.initialized = true;
  window.pyxelContext.canvas = canvas;
  window.pyxelContext.pyodide = pyodide;
  window.pyxelContext.params = params;
  window.pyxelContext.hasFatalError = false;

  try {
    await _executePyxelCommand(pyodide, params);
  } catch (error) {
    _displayFatalErrorOverlay(error);
  }
}

// Reset runtime state and relaunch the current Pyxel command.
async function resetPyxel() {
  if (!window.pyxelContext.initialized) {
    return;
  }

  if (window.pyxelContext.hasFatalError) {
    location.reload();
    return;
  }

  try {
    document.getElementById("pyxel-error-overlay")?.remove();

    window.pyxelContext.pyodide.runPython(`
      import pyxel
      pyxel.quit()
    `);

    const audioContext =
      window.pyxelContext.pyodide?._module?.SDL2?.audioContext;
    if (audioContext && audioContext.state === "running") {
      // Let pending audio callbacks finish after quit before suspending.
      await new Promise((resolve) => setTimeout(resolve, 50));
      await audioContext.suspend();
    }

    const pyodide = window.pyxelContext.pyodide;
    pyodide._module._emscripten_cancel_main_loop();

    pyodide.runPython(`
      import importlib
      import os
      import shutil
      import sys
      import tempfile
      from types import ModuleType

      import pyxel

      pyxel._reset_statics()

      work_dir = "${PYXEL_WORKING_DIRECTORY}"
      temp_dir = tempfile.gettempdir()
      mods = [
          n
          for n, m in list(sys.modules.items())
          if getattr(m, "__file__", "")
          and (m.__file__.startswith(work_dir) or m.__file__.startswith(temp_dir))
      ] + ["__main__"]

      for n in mods:
          try:
              del sys.modules[n]
          except BaseException:
              pass
      importlib.invalidate_caches()
      sys.modules["__main__"] = ModuleType("__main__")

      os.chdir("/")
      if os.path.exists(temp_dir):
          shutil.rmtree(temp_dir)
      os.makedirs(temp_dir, exist_ok=True)

      if os.path.exists(work_dir):
          shutil.rmtree(work_dir)
      os.makedirs(work_dir, exist_ok=True)
      os.chdir(work_dir)
    `);

    await _executePyxelCommand(pyodide, window.pyxelContext.params);

    // Undo the earlier suspend so the restarted app has audio.
    setTimeout(() => {
      if (audioContext && audioContext.state === "suspended") {
        audioContext.resume();
      }
    }, 0);
  } catch (error) {
    _displayFatalErrorOverlay(error);
  }
}

function dropFileToPyxel(name, data) {
  if (!window.pyxelContext.initialized) {
    return;
  }
  const path = `/tmp/${name.replace(/[^a-zA-Z0-9._-]/g, "_")}`;
  const pyodide = window.pyxelContext.pyodide;
  pyodide.FS.writeFile(path, new Uint8Array(data));
  pyodide.runPython(
    `import pyxel; pyxel._dropped_files = getattr(pyxel, '_dropped_files', []) + ['${path}']`,
  );
}

// Initialization

const _initialize = () => {
  _setIcon();
  _setStyleSheet();
  _registerCustomElements();
  _hookGlobalErrors();
};

const _scriptDir = (() => {
  for (const script of document.getElementsByTagName("script")) {
    const match = script.src.match(/(^|.*\/)pyxel\.js$/);
    if (match) return match[1];
  }
  return "";
})();

const _setIcon = () => {
  const iconLink = document.createElement("link");
  iconLink.rel = "icon";
  iconLink.href = `${_scriptDir}images/pyxel_icon_64x64.ico`;
  document.head.appendChild(iconLink);
};

const _setStyleSheet = () => {
  const styleSheetLink = document.createElement("link");
  styleSheetLink.rel = "stylesheet";
  styleSheetLink.href = `${_scriptDir}pyxel.css`;
  document.head.appendChild(styleSheetLink);
};

const _registerCustomElements = () => {
  window.customElements.define("pyxel-run", PyxelRunElement);
  window.customElements.define("pyxel-play", PyxelPlayElement);
  window.customElements.define("pyxel-edit", PyxelEditElement);
};

const _hookGlobalErrors = () => {
  window.addEventListener("error", (e) => {
    _displayFatalErrorOverlay(e.error || e.message || e);
  });

  window.addEventListener("unhandledrejection", (e) => {
    _displayFatalErrorOverlay(e.reason || e);
  });
};

const _allowGamepadConnection = () => {
  window.addEventListener("gamepadconnected", (e) => {
    console.log(`Connected '${e.gamepad.id}'`);
  });
};

const _suppressTouchZoomGestures = () => {
  // Ensure viewport disables pinch/double-tap zoom.
  let meta = document.querySelector('meta[name="viewport"]');
  if (!meta) {
    meta = document.createElement("meta");
    meta.name = "viewport";
    document.head.appendChild(meta);
  }
  meta.content =
    "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no";

  // Suppress pinch-to-zoom by preventing multi-touch gestures.
  const pinchHandler = (e) => {
    if (e.touches && e.touches.length > 1) {
      e.preventDefault();
    }
  };
  document.addEventListener("touchstart", pinchHandler, { passive: false });
  document.addEventListener("touchmove", pinchHandler, { passive: false });
};

// Screen elements

const _setMinWidthFromRatio = (selector, screenSize) => {
  const elem = document.querySelector(selector);
  if (!elem) {
    return;
  }

  const minWidthRatio = parseFloat(
    getComputedStyle(elem).getPropertyValue("--min-width-ratio"),
  );
  elem.style.minWidth = `${screenSize * minWidthRatio}px`;
};

const _updateScreenElementsSize = () => {
  const pyxelScreen = document.querySelector("div#pyxel-screen");
  const { width, height } = pyxelScreen.getBoundingClientRect();
  const screenSize = Math.max(width, height);

  _setMinWidthFromRatio("img#pyxel-logo", screenSize);
  _setMinWidthFromRatio("img#pyxel-prompt", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-cross", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-button", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-menu", screenSize);
};

// Event helpers

const _waitForEvent = (target, ...events) =>
  new Promise((resolve) => {
    const listener = (...args) => {
      for (const ev of events) {
        target.removeEventListener(ev, listener);
      }
      resolve(...args);
    };
    for (const ev of events) {
      target.addEventListener(ev, listener);
    }
  });

// Create the screen container, drop area, SDL2 canvas, and logo.
const _createScreenElements = async () => {
  let pyxelScreen = document.querySelector("div#pyxel-screen");
  if (!pyxelScreen) {
    pyxelScreen = document.createElement("div");
    pyxelScreen.id = "pyxel-screen";
    if (!document.body) {
      document.body = document.createElement("body");
    }
    document.body.appendChild(pyxelScreen);
  }

  pyxelScreen.oncontextmenu = (e) => e.preventDefault();
  if (!window._pyxelResizeListenerAttached) {
    let resizeTimer;
    window.addEventListener("resize", () => {
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(_updateScreenElementsSize, 100);
    });
    window._pyxelResizeListenerAttached = true;
  }

  pyxelScreen.addEventListener("dragover", (e) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "copy";
  });
  pyxelScreen.addEventListener("drop", (e) => {
    e.preventDefault();
    const file = e.dataTransfer.files?.[0];
    if (file) {
      file.arrayBuffer().then((buf) => dropFileToPyxel(file.name, buf));
    }
  });

  const sdl2Canvas = document.createElement("canvas");
  sdl2Canvas.id = "canvas";
  sdl2Canvas.tabIndex = -1;
  pyxelScreen.appendChild(sdl2Canvas);

  const logoImage = document.createElement("img");
  logoImage.id = "pyxel-logo";
  logoImage.src = `${_scriptDir}${PYXEL_LOGO_PATH}`;
  logoImage.tabIndex = -1;
  await _waitForEvent(logoImage, "load");
  // Wait briefly; appending the logo right after load has caused issues.
  await new Promise((resolve) => setTimeout(resolve, 50));
  pyxelScreen.appendChild(logoImage);
  _updateScreenElementsSize();

  return sdl2Canvas;
};

// Pyodide loading

const _loadScript = async (scriptSrc) => {
  const script = document.createElement("script");
  script.src = scriptSrc;
  const firstScript = document.getElementsByTagName("script")[0];
  firstScript.parentNode.insertBefore(script, firstScript);
  await _waitForEvent(script, "load");
};

// Bootstrap Pyodide, install Pyxel, and prepare its working directory.
const _loadPyodideAndPyxel = async (canvas) => {
  // Prefetch wheel and import hook in parallel with pyodide.js download and
  // runtime init. Consuming the wheel body populates the HTTP cache, so
  // pyodide.loadPackage's later fetch of the same URL hits the cache.
  const wheelUrl = `${_scriptDir}${PYXEL_WHEEL_PATH}`;
  const wheelPrefetch = fetch(wheelUrl).then((r) => r.arrayBuffer());
  const importHookFetch = fetch(`${_scriptDir}${IMPORT_HOOK_PATH}`);

  await _loadScript(PYODIDE_URL);
  const pyodide = await loadPyodide();
  // Keep Pyodide from treating Emscripten's main-loop unwind as a fatal error.
  pyodide._api._skip_unwind_fatal_error = true;
  pyodide.canvas.setCanvas2D(canvas);

  await wheelPrefetch;
  await pyodide.loadPackage(wheelUrl);

  const fs = pyodide.FS;
  fs.mkdir(PYXEL_WORKING_DIRECTORY);
  fs.chdir(PYXEL_WORKING_DIRECTORY);

  const response = await importHookFetch;
  if (!response.ok) {
    throw new Error(`Failed to fetch ${IMPORT_HOOK_PATH}: ${response.status}`);
  }
  const code = await response.text();
  pyodide.runPython(code);

  return pyodide;
};

// Error handling

const _hookPythonError = (pyodide) => {
  pyodide.setStderr({
    batched: (() => {
      let errorText = "";
      let flushTimer = null;

      return (msg) => {
        if (!flushTimer && !msg.startsWith("Traceback")) {
          return;
        }

        pyodide._module._emscripten_cancel_main_loop();
        errorText += `${msg}\n`;

        if (!flushTimer) {
          // Batch stderr lines for 100 ms so a multi-line traceback shows as one overlay.
          flushTimer = setTimeout(() => {
            _displayErrorOverlay(errorText);
            errorText = "";
            flushTimer = null;
          }, 100);
        }
      };
    })(),
  });
};

// Render Python exceptions over the canvas without replacing the page.
const _displayErrorOverlay = (message) => {
  console.error(message);
  const pyxelScreen = document.getElementById("pyxel-screen");
  let overlay = document.getElementById("pyxel-error-overlay");
  if (!overlay) {
    overlay = document.createElement("pre");
    overlay.id = "pyxel-error-overlay";
    Object.assign(overlay.style, {
      position: "absolute",
      top: "10px",
      left: "12px",
      right: "12px",
      bottom: "10px",
      zIndex: 1000,
      margin: "0",
      padding: "8px",
      boxSizing: "border-box",
      overflow: "auto",
      background: "rgba(0,0,0,0.7)",
      color: "#fff",
      fontSize: "12px",
    });
    pyxelScreen.appendChild(overlay);
  }
  overlay.textContent = message;
  overlay.scrollTop = overlay.scrollHeight;
};

const _formatUnknownError = (error) => {
  if (!error) {
    return "Unknown error";
  }
  if (typeof error === "string") {
    return error;
  }
  const name = error.name || "Error";
  const message = error.message || String(error);
  const stack = error.stack || "";
  return `${name}: ${message}${stack ? `\n${stack}` : ""}`;
};

const _displayFatalErrorOverlay = (error) => {
  window.pyxelContext.hasFatalError = true;
  _displayErrorOverlay(_formatUnknownError(error));
};

// File operations

// Mirror requested files from the hosting page into Pyodide's filesystem.
const _hookFileOperations = (pyodide, root) => {
  const fs = pyodide.FS;

  const createDirs = (absPath, isFile) => {
    const dirs = absPath.split("/");
    dirs.shift();
    if (isFile) {
      dirs.pop();
    }
    let path = "";
    for (const dir of dirs) {
      path += `/${dir}`;
      if (!fs.analyzePath(path).exists) {
        fs.mkdir(path, 0o777);
      }
    }
  };

  // Copy a requested host-side path into Pyodide on first read access.
  const copyPath = (path) => {
    if (path.startsWith("<") || path.endsWith(PYXEL_WATCH_INFO_FILE)) {
      return;
    }
    if (!path.startsWith("/")) {
      path = `${fs.cwd()}/${path}`;
    }
    if (!path.startsWith(PYXEL_WORKING_DIRECTORY)) {
      return;
    }
    path = path.slice(PYXEL_WORKING_DIRECTORY.length + 1);
    const srcPath = `${root}/${path}`;
    const dstPath = `${PYXEL_WORKING_DIRECTORY}/${path}`;
    if (fs.analyzePath(dstPath).exists) {
      return;
    }

    // Download the missing host-side path synchronously for Pyodide.
    console.log(`Attempting to fetch '${path}'`);
    const request = new XMLHttpRequest();
    request.overrideMimeType("text/plain; charset=x-user-defined");
    request.open("GET", srcPath, false);
    try {
      request.send();
    } catch {
      return;
    }
    if (request.status !== 200) {
      return;
    }
    const fileBinary = Uint8Array.from(request.response, (c) =>
      c.charCodeAt(0),
    );

    // Write fetched content as a directory marker or binary file.
    const contentType = request.getResponseHeader("Content-Type") || "";
    if (contentType.includes("text/html") && !path.includes(".")) {
      console.log(`Created directory '${dstPath}'`);
      createDirs(dstPath, false);
    } else {
      createDirs(dstPath, true);
      fs.writeFile(dstPath, fileBinary, { encoding: "binary" });
      console.log(`Copied '${srcPath}' to '${dstPath}'`);
    }
  };

  // Hook read-only open/stat calls so files are mirrored lazily.
  // SDL2 open flags: O_RDONLY | O_STAT
  const O_RDONLY_STAT = 557056;
  const open = fs.open.bind(fs);
  fs.open = (path, flags, mode) => {
    if (flags === O_RDONLY_STAT) {
      copyPath(path);
    }
    return open(path, flags, mode);
  };
  const stat = fs.stat.bind(fs);
  fs.stat = (path) => {
    copyPath(path);
    return stat(path);
  };

  // Expose a browser download helper used by the Python-side save path.
  window._savePyxelFile = (filename) => {
    const a = document.createElement("a");
    a.download = filename.split(/[\\/]/).pop();
    a.href = URL.createObjectURL(
      new Blob([fs.readFile(filename)], {
        type: "application/octet-stream",
      }),
    );
    a.style.display = "none";
    document.body.appendChild(a);
    a.click();
    // Revoke only after the click-initiated download has had time to start.
    setTimeout(() => {
      document.body.removeChild(a);
      URL.revokeObjectURL(a.href);
    }, 2000);
  };
};

// Input and startup

const _isTouchDevice = () => window.matchMedia("(pointer: coarse)").matches;

// Gate startup until the browser receives the user gesture required for audio.
const _waitForInput = async () => {
  const pyxelScreen = document.querySelector("div#pyxel-screen");
  const logoImage = document.querySelector("img#pyxel-logo");
  logoImage.remove();

  const promptImage = document.createElement("img");
  promptImage.id = "pyxel-prompt";
  promptImage.src = `${_scriptDir}${_isTouchDevice() ? TOUCH_TO_START_PATH : CLICK_TO_START_PATH}`;
  await _waitForEvent(promptImage, "load");
  pyxelScreen.appendChild(promptImage);
  _updateScreenElementsSize();

  await new Promise((resolve) => {
    window.pyxelContext.resolveInput = () => {
      window.pyxelContext.resolveInput = null;
      resolve();
    };
    _waitForEvent(document.body, "click", "touchstart").then(() => {
      if (window.pyxelContext.resolveInput) {
        window.pyxelContext.resolveInput();
      }
    });
  });

  promptImage.remove();
  // Yield one task so the prompt removal is rendered before execution resumes.
  await new Promise((resolve) => setTimeout(resolve, 1));
};

// Virtual gamepad

// Map touch positions onto virtual button states.
const _updateGamepadStateFromTouch = (
  clientX,
  clientY,
  crossRect,
  buttonRect,
  menuRect,
) => {
  const size = crossRect.width;
  const crossX = (clientX - crossRect.left) / size - 0.5;
  const crossY = (clientY - crossRect.bottom) / size + 0.5;
  const buttonX = (clientX - buttonRect.right) / size + 0.5;
  const buttonY = (clientY - buttonRect.bottom) / size + 0.5;
  const menuX = (clientX - menuRect.left) / size;
  const menuY = (clientY - menuRect.bottom) / size + 0.5;

  if (crossX ** 2 + crossY ** 2 <= 0.5 ** 2) {
    const angle = (Math.atan2(-crossY, crossX) * 180) / Math.PI;
    if (angle > 22.5 && angle < 157.5) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_UP] = true;
    }
    if (angle > -157.5 && angle < -22.5) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_DOWN] = true;
    }
    if (Math.abs(angle) >= 112.5) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_LEFT] = true;
    }
    if (Math.abs(angle) <= 67.5) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_RIGHT] = true;
    }
  }

  if (buttonX ** 2 + buttonY ** 2 <= 0.5 ** 2) {
    const angle = (Math.atan2(-buttonY, buttonX) * 180) / Math.PI;
    if (angle > -135 && angle < -45) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_A] = true;
    }
    if (Math.abs(angle) <= 45) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_B] = true;
    }
    if (Math.abs(angle) >= 135) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_X] = true;
    }
    if (angle > 45 && angle < 135) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_Y] = true;
    }
  }

  if (menuX >= 0.0 && menuX <= 1.0 && menuY >= 0.2 && menuY <= 0.5) {
    if (menuX >= 0.5) {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_START] = true;
    } else {
      _virtualGamepadStates[VIRTUAL_GAMEPAD_BACK] = true;
    }
  }
};

// Add touch controls and keep their hit areas stable across resets.
const _addVirtualGamepad = (mode) => {
  if (mode !== "enabled" || !_isTouchDevice()) {
    return;
  }

  if (
    document.getElementById("pyxel-gamepad-cross") ||
    document.getElementById("pyxel-gamepad-button") ||
    document.getElementById("pyxel-gamepad-menu")
  ) {
    return;
  }

  // Reserve vertical space for the touch controls.
  document.querySelector("canvas#canvas").style.height = "80%";

  const pyxelScreen = document.querySelector("div#pyxel-screen");

  const createGamepadElement = (id, path) => {
    const img = document.createElement("img");
    img.id = id;
    img.src = `${_scriptDir}${path}`;
    img.tabIndex = -1;
    img.onload = () => {
      pyxelScreen.appendChild(img);
      _updateScreenElementsSize();
    };
    return img;
  };

  const gamepadCrossImage = createGamepadElement(
    "pyxel-gamepad-cross",
    GAMEPAD_CROSS_PATH,
  );
  const gamepadButtonImage = createGamepadElement(
    "pyxel-gamepad-button",
    GAMEPAD_BUTTON_PATH,
  );
  const gamepadMenuImage = createGamepadElement(
    "pyxel-gamepad-menu",
    GAMEPAD_MENU_PATH,
  );

  // Remove previous handlers if any (prevents accumulation on reset).
  if (_addVirtualGamepad._handler) {
    const prev = _addVirtualGamepad._handler;
    document.removeEventListener("touchstart", prev);
    document.removeEventListener("touchmove", prev);
    document.removeEventListener("touchend", prev);
  }
  if (_addVirtualGamepad._invalidateRects) {
    window.removeEventListener("resize", _addVirtualGamepad._invalidateRects);
  }

  // Cache bounding rects and invalidate them on resize.
  let cachedRects = null;
  const invalidateRects = () => {
    cachedRects = null;
  };
  _addVirtualGamepad._invalidateRects = invalidateRects;
  window.addEventListener("resize", invalidateRects);

  const touchHandler = (e) => {
    if (!cachedRects) {
      const cross = gamepadCrossImage.getBoundingClientRect();
      const button = gamepadButtonImage.getBoundingClientRect();
      const menu = gamepadMenuImage.getBoundingClientRect();
      // Avoid caching zero rects before the controls enter the DOM.
      if (cross.width > 0 && button.width > 0 && menu.width > 0) {
        cachedRects = { cross, button, menu };
      }
    }
    const cross =
      cachedRects?.cross ?? gamepadCrossImage.getBoundingClientRect();
    const button =
      cachedRects?.button ?? gamepadButtonImage.getBoundingClientRect();
    const menu = cachedRects?.menu ?? gamepadMenuImage.getBoundingClientRect();
    _virtualGamepadStates.fill(false);
    for (const touch of e.touches) {
      const { clientX, clientY } = touch;
      _updateGamepadStateFromTouch(clientX, clientY, cross, button, menu);
    }
    e.preventDefault();
  };
  _addVirtualGamepad._handler = touchHandler;

  document.addEventListener("touchstart", touchHandler, { passive: false });
  document.addEventListener("touchmove", touchHandler, { passive: false });
  document.addEventListener("touchend", touchHandler, { passive: false });
};

// Command execution

const _installBuiltinPackages = async (pyodide, packages) => {
  if (!packages) {
    return;
  }
  await pyodide.loadPackage(packages.split(","));
};

const _copyFileFromBase64 = (pyodide, name, base64) => {
  if (!name || !base64) {
    return;
  }
  const filename = `${PYXEL_WORKING_DIRECTORY}/${name}`;
  const binary = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
  pyodide.FS.writeFile(filename, binary, { encoding: "binary" });
};

// Translate custom-element parameters into the corresponding Python command.
const _executePyxelCommand = async (pyodide, params) => {
  if (params.command === "run" || params.command === "play") {
    await _installBuiltinPackages(pyodide, params.packages);
    _addVirtualGamepad(params.gamepad);
  }

  _copyFileFromBase64(pyodide, params.name, params.base64);

  let pythonCode = "";
  switch (params.command) {
    case "run":
      if (params.name) {
        pythonCode = `
          import pyxel.cli
          pyxel.cli.run_python_script("${_escapePythonString(params.name)}")
        `;
      } else if (params.script) {
        pythonCode = params.script;
      }
      break;

    case "play":
      pythonCode = `
        import pyxel.cli
        pyxel.cli.play_pyxel_app("${_escapePythonString(params.name)}")
      `;
      break;

    case "edit":
      if (!window._pyxelEditKeyHandler) {
        window._pyxelEditKeyHandler = (e) => {
          if ((e.ctrlKey || e.metaKey) && e.key === "s") {
            e.preventDefault();
          }
        };
        document.addEventListener("keydown", window._pyxelEditKeyHandler);
      }
      params.name ||= "";
      params.editor ||= "";
      pythonCode = `
        import pyxel.cli
        pyxel.cli.edit_pyxel_resource("${_escapePythonString(params.name)}", "${_escapePythonString(params.editor)}")
      `;
      break;
  }

  try {
    pyodide.runPython(pythonCode);
  } catch (error) {
    if (error?.name === "PythonError") {
      _displayErrorOverlay(error.message);
    } else {
      _displayFatalErrorOverlay(error);
    }
  }
};

// Custom elements

class PyxelBaseElement extends HTMLElement {
  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}

class PyxelRunElement extends PyxelBaseElement {
  static get observedAttributes() {
    return ["root", "name", "script", "packages", "gamepad"];
  }

  connectedCallback() {
    launchPyxel({
      command: "run",
      root: this.root,
      name: this.name,
      script: this.script,
      packages: this.packages,
      gamepad: this.gamepad,
    });
  }
}

class PyxelPlayElement extends PyxelBaseElement {
  static get observedAttributes() {
    return ["root", "name", "packages", "gamepad"];
  }

  connectedCallback() {
    launchPyxel({
      command: "play",
      root: this.root,
      name: this.name,
      packages: this.packages,
      gamepad: this.gamepad,
    });
  }
}

class PyxelEditElement extends PyxelBaseElement {
  static get observedAttributes() {
    return ["root", "name", "editor"];
  }

  connectedCallback() {
    launchPyxel({
      command: "edit",
      root: this.root,
      name: this.name,
      editor: this.editor,
    });
  }
}

_initialize();
