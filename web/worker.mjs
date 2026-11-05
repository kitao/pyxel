const sourcePrefix = "/pyxel/launcher/source/";

async function serveSource(request, path) {
  // Only public Python files from jsDelivr's GitHub namespace can be relayed.
  const headers = {
    "Access-Control-Allow-Origin": "*",
    "Cache-Control": "no-store",
    "Content-Security-Policy": "default-src 'none'; sandbox",
    "Content-Type": "application/octet-stream",
    "X-Content-Type-Options": "nosniff",
  };
  if (request.method !== "GET") {
    return new Response(null, {
      status: 405,
      headers: { ...headers, Allow: "GET" },
    });
  }
  let decoded;
  try {
    decoded = decodeURIComponent(path);
  } catch {
    return new Response(null, { status: 404, headers });
  }
  if (
    !/^[\w.-]+\/[\w.-]+(?:@[^/]+)?\/.*\.py$/i.test(path) ||
    /(?:^|\/)\.{1,2}(?:\/|$)/.test(decoded) ||
    /\x00/.test(decoded)
  ) {
    return new Response(null, { status: 404, headers });
  }
  const url = new URL("https://cdn.jsdelivr.net");
  url.pathname = "/gh/" + path;
  let upstream;
  try {
    upstream = await fetch(url, { redirect: "manual" });
  } catch {
    return new Response(null, { status: 502, headers });
  }
  if (upstream.status !== 200) {
    await upstream.body?.cancel();
    return new Response(null, {
      status: upstream.status === 404 ? 404 : 502,
      headers,
    });
  }

  // A leading NUL keeps a synchronous browser XHR from decoding the file's BOM.
  const reader = upstream.body?.getReader();
  const body = new ReadableStream({
    start(controller) {
      controller.enqueue(Uint8Array.of(0));
    },
    async pull(controller) {
      if (!reader) {
        controller.close();
        return;
      }
      const { done, value } = await reader.read();
      if (done) controller.close();
      else controller.enqueue(value);
    },
    cancel(reason) {
      return reader?.cancel(reason);
    },
  });
  const result = new Response(body, {
    headers: {
      ...headers,
      "Cache-Control": upstream.headers.get("Cache-Control") ?? "no-cache",
      "X-Pyxel-Source": "1",
      "Access-Control-Expose-Headers": "X-Pyxel-Source",
    },
  });
  for (const name of ["Age", "Date"]) {
    const value = upstream.headers.get(name);
    if (value !== null) result.headers.set(name, value);
  }
  return result;
}

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
    const url = new URL(request.url);
    if (url.pathname.startsWith(sourcePrefix)) {
      return serveSource(request, url.pathname.slice(sourcePrefix.length));
    }
    const legacyAsset =
      /^\/pyxel\/user-guide\/cube\/videos\/blockbench-3d-assets-(ja|en)\.mp4$/.exec(
        url.pathname,
      );
    if (legacyAsset && ["GET", "HEAD"].includes(request.method)) {
      url.pathname = `/pyxel/user-guide/cube/videos/pyxel-cube-assets-${legacyAsset[1]}.mp4`;
      return Response.redirect(url.href, 301);
    }
    const response = await env.ASSETS.fetch(request);
    if (
      !/^\/pyxel\/user-guide\/cube\/videos\/[^/]+\.mp4$/.test(url.pathname) ||
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
