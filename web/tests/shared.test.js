const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const { loadArrowFunction } = require("./source-loader");

const source = fs.readFileSync(path.join(__dirname, "..", "shared.js"), "utf8");

const loadShared = (overrides = {}) => {
  const context = {
    atob,
    btoa,
    Uint8Array,
    URL,
    console,
    document: { documentElement: {} },
    fetch,
    localStorage: { getItem: () => null, setItem: () => {} },
    navigator: { language: "en" },
    setTimeout,
    ...overrides,
  };
  vm.runInNewContext(
    `${source}\n;globalThis.__test = { initPage, resolveGitHubBlobUrl, uint8ToBase64, base64ToUint8 };`,
    context,
  );
  return context;
};

const settlePromises = () => new Promise((resolve) => setImmediate(resolve));

const { resolveGitHubBlobUrl } = loadShared().__test;

test("Base64 preserves exact bytes across its chunk boundary", () => {
  const { uint8ToBase64, base64ToUint8 } = loadShared().__test;
  const bytes = Uint8Array.from({ length: 0x8001 }, (_, i) => i % 256);
  const encoded = uint8ToBase64(bytes);

  assert.equal(encoded, Buffer.from(bytes).toString("base64"));
  assert.deepEqual(base64ToUint8(` \n${encoded}\t`), bytes);
  assert.equal(uint8ToBase64(new Uint8Array()), "");
  assert.deepEqual(base64ToUint8(""), new Uint8Array());
});

for (const page of ["api-reference", "user-guide"]) {
  for (const suffix of ["", "index.html", "cube/", "cube/index.html"]) {
    const pathname = `/pyxel/web/${page}/${suffix}`;
    test(`buildVariantSwitch selects the variant and links its sibling at ${pathname}`, () => {
      const buildVariantSwitch = loadArrowFunction(
        source,
        "buildVariantSwitch",
        {
          location: { pathname },
          document: {
            createElement: (tagName) => ({
              tagName,
              children: [],
              appendChild(child) {
                this.children.push(child);
              },
            }),
          },
        },
      );
      const [base, cube] = buildVariantSwitch().children;
      const onCube = suffix.startsWith("cube/");
      const active = onCube ? cube : base;
      const sibling = onCube ? base : cube;

      assert.equal(base.textContent, "Base");
      assert.equal(cube.textContent, "Cube");
      assert.equal(active.tagName, "span");
      assert.equal(sibling.tagName, "a");
      assert.equal(
        new URL(sibling.href, `https://example.test${pathname}`).pathname,
        `/pyxel/web/${page}/${onCube ? "" : "cube/"}`,
      );
    });
  }
}

test("encodeUrlPath preserves separators and escapes each path component", () => {
  assert.equal(
    loadArrowFunction(source, "encodeUrlPath", {})("apps/demo#preview?.zip"),
    "apps/demo%23preview%3F.zip",
  );
});

test("resolveGitHubBlobUrl finds the longest slash-containing ref", async () => {
  const requested = [];
  const fetchImpl = async (url) => {
    requested.push(url);
    const ok = url.endsWith("/commits/feature%2Fphysics");
    return {
      ok,
      status: ok ? 200 : 404,
      text: async () => "0123456789abcdef0123456789abcdef01234567",
    };
  };

  const result = await resolveGitHubBlobUrl(
    "https://github.com/example/game/blob/feature/physics/apps/demo.pyxapp",
    fetchImpl,
  );

  assert.deepEqual(
    { ...result },
    {
      user: "example",
      repo: "game",
      ref: "feature/physics",
      sha: "0123456789abcdef0123456789abcdef01234567",
      path: "apps/demo.pyxapp",
    },
  );
  assert.equal(requested.at(-1).endsWith("/commits/feature%2Fphysics"), true);
});

