const PYXEL_POCKET_PATH = "pyxel_pocket.js";
const PYXEL_LOGO_PATH = "images/pyxel_logo_76x32.png";
const TOUCH_TO_START_PATH = "images/touch_to_start_114x14.png";
const CLICK_TO_START_PATH = "images/click_to_start_114x14.png";
const GAMEPAD_CROSS_PATH = "images/gamepad_cross_98x98.png";
const GAMEPAD_BUTTON_PATH = "images/gamepad_button_98x98.png";
const GAMEPAD_MENU_PATH = "images/gamepad_menu_92x26.png";
const PYXEL_WORKING_DIRECTORY = "/pyxel_working_directory";
const PYXEL_WATCH_INFO_FILE = ".pyxel_watch_info";

window.pyxelContext = {
  resolveInput: null,
  initialized: false,
  canvas: null,
  params: null,
  hasFatalError: false,
};

const _virtualGamepadStates = [
  false, // Up
  false, // Down
  false, // Left
  false, // Right
  false, // A
  false, // B
  false, // X
  false, // Y
  false, // Start
  false, // Back
];

// Safari emits Arrow key events with location=3 (numpad), which Emscripten
// does not recognize. Re-dispatch them with location=0 (standard).
if (/safari/i.test(navigator.userAgent) && !/chrome/i.test(navigator.userAgent)) {
  const fixArrowEvent = (event) => {
    if (event.isTrusted && event.location === 3 && event.key.startsWith("Arrow")) {
      event.stopImmediatePropagation();
      event.preventDefault();
      document.dispatchEvent(
        new KeyboardEvent(event.type, {
          key: event.key,
          code: event.code,
          location: 0,
          keyCode: event.keyCode,
          repeat: event.repeat,
          ctrlKey: event.ctrlKey,
          shiftKey: event.shiftKey,
          altKey: event.altKey,
          metaKey: event.metaKey,
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
//
// Unlike the previous queue-based approach, this map is keyed by physical key
// (SDL scancode) so it never goes out of sync between keydown and keyup events.
// The same correction is used for both press and release of a given key.
const _scanCorrection = {}; // Maps SDL scancode to unshifted char code

// SDL scancodes for printable ASCII keys (USB HID usage codes)
const _CODE_TO_SCANCODE = {
  KeyA: 4, KeyB: 5, KeyC: 6, KeyD: 7, KeyE: 8, KeyF: 9,
  KeyG: 10, KeyH: 11, KeyI: 12, KeyJ: 13, KeyK: 14, KeyL: 15,
  KeyM: 16, KeyN: 17, KeyO: 18, KeyP: 19, KeyQ: 20, KeyR: 21,
  KeyS: 22, KeyT: 23, KeyU: 24, KeyV: 25, KeyW: 26, KeyX: 27,
  KeyY: 28, KeyZ: 29,
  Digit1: 30, Digit2: 31, Digit3: 32, Digit4: 33, Digit5: 34,
  Digit6: 35, Digit7: 36, Digit8: 37, Digit9: 38, Digit0: 39,
  Space: 44, Minus: 45, Equal: 46, BracketLeft: 47, BracketRight: 48,
  Backslash: 49, Semicolon: 51, Quote: 52, Backquote: 53,
  Comma: 54, Period: 55, Slash: 56,
};
document.addEventListener(
  "keydown",
  (event) => {
    if (
      event.key.length === 1 &&
      !event.shiftKey && !event.ctrlKey && !event.altKey && !event.metaKey
    ) {
      const scancode = _CODE_TO_SCANCODE[event.code];
      if (scancode !== undefined) {
        _scanCorrection[scancode] = event.key.charCodeAt(0);
      }
    }
  },
  true,
);

async function launchPyxel(params) {
  console.log("Launch Pyxel Pocket");
  console.log(params);

  _suppressTouchZoomGestures();
  _allowGamepadConnection();

  const canvas = await _createScreenElements();
  await _waitForInput();

  window.pyxelContext.initialized = true;
  window.pyxelContext.canvas = canvas;
  window.pyxelContext.params = params;
  window.pyxelContext.hasFatalError = false;

  try {
    await _executePyxelCommand(canvas, params);
  } catch (error) {
    _displayFatalErrorOverlay(error);
  }
}

function resetPyxel() {
  location.reload();
}

function dropFileToPyxel(name, data) {
  if (!window.pyxelContext.initialized || typeof FS === "undefined") {
    return;
  }
  const path = "/tmp/" + name.replace(/[^a-zA-Z0-9._-]/g, "_");
  FS.writeFile(path, new Uint8Array(data));
}

function _initialize() {
  _setIcon();
  _setStyleSheet();
  _registerCustomElements();
  _hookGlobalErrors();
}

function _scriptDir() {
  const scripts = document.getElementsByTagName("script");
  for (const script of scripts) {
    const match = script.src.match(/(^|.*\/)pyxel\.js$/);
    if (match) {
      return match[1];
    }
  }
}

function _setIcon() {
  const iconLink = document.createElement("link");
  iconLink.rel = "icon";
  iconLink.href = _scriptDir() + "images/pyxel_icon_64x64.ico";
  document.head.appendChild(iconLink);
}

function _setStyleSheet() {
  const styleSheetLink = document.createElement("link");
  styleSheetLink.rel = "stylesheet";
  styleSheetLink.href = _scriptDir() + "pyxel.css";
  document.head.appendChild(styleSheetLink);
}

function _registerCustomElements() {
  window.customElements.define("pyxel-run", PyxelRunElement);
  window.customElements.define("pyxel-play", PyxelPlayElement);
  window.customElements.define("pyxel-edit", PyxelEditElement);
}

function _hookGlobalErrors() {
  window.addEventListener("error", (event) => {
    _displayFatalErrorOverlay(event.error || event.message || event);
  });

  window.addEventListener("unhandledrejection", (event) => {
    _displayFatalErrorOverlay(event.reason || event);
  });
}

function _allowGamepadConnection() {
  window.addEventListener("gamepadconnected", (event) => {
    console.log(`Connected '${event.gamepad.id}'`);
  });
}

function _suppressTouchZoomGestures() {
  // Ensure viewport disables pinch/double-tap zoom
  let m = document.querySelector('meta[name="viewport"]');
  if (!m) {
    m = document.createElement("meta");
    m.name = "viewport";
    document.head.appendChild(m);
  }
  m.content =
    "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no";

  // Suppress pinch-to-zoom by preventing multi-touch gestures
  const pinchHandler = (event) => {
    if (event.touches && event.touches.length > 1) {
      event.preventDefault();
    }
  };
  document.addEventListener("touchstart", pinchHandler, { passive: false });
  document.addEventListener("touchmove", pinchHandler, { passive: false });
}

function _setMinWidthFromRatio(selector, screenSize) {
  const elem = document.querySelector(selector);
  if (!elem) {
    return;
  }

  const minWidthRatio = parseFloat(
    getComputedStyle(elem).getPropertyValue("--min-width-ratio"),
  );
  elem.style.minWidth = `${screenSize * minWidthRatio}px`;
}

function _updateScreenElementsSize() {
  const pyxelScreen = document.querySelector("div#pyxel-screen");
  const { width, height } = pyxelScreen.getBoundingClientRect();
  const screenSize = Math.max(width, height);

  _setMinWidthFromRatio("img#pyxel-logo", screenSize);
  _setMinWidthFromRatio("img#pyxel-prompt", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-cross", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-button", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-menu", screenSize);
}

function _waitForEvent(target, ...events) {
  return new Promise((resolve) => {
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
}

async function _createScreenElements() {
  let pyxelScreen = document.querySelector("div#pyxel-screen");
  if (!pyxelScreen) {
    pyxelScreen = document.createElement("div");
    pyxelScreen.id = "pyxel-screen";
    if (!document.body) {
      document.body = document.createElement("body");
    }
    document.body.appendChild(pyxelScreen);
  }

  pyxelScreen.oncontextmenu = (event) => event.preventDefault();
  window.addEventListener("resize", _updateScreenElementsSize);

  // Handle file drop
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

  // Add canvas for SDL2
  const sdl2Canvas = document.createElement("canvas");
  sdl2Canvas.id = "canvas";
  sdl2Canvas.tabindex = -1;
  pyxelScreen.appendChild(sdl2Canvas);

  // Add image for logo
  const logoImage = document.createElement("img");
  logoImage.id = "pyxel-logo";
  logoImage.src = _scriptDir() + PYXEL_LOGO_PATH;
  logoImage.tabindex = -1;
  await _waitForEvent(logoImage, "load");
  await new Promise((resolve) => setTimeout(resolve, 50));
  pyxelScreen.appendChild(logoImage);
  _updateScreenElementsSize();

  return sdl2Canvas;
}

async function _loadScript(scriptSrc) {
  const script = document.createElement("script");
  script.src = scriptSrc;
  const firstScript = document.getElementsByTagName("script")[0];
  firstScript.parentNode.insertBefore(script, firstScript);
  await _waitForEvent(script, "load");
}


function _displayErrorOverlay(message) {
  console.error(message);
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
    document.getElementById("pyxel-screen").appendChild(overlay);
  }
  overlay.textContent = message;
  overlay.scrollTop = overlay.scrollHeight;
}

function _formatUnknownError(error) {
  if (!error) {
    return "Unknown error";
  }
  if (typeof error === "string") {
    return error;
  }
  const name = error.name || "Error";
  const message = error.message || String(error);
  const stack = error.stack || "";
  return `${name}: ${message}${stack ? "\n" + stack : ""}`;
}

function _displayFatalErrorOverlay(error) {
  window.pyxelContext.hasFatalError = true;
  const message = _formatUnknownError(error);
  _displayErrorOverlay(message);
}

function _hookFileOperations(root) {
  if (!root || typeof FS === "undefined") {
    return;
  }

  const createDirs = (absPath, isFile) => {
    const dirs = absPath.split("/");
    dirs.shift();
    if (isFile) dirs.pop();
    let path = "";
    for (const dir of dirs) {
      path += "/" + dir;
      if (!FS.analyzePath(path).exists) {
        FS.mkdir(path, 0o777);
      }
    }
  };

  let _fetching = false;
  const copyPath = (path) => {
    if (_fetching) return;
    if (path.startsWith("<") || path.endsWith(PYXEL_WATCH_INFO_FILE)) return;
    if (!path.startsWith("/")) path = FS.cwd() + "/" + path;
    if (!path.startsWith(PYXEL_WORKING_DIRECTORY)) return;
    path = path.slice(PYXEL_WORKING_DIRECTORY.length + 1);
    const srcPath = `${root}/${path}`;
    const dstPath = `${PYXEL_WORKING_DIRECTORY}/${path}`;

    _fetching = true;
    try {
      if (FS.analyzePath(dstPath).exists) return;

      console.log(`Attempting to fetch '${path}'`);
      const request = new XMLHttpRequest();
      request.overrideMimeType("text/plain; charset=x-user-defined");
      request.open("GET", srcPath, false);
      try { request.send(); } catch (error) { return; }
      if (request.status !== 200) return;
      const fileBinary = Uint8Array.from(request.response, (c) => c.charCodeAt(0));

      const contentType = request.getResponseHeader("Content-Type") || "";
      if (contentType.includes("text/html") && !path.includes(".")) {
        createDirs(dstPath, false);
      } else {
        createDirs(dstPath, true);
        FS.writeFile(dstPath, fileBinary, { encoding: "binary" });
        console.log(`Copied '${srcPath}' to '${dstPath}'`);
      }
    } finally {
      _fetching = false;
    }
  };

  const open = FS.open;
  FS.open = (path, flags, mode) => {
    copyPath(path);
    return open(path, flags, mode);
  };
  const stat = FS.stat;
  FS.stat = (path) => {
    copyPath(path);
    return stat(path);
  };

  window._savePyxelFile = (filename) => {
    const a = document.createElement("a");
    a.download = filename.split(/[\\/]/).pop();
    a.href = URL.createObjectURL(
      new Blob([FS.readFile(filename)], { type: "application/octet-stream" }),
    );
    a.style.display = "none";
    document.body.appendChild(a);
    a.click();
    setTimeout(() => { document.body.removeChild(a); URL.revokeObjectURL(a.href); }, 2000);
  };
}

function _isTouchDevice() {
  return window.matchMedia("(pointer: coarse)").matches;
}

async function _waitForInput() {
  const pyxelScreen = document.querySelector("div#pyxel-screen");
  const logoImage = document.querySelector("img#pyxel-logo");
  logoImage.remove();

  const promptImage = document.createElement("img");
  promptImage.id = "pyxel-prompt";
  promptImage.src =
    _scriptDir() +
    (_isTouchDevice() ? TOUCH_TO_START_PATH : CLICK_TO_START_PATH);
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
  await new Promise((resolve) => setTimeout(resolve, 1));
}

function _addVirtualGamepad(mode) {
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

  // Make canvas smaller
  document.querySelector("canvas#canvas").style.height = "80%";

  const pyxelScreen = document.querySelector("div#pyxel-screen");

  const createGamepadElement = (id, path) => {
    const img = document.createElement("img");
    img.id = id;
    img.src = _scriptDir() + path;
    img.tabindex = -1;
    img.onload = () => {
      pyxelScreen.appendChild(img);
      _updateScreenElementsSize();
    };
    return img;
  };

  const gamepadCrossImage = createGamepadElement("pyxel-gamepad-cross", GAMEPAD_CROSS_PATH);
  const gamepadButtonImage = createGamepadElement("pyxel-gamepad-button", GAMEPAD_BUTTON_PATH);
  const gamepadMenuImage = createGamepadElement("pyxel-gamepad-menu", GAMEPAD_MENU_PATH);

  // Set touch event handler
  const touchHandler = (event) => {
    const crossRect = gamepadCrossImage.getBoundingClientRect();
    const buttonRect = gamepadButtonImage.getBoundingClientRect();
    const menuRect = gamepadMenuImage.getBoundingClientRect();
    _virtualGamepadStates.fill(false);
    for (let i = 0; i < event.touches.length; i++) {
      const { clientX, clientY } = event.touches[i];
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
          _virtualGamepadStates[0] = true; // Up
        }
        if (angle > -157.5 && angle < -22.5) {
          _virtualGamepadStates[1] = true; // Down
        }
        if (Math.abs(angle) >= 112.5) {
          _virtualGamepadStates[2] = true; // Left
        }
        if (Math.abs(angle) <= 67.5) {
          _virtualGamepadStates[3] = true; // Right
        }
      }

      if (buttonX ** 2 + buttonY ** 2 <= 0.5 ** 2) {
        const angle = (Math.atan2(-buttonY, buttonX) * 180) / Math.PI;
        if (angle > -135 && angle < -45) {
          _virtualGamepadStates[4] = true; // A
        }
        if (Math.abs(angle) <= 45) {
          _virtualGamepadStates[5] = true; // B
        }
        if (Math.abs(angle) >= 135) {
          _virtualGamepadStates[6] = true; // X
        }
        if (angle > 45 && angle < 135) {
          _virtualGamepadStates[7] = true; // Y
        }
      }

      if (menuX >= 0.0 && menuX <= 1.0 && menuY >= 0.2 && menuY <= 0.5) {
        if (menuX >= 0.5) {
          _virtualGamepadStates[8] = true; // Start
        } else {
          _virtualGamepadStates[9] = true; // Back
        }
      }
    }
    event.preventDefault();
  };

  document.addEventListener("touchstart", touchHandler, { passive: false });
  document.addEventListener("touchmove", touchHandler, { passive: false });
  document.addEventListener("touchend", touchHandler, { passive: false });
}

async function _executePyxelCommand(canvas, params) {
  if (params.command === "run" || params.command === "play") {
    _addVirtualGamepad(params.gamepad);
  }

  // Determine script content and filename
  const scriptName = params.name || "__main__.py";
  let scriptContent;
  if (params.script) {
    scriptContent = params.script;
  } else if (params.base64) {
    scriptContent = new TextDecoder().decode(
      Uint8Array.from(atob(params.base64), (c) => c.charCodeAt(0)),
    );
  } else if (params.name) {
    const root = params.root || ".";
    const response = await fetch(`${root}/${params.name}`);
    if (!response.ok) {
      throw new Error(`Failed to fetch '${params.name}': ${response.status}`);
    }
    scriptContent = await response.text();
  } else {
    throw new Error("No script specified");
  }

  const root = params.root || ".";

  // Set up Emscripten Module and load pyxel-pocket WASM
  window.Module = {
    canvas: canvas,
    arguments: [scriptName],
    preRun: [
      function () {
        FS.mkdir(PYXEL_WORKING_DIRECTORY);
        FS.chdir(PYXEL_WORKING_DIRECTORY);
        FS.writeFile(scriptName, scriptContent);
        _hookFileOperations(root);
      },
    ],
    print: function (text) {
      console.log(text);
    },
    printErr: function (text) {
      console.error(text);
    },
  };

  await _loadScript(_scriptDir() + PYXEL_POCKET_PATH);
}

class PyxelRunElement extends HTMLElement {
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

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}

class PyxelPlayElement extends HTMLElement {
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

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}

class PyxelEditElement extends HTMLElement {
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

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}

_initialize();
