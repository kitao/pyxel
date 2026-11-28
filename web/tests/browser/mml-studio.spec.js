const { createHash } = require("node:crypto");
const { test, expect } = require("@playwright/test");

test("MML edits update a local QR image and keep oversized songs shareable", async ({
  page,
  baseURL,
}) => {
  const external = [];
  await page.route("**/*", (route) => {
    const url = new URL(route.request().url());
    // This share-link check needs neither audio playback nor the legacy decoder.
    if (url.href === baseURL + "/pyxel/mml-studio/pyxel-screen") {
      return route.fulfill({ contentType: "text/html", body: "" });
    }
    if (url.origin === baseURL) return route.continue();
    if (
      url.href ===
      "https://cdn.jsdelivr.net/npm/lz-string@1.5.0/libs/lz-string.min.js"
    ) {
      return route.fulfill({ contentType: "text/javascript", body: "" });
    }
    external.push(url.href);
    return route.abort();
  });

  await page.goto("/pyxel/mml-studio/");
  const image = page.getByAltText("QR Code");
  const link = page.locator("#share-url");
  const channel = page.locator("#ch1");
  const emptyUrl = page.url();
  const historyLength = await page.evaluate(() => history.length);
  await expect(image).toHaveAttribute("src", /^data:image\/png;base64,/);
  await expect(link).toHaveAttribute("href", emptyUrl);
  const emptyImage = await image.getAttribute("src");

  const notes = createHash("shake256", { outputLength: 4096 })
    .update("MML Studio")
    .digest();
  const longSong = Array.from(
    notes,
    (note) => "CDEFGAB"[note % 7] + [2, 4, 8, 16][(note >> 4) % 4],
  ).join(" ");
  await channel.fill(longSong);
  await expect(link).toHaveAttribute("href", /\?mml=/);
  await expect(image).toBeHidden();
  expect(await image.getAttribute("src")).toBeNull();
  expect(await link.getAttribute("href")).toBe(page.url());

  await channel.fill("T120 C4 D4 E4");
  await expect(image).toBeVisible();
  await expect(image).toHaveAttribute("src", /^data:image\/png;base64,/);
  expect(await image.getAttribute("src")).not.toBe(emptyImage);
  expect(await link.getAttribute("href")).toBe(page.url());
  await channel.fill("");
  await expect(link).toHaveAttribute("href", emptyUrl);
  await expect(image).toHaveAttribute("src", emptyImage);
  expect(await page.evaluate(() => history.length)).toBe(historyLength);
  expect(external).toEqual([]);
});
