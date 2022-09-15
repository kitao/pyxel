const PYXEL_WHEEL = "pyxel-1.8.3-cp37-abi3-emscripten_3_1_21_wasm32.whl";
const APP_STARTUP_SCRIPT_FILE = ".pyxapp_startup_script";

class Pyxel {
    constructor(pyodide) {
        this._pyodide = pyodide;
    }

    async fetchFiles(baseDir, files) {
        let FS = this._pyodide.FS;
        for (let file of files) {
            let dirs = file.split("/");
            dirs.pop();
            let path = "";
            for (let dir of dirs) {
                path += dir;
                if (!FS.analyzePath(path).exists) {
                    FS.mkdir(path);
                }
                path += "/";
            }
            console.log(baseDir, file);
            let fileResponse = await fetch(`${baseDir}/${file}`);
            let fileBinary = new Uint8Array(await fileResponse.arrayBuffer());
            FS.writeFile(file, fileBinary, { encoding: "binary" });
        }
    }

    exec(pythonScript) {
        this._pyodide.runPython(pythonScript);
    }

    run(pythonScriptFile) {
        this._pyodide.runPython(`import pyxel.cli; pyxel.cli.run_python_script("${pythonScriptFile}")`);
    }

    play(pyxelAppFile) {
        this._pyodide.runPython(`import pyxel.cli; pyxel.cli.play_pyxel_app("${pyxelAppFile}")`);
    }

    edit(pyxelResourceFile) {
        this._pyodide.runPython(`import pyxel.cli; pyxel.cli.edit_pyxel_resource("${pyxelResourceFile}")`);
    }

    package(appRootDir, startupScriptName) {
        this._pyodide.runPython(`import pyxel.cli; pyxel.cli.package_pyxel_app("${appRootDir}", "${startupScriptName}")`);
    }

    copyExamples() {
        this._pyodide.runPython(`import pyxel.cli; pyxel.cli.copy_pyxel_examples()`);
    }
}

async function loadPyxel() {
    let pyodide = await loadPyodide();
    await pyodide.loadPackage(PYXEL_WHEEL);
    return new Pyxel(pyodide);
}
