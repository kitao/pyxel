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
  const languageSwitch = {
    hidden: true,
    value: "",
    addEventListener(type, listener) {
      assert.equal(type, "change");
      this.change = (language) => {
        this.value = language;
        listener();
      };
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
  return { context, languageSwitch, storage };
};

test("home automatically chooses explicit, saved, then supported browser language", () => {
  const shared = (language) => ({ "kitao.dev.language": language });
  for (const [options, expected] of [
    [{ languages: ["ja-JP", "en-US"] }, "ja"],
    [{ languages: ["en-US", "ja-JP"] }, "en"],
    [{ languages: ["fr", "ja-JP", "en"] }, "fr"],
    [{ languages: ["ar", "de-DE"] }, "de"],
    [{ languages: ["ar", "vi"] }, "en"],
    [{ languages: ["zh-TW"] }, "cn"],
    [{ languages: [], language: "ja-JP" }, "ja"],
    [{ saved: shared("en"), languages: ["ja-JP"] }, "en"],
    [{ saved: shared("invalid"), languages: ["ja-JP"] }, "ja"],
    [{ saved: { "pyxel-lang": "ja" } }, "ja"],
    [{ saved: { ...shared("en"), "pyxel-lang": "ja" } }, "en"],
    [{ saved: { ...shared("ja"), "pyxel-lang": "fr" } }, "fr"],
    [{ url: "https://example.test/pyxel/?lang=zh-CN" }, "cn"],
    [{ url: "https://example.test/pyxel/?lang=cn" }, "cn"],
    [{ url: "https://example.test/pyxel/?lang=KO-kr" }, "ko"],
    [
      { url: "https://example.test/pyxel/?lang=invalid", saved: shared("ja") },
      "ja",
    ],
    [{ url: "https://example.test/pyxel/?lang=ja", saved: shared("en") }, "ja"],
    [{ url: "https://example.test/pyxel/?lang=en", saved: shared("ja") }, "en"],
  ]) {
    const { context, languageSwitch } = loadHome(options);
    assert.equal(
      context.document.documentElement.lang,
      expected === "cn" ? "zh" : expected,
    );
    assert.equal(languageSwitch.value, expected);
  }
});

test("home language switch is revealed by its script", () => {
  assert.equal(loadHome().languageSwitch.hidden, false);
});

test("home can initialize and switch language when browser storage is blocked", () => {
  const url = "https://example.test/pyxel/?source=guide#examples";
  const { context, languageSwitch } = loadHome({
    url,
    languages: ["ja-JP"],
    storageBlocked: true,
  });
  assert.equal(context.document.documentElement.lang, "ja");
  assert.equal(context.location.href, url);
  languageSwitch.change("en");
  assert.equal(context.document.documentElement.lang, "en");
  assert.equal(languageSwitch.value, "en");
  assert.equal(
    context.location.href,
    "https://example.test/pyxel/?source=guide&lang=en#examples",
  );
});

test("home language selection persists without changing its title or other URL parts", () => {
  const { context, languageSwitch, storage } = loadHome({
    url: "https://example.test/pyxel/?source=guide#examples",
  });
  languageSwitch.change("ja");
  assert.equal(context.document.documentElement.lang, "ja");
  assert.equal(languageSwitch.value, "ja");
  assert.equal(context.document.title, "Pyxel");
  assert.equal(
    context.location.href,
    "https://example.test/pyxel/?source=guide&lang=ja#examples",
  );
  assert.deepEqual(Object.fromEntries(storage), {
    "kitao.dev.language": "ja",
    "pyxel-lang": "ja",
  });
  assert.equal(
    loadHome({ saved: Object.fromEntries(storage) }).languageSwitch.value,
    "ja",
  );
});

test("home offers the guide's languages and remembers every Pyxel selection", () => {
  const guide = JSON.parse(
    fs.readFileSync(
      path.join(__dirname, "..", "user-guide", "user-guide.json"),
      "utf8",
    ),
  );
  const options = [
    ...source.matchAll(
      /<option value="([^"]+)" lang="([^"]+)">([^<]+)<\/option>/g,
    ),
  ];
  assert.deepEqual(
    options.map(([, code, , name]) => ({ code, name })),
    guide.languages,
  );
  assert.deepEqual(
    [...source.matchAll(/<div lang="([^"]+)">/g)].map(([, tag]) => tag),
    options.map(([, , tag]) => tag),
  );

  const { context, languageSwitch, storage } = loadHome();
  for (const { code } of guide.languages) {
    languageSwitch.change(code);
    assert.equal(
      context.document.documentElement.lang,
      code === "cn" ? "zh" : code,
    );
    assert.equal(new URL(context.location.href).searchParams.get("lang"), code);
    assert.equal(storage.get("pyxel-lang"), code);
    assert.equal(
      loadHome({ saved: Object.fromEntries(storage) }).languageSwitch.value,
      code,
    );
  }
});

test("home Pyxel-only selections preserve the shared site's language", () => {
  const { languageSwitch, storage } = loadHome({
    saved: { "kitao.dev.language": "ja" },
  });
  languageSwitch.change("cn");
  assert.equal(storage.get("kitao.dev.language"), "ja");
  assert.equal(storage.get("pyxel-lang"), "cn");
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