test("resolveGitHubBlobUrl continues after GitHub rejects a ref candidate", async () => {
  const sha = "0123456789abcdef0123456789abcdef01234567";
  const requested = [];
  const fetchImpl = async (url) => {
    requested.push(url);
    const ok = url.endsWith("/commits/main");
    return {
      ok,
      status: ok ? 200 : 422,
      text: async () =>
        ok
          ? sha
          : JSON.stringify({ message: "No commit found for SHA: main/python" }),
    };
  };

  const result = await resolveGitHubBlobUrl(
    "https://github.com/example/game/blob/main/python/pyproject.toml",
    fetchImpl,
  );

  assert.deepEqual(
    { ...result },
    {
      user: "example",
      repo: "game",
      ref: "main",
      sha,
      path: "python/pyproject.toml",
    },
  );
  assert.deepEqual(
    requested.map((url) => url.split("/commits/")[1]),
    ["main%2Fpython", "main"],
  );
});

test("resolveGitHubBlobUrl stops after an unrelated validation error", async () => {
  let fetchCount = 0;
  await assert.rejects(
    resolveGitHubBlobUrl(
      "https://github.com/example/game/blob/main/python/pyproject.toml",
      async () => {
        fetchCount += 1;
        return {
          ok: false,
          status: 422,
          text: async () => JSON.stringify({ message: "Validation Failed" }),
        };
      },
    ),
    {
      name: "Error",
      message: "Failed to resolve the GitHub ref and file path",
    },
  );
  assert.equal(fetchCount, 1);
});

test("resolveGitHubBlobUrl preserves an established ref boundary", async () => {
  const requested = [];
  const fetchImpl = async (url) => {
    requested.push(url);
    return {
      ok: url.endsWith("/commits/main") || url.endsWith("/commits/main%2Fapps"),
      status: 200,
      text: async () => "0123456789abcdef0123456789abcdef01234567",
    };
  };

  const result = await resolveGitHubBlobUrl(
    "https://github.com/example/game/blob/main/apps/demo.pyxapp",
    fetchImpl,
    "main",
  );

  assert.deepEqual(
    { ...result },
    {
      user: "example",
      repo: "game",
      ref: "main",
      sha: "0123456789abcdef0123456789abcdef01234567",
      path: "apps/demo.pyxapp",
    },
  );
  assert.equal(requested[0].endsWith("/commits/main"), true);
});

test("resolveGitHubBlobUrl preserves a missing explicit ref boundary", async () => {
  for (const status of [404, 422]) {
    const requested = [];
    const fetchImpl = async (url) => {
      requested.push(url);
      const ok = url.endsWith("/commits/feature%2Ffoo");
      return {
        ok,
        status: ok ? 200 : status,
        text: async () =>
          ok
            ? "0123456789abcdef0123456789abcdef01234567"
            : JSON.stringify({ message: "No commit found for SHA: feature" }),
      };
    };

    const result = await resolveGitHubBlobUrl(
      "https://github.com/example/game/blob/feature/foo/apps/demo.pyxapp",
      fetchImpl,
      "feature",
    );

    assert.deepEqual(
      { ...result },
      {
        user: "example",
        repo: "game",
        ref: "feature",
        sha: null,
        path: "foo/apps/demo.pyxapp",
      },
    );
    assert.deepEqual(requested, [
      "https://api.github.com/repos/example/game/commits/feature",
    ]);
  }
});

test("resolveGitHubBlobUrl accepts a commit SHA without an API request", async () => {
  let fetchCount = 0;
  const sha = "0123456789abcdef0123456789abcdef01234567";

  const result = await resolveGitHubBlobUrl(
    `https://github.com/example/game/blob/${sha}/apps/demo.pyxapp`,
    async () => {
      fetchCount += 1;
      return { ok: false, status: 403 };
    },
    sha,
  );

  assert.deepEqual(
    { ...result },
    {
      user: "example",
      repo: "game",
      ref: sha,
      sha,
      path: "apps/demo.pyxapp",
    },
  );
  assert.equal(fetchCount, 0);
});

