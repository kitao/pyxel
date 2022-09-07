const PYXEL_WHEEL = "pyxel-1.8.3-cp37-abi3-emscripten_3_1_20_wasm32.whl";

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
            const fileResponse = await fetch(baseDir + '/' + file);
            const fileBinary = new Uint8Array(await fileResponse.arrayBuffer());
            pyodide.FS.writeFile(file, fileBinary, { encoding: 'binary' });
        }
    }

    run(codeOrFile) {
        if (codeOrFile.endsWith('.py')) {
            let file = codeOrFile;
            this._pyodide.runPython(`
                import runpy
                runpy.run_path("${file}")
            `);
        } else {
            let code = codeOrFile;
            this._pyodide.runPython(code);
        }
    }

    play(pyxelAppFile) {
        let pyodide = this._pyodide;
        (async function () {
            let zipResponse = await fetch(pyxelAppFile);
            let zipBinary = await zipResponse.arrayBuffer();
            await pyodide.unpackArchive(zipBinary, "zip");
            let appName = pyxelAppFile.split('/').pop();
            console.log(appName);
            for (let file of pyodide.FS.readdir(appName)) {
                //    if (file != ".pyxapp_startup_script") {
                //        continue;
                //    }
                //    const startup = pyodide.FS.readFile(`${PYXAPP_NAME}/${file}`, { encoding: "utf8" });
                //    pyodide.FS.chdir(pyodide.FS.lookupPath(`${PYXAPP_NAME}/${startup}`, { parent: true }).path);
                //    break;
            }
        })();
    }
}

async function loadPyxel(canvasQuery) {
    let canvas = document.querySelector(canvasQuery);
    let pyodide = await loadPyodide();
    await pyodide.loadPackage(PYXEL_WHEEL);
    return new Pyxel(pyodide, canvas);
}
