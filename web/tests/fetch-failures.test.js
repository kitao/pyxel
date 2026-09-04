const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const { loadArrowFunction, loadNamedFunction } = require("./source-loader");

const codeMakerSource = fs.readFileSync(
  path.join(__dirname, "..", "code-maker", "index.html"),
  "utf8",
);
const sharedSource = fs.readFileSync(
  path.join(__dirname, "..", "shared.js"),
  "utf8",
);
const pyxelSource = fs.readFileSync(
  path.join(__dirname, "..", "..", "wasm", "pyxel.js"),
  "utf8",
);
const encodeUrlPath = loadArrowFunction(sharedSource, "encodeUrlPath", {});

test("loadFromGist rejects a missing truncated file before reading its body", async () => {
  let textCount = 0;
  let loadCount = 0;
  const responses = [
    {
      ok: true,
      json: async () => ({
        files: {
          "project.zip": {
            content: "",
            filename: "project.zip",
            raw_url: "https://example.com/project.zip",
            truncated: true,
          },
        },
      }),
    },
    {
      ok: false,
      status: 404,
      text: async () => {
        textCount += 1;
      },
    },
  ];
  const context = {
    base64ToUint8: () => new Uint8Array(),
    fetch: async () => responses.shift(),
    loadProjectFromZip: async () => {
      loadCount += 1;
    },
    updateShareUrl: () => {},
  };
  const loadFromGist = loadNamedFunction(
    codeMakerSource,
    "loadFromGist",
    context,
  );

  await assert.rejects(
    loadFromGist("https://gist.github.com/example/0123456789abcdef0123"),
    { name: "Error", message: "Gist file not found" },
  );
  assert.equal(textCount, 0);
  assert.equal(loadCount, 0);
});

test("loadFromGitHub fetches a commit but keeps the source ref in share URLs", async () => {
  let requestedUrl;
  let sharedSource;
  const context = {
    encodeUrlPath,
    fetch: async (url) => {
      requestedUrl = url;
      return { ok: true, arrayBuffer: async () => new ArrayBuffer(0) };
    },
    loadProjectFromZip: async () => {},
    resolveGitHubBlobUrl: async () => ({
      user: "example",
      repo: "game",
      ref: "main",
      sha: "0123456789abcdef0123456789abcdef01234567",
      path: "apps/demo#preview?.zip",
    }),
    updateShareUrl: (param, value, ref, sha) => {
      sharedSource = { param, value, ref, sha };
    },
  };
  const loadFromGitHub = loadNamedFunction(
    codeMakerSource,
    "loadFromGitHub",
    context,
  );

  await loadFromGitHub("https://github.com/example/game/blob/main/project.zip");

  assert.equal(
    requestedUrl,
    "https://raw.githubusercontent.com/example/game/0123456789abcdef0123456789abcdef01234567/apps/demo%23preview%3F.zip",
  );
  assert.deepEqual(sharedSource, {
    param: "github",
    value: "example/game/main/apps/demo#preview?",
    ref: "main",
    sha: "0123456789abcdef0123456789abcdef01234567",
  });
});

test("loadFromGitHub keeps a compact ref when SHA lookup is unavailable", async () => {
  let preferredRef;
  let requestedUrl;
  let sharedSource;
  const context = {
    encodeUrlPath,
    fetch: async (url) => {
      requestedUrl = url;
      return {
        ok: true,
        arrayBuffer: async () => new ArrayBuffer(0),
      };
    },
    loadProjectFromZip: async () => {},
    resolveGitHubBlobUrl: async (_input, _fetchImpl, preferred) => {
      preferredRef = preferred;
      return {
        user: "example",
        repo: "game",
        ref: "main",
        sha: null,
        path: "apps/demo.zip",
      };
    },
    updateShareUrl: (param, value, ref, sha) => {
      sharedSource = { param, value, ref, sha };
    },
  };
  const loadFromGitHub = loadNamedFunction(
    codeMakerSource,
    "loadFromGitHub",
    context,
  );

  await loadFromGitHub(
    "https://github.com/example/game/blob/main/apps/demo.zip",
    "main",
  );

  assert.equal(preferredRef, "main");
  assert.equal(
    requestedUrl,
    "https://raw.githubusercontent.com/example/game/main/apps/demo.zip",
  );
  assert.deepEqual(sharedSource, {
    param: "github",
    value: "example/game/main/apps/demo",
    ref: "main",
    sha: null,
  });
});

