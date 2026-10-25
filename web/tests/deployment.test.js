const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const { experimental_readRawConfig } = require("wrangler");

const WEB_DIR = path.join(__dirname, "..");
const ROOT_DIR = path.join(WEB_DIR, "..");

const { rawConfig: config } = experimental_readRawConfig({
  config: path.join(WEB_DIR, "wrangler.jsonc"),
});

// Published sources, read from the build script's path lists and direct web inputs.
const buildSources = () => {
  const script = fs.readFileSync(
    path.join(ROOT_DIR, "scripts", "build_site"),
    "utf8",
  );
  const quoted = (text) =>
    [...text.matchAll(/"([^"]+)"/g)].map((match) => match[1]);
  const pages = [
    ...quoted(script.match(/^PAGE_DIRS = \(([^)]*)\)/m)[1]),
    ...quoted(script.match(/for name in \(\*PAGE_DIRS, ([^)]*)\)/)[1]),
  ];
  return [
    ...pages.map((name) => `web/${name}`),
    `web/${script.match(/"-i",\s*"([^"]+)"/)[1]}`,
    ...[...script.matchAll(/tracked_files\(root, "([^"]+)"\)/g)].map(
      (match) => match[1],
    ),
    ...[...script.matchAll(/root \/ "(web\/(?!node_modules\/)[^"]+)"/g)].map(
      (match) => match[1],
    ),
    ...[
      ...script
        .match(/for source_path, target_path in \(([^]*?)\):/)[1]
        .matchAll(/\("([^"]+)", "[^"]+"\)/g),
    ].map((match) => match[1]),
  ];
};

test("the Pyxel Worker serves only the built site", () => {
  const {
    $schema,
    compatibility_date: compatibilityDate,
    build,
    ...deployment
  } = config;
  assert.equal($schema, "./node_modules/wrangler/config-schema.json");
  assert.match(compatibilityDate, /^\d{4}-\d{2}-\d{2}$/);
  // An exact match also rejects additions such as "main", which would run Worker code.
  assert.deepEqual(deployment, {
    name: "pyxel",
    workers_dev: false,
    preview_urls: false,
    // A pattern ending in * also matches /pyxel with a query string.
    routes: [{ pattern: "kitao.dev/pyxel*", zone_name: "kitao.dev" }],
    assets: { directory: "../dist/site" },
  });
  assert.equal(build.command, "python3 ../scripts/build_site");
});

test("Wrangler watches exactly the sources the site build publishes", () => {
  // Strict deploys check these paths for uncommitted changes, and wrangler dev rebuilds on them.
  const watched = config.build.watch_dir.map((name) =>
    path.relative(ROOT_DIR, path.join(WEB_DIR, name)).split(path.sep).join("/"),
  );
  const covers = (directory, source) =>
    source === directory || source.startsWith(`${directory}/`);
  const sources = buildSources();
  assert.ok(sources.includes("web/index.html") && sources.includes("wasm"));
  // The build script decides what a deploy publishes, so its changes count too.
  assert.ok(watched.includes("scripts/build_site"));
  for (const source of sources) {
    assert.ok(
      watched.some((directory) => covers(directory, source)),
      source,
    );
  }
  for (const directory of watched) {
    assert.ok(fs.existsSync(path.join(ROOT_DIR, directory)), directory);
    assert.ok(
      directory === "scripts/build_site" ||
        sources.some((source) => covers(directory, source)),
      directory,
    );
  }
});
