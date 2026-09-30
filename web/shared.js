// Language detection and localized page initialization helpers

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

// Standard page header with title, subtitle, and a language selector that
// re-renders through the page's text updater.
const buildPageHeader = (updateFn) => {
  const header = document.createElement("header");
  header.className = "flex items-start gap-4 mb-6";

  const titleBlock = document.createElement("div");
  titleBlock.className = "flex-1 min-w-0 order-first";

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

  header.appendChild(langSelect);
  header.appendChild(titleBlock);
  return header;
};

// Shared HTML string helpers for generated static pages

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

// Localized copy-button labels and icons for code blocks
const codeCopyLabels = {
  copy: {
    en: "Copy code",
    cn: "复制代码",
    de: "Code kopieren",
    es: "Copiar código",
    fr: "Copier le code",
    it: "Copia il codice",
    ja: "コードをコピー",
    ko: "코드 복사",
    pt: "Copiar código",
    ru: "Копировать код",
    tr: "Kodu kopyala",
    uk: "Копіювати код",
  },
  copied: {
    en: "Copied!",
    cn: "已复制！",
    de: "Kopiert!",
    es: "¡Copiado!",
    fr: "Copié !",
    it: "Copiato!",
    ja: "コピーしました！",
    ko: "복사되었습니다!",
    pt: "Copiado!",
    ru: "Скопировано!",
    tr: "Kopyalandı!",
    uk: "Скопійовано!",
  },
};

const codeCopyIcons = {
  copy: `<svg class="code-copy-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>`,
  copied: `<svg class="code-copied-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>`,
};

const code = (s, syntax = "plaintext") => {
  const copyLabel = t(codeCopyLabels.copy);
  const copiedLabel = t(codeCopyLabels.copied);
  return `<div class="code-block-wrap"><pre class="code-block"><code class="language-${syntax}">${esc(s)}</code></pre><button type="button" class="code-copy-btn" title="${esc(copyLabel)}" aria-label="${esc(copyLabel)}" data-copy-label="${esc(copyLabel)}" data-copied-label="${esc(copiedLabel)}">${codeCopyIcons.copy}${codeCopyIcons.copied}</button></div>`;
};

const btnChip = (s) => `<span class="btn-chip">${esc(s)}</span>`;

const linkChip = (s) => `<span class="link-chip">${esc(s)}</span>`;

// Chunked Base64 and Uint8Array conversion for archive payloads

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

// Resolve the ref/path boundary in a GitHub blob URL and return the ref with its
// commit SHA. Trying the longest ref first supports branch names with slashes.
const resolveGitHubBlobUrl = async (input, fetchImpl = fetch) => {
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
  for (let split = refAndPath.length - 1; split >= 1; split--) {
    const ref = refAndPath.slice(0, split).join("/");
    const path = refAndPath.slice(split).join("/");
    const response = await fetchImpl(
      `https://api.github.com/repos/${user}/${repo}/commits/${encodeURIComponent(ref)}`,
      {
        headers: { Accept: "application/vnd.github.sha" },
        cache: "no-cache",
      },
    );
    if (!response.ok) continue;

    const sha = (await response.text()).trim();
    if (/^[0-9a-f]{40}$/i.test(sha)) {
      return { user, repo, ref, sha, path };
    }
  }

  throw new Error("Failed to resolve the GitHub ref and file path");
};

// Readiness polling for embedded Pyxel frame runtime hooks

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

// Localized JSON loading, language selection, and page rendering

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

// Clipboard writes for the code-block copy buttons

const CODE_COPY_RESET_MS = 2000;

const writeClipboardText = async (text) => {
  if (navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      // Fall through to the legacy path when the API is unavailable.
    }
  }
  const el = document.createElement("textarea");
  el.value = text;
  el.setAttribute("readonly", "");
  el.style.position = "fixed";
  el.style.opacity = "0";
  document.body.appendChild(el);
  el.select();
  const ok = document.execCommand("copy");
  el.remove();
  return ok;
};

const handleCodeCopyClick = async (e) => {
  const btn = e.target.closest?.(".code-copy-btn");
  if (!btn) return;
  const text = btn.parentElement?.querySelector("pre code")?.textContent ?? "";
  if (!(await writeClipboardText(text))) return;
  btn.classList.add("copied");
  btn.title = btn.dataset.copiedLabel;
  btn.setAttribute("aria-label", btn.dataset.copiedLabel);
  setTimeout(() => {
    btn.classList.remove("copied");
    btn.title = btn.dataset.copyLabel;
    btn.setAttribute("aria-label", btn.dataset.copyLabel);
  }, CODE_COPY_RESET_MS);
};

document.addEventListener("click", handleCodeCopyClick);
