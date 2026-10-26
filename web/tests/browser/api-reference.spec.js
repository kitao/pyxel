const { test, expect } = require("@playwright/test");

const advancedClasses = [
  "quat",
  "primitive",
  "collider",
  "contact",
  "raycasthit",
];
const preferenceKey = "pyxel-api-ref-advanced";
const cubePath = "/pyxel/api-reference/cube/";

test("Cube keeps drawing and model playback visible and advanced classes optional", async ({
  page,
}) => {
  await page.goto(cubePath);
  await expect(page.locator("#advanced-toggle")).not.toBeChecked();
  for (const id of [
    "vec3",
    "mat4",
    "node",
    "camera",
    "shading",
    "mesh",
    "motion",
  ]) {
    await expect(page.locator(`#${id}`)).toBeVisible();
  }
  for (const id of advancedClasses) {
    await expect(page.locator(`#${id}`)).toBeHidden();
  }
  await page.locator("#advanced-toggle").check();
  for (const id of advancedClasses) {
    await expect(page.locator(`#${id}`)).toBeVisible();
  }
});

for (const saved of [null, "0"]) {
  test(`class links reveal Advanced without replacing preference ${saved}`, async ({
    page,
  }) => {
    await page.goto(cubePath);
    if (saved !== null) {
      await page.evaluate(
        ([key, value]) => localStorage.setItem(key, value),
        [preferenceKey, saved],
      );
    }
    for (const id of advancedClasses) {
      await page.goto(`${cubePath}#${id}`);
      await expect(page.locator("#advanced-toggle")).toBeChecked();
      await expect(page.locator(`#${id}`)).toBeVisible();
      await expect(page.locator(`#${id}`)).toHaveAttribute("open", "");
      expect(
        await page.evaluate((key) => localStorage.getItem(key), preferenceKey),
      ).toBe(saved);
      await page.locator("#lang-select").selectOption("ja");
      expect(
        await page.evaluate((key) => localStorage.getItem(key), preferenceKey),
      ).toBe(saved);
      await page.goto(cubePath);
      await expect(page.locator("#advanced-toggle")).not.toBeChecked();
      await expect(page.locator(`#${id}`)).toBeHidden();
    }
    await page.goto("/pyxel/api-reference/#font");
    await expect(page.locator("#advanced-toggle")).toBeChecked();
    await expect(page.locator("#font")).toBeVisible();
    expect(
      await page.evaluate((key) => localStorage.getItem(key), preferenceKey),
    ).toBe(saved);
    await page.goto("/pyxel/api-reference/");
    await expect(page.locator("#advanced-toggle")).not.toBeChecked();
  });
}

test("fragment navigation reveals its target while explicit toggle changes persist", async ({
  page,
}) => {
  await page.goto(cubePath);
  const toggle = page.locator("#advanced-toggle");
  await page.locator("#search-input").fill("no-matching-api");
  await page.evaluate(() => {
    location.hash = "primitive";
  });
  await expect(toggle).toBeChecked();
  await expect(page.locator("#search-input")).toHaveValue("");
  await expect(page.locator("#primitive")).toBeVisible();
  expect(
    await page.evaluate((key) => localStorage.getItem(key), preferenceKey),
  ).toBeNull();

  await toggle.uncheck();
  expect(
    await page.evaluate((key) => localStorage.getItem(key), preferenceKey),
  ).toBe("0");
  await page.evaluate(() => {
    location.hash = "collider";
  });
  await expect(toggle).toBeChecked();
  await expect(page.locator("#collider")).toBeVisible();
  expect(
    await page.evaluate((key) => localStorage.getItem(key), preferenceKey),
  ).toBe("0");

  await toggle.uncheck();
  await toggle.check();
  await page.goto(cubePath);
  await expect(toggle).toBeChecked();
  await expect(page.locator("#quat")).toBeVisible();
  await toggle.uncheck();
  await page.reload();
  await expect(toggle).not.toBeChecked();
  await expect(page.locator("#quat")).toBeHidden();
});

test("Cube guide marks advanced topics without hiding text or adding a toggle", async ({
  page,
}) => {
  await page.route(
    /^https:\/\/cdn\.jsdelivr\.net\/gh\/highlightjs\//,
    (route) =>
      route.fulfill({
        body: route.request().url().endsWith(".js")
          ? "window.hljs = { highlightAll() {} };"
          : "",
      }),
  );
  await page.goto("/pyxel/user-guide/cube/");
  const { languages } = require("../../user-guide/cube/user-guide.json");
  for (const { code } of languages) {
    await page.locator("#lang-select").selectOption(code);
    await expect(page.locator("#s-overview")).toContainText("ADV");
    await expect(page.locator("#guide .adv-mark")).toHaveCount(3);
    for (const selector of [
      "#s-quaternions .adv-mark",
      "#s-primitives .adv-mark",
      "#s-interaction > h2 .adv-mark",
    ]) {
      await expect(page.locator(selector)).toHaveText("ADV");
    }
    await expect(page.locator("#s-interaction h3 .adv-mark")).toHaveCount(0);
    await expect(page.locator("input[type=checkbox]")).toHaveCount(0);
    await expect(page.locator("#s-interaction")).toBeVisible();
  }
});
