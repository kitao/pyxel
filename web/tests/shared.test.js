const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const source = fs.readFileSync(path.join(__dirname, "..", "shared.js"), "utf8");

const loadShared = (overrides = {}) => {
  const context = {
    URL,
    console,
    document: { documentElement: {}, addEventListener: () => {} },
    fetch,
    lang: "en",
    localStorage: { getItem: () => null, setItem: () => {} },
    navigator: { language: "en" },
    setTimeout,
    ...overrides,
  };
  vm.runInNewContext(
    `${source}\n;globalThis.__test = { code, handleCodeCopyClick, initPage, resolveGitHubBlobUrl, writeClipboardText };`,
    context,
  );
  return context;
};

const settlePromises = () => new Promise((resolve) => setImmediate(resolve));

const { resolveGitHubBlobUrl } = loadShared().__test;

test("resolveGitHubBlobUrl finds the longest slash-containing ref", async () => {
  const requested = [];
  const fetchImpl = async (url) => {
    requested.push(url);
    return {
      ok: url.endsWith("/commits/feature%2Fphysics"),
      status: 200,
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

test("resolveGitHubBlobUrl does not decode a failed response body", async () => {
  const fetchImpl = async () => ({
    ok: false,
    status: 404,
    text: async () => {
      throw new Error("failed response body was decoded");
    },
  });

  await assert.rejects(
    resolveGitHubBlobUrl(
      "https://github.com/example/game/blob/missing/apps/demo.py",
      fetchImpl,
    ),
    /Failed to resolve the GitHub ref and file path/,
  );
});

test("resolveGitHubBlobUrl rejects non-blob GitHub URLs before fetching", async () => {
  let fetchCount = 0;
  await assert.rejects(
    resolveGitHubBlobUrl("https://github.com/example/game", async () => {
      fetchCount += 1;
    }),
    /Invalid GitHub blob URL/,
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

test("code renders a copy button with localized labels and escaped text", () => {
  const html = loadShared({ lang: "ja" }).__test.code('print("hi")', "python");
  assert.match(html, /class="code-block-wrap"/);
  assert.match(html, /class="code-copy-btn"/);
  assert.match(html, /aria-label="コードをコピー"/);
  assert.match(html, /data-copied-label="コピーしました！"/);
  assert.match(html, /language-python/);
  assert.match(html, /print\(&quot;hi&quot;\)/);
});

test("code copy labels fall back to English", () => {
  const html = loadShared({ lang: "xx" }).__test.code("pyxel", "sh");
  assert.match(html, /aria-label="Copy code"/);
  assert.match(html, /data-copied-label="Copied!"/);
});

test("writeClipboardText uses the async clipboard API when available", async () => {
  let written = null;
  const { writeClipboardText } = loadShared({
    navigator: {
      language: "en",
      clipboard: {
        writeText: async (text) => {
          written = text;
        },
      },
    },
  }).__test;

  assert.equal(await writeClipboardText("pyxel run game.py"), true);
  assert.equal(written, "pyxel run game.py");
});

test("writeClipboardText falls back to execCommand when the API is missing", async () => {
  const calls = [];
  const el = {
    value: "",
    style: {},
    setAttribute: () => {},
    select: () => calls.push("select"),
    remove: () => calls.push("remove"),
  };
  const { writeClipboardText } = loadShared({
    document: {
      documentElement: {},
      addEventListener: () => {},
      createElement: (tag) => {
        calls.push(`create:${tag}`);
        return el;
      },
      body: {
        appendChild: () => calls.push("append"),
      },
      execCommand: (cmd) => {
        calls.push(`exec:${cmd}`);
        return cmd === "copy";
      },
    },
  }).__test;

  assert.equal(await writeClipboardText("text"), true);
  assert.equal(el.value, "text");
  assert.deepEqual(calls, [
    "create:textarea",
    "append",
    "select",
    "exec:copy",
    "remove",
  ]);
});

test("handleCodeCopyClick copies code text and resets its feedback", async () => {
  const written = [];
  const classes = new Set();
  const attrs = {};
  const resetFns = [];
  const btn = {
    classList: {
      add: (name) => classes.add(name),
      remove: (name) => classes.delete(name),
    },
    dataset: { copyLabel: "Copy code", copiedLabel: "Copied!" },
    title: "Copy code",
    setAttribute: (name, value) => {
      attrs[name] = value;
    },
    parentElement: {
      querySelector: (selector) =>
        selector === "pre code" ? { textContent: "pyxel run game.py" } : null,
    },
  };
  const { handleCodeCopyClick } = loadShared({
    navigator: {
      language: "en",
      clipboard: {
        writeText: async (text) => {
          written.push(text);
        },
      },
    },
    setTimeout: (fn) => {
      resetFns.push(fn);
      return 0;
    },
  }).__test;

  await handleCodeCopyClick({ target: { closest: () => btn } });
  assert.deepEqual(written, ["pyxel run game.py"]);
  assert.equal(classes.has("copied"), true);
  assert.equal(btn.title, "Copied!");
  assert.equal(attrs["aria-label"], "Copied!");

  resetFns[0]();
  assert.equal(classes.has("copied"), false);
  assert.equal(btn.title, "Copy code");
  assert.equal(attrs["aria-label"], "Copy code");
});
