function sliceStream(body, start, end) {
  const reader = body.getReader();
  let offset = 0;
  return new ReadableStream({
    async pull(controller) {
      while (true) {
        const { value, done } = await reader.read();
        if (done) {
          controller.close();
          return;
        }
        const previous = offset;
        offset += value.byteLength;
        if (offset <= start) continue;
        controller.enqueue(
          value.subarray(Math.max(0, start - previous), end + 1 - previous),
        );
        if (offset > end) {
          controller.close();
          await reader.cancel();
        }
        return;
      }
    },
    cancel(reason) {
      return reader.cancel(reason);
    },
  });
}

export default {
  async fetch(request, env) {
    const response = await env.ASSETS.fetch(request);
    const path = new URL(request.url).pathname;
    if (
      !/^\/pyxel\/user-guide\/cube\/videos\/[^/]+\.mp4$/.test(path) ||
      !["GET", "HEAD"].includes(request.method) ||
      response.status !== 200
    ) {
      return response;
    }

    // ASSETS responses omit Content-Length; the site build supplies the exact size.
    const size = Number(response.headers.get("X-Video-Size"));
    if (!Number.isSafeInteger(size) || size <= 0) return response;
    const headers = new Headers(response.headers);
    headers.delete("X-Video-Size");
    headers.set("Content-Length", String(size));
    headers.set("Accept-Ranges", "bytes");
    const full = () => new Response(response.body, { status: 200, headers });
    if (request.method === "HEAD") return full();

    // Static Assets ignores Range. Serve a single interval without buffering the video.
    const range = /^bytes=(\d*)-(\d*)$/.exec(
      request.headers.get("Range") ?? "",
    );
    if (!range || (!range[1] && !range[2])) return full();
    const ifRange = request.headers.get("If-Range");
    if (
      ifRange &&
      (ifRange.startsWith("W/") ||
        (ifRange !== headers.get("ETag") &&
          ifRange !== headers.get("Last-Modified")))
    ) {
      return full();
    }
    const start = range[1]
      ? Number(range[1])
      : Math.max(0, size - Number(range[2]));
    const end =
      range[1] && range[2] ? Math.min(size - 1, Number(range[2])) : size - 1;
    if (!Number.isSafeInteger(start) || start >= size || end < start) {
      await response.body.cancel();
      headers.set("Content-Range", `bytes */${size}`);
      headers.set("Content-Length", "0");
      return new Response(null, { status: 416, headers });
    }
    headers.set("Content-Range", `bytes ${start}-${end}/${size}`);
    headers.set("Content-Length", String(end - start + 1));
    return new Response(sliceStream(response.body, start, end), {
      status: 206,
      headers,
    });
  },
};
