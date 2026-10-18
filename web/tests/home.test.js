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
const title = source.match(/<title>(.*?)<\/title>/)[1];

const attributes = (tag) =>
  Object.fromEntries(
    [...tag.matchAll(/([\w:-]+)="([^"]*)"/g)].map((match) => match.slice(1)),
  );

const images = [...source.matchAll(/<img\b[^>]*>/g)].map(([tag]) =>
  attributes(tag),
);
const meta = Object.fromEntries(
  [...source.matchAll(/<meta\b[^>]*>/g)]
    .map(([tag]) => attributes(tag))
    .filter((tag) => "content" in tag)
    .map((tag) => [tag.property ?? tag.name, tag.content]),
);
// PNG stores big-endian sizes in its header; GIF stores little-endian ones.
const imageSize = (file) => {
  const bytes = fs.readFileSync(file);
  const signature = bytes.toString("latin1", 0, 4);
  if (signature === "\x89PNG") {
    return [bytes.readUInt32BE(16), bytes.readUInt32BE(20)];
  }
  assert.equal(signature, "GIF8", file);
  return [bytes.readUInt16LE(6), bytes.readUInt16LE(8)];
};

const loadHome = ({
  url = "https://example.test/pyxel/",
  saved = {},
  languages = ["en-US"],
  language = languages?.[0],
  storageBlocked = false,
} = {}) => {
  const storage = new Map(Object.entries(saved));
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
  const languageSwitch = {
    hidden: true,
    querySelectorAll(selector) {
      assert.equal(selector, "button");
      return Object.values(buttons);
    },
  };
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
      title,
      querySelector(selector) {
        assert.equal(selector, ".language-switch");
        return languageSwitch;
      },
    },
    history: {
      replaceState(_state, _title, next) {
        context.location = new URL(next);
      },
    },
  };
  vm.runInNewContext(scripts.join("\n"), context);
  return { context, languageSwitch, buttons, storage };
};

test("home automatically chooses explicit, saved, then supported browser language", () => {
  const shared = (language) => ({ "kitao.dev.language": language });
  for (const [options, expected] of [
    [{ languages: ["ja-JP", "en-US"] }, "ja"],
    [{ languages: ["en-US", "ja-JP"] }, "en"],
    [{ languages: ["fr", "ja-JP", "en"] }, "ja"],
    [{ languages: ["fr", "de"] }, "en"],
    [{ languages: [], language: "ja-JP" }, "ja"],
    [{ saved: shared("en"), languages: ["ja-JP"] }, "en"],
    [{ saved: shared("fr"), languages: ["ja-JP"] }, "ja"],
    [{ saved: { "pyxel-lang": "ja" } }, "ja"],
    [{ saved: { ...shared("en"), "pyxel-lang": "ja" } }, "en"],
    [{ url: "https://example.test/pyxel/?lang=ja", saved: shared("en") }, "ja"],
    [{ url: "https://example.test/pyxel/?lang=en", saved: shared("ja") }, "en"],
  ]) {
    const { context, buttons } = loadHome(options);
    assert.equal(context.document.documentElement.lang, expected);
    assert.equal(buttons[expected].pressed, "true");
    assert.equal(buttons[expected === "en" ? "ja" : "en"].pressed, "false");
  }
});

test("home language switch is revealed by its script", () => {
  assert.equal(loadHome().languageSwitch.hidden, false);
});

test("home can initialize and switch language when browser storage is blocked", () => {
  const url = "https://example.test/pyxel/?source=guide#examples";
  const { context, buttons } = loadHome({
    url,
    languages: ["ja-JP"],
    storageBlocked: true,
  });
  assert.equal(context.document.documentElement.lang, "ja");
  assert.equal(context.location.href, url);
  buttons.en.click();
  assert.equal(context.document.documentElement.lang, "en");
  assert.equal(buttons.en.pressed, "true");
  assert.equal(buttons.ja.pressed, "false");
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
  assert.deepEqual(Object.fromEntries(storage), { "kitao.dev.language": "ja" });
  assert.equal(
    loadHome({ saved: Object.fromEntries(storage) }).buttons.ja.pressed,
    "true",
  );
});

test("home link preview matches the page and its published image", () => {
  assert.equal(meta["og:title"], title);
  assert.equal(meta["og:description"], meta.description);
  const preview = new URL(meta["og:image"]);
  assert.equal(preview.origin, new URL(meta["og:url"]).origin);
  // The site build publishes docs/images/ at /pyxel/assets/images/.
  const [, name] =
    preview.pathname.match(/^\/pyxel\/assets\/images\/(.+)$/) ?? [];
  assert.ok(name, preview.pathname);
  assert.deepEqual(
    imageSize(path.join(__dirname, "..", "..", "docs", "images", name)),
    [Number(meta["og:image:width"]), Number(meta["og:image:height"])],
  );
});

test("home image dimensions keep each local image's aspect ratio", () => {
  // Width and height reserve the space before the image loads.
  for (const image of images.filter((image) => !/^https:/.test(image.src))) {
    const [width, height] = imageSize(path.join(__dirname, "..", image.src));
    assert.equal(
      Number(image.height),
      Math.round((Number(image.width) * height) / width),
      image.src,
    );
  }
});
