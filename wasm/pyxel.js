const NO_SLEEP_URL =
  "https://cdnjs.cloudflare.com/ajax/libs/nosleep/0.12.0/NoSleep.min.js";
const PYODIDE_SDL2_URL =
  "https://cdn.jsdelivr.net/gh/kitao/pyodide-sdl2@20220923/pyodide.js";
const PYXEL_WHEEL_PATH = "pyxel-1.8.8-cp37-abi3-emscripten_3_1_21_wasm32.whl";
const PYXEL_LOGO_PATH = "../docs/images/pyxel_logo_152x64.png";
const TOUCH_TO_START_PATH = "../docs/images/touch_to_start_228x28.png";
const CLICK_TO_START_PATH = "../docs/images/click_to_start_228x28.png";

class Pyxel {
  constructor(pyodide) {
    this.pyodide = pyodide;
  }

  async fetchFiles(root, names) {
    let FS = this.pyodide.FS;
    for (let name of names) {
      if (!name) {
        continue;
      }
      let dirs = name.split("/");
      dirs.pop();
      let path = "";
      for (let dir of dirs) {
        path += dir;
        if (!FS.analyzePath(path).exists) {
          FS.mkdir(path);
        }
        path += "/";
      }
      let fileResponse = await fetch(`${root}/${name}`);
      let fileBinary = new Uint8Array(await fileResponse.arrayBuffer());
      FS.writeFile(name, fileBinary, { encoding: "binary" });
      console.log(`Fetched ${root}${name}`);
    }
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

  edit(pyxelResourceFile) {
    this.pyodide.runPython(
      `import pyxel.cli; pyxel.cli.edit_pyxel_resource("${pyxelResourceFile}")`
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
  let body = document.getElementsByTagName("body").item(0);
  if (!body) {
    body = document.createElement("body");
    document.body = body;
  }
  let canvas = document.querySelector("canvas#canvas")
  if (!canvas) {
    canvas = document.createElement("canvas");
    canvas.id = "canvas";
    canvas.tabindex = -1;
    body.appendChild(canvas);
  }
  canvas.oncontextmenu = (event) => event.preventDefault();
  if (!document.querySelector("img#logo")) {
    let img = document.createElement("img");
    img.id = "logo";
    img.src = _scriptDir() + PYXEL_LOGO_PATH;
    img.oncontextmenu = (event) => event.preventDefault();
    img.tabindex = -1;
    body.appendChild(img);
  }
}

function _isMobileDevice() {
  let userAgent = navigator.userAgent.toLowerCase();
  return (
    userAgent.indexOf("iphone") > -1 ||
    userAgent.indexOf("ipad") > -1 ||
    userAgent.indexOf("android") > -1 ||
    (userAgent.indexOf("macintosh") > -1 && "ontouchend" in document)
  );
}

function _waitForInput(callback) {
  let img = document.querySelector("img#logo");
  if (img) {
    img.src =
      _scriptDir() +
      (_isMobileDevice() ? TOUCH_TO_START_PATH : CLICK_TO_START_PATH);
  }
  document.body.onclick = () => {
    document.body.onclick = "";
    if (img) {
      img.remove();
    }
    try {
      callback();
    } catch (e) {
      if (e !== "unwind") {
        throw e;
      }
    }
  };
}

async function _loadScript(scriptSrc) {
  await new Promise((resolve) => {
    var firstScript = document.getElementsByTagName("script")[0];
    var script = document.createElement("script");
    script.src = scriptSrc;
    firstScript.parentNode.insertBefore(script, firstScript);
    script.onload = () => resolve();
  });
}

async function loadPyxel(callback) {
  await _loadScript(NO_SLEEP_URL);
  let noSleep = new NoSleep();
  noSleep.enable();
  await _loadScript(PYODIDE_SDL2_URL);
  let pyodide = await loadPyodide();
  await pyodide.loadPackage(_scriptDir() + PYXEL_WHEEL_PATH);
  let pyxel = new Pyxel(pyodide);
  await callback(pyxel).catch((e) => {
    if (e !== "unwind") {
      throw e;
    }
  });
}

class PyxelAsset extends HTMLElement {
  static names = [];

  static get observedAttributes() {
    return ["name"];
  }

  constructor() {
    super();
  }

  connectedCallback() {
    PyxelAsset.names.push(this.name);
  }

  attributeChangedCallback(name, _oldValue, newValue) {
    this[name] = newValue;
  }
}
window.customElements.define("pyxel-asset", PyxelAsset);

class PyxelRun extends HTMLElement {
  static get observedAttributes() {
    return ["root", "name", "script"];
  }

  constructor() {
    super();
    this.root = ".";
    this.name = "";
    this.script = "";
  }

  connectedCallback() {
    loadPyxel(async (pyxel) => {
      await pyxel.fetchFiles(this.root, PyxelAsset.names.concat(this.name));
      _waitForInput(() => {
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
    return ["root", "name"];
  }

  constructor() {
    super();
    this.root = ".";
    this.name = "";
  }

  connectedCallback() {
    loadPyxel(async (pyxel) => {
      await pyxel.fetchFiles(this.root, PyxelAsset.names.concat(this.name));
      _waitForInput(() => {
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
    return ["root", "name"];
  }

  constructor() {
    super();
    this.root = ".";
    this.name = "";
  }

  connectedCallback() {
    loadPyxel(async (pyxel) => {
      await pyxel.fetchFiles(this.root, PyxelAsset.names.concat(this.name));
      _waitForInput(() => {
        pyxel.edit(this.name);
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
