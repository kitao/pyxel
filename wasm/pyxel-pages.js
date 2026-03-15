// Shared utilities for Pyxel WASM pages

const PYXEL_LANG_KEY = "pyxel-lang";

function detectLanguage(languages) {
  const stored = localStorage.getItem(PYXEL_LANG_KEY);
  if (stored && languages.some((l) => l.code === stored)) return stored;
  const nav = (navigator.language || "").toLowerCase();
  if (nav.startsWith("zh")) return "cn";
  for (const l of languages) {
    if (nav.startsWith(l.code)) return l.code;
  }
  return "en";
}

function saveLang(lang) {
  localStorage.setItem(PYXEL_LANG_KEY, lang);
  setDocLang(lang);
}

function setDocLang(lang) {
  document.documentElement.lang = lang === "cn" ? "zh" : lang;
}

function buildLangSelector(languages, currentLang, onChange, existingSelect) {
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
}

// HTML helpers

function esc(s) {
  const d = document.createElement("span");
  d.textContent = s;
  return d.innerHTML;
}

function link(href, text) {
  return `<a href="${href}" target="_blank" rel="noopener noreferrer" class="link">${text}</a>`;
}

function chip(s) {
  return `<code class="chip">${esc(s)}</code>`;
}
