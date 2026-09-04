const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const { loadArrowFunction } = require("./source-loader");

const apiReferenceSource = fs.readFileSync(
  path.join(__dirname, "..", "api-reference", "api-reference.js"),
  "utf8",
);

test("constantMatchesQuery filters constants case-insensitively", () => {
  const constantMatchesQuery = loadArrowFunction(
    apiReferenceSource,
    "constantMatchesQuery",
    {},
  );

  assert.equal(constantMatchesQuery("KEY_A", "key_a"), true);
  assert.equal(constantMatchesQuery("KEY_A", "mouse"), false);
  assert.equal(constantMatchesQuery("KEY_A", ""), true);
});
