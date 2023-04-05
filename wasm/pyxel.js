const NO_SLEEP_URL =
  "https://cdnjs.cloudflare.com/ajax/libs/nosleep/0.12.0/NoSleep.min.js";
const PYODIDE_SDL2_URL =
  "https://cdn.jsdelivr.net/gh/kitao/pyodide-sdl2@0.22.1/pyodide.js";
const PYXEL_WHEEL_PATH = "pyxel-1.9.14-cp37-abi3-emscripten_3_1_34_wasm32.whl";
const PYXEL_LOGO_PATH = "../docs/images/pyxel_logo_76x32.png";
const TOUCH_TO_START_PATH = "../docs/images/touch_to_start_114x14.png";
const CLICK_TO_START_PATH = "../docs/images/click_to_start_114x14.png";
const GAMEPAD_CROSS_PATH = "../docs/images/gamepad_cross_98x98.png";
const GAMEPAD_BUTTON_PATH = "../docs/images/gamepad_button_98x98.png";
const PYXEL_WORKING_DIRECTORY = "/pyxel_working_directory";
const PYXEL_WATCH_INFO_FILE = ".pyxel_watch_info";

function _initialize() {
  _setIcon();
  _setStyleSheet();
  _registerCustomElements();
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
  let iconLink = document.createElement("link");
  iconLink.rel = "icon";
  iconLink.href = _scriptDir() + "../docs/images/pyxel_icon_64x64.ico";
  document.head.appendChild(iconLink);
}

function _setStyleSheet() {
  styleSheetLink = document.createElement("link");
  styleSheetLink.rel = "stylesheet";
  styleSheetLink.href = _scriptDir() + "pyxel.css";
  document.head.appendChild(styleSheetLink);
}

async function launchPyxel(params) {
  console.log("Launch Pyxel");
  console.log(params);
  _allowGamepadConnection();
  _suppressPinchOperations();
  await _createScreenElements();
  let pyodide = await _loadPyodideAndPyxel();
  _hookFileOperations(pyodide, params.root || ".");
  await _waitForInput();
  await _executePyxelCommand(pyodide, params);
}

function _allowGamepadConnection() {
  window.addEventListener("gamepadconnected", (event) => {
    console.log(`Connected '${event.gamepad.id}'`);
  });
}

function _suppressPinchOperations() {
  let touchHandler = (event) => {
    if (event.touches.length > 1) {
      event.preventDefault();
    }
  };
  document.addEventListener("touchstart", touchHandler, { passive: false });
  document.addEventListener("touchmove", touchHandler, { passive: false });
}

function _setMinWidthFromRatio(selector, screenSize) {
  let elem = document.querySelector(selector);
  if (!elem) {
    return;
  }
  let minWidthRatio = parseFloat(
    getComputedStyle(elem).getPropertyValue("--min-width-ratio")
  );
  elem.style.minWidth = `${screenSize * minWidthRatio}px`;
}

