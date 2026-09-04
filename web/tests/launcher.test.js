const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const { loadArrowFunction, loadNamedFunction } = require("./source-loader");

const launcherSource = fs.readFileSync(
  path.join(__dirname, "..", "launcher", "index.html"),
  "utf8",
);
const urlBuilderSource = fs.readFileSync(
  path.join(__dirname, "..", "launcher", "url-builder.html"),
  "utf8",
);

test("launcher consumes launch failures through the fatal overlay", async () => {
  const failure = new Error("launch failed");
  const errors = [];
  vm.runInNewContext(launcherSource.match(/<script>([\s\S]*?)<\/script>/)[1], {
    URL,
    document: { location: "https://example.test/?play=owner.repo.apps.game" },
    location: { replace: () => assert.fail("unexpected redirect") },
    launchPyxel: async () => {
      throw failure;
    },
    _displayFatalErrorOverlay: (error) => errors.push(error),
  });
  await new Promise((resolve) => setImmediate(resolve));

  assert.deepEqual(errors, [failure]);
});

test("getFileExt ignores GitHub query strings and line fragments", () => {
  const getFileExt = loadNamedFunction(urlBuilderSource, "getFileExt", {
    URL,
  });

  assert.equal(
    getFileExt("https://github.com/example/game/blob/main/demo.py?plain=1"),
    "py",
  );
  assert.equal(
    getFileExt("https://github.com/example/game/blob/main/demo.py#L20"),
    "py",
  );
});

test("createLaunchUrl preserves reserved path characters", () => {
  const createLaunchUrl = loadArrowFunction(
    urlBuilderSource,
    "createLaunchUrl",
    { URL },
  );
  const target =
    "example/game/0123456789abcdef0123456789abcdef01234567/apps/demo&debug#fragment";

  const result = new URL(
    createLaunchUrl(
      "https://example.test/web/launcher/",
      "run",
      target,
      "image",
      true,
    ),
  );

  assert.equal(result.searchParams.get("run"), target);
  assert.equal(result.searchParams.get("gamepad"), "enabled");
  assert.deepEqual([...result.searchParams.keys()], ["run", "gamepad"]);
});

test("createLaunchUrl records an explicit slash-ref boundary", () => {
  const createLaunchUrl = loadArrowFunction(
    urlBuilderSource,
    "createLaunchUrl",
    { URL },
  );
  const target = "example/game/feature/physics/apps/demo";

  const result = new URL(
    createLaunchUrl(
      "https://example.test/web/launcher/",
      "run",
      target,
      "image",
      false,
      "feature/physics",
      "0123456789abcdef0123456789abcdef01234567",
    ),
  );

  assert.equal(result.searchParams.get("run"), target);
  assert.equal(result.searchParams.get("ref"), "feature/physics");
  assert.equal(
    result.searchParams.get("sha"),
    "0123456789abcdef0123456789abcdef01234567",
  );
});

test("createLaunchUrl emits only editor options for resources", () => {
  const createLaunchUrl = loadArrowFunction(
    urlBuilderSource,
    "createLaunchUrl",
    { URL },
  );

  const result = new URL(
    createLaunchUrl(
      "https://example.test/web/launcher/",
      "edit",
      "example/game/0123456789abcdef0123456789abcdef01234567/assets/sample",
      "tilemap",
      true,
    ),
  );

  assert.equal(result.searchParams.get("editor"), "tilemap");
  assert.equal(result.searchParams.has("gamepad"), false);
});

