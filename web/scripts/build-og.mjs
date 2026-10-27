import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { chromium } from "@playwright/test";

// Integer scaling preserves the logo's pixels; the margins keep it inside wide cards.
const logo = await readFile(
  new URL("../../docs/images/pyxel_logo_152x64.png", import.meta.url),
);
const browser = await chromium.launch();
try {
  const page = await browser.newPage({
    viewport: { width: 1200, height: 630 },
    deviceScaleFactor: 1,
  });
  await page.setContent(`<!doctype html><html lang="en"><meta charset="utf-8"><title>Pyxel</title>
    <style>
      html, body { margin: 0; width: 1200px; height: 630px; background: #111827; }

      body { display: grid; place-items: center; }

      img { display: block; width: 760px; height: 320px; image-rendering: pixelated; }
    </style>
    <img alt="Pyxel logo" src="data:image/png;base64,${logo.toString("base64")}">`);
  await page.locator("img").evaluate((image) => image.decode());
  const path = fileURLToPath(
    new URL("../../docs/images/pyxel_og.png", import.meta.url),
  );
  await page.screenshot({ path });
  console.log("Generated docs/images/pyxel_og.png from the Pyxel logo.");
} finally {
  await browser.close();
}
