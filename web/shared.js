// Language selection

const PYXEL_LANG_KEY = "pyxel-lang";

const detectLang = (languages) => {
  const stored = localStorage.getItem(PYXEL_LANG_KEY);
  if (stored && languages.some((l) => l.code === stored)) return stored;
  const nav = (navigator.language ?? "").toLowerCase();
  if (nav.startsWith("zh")) return "cn";
  for (const l of languages) {
    if (nav.startsWith(l.code)) return l.code;
  }
  return "en";
};

const setDocLang = (lang) => {
  document.documentElement.lang = lang === "cn" ? "zh" : lang;
};

const saveLang = (lang) => {
  localStorage.setItem(PYXEL_LANG_KEY, lang);
  setDocLang(lang);
};

const buildLangSelector = (
  languages,
  currentLang,
  onChange,
  existingSelect,
) => {
  const select = existingSelect || document.createElement("select");
  if (!existingSelect) {
    select.id = "lang-select";
    select.className = "lang-select mt-1";
  }
  select.setAttribute("aria-label", "Language");
  for (const l of languages) {
    const o = document.createElement("option");
    o.value = l.code;
    o.textContent = l.name;
    select.appendChild(o);
  }
  select.value = currentLang;
  select.addEventListener("change", () => {
    saveLang(select.value);
    onChange(select.value);
  });
  if (existingSelect) select.style.display = "";
  return select;
};

const buildVariantSwitch = () => {
  const onCube = /\/cube(?:\/(?:index\.html)?)?$/.test(location.pathname);
  const sw = document.createElement("div");
  sw.className = "seg mt-1";
  const variants = [
    { label: "Base", href: onCube ? "../" : "./", active: !onCube },
    { label: "Cube", href: onCube ? "./" : "cube/", active: onCube },
  ];
  for (const v of variants) {
    const seg = document.createElement(v.active ? "span" : "a");
    seg.className = v.active ? "seg-btn seg-active" : "seg-btn";
    if (!v.active) seg.href = v.href;
    seg.textContent = v.label;
    sw.appendChild(seg);
  }
  return sw;
};

const buildPageHeader = (updateFn, leadingControl = null) => {
  const header = document.createElement("header");
  header.className = "flex flex-wrap sm:flex-nowrap items-start gap-4 mb-6";

  const titleBlock = document.createElement("div");
  titleBlock.className = "w-full sm:w-auto sm:flex-1 min-w-0 shrink-0";

  const h1 = document.createElement("h1");
  h1.className = "font-semibold text-2xl tracking-tight";
  h1.id = "page-title";
  titleBlock.appendChild(h1);

  const subtitle = document.createElement("p");
  subtitle.className = "mt-2 text-gray-300 text-sm";
  subtitle.id = "page-subtitle";
  titleBlock.appendChild(subtitle);

  const langSelect = buildLangSelector(data.languages, lang, (v) => {
    lang = v;
    updateFn();
  });

  header.appendChild(titleBlock);
  if (leadingControl) header.appendChild(leadingControl);
  header.appendChild(langSelect);
  return header;
};

// HTML helpers

const esc = (s) =>
  String(s)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");

const link = (href, text) =>
  `<a href="${esc(href)}" target="_blank" rel="noopener noreferrer" class="link">${esc(text)}</a>`;

const chip = (s) => `<code class="chip">${esc(s)}</code>`;

const t = (o) => {
  if (!o) return "";
  if (typeof o === "string") return data?.ui?.[o] ? t(data.ui[o]) : o;
  return o[lang] ?? o.en ?? "";
};

const code = (s, syntax = "plaintext") =>
  `<pre class="code-block"><code class="language-${syntax}">${esc(s)}</code></pre>`;

const btnChip = (s) => `<span class="btn-chip">${esc(s)}</span>`;

const linkChip = (s) => `<span class="link-chip">${esc(s)}</span>`;

const encodeUrlPath = (path) =>
  path.split("/").map(encodeURIComponent).join("/");