test("URL Builder retains the newest source ref and clears stale resolutions", async () => {
  const pending = [];
  const controls = Object.fromEntries(
    ["startup-file", "virtual-gamepad", "editor-screen", "launch-url"].map(
      (id) => [
        id,
        {
          value: "",
          checked: false,
          addEventListener() {},
          removeAttribute(name) {
            delete this[name];
          },
        },
      ],
    ),
  );
  const context = {
    URL,
    document: { getElementById: (id) => controls[id] },
    location: { href: "https://example.test/web/launcher/url-builder.html" },
    resolveGitHubBlobUrl: () => new Promise((resolve) => pending.push(resolve)),
    initPage() {},
  };
  vm.runInNewContext(
    urlBuilderSource.match(/<script>([\s\S]*?)<\/script>/)[1],
    context,
  );
  const target = (name) => ({
    user: "example",
    repo: "game",
    ref: "feature/physics",
    sha: "0123456789abcdef0123456789abcdef01234567",
    path: `${name}.py`,
  });
  controls["startup-file"].value =
    "https://github.com/example/game/blob/main/old.py";
  const old = context.buildLaunchUrl();
  controls["startup-file"].value =
    "https://github.com/example/game/blob/main/new.py";
  const newer = context.buildLaunchUrl();
  pending[1](target("new"));
  await newer;
  const newestUrl = controls["launch-url"].href;
  const url = new URL(newestUrl);
  assert.equal(url.searchParams.get("run"), "example/game/feature/physics/new");
  assert.equal(url.searchParams.get("ref"), "feature/physics");
  assert.equal(url.searchParams.get("sha"), target("new").sha);
  pending[0](target("old"));
  await old;
  assert.equal(controls["launch-url"].href, newestUrl);

  const stale = context.buildLaunchUrl();
  controls["startup-file"].value = "";
  await context.buildLaunchUrl();
  pending[2](target("old"));
  await stale;
  assert.equal(controls["launch-url"].href, undefined);
  assert.equal(controls["launch-url"].textContent, "");
});

test("launcher preserves dotted archive paths and options without a ref lookup", async () => {
  const launched = [];
  vm.runInNewContext(launcherSource.match(/<script>([\s\S]*?)<\/script>/)[1], {
    URL,
    document: {
      location:
        "https://example.test/?play=owner.repo.apps.game&gamepad=enabled",
    },
    location: { replace: () => assert.fail("unexpected redirect") },
    fetch: () => assert.fail("unexpected ref lookup"),
    launchPyxel: (params) => launched.push({ ...params }),
    _displayFatalErrorOverlay: (error) => {
      throw error;
    },
  });
  await new Promise((resolve) => setImmediate(resolve));

  assert.deepEqual(launched, [
    {
      root: "https://cdn.jsdelivr.net/gh/owner/repo",
      command: "play",
      name: "apps/game.pyxapp",
      packages: null,
      gamepad: "enabled",
      editor: null,
    },
  ]);
});