test("resolveGitHubBlobUrl keeps a preferred ref when the API is unavailable", async () => {
  const requested = [];

  const result = await resolveGitHubBlobUrl(
    "https://github.com/example/game/blob/main/apps/demo.pyxapp",
    async (url) => {
      requested.push(url);
      return { ok: false, status: 403 };
    },
    "main",
  );

  assert.deepEqual(
    { ...result },
    {
      user: "example",
      repo: "game",
      ref: "main",
      sha: null,
      path: "apps/demo.pyxapp",
    },
  );
  assert.deepEqual(
    requested.map((url) => url.split("/commits/")[1]),
    ["main"],
  );
});

test("resolveGitHubBlobUrl keeps a preferred ref when the API body is unreadable", async () => {
  let fetchCount = 0;
  const result = await resolveGitHubBlobUrl(
    "https://github.com/example/game/blob/main/apps/demo.pyxapp",
    async () => {
      fetchCount += 1;
      return {
        ok: true,
        status: 200,
        text: async () => {
          throw new Error("response stream failed");
        },
      };
    },
    "main",
  );

  assert.deepEqual(
    { ...result },
    {
      user: "example",
      repo: "game",
      ref: "main",
      sha: null,
      path: "apps/demo.pyxapp",
    },
  );
  assert.equal(fetchCount, 1);
});

test("resolveGitHubBlobUrl does not decode a failed response body", async () => {
  let textCalls = 0;
  const fetchImpl = async () => ({
    ok: false,
    status: 404,
    text: async () => {
      textCalls += 1;
      throw new Error("failed response body was decoded");
    },
  });

  await assert.rejects(
    resolveGitHubBlobUrl(
      "https://github.com/example/game/blob/missing/apps/demo.py",
      fetchImpl,
    ),
    {
      name: "Error",
      message: "Failed to resolve the GitHub ref and file path",
    },
  );
  assert.equal(textCalls, 0);
});

test("resolveGitHubBlobUrl rejects non-blob GitHub URLs before fetching", async () => {
  let fetchCount = 0;
  await assert.rejects(
    resolveGitHubBlobUrl("https://github.com/example/game", async () => {
      fetchCount += 1;
    }),
    { name: "Error", message: "Invalid GitHub blob URL" },
  );
  assert.equal(fetchCount, 0);
});

test("initPage builds after a successful JSON response", async () => {
  let buildCount = 0;
  const context = loadShared({
    fetch: async () => ({
      ok: true,
      status: 200,
      json: async () => ({ languages: [{ code: "en", name: "English" }] }),
    }),
  });

  context.__test.initPage("data.json", () => {
    buildCount += 1;
  });
  await settlePromises();

  assert.equal(buildCount, 1);
  assert.equal(context.lang, "en");
});

test("initPage rejects an HTTP error before decoding JSON", async () => {
  let jsonCount = 0;
  let buildCount = 0;
  const errors = [];
  const context = loadShared({
    console: { error: (...args) => errors.push(args) },
    fetch: async () => ({
      ok: false,
      status: 503,
      json: async () => {
        jsonCount += 1;
      },
    }),
  });

  context.__test.initPage("data.json", () => {
    buildCount += 1;
  });
  await settlePromises();

  assert.equal(jsonCount, 0);
  assert.equal(buildCount, 0);
  assert.equal(errors[0][1].message, "Failed to fetch data.json: 503");
});

test("initPage reports malformed JSON without building", async () => {
  let buildCount = 0;
  const errors = [];
  const context = loadShared({
    console: { error: (...args) => errors.push(args) },
    fetch: async () => ({
      ok: true,
      status: 200,
      json: async () => {
        throw new SyntaxError("invalid JSON");
      },
    }),
  });

  context.__test.initPage("data.json", () => {
    buildCount += 1;
  });
  await settlePromises();

  assert.equal(buildCount, 0);
  assert.equal(errors[0][1].message, "invalid JSON");
});
