import assert from "node:assert/strict";
import test from "node:test";
import worker from "../worker.mjs";

const bytes = Uint8Array.from({ length: 20 }, (_, i) => i);
const url = "https://kitao.dev/pyxel/user-guide/cube/videos/example.mp4";
let canceled = false;
function assets(status = 200) {
  return {
    ASSETS: {
      async fetch(request) {
        let offset = 0;
        const body =
          request.method === "HEAD"
            ? null
            : new ReadableStream({
                pull(controller) {
                  if (offset === bytes.length) return controller.close();
                  controller.enqueue(bytes.slice(offset, (offset += 4)));
                },
                cancel() {
                  canceled = true;
                },
              });
        return new Response(body, {
          status,
          headers: {
            "X-Video-Size": String(bytes.length),
            "Content-Type": "video/mp4",
            ETag: '"video-v1"',
            "X-Content-Type-Options": "nosniff",
          },
        });
      },
    },
  };
}
const data = async (r) => Array.from(new Uint8Array(await r.arrayBuffer()));

test("video ranges contain exactly the requested bytes across stream chunks", async () => {
  canceled = false;
  const r = await worker.fetch(
    new Request(url, { headers: { Range: "bytes=5-10" } }),
    assets(),
  );
  assert.equal(r.status, 206);
  assert.equal(r.headers.get("Content-Range"), "bytes 5-10/20");
  assert.equal(r.headers.get("Content-Length"), "6");
  assert.equal(r.headers.get("Accept-Ranges"), "bytes");
  assert.equal(r.headers.get("Content-Type"), "video/mp4");
  assert.equal(r.headers.has("X-Video-Size"), false);
  assert.equal(r.headers.get("X-Content-Type-Options"), "nosniff");
  assert.deepEqual(await data(r), [5, 6, 7, 8, 9, 10]);
  assert.equal(canceled, true);
});

test("open-ended, suffix and oversized end ranges stop at the end of the video", async () => {
  for (const range of ["bytes=17-", "bytes=-3", "bytes=17-999"]) {
    const r = await worker.fetch(
      new Request(url, { headers: { Range: range } }),
      assets(),
    );
    assert.equal(r.status, 206);
    assert.equal(r.headers.get("Content-Range"), "bytes 17-19/20");
    assert.deepEqual(await data(r), [17, 18, 19]);
  }
});

test("unsatisfiable ranges return 416 with the complete length and no body", async () => {
  for (const range of [
    "bytes=20-",
    "bytes=8-4",
    "bytes=-0",
    "bytes=999999999999999999999-",
  ]) {
    const r = await worker.fetch(
      new Request(url, { headers: { Range: range } }),
      assets(),
    );
    assert.equal(r.status, 416);
    assert.equal(r.headers.get("Content-Range"), "bytes */20");
    assert.deepEqual(await data(r), []);
  }
});

test("unsupported or malformed ranges use the complete representation", async () => {
  for (const range of ["bytes=0-2,8-9", "bytes=-", "invalid"]) {
    const r = await worker.fetch(
      new Request(url, { headers: { Range: range } }),
      assets(),
    );
    assert.equal(r.status, 200);
    assert.deepEqual(await data(r), Array.from(bytes));
  }
});

test("If-Range returns partial content only for the current validator", async () => {
  for (const validator of ['"video-v1"', '"video-v0"']) {
    const r = await worker.fetch(
      new Request(url, {
        headers: { Range: "bytes=0-1", "If-Range": validator },
      }),
      assets(),
    );
    assert.equal(r.status, validator === '"video-v1"' ? 206 : 200);
    assert.equal((await data(r)).length, validator === '"video-v1"' ? 2 : 20);
  }
});

test("HEAD preserves length and advertises range support without a body", async () => {
  const r = await worker.fetch(
    new Request(url, { method: "HEAD", headers: { Range: "bytes=0-1" } }),
    assets(),
  );
  assert.equal(r.status, 200);
  assert.equal(r.headers.get("Content-Length"), "20");
  assert.equal(r.headers.get("Accept-Ranges"), "bytes");
  assert.deepEqual(await data(r), []);
});

test("other routes and unsuccessful responses pass through unchanged", async () => {
  for (const [path, status] of [
    ["https://kitao.dev/pyxel/index.html", 200],
    [url, 404],
  ]) {
    const r = await worker.fetch(
      new Request(path, { headers: { Range: "bytes=0-1" } }),
      assets(status),
    );
    assert.equal(r.status, status);
    assert.equal(r.headers.has("Accept-Ranges"), false);
    assert.deepEqual(await data(r), Array.from(bytes));
  }
});
