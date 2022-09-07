const PYXEL_WHEEL = "pyxel-1.8.3-cp37-abi3-emscripten_3_1_20_wasm32.whl";
const APP_STARTUP_SCRIPT_FILE = ".pyxapp_startup_script";

class Pyxel {
    constructor(pyodide, canvas) {
        pyodide._module.canvas = canvas;
        this._pyodide = pyodide;
    }

    async fetchFiles(baseDir, files) {
        let pyodide = this._pyodide;
        for (let file of files) {
            let dirs = file.split("/");
            dirs.pop();
            let path = "";
            for (let dir of dirs) {
                path += dir;
                pyodide.FS.mkdir(path);
                path += "/";
            }
            const fileResponse = await fetch(`${baseDir}/${file}`);
            const fileBinary = new Uint8Array(await fileResponse.arrayBuffer());
            pyodide.FS.writeFile(file, fileBinary, { encoding: "binary" });
        }
    }

    run(codeOrFile) {
        if (codeOrFile.endsWith(".py")) {
            const file = codeOrFile;
            const code = this._pyodide.FS.readFile(file, { encoding: "utf8" });
            const dir = file.substring(0, file.lastIndexOf("/"))
            this._pyodide.FS.chdir(dir || ".");
            this._pyodide.runPython(`import os, sys; sys.path.append(os.getcwd()); del os, sys; \n${code} `);
        } else {
            const code = codeOrFile;
            this._pyodide.runPython(code);
        }
    }

    play(pyxelAppFile) {
        let pyodide = this._pyodide;
        let pyxel = this;
        (async function () {
            let zipResponse = await fetch(pyxelAppFile);
            let zipBinary = await zipResponse.arrayBuffer();
            pyodide.unpackArchive(zipBinary, "zip");
            let archiveDir = pyxelAppFile.split("/").pop().split(".").shift();
            for (let file of pyodide.FS.readdir(archiveDir)) {
                if (file != APP_STARTUP_SCRIPT_FILE) {
                    continue;
                }
                const startupFile = pyodide.FS.readFile(`${archiveDir}/${file}`, { encoding: "utf8" });
                pyxel.run(`${archiveDir}/${startupFile}`);
            }
        })();
    }
}

async function loadPyxel(canvasId) {
    let canvas = document.getElementById(canvasId);
    let pyodide = await loadPyodide();
    await pyodide.loadPackage(PYXEL_WHEEL);
    return new Pyxel(pyodide, canvas);
}