test("Code Maker restores reserved characters from a GitHub share URL", () => {
  const buildGitHubProjectUrl = loadNamedFunction(
    codeMakerSource,
    "buildGitHubProjectUrl",
    { encodeUrlPath },
  );

  assert.equal(
    buildGitHubProjectUrl("example/game/feature/physics/apps/demo#preview?"),
    "https://github.com/example/game/blob/feature/physics/apps/demo%23preview%3F.zip",
  );
});

test("Code Maker preserves longest-ref resolution for legacy share URLs", async () => {
  const sha = "0123456789abcdef0123456789abcdef01234567";
  const requested = [];
  let loaded = false;
  let sharedUpdate;
  const fetchImpl = async (url) => {
    requested.push(url);
    if (url.startsWith("https://api.github.com/")) {
      const ok =
        url.endsWith("/commits/feature%2Fphysics") ||
        url.endsWith("/commits/feature");
      return {
        ok,
        status: ok ? 200 : 422,
        text: async () =>
          ok
            ? sha
            : JSON.stringify({
                message: `No commit found for SHA: ${url.split("/commits/")[1]}`,
              }),
      };
    }
    return {
      ok: true,
      arrayBuffer: async () => new ArrayBuffer(0),
    };
  };
  const resolveGitHubBlobUrl = loadArrowFunction(
    sharedSource,
    "resolveGitHubBlobUrl",
    { URL },
  );
  const loadFromGitHub = loadNamedFunction(codeMakerSource, "loadFromGitHub", {
    encodeUrlPath,
    fetch: fetchImpl,
    loadProjectFromZip: async () => {
      loaded = true;
    },
    resolveGitHubBlobUrl,
    updateShareUrl: (param, value, ref, resolvedSha) => {
      sharedUpdate = { param, value, ref, sha: resolvedSha };
    },
  });
  const buildGitHubProjectUrl = loadNamedFunction(
    codeMakerSource,
    "buildGitHubProjectUrl",
    { encodeUrlPath },
  );
  const loadFromGitHubShare = loadNamedFunction(
    codeMakerSource,
    "loadFromGitHubShare",
    { buildGitHubProjectUrl, loadFromGitHub },
  );

  await loadFromGitHubShare("example/game/feature/physics/apps/demo");

  assert.equal(loaded, true);
  assert.deepEqual(requested, [
    "https://api.github.com/repos/example/game/commits/feature%2Fphysics%2Fapps",
    "https://api.github.com/repos/example/game/commits/feature%2Fphysics",
    `https://raw.githubusercontent.com/example/game/${sha}/apps/demo.zip`,
  ]);
  assert.deepEqual(sharedUpdate, {
    param: "github",
    value: "example/game/feature/physics/apps/demo",
    ref: "feature/physics",
    sha,
  });
});

test("Code Maker uses the recorded SHA when an explicit ref is missing", async () => {
  for (const status of [404, 422]) {
    const savedSha = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const otherSha = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const requested = [];
    let loaded = false;
    let sharedUpdate;
    const fetchImpl = async (url) => {
      requested.push(url);
      if (url.startsWith("https://api.github.com/")) {
        const ok = url.endsWith("/commits/feature%2Fphysics%2Fapps");
        return {
          ok,
          status: ok ? 200 : status,
          text: async () =>
            ok
              ? otherSha
              : JSON.stringify({
                  message: "No commit found for SHA: feature/physics",
                }),
        };
      }
      return {
        ok: true,
        arrayBuffer: async () => new ArrayBuffer(0),
      };
    };
    const resolveGitHubBlobUrl = loadArrowFunction(
      sharedSource,
      "resolveGitHubBlobUrl",
      { URL },
    );
    const loadFromGitHub = loadNamedFunction(
      codeMakerSource,
      "loadFromGitHub",
      {
        encodeUrlPath,
        fetch: fetchImpl,
        loadProjectFromZip: async () => {
          loaded = true;
        },
        resolveGitHubBlobUrl,
        updateShareUrl: (param, value, ref, resolvedSha) => {
          sharedUpdate = { param, value, ref, sha: resolvedSha };
        },
      },
    );
    const buildGitHubProjectUrl = loadNamedFunction(
      codeMakerSource,
      "buildGitHubProjectUrl",
      { encodeUrlPath },
    );
    const loadFromGitHubShare = loadNamedFunction(
      codeMakerSource,
      "loadFromGitHubShare",
      { buildGitHubProjectUrl, loadFromGitHub },
    );

    await loadFromGitHubShare(
      "example/game/feature/physics/apps/demo",
      "feature/physics",
      savedSha,
    );

    assert.equal(loaded, true);
    assert.deepEqual(requested, [
      "https://api.github.com/repos/example/game/commits/feature%2Fphysics",
      `https://raw.githubusercontent.com/example/game/${savedSha}/apps/demo.zip`,
    ]);
    assert.deepEqual(sharedUpdate, {
      param: "github",
      value: "example/game/feature/physics/apps/demo",
      ref: "feature/physics",
      sha: savedSha,
    });
  }
});

