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

test("a tapped language button keeps no hover underline", async ({
  page,
  isMobile,
}) => {
  test.skip(!isMobile, "only touch screens keep a tapped element hovered");
  await page.goto("/pyxel/");
  const japanese = page.locator("[data-language='ja']");
  await japanese.tap();
  await expect(page.locator("html")).toHaveAttribute("lang", "ja");
  expect(
    await japanese.evaluate(
      (button) => getComputedStyle(button).textDecorationLine,
    ),
  ).toBe("none");
});

test("the homepage in both languages passes axe", async ({ page }) => {
  for (const path of ["/pyxel/?lang=en", "/pyxel/?lang=ja"]) {
    await page.goto(path);
    await page.evaluate(axe.source);
    // In-text links keep the maintainer's colour-only design.
    const results = await page.evaluate(() =>
      axe.run({ rules: { "link-in-text-block": { enabled: false } } }),
    );
    expect(results.violations, path).toEqual([]);
  }
});
