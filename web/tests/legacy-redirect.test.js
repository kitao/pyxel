const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const root = path.resolve(__dirname, "../..");
const pages = [
  "index.html",
  "api-reference/index.html",
  "api-reference/cube/index.html",
  "code-maker/index.html",
  "code-maker/manual.html",
  "editor-manual/index.html",
  "launcher/index.html",
  "launcher/url-builder.html",
  "mml-studio/index.html",
  "mml-studio/manual.html",
  "mml-studio/mml-commands.html",
  "showcase/index.html",
  "user-guide/index.html",
  "user-guide/cube/index.html",
  "web-usage/index.html",
];

const evaluate = (source, url, embedded = false) => {
  const location = new URL(url);
  const redirects = [];
  location.replace = (target) => redirects.push(new URL(target, location).href);
  const frame = {};
  vm.runInNewContext(source, {
    location,
    window: { self: frame, top: embedded ? {} : frame },
  });
  return redirects;
};

test("old web navigation forwards to canonical paths without changing URL state", () => {
  for (const page of pages) {
    const source = fs
      .readFileSync(path.join(root, "web", page), "utf8")
      .match(/<script data-legacy-redirect>([\s\S]*?)<\/script>/)[1];
    for (const suffix of [
      "",
      "?run=owner.repo.app#section",
      "?run=a%2Fb&x=1+2&x=%2523#%E6%97%A5",
    ]) {
      for (const route of new Set([page, page.replace(/index.html$/, "")])) {
        const old = `https://kitao.github.io/pyxel/web/${route}${suffix}`;
        assert.deepEqual(evaluate(source, old), [
          `https://kitao.dev/pyxel/${route}${suffix}`,
        ]);
        assert.deepEqual(evaluate(source, old, true), []);
        for (const host of [
          "kitao.dev",
          "localhost",
          "preview.example",
          "kitao.github.io.evil.example",
        ]) {
          assert.deepEqual(
            evaluate(source, old.replace("kitao.github.io", host)),
            [],
          );
        }
        assert.deepEqual(
          evaluate(source, old.replace("/pyxel/web/", "/other/web/")),
          [],
        );
      }
    }
  }
});

test("legacy wasm navigation redirects directly but retains local and iframe behavior", () => {
  for (const route of [
    "",
    "api-reference/",
    "api-reference/cube/",
    "code-maker/",
    "editor-manual/",
    "launcher/",
    "mml-studio/",
    "showcase/",
    "user-guide/",
    "user-guide/cube/",
    "launcher/url-builder.html",
    "code-maker/manual.html",
    "mml-studio/manual.html",
    "mml-studio/mml-commands.html",
  ]) {
    const source = fs
      .readFileSync(
        path.join(
          root,
          "wasm",
          route.endsWith(".html") ? route : route + "index.html",
        ),
        "utf8",
      )
      .match(/<script>([\s\S]*?)<\/script>/)[1];
    const suffix = "?run=a%2Fb&x=1+2#%E6%97%A5";
    const old = `https://kitao.github.io/pyxel/wasm/${route}${suffix}`;
    assert.deepEqual(evaluate(source, old), [
      `https://kitao.dev/pyxel/${route}${suffix}`,
    ]);
    assert.deepEqual(evaluate(source, old, true), [
      `https://kitao.github.io/pyxel/web/${route}${suffix}`,
    ]);
    assert.deepEqual(
      evaluate(source, old.replace("kitao.github.io", "localhost")),
      [`https://localhost/pyxel/web/${route}${suffix}`],
    );
  }
});
