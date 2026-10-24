const { execFileSync } = require("node:child_process");
const path = require("node:path");
const { test, expect } = require("@playwright/test");

const root = path.resolve(__dirname, "../../..");
const names = [
  "game.pyxapp",
  'quoted"name.pyxapp',
  "日本語.pyxapp",
  "<!--<script>.pyxapp",
];

test("app2html preserves filenames through HTML script parsing", async ({
  page,
}) => {
  // Stub file I/O so POSIX filenames also exercise the HTML boundary on Windows.
  const htmlPages = JSON.parse(
    execFileSync(
      process.env.PYTHON ?? "python",
      [
        "-c",
        `
import importlib.util
import json
import sys
from pathlib import Path
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("app2html_cli", "python/pyxel/cli.py")
cli = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cli)
pages = []
with patch.object(cli, "_check_file_exists"), \\
     patch.object(Path, "read_bytes", return_value=b"app bytes"), \\
     patch.object(Path, "write_text", side_effect=lambda text, **kwargs: pages.append(text)):
    for name in json.load(sys.stdin):
        cli.create_html_from_pyxel_app(name)
print(json.dumps(pages))
`,
      ],
      { cwd: root, input: JSON.stringify(names), encoding: "utf8" },
    ),
  );

  await page.route(/^https:\/\/kitao\.dev\/pyxel\/[^/]+\/pyxel\.js$/, (route) =>
    route.fulfill({
      contentType: "application/javascript",
      body: "window.launchPyxel = (options) => { window.launched = options; };",
    }),
  );
  for (const [index, name] of names.entries()) {
    await page.route("https://app2html.test/", (route) =>
      route.fulfill({ contentType: "text/html", body: htmlPages[index] }),
    );
    await page.goto("https://app2html.test/");
    const options = await page.evaluate(() => window.launched);
    expect(options, name).toEqual({
      command: "play",
      name,
      gamepad: "enabled",
      base64: Buffer.from("app bytes").toString("base64"),
    });
    await page.unroute("https://app2html.test/");
  }
});
