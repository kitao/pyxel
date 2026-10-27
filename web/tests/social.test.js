const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const web = path.join(__dirname, "..");
const directories = [
  "api-reference",
  "code-maker",
  "editor-manual",
  "launcher",
  "mml-studio",
  "showcase",
  "user-guide",
  "web-usage",
];
const embedded = new Set([
  "code-maker/pyxel-editor.html",
  "code-maker/pyxel-screen.html",
  "mml-studio/pyxel-screen.html",
]);

test("standalone subpages have static metadata and share the correctly sized Pyxel logo card", () => {
  const image = fs.readFileSync(
    path.join(web, "..", "docs/images/pyxel_og.png"),
  );
  assert.equal(image.toString("latin1", 0, 8), "\x89PNG\r\n\x1a\n");
  const size = [image.readUInt32BE(16), image.readUInt32BE(20)];
  const pages = directories
    .flatMap((directory) =>
      fs
        .readdirSync(path.join(web, directory), { recursive: true })
        .filter((name) => name.endsWith(".html"))
        .map((name) => `${directory}/${name.split(path.sep).join("/")}`),
    )
    .filter(
      (name) =>
        !embedded.has(name) && !/^showcase\/(apps|examples|tools)\//.test(name),
    );
  for (const name of pages) {
    const head = fs
      .readFileSync(path.join(web, name), "utf8")
      .split("</head>")[0];
    const meta = {};
    for (const [tag] of head.matchAll(/<meta\b[^>]*>/g)) {
      const attributes = readAttributes(tag);
      const key = attributes.property ?? attributes.name;
      if (!key) continue;
      assert.ok(!(key in meta), `${name}: duplicate ${key}`);
      meta[key] = attributes.content;
    }
    for (const key of ["og:title", "og:description", "og:image:alt"])
      assert.ok(meta[key], `${name}: missing ${key}`);
    assert.equal(meta.description, meta["og:description"], name);
    assert.equal(meta["og:type"], "website", name);
    assert.equal(meta["twitter:card"], "summary_large_image", name);
    const suffix = name.endsWith("/index.html")
      ? name.slice(0, -"index.html".length)
      : name.slice(0, -".html".length);
    const url = `https://kitao.dev/pyxel/${suffix}`;
    assert.equal(meta["og:url"], url, name);
    const canonical = [...head.matchAll(/<link\b[^>]*>/g)]
      .map(([tag]) => readAttributes(tag))
      .filter((attributes) => attributes.rel === "canonical");
    assert.deepEqual(
      canonical.map((attributes) => attributes.href),
      [url],
      name,
    );
    assert.equal(
      meta["og:image"],
      "https://kitao.dev/pyxel/assets/images/pyxel_og.png",
      name,
    );
    assert.deepEqual(
      [Number(meta["og:image:width"]), Number(meta["og:image:height"])],
      size,
      name,
    );
  }
});

function readAttributes(tag) {
  return Object.fromEntries(
    [...tag.matchAll(/([\w:-]+)="([^"]*)"/g)].map((match) => match.slice(1)),
  );
}
