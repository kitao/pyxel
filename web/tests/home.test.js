const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const source = fs.readFileSync(
  path.join(__dirname, "..", "index.html"),
  "utf8",
);
const scripts = [...source.matchAll(/<script>([\s\S]*?)<\/script>/g)].map(
  (match) => match[1],
);

const loadHome = ({
  url = "https://example.test/pyxel/",
  saved = null,
  languages = ["en-US"],
  language = languages?.[0],
  storageBlocked = false,
} = {}) => {
  const storage = new Map(saved ? [["pyxel-lang", saved]] : []);
  const buttons = Object.fromEntries(
    ["en", "ja"].map((code) => [
      code,
      {
        dataset: { language: code },
        setAttribute(name, value) {
          assert.equal(name, "aria-pressed");
          this.pressed = String(value);
        },
        addEventListener(type, listener) {
          assert.equal(type, "click");
          this.click = listener;
        },
      },
    ]),
  );
  const context = {
    URL,
    URLSearchParams,
    location: new URL(url),
    navigator: { languages, language },
    localStorage: {
      getItem(key) {
        if (storageBlocked) throw new Error("Storage blocked");
        return storage.get(key) ?? null;
      },
      setItem(key, value) {
        if (storageBlocked) throw new Error("Storage blocked");
        storage.set(key, value);
      },
    },
    document: {
      documentElement: { lang: "en" },
      title: source.match(/<title>(.*?)<\/title>/)[1],
      querySelectorAll: () => Object.values(buttons),
    },
    history: {
      replaceState(_state, _title, next) {
        context.location = new URL(next);
      },
    },
  };
  vm.runInNewContext(scripts.join("\n"), context);
  return { context, buttons, storage };
};

test("home automatically chooses explicit, saved, then supported browser language", () => {
  for (const [options, expected] of [
    [{ languages: ["ja-JP", "en-US"] }, "ja"],
    [{ languages: ["en-US", "ja-JP"] }, "en"],
    [{ languages: ["fr", "ja-JP", "en"] }, "ja"],
    [{ languages: ["fr", "de"] }, "en"],
    [{ languages: [], language: "ja-JP" }, "ja"],
    [{ saved: "en", languages: ["ja-JP"] }, "en"],
    [{ saved: "fr", languages: ["ja-JP"] }, "ja"],
    [{ url: "https://example.test/pyxel/?lang=ja", saved: "en" }, "ja"],
    [{ url: "https://example.test/pyxel/?lang=en", saved: "ja" }, "en"],
  ]) {
    const { context, buttons } = loadHome(options);
    assert.equal(context.document.documentElement.lang, expected);
    assert.equal(buttons[expected].pressed, "true");
    assert.equal(buttons[expected === "en" ? "ja" : "en"].pressed, "false");
    assert.equal(context.document.title, "Pyxel");
  }
});

test("home can initialize and switch language when browser storage is blocked", () => {
  const url = "https://example.test/pyxel/?source=guide#examples";
  const { context, buttons } = loadHome({
    url,
    languages: ["ja-JP"],
    storageBlocked: true,
  });
  assert.equal(context.document.documentElement.lang, "ja");
  assert.equal(context.document.title, "Pyxel");
  assert.equal(context.location.href, url);
  buttons.en.click();
  assert.equal(context.document.documentElement.lang, "en");
  assert.equal(buttons.en.pressed, "true");
  assert.equal(buttons.ja.pressed, "false");
  assert.equal(context.document.title, "Pyxel");
  assert.equal(
    context.location.href,
    "https://example.test/pyxel/?source=guide&lang=en#examples",
  );
});

test("home language selection persists without changing its title or other URL parts", () => {
  const { context, buttons, storage } = loadHome({
    url: "https://example.test/pyxel/?source=guide#examples",
  });
  buttons.ja.click();
  assert.equal(context.document.documentElement.lang, "ja");
  assert.equal(buttons.ja.pressed, "true");
  assert.equal(buttons.en.pressed, "false");
  assert.equal(context.document.title, "Pyxel");
  assert.equal(
    context.location.href,
    "https://example.test/pyxel/?source=guide&lang=ja#examples",
  );
  assert.equal(storage.get("pyxel-lang"), "ja");
  assert.equal(
    loadHome({ saved: storage.get("pyxel-lang") }).buttons.ja.pressed,
    "true",
  );
});
