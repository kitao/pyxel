import assert from "node:assert/strict";
import test from "node:test";
import worker from "../worker.mjs";

const origin = "https://kitao.dev/pyxel/launcher/source/";

test("source responses preserve bytes across chunks and isolate upstream headers", async (t) => {
  const data = Uint8Array.from([
    0xef,
    0xbb,
    0xbf,
    ...Array.from({ length: 256 }, (_, i) => i),
  ]);
  const remote = t.mock.method(globalThis, "fetch", async (url, options) => {
    assert.equal(
      url.href,
      "https://cdn.jsdelivr.net/gh/game/demo@main/pkg/a%23b.py",
    );
    assert.deepEqual(options, { redirect: "manual" });
    return new Response(
      new ReadableStream({
        start(controller) {
          controller.enqueue(data.subarray(0, 2));
          controller.enqueue(data.subarray(2));
          controller.close();
        },
      }),
      {
        headers: {
          "Content-Type": "text/html",
          "Set-Cookie": "fixture=ignored",
          "Cache-Control": "public, max-age=604800",
          Age: "518400",
          Date: "Thu, 05 Nov 2026 00:00:00 GMT",
        },
      },
    );
  });
  const request = new Request(
    origin + "game/demo@main/pkg/a%23b.py?ignore=this",
    {
      headers: { Cookie: "fixture=local", Range: "bytes=3-" },
    },
  );
  const result = await worker.fetch(request, {});
  assert.equal(remote.mock.callCount(), 1);
  assert.equal(result.status, 200);
  assert.equal(result.headers.get("Content-Type"), "application/octet-stream");
  assert.equal(result.headers.get("X-Content-Type-Options"), "nosniff");
  assert.equal(result.headers.get("X-Pyxel-Source"), "1");
  assert.equal(result.headers.get("Set-Cookie"), null);
  assert.equal(result.headers.get("Age"), "518400");
  assert.equal(result.headers.get("Date"), "Thu, 05 Nov 2026 00:00:00 GMT");
  assert.deepEqual(
    new Uint8Array(await result.arrayBuffer()),
    Uint8Array.of(0, ...data),
  );
});

test("invalid source paths and methods never generate an upstream request", async (t) => {
  t.mock.method(globalThis, "fetch", () =>
    assert.fail("unexpected upstream request"),
  );
  for (const path of [
    "https://elsewhere.test/game.py",
    "owner/repo/game.js",
    "owner/repo/a%2f..%2fescape.py",
    "owner/repo/%00a.py",
    "owner/repo/%zz.py",
  ]) {
    assert.equal(
      (await worker.fetch(new Request(origin + path), {})).status,
      404,
    );
  }
  assert.equal(
    (
      await worker.fetch(
        new Request(origin + "owner/repo/game.py", { method: "POST" }),
        {},
      )
    ).status,
    405,
  );
});

test("missing, partial and redirected responses never become source files", async (t) => {
  for (const [status, expected] of [
    [404, 404],
    [206, 502],
    [301, 502],
    [503, 502],
  ]) {
    const mock = t.mock.method(
      globalThis,
      "fetch",
      async () => new Response("not source", { status }),
    );
    const result = await worker.fetch(
      new Request(origin + "owner/repo/pkg/__init__.py"),
      {},
    );
    assert.equal(result.status, expected);
    assert.equal(result.headers.get("X-Pyxel-Source"), null);
    assert.equal(await result.text(), "");
    mock.mock.restore();
  }
});

test("an empty Python module still has a valid source frame", async (t) => {
  for (const body of [null, ""]) {
    const mock = t.mock.method(
      globalThis,
      "fetch",
      async () => new Response(body),
    );
    const result = await worker.fetch(
      new Request(origin + "owner/repo/pkg/__init__.py"),
      {},
    );
    assert.equal(result.status, 200);
    assert.equal(result.headers.get("X-Pyxel-Source"), "1");
    assert.deepEqual(
      new Uint8Array(await result.arrayBuffer()),
      Uint8Array.of(0),
    );
    mock.mock.restore();
  }
});

test("encoded legal filename characters retain their exact GitHub path", async (t) => {
  const urls = [];
  t.mock.method(globalThis, "fetch", async (url) => {
    urls.push(url.href);
    return new Response("fixture");
  });
  const names = [
    ".py",
    "UPPER.PY",
    "a%0Ab.py",
    "a%09b.py",
    "a%5Cb.py",
    "%E6%97%A5%E6%9C%AC.py",
  ];
  for (const name of names) {
    const response = await worker.fetch(
      new Request(origin + "owner/repo@main/" + name),
      {},
    );
    assert.equal(response.status, 200);
    assert.equal(await response.text(), "\0fixture");
  }
  assert.deepEqual(
    urls,
    names.map((name) => "https://cdn.jsdelivr.net/gh/owner/repo@main/" + name),
  );
});
