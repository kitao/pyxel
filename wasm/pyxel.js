const NO_SLEEP_URL =
  "https://cdnjs.cloudflare.com/ajax/libs/nosleep/0.12.0/NoSleep.min.js";
const PYODIDE_SDL2_URL =
  "https://cdn.jsdelivr.net/gh/kitao/pyodide-sdl2@20220923/pyodide.js";
const PYXEL_WHEEL_PATH = "pyxel-1.8.13-cp37-abi3-emscripten_3_1_21_wasm32.whl";
const PYXEL_LOGO_PATH = "../docs/images/pyxel_logo_228x96.png";
const TOUCH_TO_START_PATH = "../docs/images/touch_to_start_342x42.png";
const CLICK_TO_START_PATH = "../docs/images/click_to_start_342x42.png";
const GAMEPAD_CROSS_PATH = "../docs/images/gamepad_cross_98x98.png";
const GAMEPAD_BUTTON_PATH = "../docs/images/gamepad_button_98x98.png";
const PYXEL_WORKING_DIRECTORY = "/pyxel_working_directory";

class Pyxel {
  constructor(pyodide) {
    this.pyodide = pyodide;
  }

  run(pythonScriptFile) {
    if (!pythonScriptFile) {
      return;
    }
    if (pythonScriptFile.endsWith(".py")) {
      this.pyodide.runPython(
        `import pyxel.cli; pyxel.cli.run_python_script("${pythonScriptFile}")`
      );
    } else {
      this.pyodide.runPython(pythonScriptFile);
    }
  }

  play(pyxelAppFile) {
    if (pyxelAppFile) {
      this.pyodide.runPython(
        `import pyxel.cli; pyxel.cli.play_pyxel_app("${pyxelAppFile}")`
      );
    }
  }

  edit(pyxelResourceFile, startingEditor) {
    this.pyodide.runPython(
      `import pyxel.cli; pyxel.cli.edit_pyxel_resource("${pyxelResourceFile}", "${startingEditor}")`
    );
  }
}

function _scriptDir() {
  let scripts = document.getElementsByTagName("script");
  for (const script of scripts) {
    let match = script.src.match(/(^|.*\/)pyxel\.js$/);
    if (match) {
      return match[1];
    }
  }
}

function _setIcon() {
  let head = document.getElementsByTagName("head").item(0);
  let link = document.createElement("link");
  link.rel = "icon";
  link.href = _scriptDir() + "../docs/images/pyxel_icon_64x64.ico";
  head.appendChild(link);
}

function _setStyleSheet() {
  let head = document.getElementsByTagName("head").item(0);
  link = document.createElement("link");
  link.rel = "stylesheet";
  link.href = _scriptDir() + "pyxel.css";
  head.appendChild(link);
}

function _addElements() {
  // Add body
  if (!document.getElementsByTagName("body").item(0)) {
    let body = document.createElement("body");
    body.style.overflow = "hidden";
    body.style.touchAction = "none";
    document.body = body;
  }

  // Add canvas for SDL2
  let canvas = document.createElement("canvas");
  canvas.id = "canvas";
  canvas.tabindex = -1;
  document.body.appendChild(canvas);

  // Add image for logo
  let img = document.createElement("img");
  img.id = "logo";
  img.src = _scriptDir() + PYXEL_LOGO_PATH;
  img.tabindex = -1;
  document.body.appendChild(img);

  // Prevent normal operation
  let touchHandler = (event) => {
    if (event.touches.length > 1) {
      event.preventDefault();
    }
  };
  document.addEventListener("touchstart", touchHandler, { passive: false });
  document.addEventListener("touchmove", touchHandler, { passive: false });
  document.oncontextmenu = (event) => event.preventDefault();

  // Enable gamepad
  window.addEventListener("gamepadconnected", (event) => {
    console.log(`Connected '${event.gamepad.id}'`);
  });
}

function _isTouchDevice() {
  return (
    "ontouchstart" in window ||
    navigator.maxTouchPoints > 0 ||
    navigator.msMaxTouchPoints > 0
  );
}

