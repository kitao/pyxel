const { test, expect } = require("@playwright/test");

for (const language of ["ja", "en"]) {
  test(`the ${language} tutorial supports playback after seeking`, async ({
    page,
    request,
    baseURL,
  }) => {
    const url = `${baseURL}/pyxel/user-guide/cube/videos/blockbench-3d-assets-${language}.mp4`;
    const partial = await request.get(url, {
      headers: { Range: "bytes=0-1023" },
    });
    expect(partial.status()).toBe(206);
    expect(partial.headers()["content-range"]).toMatch(/^bytes 0-1023\/\d+$/);
    expect((await partial.body()).length).toBe(1024);

    await page.setContent(`<video src="${url}" controls muted></video>`);
    await page.waitForFunction(
      () => document.querySelector("video").readyState >= 1,
    );
    await page.locator("video").evaluate(async (video) => {
      video.currentTime = 400;
      await video.play();
    });
    await expect
      .poll(() => page.locator("video").evaluate((video) => video.currentTime))
      .toBeGreaterThan(400);
    expect(
      await page.locator("video").evaluate((video) => video.error),
    ).toBeNull();
  });
}