function _updateScreenElementsSize() {
  let pyxelScreen = document.querySelector("div#pyxel-screen");
  let { width, height } = pyxelScreen.getBoundingClientRect();
  let screenSize = Math.max(width, height);
  _setMinWidthFromRatio("img#pyxel-logo", screenSize);
  _setMinWidthFromRatio("img#pyxel-prompt", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-cross", screenSize);
  _setMinWidthFromRatio("img#pyxel-gamepad-button", screenSize);
}

function _waitForEvent(target, event) {
  return new Promise((resolve) => {
    let listener = (...args) => {
      target.removeEventListener(event, listener);
      resolve(...args);
    };
    target.addEventListener(event, listener);
  });
}

async function _createScreenElements() {
  let pyxelScreen = document.querySelector("div#pyxel-screen");
  if (!pyxelScreen) {
    pyxelScreen = document.createElement("div");
    pyxelScreen.id = "pyxel-screen";
    pyxelScreen.classList.add("default-pyxel-screen");
    if (!document.body) {
      document.body = document.createElement("body");
    }
    document.body.appendChild(pyxelScreen);
  }
  pyxelScreen.oncontextmenu = (event) => event.preventDefault();
  window.addEventListener("resize", _updateScreenElementsSize);

  // Add canvas for SDL2
  let sdl2Canvas = document.createElement("canvas");
  sdl2Canvas.id = "canvas";
  sdl2Canvas.tabindex = -1;
  pyxelScreen.appendChild(sdl2Canvas);

  // Add image for logo
  let logoImage = document.createElement("img");
  logoImage.id = "pyxel-logo";
  logoImage.src = _scriptDir() + PYXEL_LOGO_PATH;
  logoImage.tabindex = -1;
  await _waitForEvent(logoImage, "load");
  await new Promise((resolve) => setTimeout(resolve, 50));
  pyxelScreen.appendChild(logoImage);
  _updateScreenElementsSize();
}

async function _loadScript(scriptSrc) {
  let script = document.createElement("script");
  script.src = scriptSrc;
  let firstScript = document.getElementsByTagName("script")[0];
  firstScript.parentNode.insertBefore(script, firstScript);
  await _waitForEvent(script, "load");
}

async function _loadPyodideAndPyxel() {
  await _loadScript(NO_SLEEP_URL);
  let noSleep = new NoSleep();
  noSleep.enable();
  await _loadScript(PYODIDE_SDL2_URL);
  let pyodide = await loadPyodide();
  await pyodide.loadPackage(_scriptDir() + PYXEL_WHEEL_PATH);
  let FS = pyodide.FS;
  FS.mkdir(PYXEL_WORKING_DIRECTORY);
  FS.chdir(PYXEL_WORKING_DIRECTORY);
  return pyodide;
}

function _hookFileOperations(pyodide, root) {
  // Define function to copy file
  let fs = pyodide.FS;
  let copyFile = (filename) => {
    // Check file
    if (filename.startsWith("<")) {
      return;
    }
    if (!filename.startsWith("/")) {
      filename = fs.cwd() + "/" + filename;
    }
    if (!filename.startsWith(PYXEL_WORKING_DIRECTORY)) {
      return;
    }
    if (filename.endsWith(PYXEL_WATCH_INFO_FILE)) {
      return;
    }
    filename = filename.slice(PYXEL_WORKING_DIRECTORY.length + 1);
    let srcFile = `${root}/${filename}`;
    let dstFile = `${PYXEL_WORKING_DIRECTORY}/${filename}`;
    if (fs.analyzePath(dstFile).exists) {
      return;
    }

    // Download file
    let request = new XMLHttpRequest();
    request.overrideMimeType("text/plain; charset=x-user-defined");
    request.open("GET", srcFile, false);
    request.send();
    if (request.status !== 200) {
      console.log(`Failed to copy '${srcFile}' to '${dstFile}'`);
      return;
    }
    let fileBinary = Uint8Array.from(request.response, (c) => c.charCodeAt(0));

    // Secure directories
    let dirs = filename.split("/");
    dirs.pop();
    let path = "";
    for (let dir of dirs) {
      path += dir;
      if (!fs.analyzePath(path).exists) {
        fs.mkdir(path);
      }
      path += "/";
    }

    // Write file to Emscripten file system
    fs.writeFile(dstFile, fileBinary, {
      encoding: "binary",
    });
    console.log(`Copied '${srcFile}' to '${dstFile}'`);
  };

  // Hook file operations
  let open = fs.open;
  fs.open = (path, flags, mode) => {
    if (flags === 557056) {
      copyFile(path);
    }
    return open(path, flags, mode);
  };
  let stat = fs.stat;
  fs.stat = (path) => {
    copyFile(path);
    return stat(path);
  };

  // Define function to save file
  _savePyxelFile = (filename) => {
    let a = document.createElement("a");
    a.download = filename.split(/[\\/]/).pop();
    a.href = URL.createObjectURL(
      new Blob([fs.readFile(filename)], {
        type: "application/octet-stream",
      })
    );
    a.style.display = "none";
    document.body.appendChild(a);
    a.click();
    setTimeout(() => {
      document.body.removeChild(a);
      URL.revokeObjectURL(a.href);
    }, 2000);
  };
}

function _isTouchDevice() {
  return (
    "ontouchstart" in window ||
    navigator.maxTouchPoints > 0 ||
    navigator.msMaxTouchPoints > 0
  );
}

async function _waitForInput() {
  let pyxelScreen = document.querySelector("div#pyxel-screen");
  let logoImage = document.querySelector("img#pyxel-logo");
  logoImage.remove();
  let promptImage = document.createElement("img");
  promptImage.id = "pyxel-prompt";
  promptImage.src =
    _scriptDir() +
    (_isTouchDevice() ? TOUCH_TO_START_PATH : CLICK_TO_START_PATH);
  await _waitForEvent(promptImage, "load");
  pyxelScreen.appendChild(promptImage);
  _updateScreenElementsSize();
  await _waitForEvent(document.body, "click");
  promptImage.remove();
}

async function _installBuiltinPackages(pyodide, packages) {
  if (!packages) {
    return;
  }
  await pyodide.loadPackage(packages.split(","));
}

_virtualGamepadStates = [
  false, // Up
  false, // Down
  false, // Left
  false, // Right
  false, // A
  false, // B
  false, // X
  false, // Y
];

function _addVirtualGamepad(mode) {
  if (mode !== "enabled" || !_isTouchDevice()) {
    return;
  }

  // Make canvas smaller
  document.querySelector("canvas#canvas").style.height = "80%";

  // Add virtual cross key
  let pyxelScreen = document.querySelector("div#pyxel-screen");
  let crossImage = document.createElement("img");
  crossImage.id = "pyxel-gamepad-cross";
  crossImage.src = _scriptDir() + GAMEPAD_CROSS_PATH;
  crossImage.tabindex = -1;
  crossImage.onload = () => {
    pyxelScreen.appendChild(crossImage);
    _updateScreenElementsSize();
  };

  // Add virtual buttons
  let buttonImage = document.createElement("img");
  buttonImage.id = "pyxel-gamepad-button";
  buttonImage.src = _scriptDir() + GAMEPAD_BUTTON_PATH;
  buttonImage.tabindex = -1;
  buttonImage.onload = () => {
    pyxelScreen.appendChild(buttonImage);
    _updateScreenElementsSize();
  };

  // Set touch event handler
  let touchHandler = (event) => {
    let crossRect = crossImage.getBoundingClientRect();
    let buttonRect = buttonImage.getBoundingClientRect();
    for (let i = 0; i < _virtualGamepadStates.length; i++) {
      _virtualGamepadStates[i] = false;
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
        let angle = (Math.atan2(-buttonY, buttonX) * 180) / Math.PI;
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
    }
    event.preventDefault();
  };
  document.addEventListener("touchstart", touchHandler, { passive: false });
  document.addEventListener("touchmove", touchHandler, { passive: false });
  document.addEventListener("touchend", touchHandler, { passive: false });
}

function _copyFileFromBase64(pyodide, name, base64) {
  if (!name || !base64) {
    return;
  }
  let filename = `${PYXEL_WORKING_DIRECTORY}/${name}`;
  let binary = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
  pyodide.FS.writeFile(filename, binary, { encoding: "binary" });
}

async function _executePyxelCommand(pyodide, params) {
  if (params.command === "run" || params.command === "play") {
    await _installBuiltinPackages(pyodide, params.packages);
  }
  if (params.command === "run" || params.command === "play") {
    _addVirtualGamepad(params.gamepad);
  }
  _copyFileFromBase64(pyodide, params.name, params.base64);
  let pythonCode = "";
  switch (params.command) {
    case "run":
      if (params.name) {
        pythonCode = `
          import pyxel.cli
          pyxel.cli.run_python_script("${params.name}")
        `;
      } else if (params.script) {
        pythonCode = params.script;
      }
      break;
    case "play":
      pythonCode = `
        import pyxel.cli
        pyxel.cli.play_pyxel_app("${params.name}")
      `;
      break;
    case "edit":
      document.addEventListener("keydown", (event) => {
        if ((event.ctrlKey || event.metaKey) && event.key === "s") {
          event.preventDefault();
        }
      });
      params.name ||= "";
      pythonCode = `
        import pyxel.cli
        pyxel.cli.edit_pyxel_resource("${params.name}", "${params.editor}")
      `;
      break;
  }
  try {
    pyodide.runPython(pythonCode);
  } catch (error) {
    if (error.name === "PythonError") {
      document.body.innerHTML = `
          <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          <pre>${error.message}</pre>
        `;
    } else if (error !== "unwind") {
      throw error;
    }
  }
}

class PyxelRunElement extends HTMLElement {
  static get observedAttributes() {
    return ["root", "name", "script", "packages", "gamepad"];
  }

  hasLaunchPyxel = false

  constructor() {
    super();
    this.observer = new MutationObserver(() => {
      if (!this.hasLaunchPyxel) {
        this.hasLaunchPyxel = true
        launchPyxel({
          command: "run",
          root: this.root,
          name: this.name,
          script: this.textContent || this.script,
          packages: this.packages,
          gamepad: this.gamepad,
        });
      }
    });
  }

  connectedCallback() {
    this.observer.observe(this, { subtree: true, childList: true, characterData: true });
  }
  
  disconnectedCallback() {
    this.observer.disconnect();
  }

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}

class PyxelPlayElement extends HTMLElement {
  static get observedAttributes() {
    return ["root", "name", "packages", "gamepad"];
  }

  constructor() {
    super();
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

  constructor() {
    super();
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

function _registerCustomElements() {
  window.customElements.define("pyxel-run", PyxelRunElement);
  window.customElements.define("pyxel-play", PyxelPlayElement);
  window.customElements.define("pyxel-edit", PyxelEditElement);
}

_initialize();