function _waitForInput(callback) {
  let img = document.querySelector("img#logo");
  if (img) {
    img.src =
      _scriptDir() +
      (_isTouchDevice() ? TOUCH_TO_START_PATH : CLICK_TO_START_PATH);
  }
  document.body.onclick = () => {
    document.body.onclick = "";
    if (img) {
      img.remove();
    }
    try {
      callback();
    } catch (error) {
      if (error.name === "PythonError") {
        document.oncontextmenu = null;
        document.body.style.overflow = "";
        document.body.style.touchAction = "";
        document.body.innerHTML = `
          <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          <pre>${error.message}</pre>
        `;
      }
      if (error !== "unwind") {
        throw error;
      }
    }
  };
}

function _addVirtualGamepad(mode) {
  if (mode !== "enabled" || !_isTouchDevice() || navigator.getGamepads()[0]) {
    return;
  }

  // Make canvas smaller
  document.querySelector("canvas#canvas").style.height = "80%";

  // Add virtual cross key
  let imgCross = document.createElement("img");
  imgCross.id = "gamepad-cross";
  imgCross.src = _scriptDir() + GAMEPAD_CROSS_PATH;
  imgCross.tabindex = -1;
  document.body.appendChild(imgCross);

  // Add virtual buttons
  let imgButton = document.createElement("img");
  imgButton.id = "gamepad-button";
  imgButton.src = _scriptDir() + GAMEPAD_BUTTON_PATH;
  imgButton.tabindex = -1;
  document.body.appendChild(imgButton);

  // Register virtual gamepad
  let gamepad = {
    connected: true,
    axes: [0, 0, 0, 0],
    buttons: [],
    id: "Virtual Gamepad for Pyxel",
    index: 0,
    mapping: "standard",
    timestamp: Date.now(),
  };
  for (let i = 0; i < 17; i++) {
    gamepad.buttons.push({ pressed: false, touched: false, value: 0 });
  }
  navigator.getGamepads = () => {
    return [gamepad];
  };
  let event = new Event("gamepadconnected");
  event.gamepad = gamepad;
  window.dispatchEvent(event);

  // Set touch event handler
  let crossRect = imgCross.getBoundingClientRect();
  let buttonRect = imgButton.getBoundingClientRect();
  let touchHandler = (event) => {
    for (let i = 0; i < gamepad.buttons.length; i++) {
      gamepad.buttons[i].pressed = false;
    }
    for (let i = 0; i < event.touches.length; i++) {
      let { clientX, clientY } = event.touches[i];
      let size = crossRect.width;
      let crossX = (clientX - crossRect.left) / size - 0.5;
      let crossY = (clientY - crossRect.bottom) / size + 0.5;
      let buttonX = (clientX - buttonRect.right) / size + 0.5;
      let buttonY = (clientY - buttonRect.bottom) / size + 0.5;
      if (crossX ** 2 + crossY ** 2 <= 0.5 ** 2) {
        let angle = (Math.atan2(-crossY, crossX) * 180) / Math.PI;
        if (angle > 22.5 && angle < 157.5) {
          gamepad.buttons[12].pressed = true; // Up
        }
        if (angle > -157.5 && angle < -22.5) {
          gamepad.buttons[13].pressed = true; // Down
        }
        if (Math.abs(angle) <= 67.5) {
          gamepad.buttons[15].pressed = true; // Right
        }
        if (Math.abs(angle) >= 112.5) {
          gamepad.buttons[14].pressed = true; // Left
        }
      }
      if (buttonX ** 2 + buttonY ** 2 <= 0.5 ** 2) {
        let angle = (Math.atan2(-buttonY, buttonX) * 180) / Math.PI;
        if (angle > -135 && angle < -45) {
          gamepad.buttons[0].pressed = true; // A
        }
        if (Math.abs(angle) <= 45) {
          gamepad.buttons[1].pressed = true; // B
        }
        if (Math.abs(angle) >= 135) {
          gamepad.buttons[2].pressed = true; // X
        }
        if (angle > 45 && angle < 135) {
          gamepad.buttons[3].pressed = true; // Y
        }
      }
    }
    gamepad.timestamp = Date.now();
    event.preventDefault();
  };
  let onTouchEnd = (event) => {
    for (let i = 0; i < gamepad.buttons.length; i++) {
      gamepad.buttons[i].pressed = false;
    }
    gamepad.timestamp = Date.now();
    event.preventDefault();
  };
  document.addEventListener("touchstart", touchHandler, { passive: false });
  document.addEventListener("touchmove", touchHandler, { passive: false });
  document.addEventListener("touchend", touchHandler, { passive: false });
}

