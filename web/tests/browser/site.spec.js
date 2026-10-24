const { test, expect } = require("@playwright/test");
const axe = require("axe-core");

// Badges and fonts load from Shields.io, Ko-fi, and Google Fonts; stand-ins keep the tests
// independent of the network, and the fonts fall back to local ones.
const badgeStandIn =
  '<svg xmlns="http://www.w3.org/2000/svg" width="100" height="20"><rect width="100" height="20"/></svg>';

const fontStandIn =
  "@font-face { font-family: 'Noto Sans JP'; src: url(https://fonts.gstatic.com/s/stand-in.woff2) format('woff2'); }";

const stubExternal = async (context) => {
  await context.route(
    /^https:\/\/(img\.shields\.io|ko-fi\.com\/img)\//,
    (route) =>
      route.fulfill({ contentType: "image/svg+xml", body: badgeStandIn }),
  );
  await context.route(/^https:\/\/fonts\.googleapis\.com\//, (route) =>
    route.fulfill({ contentType: "text/css", body: fontStandIn }),
  );
  await context.route(/^https:\/\/fonts\.gstatic\.com\//, (route) =>
    route.fulfill({ contentType: "font/woff2", body: "" }),
  );
};

test.beforeEach(async ({ context }) => {
  await stubExternal(context);
});

test("pages and redirects carry the shared security headers", async ({
  request,
}) => {
  for (const [path, status] of [
    ["/pyxel", 307],
    ["/pyxel/", 200],
    ["/pyxel/user-guide/", 200],
    // Rules for the runtime add to the shared lines instead of replacing them.
    ["/pyxel/2.9.9/pyxel.js", 200],
  ]) {
    const response = await request.get(path, { maxRedirects: 0 });
    expect(response.status(), path).toBe(status);
    const headers = response.headers();
    expect(headers["x-content-type-options"], path).toBe("nosniff");
    expect(headers["referrer-policy"], path).toBe(
      "strict-origin-when-cross-origin",
    );
  }
});

test("pages on other sites can load the runtime and examples", async ({
  request,
}) => {
  for (const path of [
    "/pyxel/2.9.9/pyxel.js",
    "/pyxel/3.0.0/pyxel.js",
    "/pyxel/assets/examples/01_hello_pyxel.py",
  ]) {
    const response = await request.get(path);
    expect(response.status(), path).toBe(200);
    expect(response.headers()["access-control-allow-origin"], path).toBe("*");
  }
});

test("released runtime bundles may be cached for good, unlike the provisional one", async ({
  request,
}) => {
  const cacheControl = async (path) =>
    (await request.get(path)).headers()["cache-control"];
  expect(await cacheControl("/pyxel/2.9.9/pyxel.js")).toBe(
    "public, max-age=31536000, immutable",
  );
  expect(await cacheControl("/pyxel/3.0.0/pyxel.js")).not.toContain(
    "immutable",
  );
});

test("without JavaScript the homepage shows English and no language switch", async ({
  browser,
}) => {
  const context = await browser.newContext({
    javaScriptEnabled: false,
    locale: "ja-JP",
  });
  try {
    await stubExternal(context);
    const page = await context.newPage();
    await page.goto("/pyxel/");
    await expect(page.locator(".language-switch")).toBeHidden();
    await expect(page.locator("main > div[lang='en']")).toBeVisible();
    await expect(page.locator("main > div[lang='ja']")).toBeHidden();
  } finally {
    await context.close();
  }
});

test("homepage languages remain visible and usable on every screen size", async ({
  page,
}) => {
  const guide = require("../../user-guide/user-guide.json");
  await page.goto("/pyxel/");
  for (const { code } of guide.languages) {
    await page.locator("#lang-select").selectOption(code);
    const tag = code === "cn" ? "zh" : code;
    await expect(page.locator("html")).toHaveAttribute("lang", tag);
    await expect(page.locator("main > div[lang]:visible")).toHaveCount(1);
    await expect(page.locator(`main > div[lang='${tag}']`)).toBeVisible();
    await expect(
      page.locator("main > div[lang]:visible .quick-links a"),
    ).toHaveCount(8);
    expect(
      await page.evaluate(
        () => document.documentElement.scrollWidth <= innerWidth,
      ),
      code,
    ).toBe(true);
  }
  await page.reload();
  await expect(page.locator("#lang-select")).toHaveValue("uk");
});

test("homepage and guide keep the language selected on either page", async ({
  page,
}) => {
  // Syntax highlighting is unrelated to language selection.
  await page.route(
    /^https:\/\/cdn\.jsdelivr\.net\/gh\/highlightjs\//,
    (route) => route.fulfill({ body: "" }),
  );
  await page.goto("/pyxel/?lang=fr");
  await page.locator("#lang-select").selectOption("cn");
  const guide = new URL(
    await page
      .locator("main > div[lang='zh'] .quick-links a")
      .first()
      .getAttribute("href"),
  );
  await page.goto(guide.pathname);
  await expect(page.locator("#lang-select")).toHaveValue("cn");
  await page.locator("#lang-select").selectOption("de");
  await page.goto("/pyxel/");
  await expect(page.locator("#lang-select")).toHaveValue("de");
  await page.locator("#lang-select").selectOption("ja");
  await page.goto(guide.pathname);
  await expect(page.locator("#lang-select")).toHaveValue("ja");
});

test("the homepage in every language passes axe", async ({ page }) => {
  for (const { code } of require("../../user-guide/user-guide.json")
    .languages) {
    const path = `/pyxel/?lang=${code}`;
    await page.goto(path);
    await page.evaluate(axe.source);
    // In-text links keep the maintainer's colour-only design.
    const results = await page.evaluate(() =>
      axe.run({ rules: { "link-in-text-block": { enabled: false } } }),
    );
    expect(results.violations, path).toEqual([]);
  }
});
