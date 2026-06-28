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

const code = (s, syntax = "plaintext") =>
  `<pre class="code-block"><code class="language-${syntax}">${esc(s)}</code></pre>`;

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

// Poll embedded Pyxel frames until their runtime hooks are ready.

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

// Fetch localized JSON, select a language, and render the page.

const initPage = (jsonFile, buildFn) => {
  fetch(jsonFile)
    .then((r) => r.json())
    .then((json) => {
      data = json;
      lang = detectLang(data.languages);
      buildFn();
    })
    .catch((e) => console.error("Failed to load data:", e));
};