test("Code Maker share URLs preserve and encode an explicit ref boundary", () => {
  let replacedUrl;
  const updateShareUrl = loadNamedFunction(codeMakerSource, "updateShareUrl", {
    location: { href: "https://example.test/web/code-maker/?old=value" },
    history: {
      replaceState: (_state, _title, url) => {
        replacedUrl = url;
      },
    },
  });

  updateShareUrl(
    "github",
    "example/game/feature/physics/apps/demo#preview?",
    "feature/physics",
    "0123456789abcdef0123456789abcdef01234567",
  );

  const result = new URL(replacedUrl);
  assert.equal(
    result.searchParams.get("github"),
    "example/game/feature/physics/apps/demo#preview?",
  );
  assert.equal(result.searchParams.get("ref"), "feature/physics");
  assert.equal(
    result.searchParams.get("sha"),
    "0123456789abcdef0123456789abcdef01234567",
  );
});

test("sanitizeProjectName neutralizes unsafe archive path syntax", () => {
  const sanitizeProjectName = loadNamedFunction(
    codeMakerSource,
    "sanitizeProjectName",
    {},
  );

  assert.equal(sanitizeProjectName("Space Game"), "Space Game");
  assert.equal(sanitizeProjectName("Foo/Bar"), "Foo_Bar");
  assert.equal(sanitizeProjectName("\\evil"), "_evil");
  assert.equal(sanitizeProjectName("."), "untitled");
  assert.equal(sanitizeProjectName(".."), "untitled");
  assert.equal(sanitizeProjectName("..."), "untitled");
  assert.equal(sanitizeProjectName("Bad\0Name"), "Bad_Name");
  assert.equal(sanitizeProjectName('Bad<Name>:"|?*'), "Bad_Name______");
  assert.equal(sanitizeProjectName("Project. "), "Project");
  assert.equal(sanitizeProjectName("CON"), "_CON");
  assert.equal(sanitizeProjectName("nul.txt"), "_nul.txt");
  assert.equal(sanitizeProjectName("COM1"), "_COM1");
  assert.equal(sanitizeProjectName("LPT9.log"), "_LPT9.log");
  assert.equal(sanitizeProjectName("COM¹"), "_COM¹");
  assert.equal(sanitizeProjectName("CONIN$"), "_CONIN$");
});

test("loadProjectFromZip sanitizes the project name at assignment", async () => {
  const sanitizeProjectName = loadNamedFunction(
    codeMakerSource,
    "sanitizeProjectName",
    {},
  );
  const window = {
    _project: {},
    _codeEditor: { setValue: () => {} },
  };
  const archiveEntry = {
    dir: false,
    async: async () => new Uint8Array(),
  };
  const context = {
    JSZip: {
      loadAsync: async () => ({
        files: {
          "project/main.py": archiveEntry,
          "project/my_resource.pyxres": archiveEntry,
        },
      }),
    },
    TextDecoder: class {
      decode() {
        return "";
      }
    },
    base64ToUint8: () => new Uint8Array(),
    resetResourceEditor: () => {},
    resetRuntimeScreen: () => {},
    sanitizeProjectName,
    uint8ToBase64: () => "encoded",
    window,
  };
  const loadProjectFromZip = loadNamedFunction(
    codeMakerSource,
    "loadProjectFromZip",
    context,
  );

  await loadProjectFromZip(new ArrayBuffer(), "..");

  assert.equal(window._project.name, "untitled");
});

test("buildArchiveBlob writes a canonical startup marker", async () => {
  let generatedArchive;
  class FakeJSZip {
    constructor() {
      generatedArchive = this;
      this.files = new Map();
    }

    file(path, content) {
      this.files.set(path, content);
      return this;
    }

    async generateAsync() {
      return "archive";
    }
  }
  const sanitizeProjectName = loadNamedFunction(
    codeMakerSource,
    "sanitizeProjectName",
    {},
  );
  const context = {
    JSZip: FakeJSZip,
    base64ToUint8: (value) => value,
    sanitizeProjectName,
    window: {
      _project: {
        name: "Space Game",
        code: "print('hello')",
        resource: "resource",
        files: { ".pyxapp_startup_script": "old.py" },
      },
    },
  };
  const buildArchiveBlob = loadNamedFunction(
    codeMakerSource,
    "buildArchiveBlob",
    context,
  );

  await buildArchiveBlob();

  assert.equal(
    generatedArchive.files.get("Space Game/.pyxapp_startup_script"),
    "main.py",
  );
});

