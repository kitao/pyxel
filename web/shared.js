// Shared utilities for Pyxel WASM pages

const PYXEL_LANG_KEY = "pyxel-lang";

const detectLanguage = (languages) => {
  const stored = localStorage.getItem(PYXEL_LANG_KEY);
  if (stored && languages.some((l) => l.code === stored)) return stored;
  const nav = (navigator.language ?? "").toLowerCase();
  if (nav.startsWith("zh")) return "cn";
  for (const l of languages) {
    if (nav.startsWith(l.code)) return l.code;
  }
  return "en";
};

const saveLang = (lang) => {
  localStorage.setItem(PYXEL_LANG_KEY, lang);
  setDocLang(lang);
};

const setDocLang = (lang) => {
  document.documentElement.lang = lang === "cn" ? "zh" : lang;
};

const buildLangSelector = (
  languages,
  currentLang,
  onChange,
  existingSelect,
) => {
  const sel = existingSelect || document.createElement("select");
  if (!existingSelect) sel.className = "lang-select mt-1";
  for (const l of languages) {
    const o = document.createElement("option");
    o.value = l.code;
    o.textContent = l.name;
    sel.appendChild(o);
  }
  sel.value = currentLang;
  sel.addEventListener("change", () => {
    saveLang(sel.value);
    onChange(sel.value);
  });
  if (existingSelect) sel.style.display = "";
  return sel;
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

// Base64 <-> Uint8Array conversion

const uint8ToBase64 = (u8) => {
  let bin = "";
  for (const b of u8) bin += String.fromCharCode(b);
  return btoa(bin);
};

const base64ToUint8 = (b64) => {
  const bin = atob((b64 || "").replace(/\s/g, ""));
  return Uint8Array.from(bin, (c) => c.charCodeAt(0));
};

// Pyxel iframe readiness polling

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

// Page initialization

const initPage = (jsonFile, buildFn) => {
  fetch(jsonFile)
    .then((r) => r.json())
    .then((json) => {
      data = json;
      lang = detectLanguage(data.languages);
      buildFn();
    })
    .catch((e) => console.error("Failed to load data:", e));
};