test("resolveLaunchTarget preserves a legacy ref when a longer tag exists", async () => {
  const requested = [];
  const mainSha = "0123456789abcdef0123456789abcdef01234567";
  const tagSha = "89abcdef0123456789abcdef0123456789abcdef";
  const context = {
    fetch: async (url) => {
      requested.push(url);
      return {
        ok:
          url.endsWith("/commits/main") || url.endsWith("/commits/main%2Fsrc"),
        text: async () =>
          url.endsWith("/commits/main%2Fsrc") ? tagSha : mainSha,
      };
    },
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  const result = await resolveLaunchTarget([
    "example",
    "game",
    "main",
    "src",
    "demo",
  ]);

  assert.deepEqual(
    { ...result },
    {
      ref: mainSha,
      path: "src/demo",
    },
  );
  assert.deepEqual(requested, [
    "https://api.github.com/repos/example/game/commits/main",
  ]);
});

test("resolveLaunchTarget falls back to the longest slash-containing ref", async () => {
  const requested = [];
  const context = {
    fetch: async (url) => {
      requested.push(url);
      const ok = url.endsWith("/commits/feature%2Fphysics");
      return {
        ok,
        status: ok ? 200 : 422,
        text: async () =>
          ok
            ? "0123456789abcdef0123456789abcdef01234567"
            : JSON.stringify({
                message: `No commit found for SHA: ${url.split("/commits/")[1]}`,
              }),
      };
    },
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  const result = await resolveLaunchTarget([
    "example",
    "game",
    "feature",
    "physics",
    "apps",
    "demo",
  ]);

  assert.deepEqual(
    { ...result },
    {
      ref: "0123456789abcdef0123456789abcdef01234567",
      path: "apps/demo",
    },
  );
  assert.deepEqual(requested, [
    "https://api.github.com/repos/example/game/commits/feature",
    "https://api.github.com/repos/example/game/commits/feature%2Fphysics%2Fapps",
    "https://api.github.com/repos/example/game/commits/feature%2Fphysics",
  ]);
});

test("resolveLaunchTarget honors an explicit slash-ref boundary", async () => {
  const requested = [];
  const sha = "0123456789abcdef0123456789abcdef01234567";
  const context = {
    fetch: async (url) => {
      requested.push(url);
      return {
        ok: true,
        status: 200,
        text: async () => sha,
      };
    },
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  const result = await resolveLaunchTarget(
    ["example", "game", "feature", "physics", "apps", "demo"],
    "feature/physics",
  );

  assert.deepEqual({ ...result }, { ref: sha, path: "apps/demo" });
  assert.deepEqual(requested, [
    "https://api.github.com/repos/example/game/commits/feature%2Fphysics",
  ]);
});

test("resolveLaunchTarget uses a recorded SHA when GitHub is unavailable", async () => {
  const context = {
    fetch: async () => ({ ok: false, status: 429 }),
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  const result = await resolveLaunchTarget(
    ["example", "game", "feature", "physics", "apps", "demo"],
    "feature/physics",
    "0123456789abcdef0123456789abcdef01234567",
  );

  assert.deepEqual(
    { ...result },
    {
      ref: "0123456789abcdef0123456789abcdef01234567",
      path: "apps/demo",
    },
  );
});

test("resolveLaunchTarget rejects an unresolved explicit slash ref", async () => {
  const context = {
    fetch: async () => ({ ok: false, status: 429 }),
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  await assert.rejects(
    resolveLaunchTarget(
      ["example", "game", "feature", "physics", "apps", "demo"],
      "feature/physics",
    ),
    { name: "Error", message: "Failed to resolve the GitHub ref" },
  );
});

test("resolveLaunchTarget stops after an unrelated validation error", async () => {
  const requested = [];
  const context = {
    fetch: async (url) => {
      requested.push(url);
      return {
        ok: false,
        status: 422,
        text: async () => JSON.stringify({ message: "Validation Failed" }),
      };
    },
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  const result = await resolveLaunchTarget([
    "example",
    "game",
    "feature",
    "physics",
    "apps",
    "demo",
  ]);

  assert.deepEqual(
    { ...result },
    {
      ref: "feature",
      path: "physics/apps/demo",
    },
  );
  assert.equal(requested.length, 1);
});

test("resolveLaunchTarget stops slash probing when GitHub API is unavailable", async () => {
  for (const status of [403, 429, 500]) {
    const requested = [];
    const context = {
      fetch: async (url) => {
        requested.push(url);
        return { ok: false, status };
      },
    };
    loadNamedFunction(launcherSource, "resolveRefSha", context);
    const resolveLaunchTarget = loadNamedFunction(
      launcherSource,
      "resolveLaunchTarget",
      context,
    );

    const result = await resolveLaunchTarget([
      "example",
      "game",
      "main",
      "apps",
      "demo",
    ]);

    assert.deepEqual(
      { ...result },
      {
        ref: "main",
        path: "apps/demo",
      },
    );
    assert.equal(requested.length, 1, `status ${status}`);
  }
});

test("resolveLaunchTarget stops slash probing after a network error", async () => {
  const requested = [];
  const context = {
    fetch: async (url) => {
      requested.push(url);
      throw new Error("network unavailable");
    },
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  const result = await resolveLaunchTarget([
    "example",
    "game",
    "main",
    "apps",
    "demo",
  ]);

  assert.deepEqual(
    { ...result },
    {
      ref: "main",
      path: "apps/demo",
    },
  );
  assert.equal(requested.length, 1);
});

test("resolveLaunchTarget accepts an immutable SHA without a network request", async () => {
  let fetchCount = 0;
  const context = {
    fetch: async () => {
      fetchCount += 1;
      throw new Error("unexpected network request");
    },
  };
  loadNamedFunction(launcherSource, "resolveRefSha", context);
  const resolveLaunchTarget = loadNamedFunction(
    launcherSource,
    "resolveLaunchTarget",
    context,
  );

  const result = await resolveLaunchTarget([
    "example",
    "game",
    "0123456789abcdef0123456789abcdef01234567",
    "apps",
    "demo",
  ]);

  assert.deepEqual(
    { ...result },
    {
      ref: "0123456789abcdef0123456789abcdef01234567",
      path: "apps/demo",
    },
  );
  assert.equal(fetchCount, 0);
});

test("resolveRefSha accepts only a raw 40-character GitHub SHA", async () => {
  let requestOptions;
  const context = {
    fetch: async (_url, options) => {
      requestOptions = options;
      return {
        ok: true,
        text: async () => '{"sha":"not-a-raw-sha-response"}',
      };
    },
  };
  const resolveRefSha = loadNamedFunction(
    launcherSource,
    "resolveRefSha",
    context,
  );

  assert.equal(await resolveRefSha("example", "game", "main"), undefined);
  assert.equal(requestOptions.headers.Accept, "application/vnd.github.sha");
  assert.equal(requestOptions.cache, "no-cache");
});
