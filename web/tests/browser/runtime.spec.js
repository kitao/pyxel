const fs = require("node:fs");
const path = require("node:path");
const { test, expect } = require("@playwright/test");

const project = fs.readFileSync(
  path.resolve(__dirname, "../../../python/pyproject.toml"),
  "utf8",
);
const version = project.match(/^version = "([^"]+)"/m)[1];

test.use({ serviceWorkers: "block" });

test("runtime resets scripts before and after Pyxel initialization", async ({
  page,
  baseURL,
}) => {
  test.setTimeout(60_000);
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });

  // Exercise the built runtime with the same verified Pyodide files served locally.
  await page.route("https://kitao.dev/pyxel/pyodide/**", async (route) => {
    const url = new URL(new URL(route.request().url()).pathname, baseURL);
    const response = await route.fetch({ url: url.href });
    await route.fulfill({ response });
  });
  const noInit = "import js\njs.window.runtimeExecutions += 1";
  await page.route("**/pyxel/runtime-reset-test.html", (route) =>
    route.fulfill({
      contentType: "text/html",
      body: `<!doctype html>
<html lang="en"><head><title>Runtime reset test</title>
<script src="/pyxel/${version}/pyxel.js"></script>
<script>window.runtimeExecutions = 0;</script>
</head><body><pyxel-run script="${noInit}"></pyxel-run></body></html>`,
    }),
  );
  await page.goto("/pyxel/runtime-reset-test.html");
  await expect(page.locator("#pyxel-prompt")).toBeVisible();
  await page.locator("#canvas").click();
  await expect
    .poll(() => page.evaluate(() => window.runtimeExecutions))
    .toBe(1);

  const reset = async (script) => {
    await page.evaluate(async (source) => {
      window.pyxelContext.params.script = source;
      await resetPyxel();
    }, script);
    expect(await page.evaluate(() => window.pyxelContext.hasFatalError)).toBe(
      false,
    );
    await expect(page.locator("#pyxel-error-overlay")).toHaveCount(0);
  };
  await reset(noInit);
  await expect
    .poll(() => page.evaluate(() => window.runtimeExecutions))
    .toBe(2);
  await reset("");

  const game = `import pyxel
pyxel.init(32, 32)
pyxel.run(lambda: None, lambda: pyxel.cls(3))`;
  const expectFrames = async () => {
    await expect
      .poll(() =>
        page.evaluate(() =>
          window.pyxelContext.pyodide.runPython("pyxel.frame_count"),
        ),
      )
      .toBeGreaterThan(1);
    expect(
      await page.evaluate(() =>
        window.pyxelContext.pyodide.runPython("pyxel.pget(0, 0)"),
      ),
    ).toBe(3);
  };
  await reset(game);
  await expectFrames();
  await reset(noInit);
  await expect
    .poll(() => page.evaluate(() => window.runtimeExecutions))
    .toBe(3);
  await reset(game);
  await expectFrames();
  expect(errors).toEqual([]);
});