async function _loadScript(scriptSrc) {
  await new Promise((resolve) => {
    let firstScript = document.getElementsByTagName("script")[0];
    let script = document.createElement("script");
    script.src = scriptSrc;
    firstScript.parentNode.insertBefore(script, firstScript);
    script.onload = () => resolve();
  });
}

async function loadPyxel(root, callback) {
  // Load libraries
  await _loadScript(NO_SLEEP_URL);
  let noSleep = new NoSleep();
  noSleep.enable();
  await _loadScript(PYODIDE_SDL2_URL);
  let pyodide = await loadPyodide();
  await pyodide.loadPackage(_scriptDir() + PYXEL_WHEEL_PATH);
  let pyxel = new Pyxel(pyodide);
  let FS = pyodide.FS;
  FS.mkdir(PYXEL_WORKING_DIRECTORY);
  FS.chdir(PYXEL_WORKING_DIRECTORY);

  // Create function to load file
  let loadFile = (filename) => {
    // Check filename
    if (filename.startsWith("<")) {
      return;
    }
    if (!filename.startsWith("/")) {
      filename = FS.cwd() + "/" + filename;
    }
    if (!filename.startsWith(PYXEL_WORKING_DIRECTORY)) {
      return;
    }
    filename = filename.slice(PYXEL_WORKING_DIRECTORY.length + 1);
    if (FS.analyzePath(filename).exists) {
      return;
    }

    // Download file
    let request = new XMLHttpRequest();
    request.overrideMimeType("text/plain; charset=x-user-defined");
    request.open("GET", `${root}/${filename}`, false);
    request.send();
    if (request.status !== 200) {
      return;
    }
    let fileBinary = Uint8Array.from(request.response, (c) => c.charCodeAt(0));

    // Secure directories
    let dirs = filename.split("/");
    dirs.pop();
    let path = "";
    for (let dir of dirs) {
      path += dir;
      if (!FS.analyzePath(path).exists) {
        FS.mkdir(path);
      }
      path += "/";
    }

    // Write file to Emscripten file system
    pyodide.FS.writeFile(filename, fileBinary, { encoding: "binary" });
    console.log(`Loaded '${root}${filename}'`);
  };

  // Hook file operations
  let open = FS.open;
  FS.open = (path, flags, mode) => {
    if (flags === 557056) {
      loadFile(path);
    }
    return open(path, flags, mode);
  };
  let stat = FS.stat;
  FS.stat = (path) => {
    loadFile(path);
    return stat(path);
  };

  // Invoke callback
  await callback(pyxel).catch((error) => {
    if (error !== "unwind") {
      throw error;
    }
  });
}

class PyxelRun extends HTMLElement {
  static get observedAttributes() {
    return ["root", "name", "script", "gamepad"];
  }

  constructor() {
    super();
    this.root = ".";
    this.name = "";
    this.script = "";
    this.gamepad = "disabled";
  }

  connectedCallback() {
    loadPyxel(this.root, async (pyxel) => {
      _waitForInput(() => {
        _addVirtualGamepad(this.gamepad);
        pyxel.run(this.name);
        pyxel.run(this.script);
      });
    });
  }

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}
window.customElements.define("pyxel-run", PyxelRun);

class PyxelPlay extends HTMLElement {
  static get observedAttributes() {
    return ["root", "name", "gamepad"];
  }

  constructor() {
    super();
    this.root = ".";
    this.name = "";
    this.gamepad = "disabled";
  }

  connectedCallback() {
    loadPyxel(this.root, async (pyxel) => {
      _waitForInput(() => {
        _addVirtualGamepad(this.gamepad);
        pyxel.play(this.name);
      });
    });
  }

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}
window.customElements.define("pyxel-play", PyxelPlay);

class PyxelEdit extends HTMLElement {
  static get observedAttributes() {
    return ["root", "name", "editor"];
  }

  constructor() {
    super();
    this.root = ".";
    this.name = "";
    this.editor = "image";
  }

  connectedCallback() {
    loadPyxel(this.root, async (pyxel) => {
      _waitForInput(() => {
        pyxel.edit(this.name, this.editor);
      });
    });
  }

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}
window.customElements.define("pyxel-edit", PyxelEdit);

_setIcon();
_setStyleSheet();
_addElements();