test("buildArchiveBlob sanitizes its archive root defensively", async () => {
  let generatedArchive;
  class FakeJSZip {
    constructor() {
      generatedArchive = this;
      this.files = new Map();
    }

    file(path, content) {
      this.files.set(path, content);
      return this;
    }

    async generateAsync() {
      return "archive";
    }
  }
  const sanitizeProjectName = loadNamedFunction(
    codeMakerSource,
    "sanitizeProjectName",
    {},
  );
  const context = {
    JSZip: FakeJSZip,
    base64ToUint8: (value) => value,
    sanitizeProjectName,
    window: {
      _project: {
        name: "CON.",
        code: "print('hello')",
        resource: "resource",
        files: {},
      },
    },
  };
  const buildArchiveBlob = loadNamedFunction(
    codeMakerSource,
    "buildArchiveBlob",
    context,
  );

  await buildArchiveBlob();

  assert.equal(context.window._project.name, "_CON");
  assert.equal(
    generatedArchive.files.get("_CON/.pyxapp_startup_script"),
    "main.py",
  );
});

test("loadFromUrl derives the project name from the URL pathname", async () => {
  const loadedNames = [];
  const sanitizeProjectName = loadNamedFunction(
    codeMakerSource,
    "sanitizeProjectName",
    {},
  );
  const context = {
    fetch: async () => ({
      ok: true,
      arrayBuffer: async () => new ArrayBuffer(0),
    }),
    loadProjectFromZip: async (_buffer, name) => {
      loadedNames.push(name);
    },
    updateShareUrl: () => {},
    sanitizeProjectName,
    URL,
  };
  const loadFromUrl = loadNamedFunction(
    codeMakerSource,
    "loadFromUrl",
    context,
  );

  await loadFromUrl(
    "https://example.test/downloads/Space%20Game.zip?token=abc#download",
  );
  await loadFromUrl("https://example.test/downloads/Bad%ZZ.zip");
  await loadFromUrl("https://example.test/downloads/Foo%2FBar.zip");
  await loadFromUrl("https://example.test/downloads/%5Cevil.zip");
  await loadFromUrl("https://example.test/downloads/%2E%2E.zip");
  await loadFromUrl("https://example.test/downloads/Bad%00Name.zip");

  assert.deepEqual(loadedNames, [
    "Space Game",
    "Bad%ZZ",
    "Foo_Bar",
    "_evil",
    "untitled",
    "Bad_Name",
  ]);
});

test("Code Maker reports a missing starter project without loading it", async () => {
  let arrayBufferCount = 0;
  let loadCount = 0;
  let initialize;
  const errors = [];
  const context = {
    fetch: async () => ({
      ok: false,
      status: 500,
      arrayBuffer: async () => {
        arrayBufferCount += 1;
      },
    }),
    loadProjectFromZip: async () => {
      loadCount += 1;
    },
    window: {
      _codeEditor: { focus: () => {} },
      addEventListener: (_name, callback) => {
        initialize = callback;
      },
    },
    initAceEditor() {},
    setupButtonHandlers() {},
    setupSplitter() {},
    alert: (message) => errors.push(message),
  };
  context.loadInitialFiles = loadNamedFunction(
    codeMakerSource,
    "loadInitialFiles",
    context,
  );
  const start = codeMakerSource.lastIndexOf(
    'window.addEventListener("DOMContentLoaded",',
  );
  assert.notEqual(start, -1);
  const end = codeMakerSource.indexOf("</script>", start);
  vm.runInNewContext(codeMakerSource.slice(start, end), context);

  await initialize();
  assert.deepEqual(errors, ["Load failed: Starter project not found"]);
  assert.equal(arrayBufferCount, 0);
  assert.equal(loadCount, 0);
});

for (const [name, status] of [
  ["pyxel wheel", 503],
  ["import_hook.py", 404],
]) {
  test(`_fetchAsset rejects a non-success ${name} response`, async () => {
    const context = {
      fetch: async () => ({ ok: false, status }),
    };
    const fetchAsset = loadArrowFunction(pyxelSource, "_fetchAsset", context);

    await assert.rejects(fetchAsset(`https://example.com/${name}`, name), {
      name: "Error",
      message: `Failed to fetch ${name}: ${status}`,
    });
  });
}