// Keep spread calls below browser argument limits.
const BASE64_CHUNK_SIZE = 0x8000;

const uint8ToBase64 = (u8) => {
  let bin = "";
  for (let i = 0; i < u8.length; i += BASE64_CHUNK_SIZE) {
    bin += String.fromCharCode(...u8.subarray(i, i + BASE64_CHUNK_SIZE));
  }
  return btoa(bin);
};

const base64ToUint8 = (b64) => {
  const bin = atob((b64 || "").replace(/\s/g, ""));
  return Uint8Array.from(bin, (c) => c.charCodeAt(0));
};

// Preserve a supplied ref boundary; otherwise try candidates from longest to
// shortest.
const resolveGitHubBlobUrl = async (
  input,
  fetchImpl = fetch,
  preferredRef = null,
) => {
  let url;
  try {
    url = new URL(input);
  } catch {
    throw new Error("Invalid GitHub blob URL");
  }

  const parts = url.pathname
    .split("/")
    .filter(Boolean)
    .map((part) => decodeURIComponent(part));
  if (
    url.protocol !== "https:" ||
    url.hostname !== "github.com" ||
    parts.length < 5 ||
    parts[2] !== "blob"
  ) {
    throw new Error("Invalid GitHub blob URL");
  }

  const [user, repo] = parts;
  const refAndPath = parts.slice(3);
  const resolveSplit = async (split) => {
    const ref = refAndPath.slice(0, split).join("/");
    const path = refAndPath.slice(split).join("/");
    if (/^[0-9a-f]{40}$/i.test(ref)) {
      return { user, repo, ref, sha: ref, path };
    }
    try {
      const response = await fetchImpl(
        `https://api.github.com/repos/${user}/${repo}/commits/${encodeURIComponent(ref)}`,
        {
          headers: { Accept: "application/vnd.github.sha" },
          cache: "no-cache",
        },
      );
      if (!response.ok) {
        if (response.status === 404) {
          return null;
        }
        if (response.status === 422) {
          try {
            const error = JSON.parse(await response.text());
            if (error.message?.startsWith("No commit found for SHA:")) {
              return null;
            }
          } catch {}
        }
        return undefined;
      }

      const sha = (await response.text()).trim();
      return /^[0-9a-f]{40}$/i.test(sha)
        ? { user, repo, ref, sha, path }
        : undefined;
    } catch {
      return undefined;
    }
  };

  if (preferredRef) {
    const preferredSplit = preferredRef.split("/").length;
    if (
      preferredSplit < refAndPath.length &&
      refAndPath.slice(0, preferredSplit).join("/") === preferredRef
    ) {
      const path = refAndPath.slice(preferredSplit).join("/");
      const resolved = await resolveSplit(preferredSplit);
      if (resolved) return resolved;
      return {
        user,
        repo,
        ref: preferredRef,
        sha: null,
        path,
      };
    }
  }

  for (let split = refAndPath.length - 1; split >= 1; split--) {
    const resolved = await resolveSplit(split);
    if (resolved) return resolved;
    if (resolved === undefined) break;
  }

  throw new Error("Failed to resolve the GitHub ref and file path");
};

const waitForPyxelReady = (
  checkFn,
  onReady,
  { maxRetries = 300, interval = 100 } = {},
) => {
  let retries = 0;
  (function poll() {
    if (checkFn()) {
      onReady();
    } else if (++retries < maxRetries) {
      setTimeout(poll, interval);
    } else {
      console.error("Pyxel runtime failed to initialize within timeout");
    }
  })();
};

const initPage = (jsonFile, buildFn) => {
  fetch(jsonFile)
    .then((response) => {
      if (!response.ok) {
        throw new Error(`Failed to fetch ${jsonFile}: ${response.status}`);
      }
      return response.json();
    })
    .then((json) => {
      data = json;
      lang = detectLang(data.languages);
      buildFn();
    })
    .catch((e) => console.error("Failed to load data:", e));
};
