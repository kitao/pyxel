# Documentation Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Documentation policy](../design-policy.md#documentation) · [Release Notes policy](../design-policy.md#release-notes)

## Reader and Prose

### The introductory description of 16 colors

**Decision:** Keep the concise basic-specification description in the [project
introduction](../../../README.md) and [user
guide](../../../web/user-guide/user-guide.json): Pyxel provides a 16-color
environment. Do not append palette-extension details at every introductory
mention of the color count. The same scope applies to other introductory tables
of the basic configuration, such as the image-bank range of the special key
table.

**Reason:** The basic specifications give newcomers a small, comprehensible
starting point. Pyxel deliberately separates that initial experience from
advanced capabilities that users can discover as they experiment. Listing the
extensions and qualifications immediately would obscure that starting point
and increase what a user must learn before making something.

An [advanced API](../../../web/api-reference/index.html) that can extend the
palette and a basic introduction that says "16 colors" serve different reading
tasks. They are not automatically contradictory. Relevant conditions and
consequences still belong in the documentation for the advanced operation
itself. This choice preserves the distinction across translations and generated
documents.

### Advanced API labels

**Decision:** Keep 「上級者向け」 for the Japanese advanced-API control in the
[base reference](../../../web/api-reference/api-reference.json) and
[Cube reference](../../../web/api-reference/cube/api-reference.json).
Translations retain the same distinction between basic and advanced use.

**Reason:** The label helps readers choose the amount of API detail they see.
It names the intended audience rather than adding implementation terminology;
changing the label is not a purely cosmetic wording correction.

### Reference reading order

**Decision:** The base reference introduces module APIs before class references;
Cube introduces mathematical values before rendering assets and scene-tree
facilities. Within a class, keep related properties and operation groups
together. Creation facilities precede operations on the resulting object,
without requiring every factory to appear immediately after the constructor.
Node's factory follows its property block, before the contiguous tree-edit
operations `add_child`, `remove_child`, and `destroy`.

The Resource section introduces loading, obtaining a writable directory, then
saving and exporting. Image introduces construction and dimensions, ordinary
image editing, then direct data access and export. Input introduces device
state, ordinary button/mouse queries, then analog and injected input. Preserve
these groups rather than sorting all advanced entries or mechanically pairing
similarly named operations across them.

**Reason:** The [reference source](../../../web/api-reference/) serves someone
learning or finding an operation. A writable directory prepares for saving;
whole-image export differs from loading pixels into an editing region. Such
reader relationships can differ from implementation or stub order. Moving a
factory out of the middle of tree editing restores a continuous task without
rearranging the class's properties. Generated references use this same source
order; their text, signatures, and advanced labels have separate ownership.

### Short HTML examples

**Decision:** Keep the launch pages under `web/showcase/apps/`,
`web/showcase/examples/`, and `web/showcase/tools/` as short executable HTML
examples, centered on the web runtime script and its custom element, as in
[Hello Pyxel](../../../web/showcase/examples/01-hello-pyxel.html).
Do not add `lang="en"` or a general page-metadata template to this family merely
to make it resemble the surrounding documentation pages.

**Reason:** Readers need to identify the code that runs their app. Unrelated
page scaffolding adds material to copy without explaining that operation.
Ordinary tool and documentation pages retain their own language metadata.
The compact [app2html output](../../../python/pyxel/cli.py) and embedded
[Web usage snippets](../../../web/web-usage/index.html) serve the same purpose
without requiring identical page scaffolding.

### Japanese spacing, punctuation, and technical spellings

**Decision:** Separate Japanese text from adjacent alphanumeric tokens with
one half-width space: 「Web 版 Pyxel」「16 色」「.pyxres ファイル」.
Preserve literal spacing in code spans. Use full-width parentheses flush with
Japanese content and half-width parentheses for ASCII-only content, separated
by spaces except next to punctuation:
「イメージバンク（Image クラスのインスタンス）のリスト (0-2)」.

Use 「ブラウザ」「エディタ」「パラメータ」「バッファ」「コンストラクタ」
「ユーザー」「サーバー」「コンピュータ」「ディレクトリ」 as the adopted spellings.

**Reason:** These choices give Japanese prose consistent word boundaries and
technical vocabulary while keeping code literal. They settle recurring
typographical choices; they do not authorize changing meaning or imposing
Japanese loanword conventions on other languages.

## Sources and Organization

### Sources of generated guides and stub docstrings

**Decision:** Web source data and templates own generated guides and API
descriptions. Their Markdown outputs come from
[generate_docs](../../../scripts/generate_docs), and the base stub's docstrings
from [generate_pyi_docstrings](../../../scripts/generate_pyi_docstrings).
Generated Markdown uses English and links to the multilingual web pages.
The docstring generator covers the base stub; it does not rewrite the Cube stub.

**Reason:** The web pages, generated reference files, and editor type help
present the same descriptions. Editing each output independently creates
competing copies that the next generation overwrites. Stub signatures still
need their own correspondence with the bindings; regenerating docstrings alone
does not verify those signatures.
These representations serve different reading contexts; correspondence does
not require generating every language or making both stubs identical in detail.

### Handwritten documents and portable instructions

**Decision:** Keep handwritten Markdown's paragraph structure, line breaks, and
layout under editorial control. The limited typography corrections in
[format_prose](../../../scripts/format_prose) also apply to its selected
handwritten inputs, `README.md` and `docs/faq-ja.md`; they do not reflow
paragraphs or choose their structure. Use relative paths or generic placeholders
in published instructions; preserve literal code and paths that are part of an
interface.

**Reason:** Prose structure needs editorial judgment, while private machine
paths would make the instructions unusable to other contributors. Generated
documents retain their source-owned formatting rather than being edited as
handwritten prose.

### Contributor workflow and command instructions

**Decision:** The [Makefile](../../../Makefile) owns development commands and
their setup instructions. Its opening comments follow the working sequence:
prerequisites, one-time setup, per-shell setup, common checks, then native,
WASM, and web-page commands. Keep these instructions brief and usable from the
repository root.

[CONTRIBUTING](../../../.github/CONTRIBUTING.md) owns the path from reporting or
discussing a change to submitting a pull request. It links to the Makefile for
setup and commands, and to the design documents for standards and verification.

**Reason:** Contributors need both a workflow and commands, at different steps.
Duplicating the command catalogue in the PR guide creates competing
instructions; putting the PR workflow into the Makefile obscures its command
reference.

### Organization of design documents

**Decision:** Keep contributor design documents under `docs/design/`, separate
from user guides. Retain the `design-` prefix on the policy, audit, and
decisions directory. Group decisions by policy area and the subject being
reviewed, with further separation where an interface boundary gives the work a
distinct scope; the blank-line decisions of every language share one file so
that the shared criteria and their language-specific boundaries are read
together. Use descriptive file names such as `source-code-comments.md` and
`public-contract-rust-python.md`.

Entries record reusable rules with their conditions and reasons; the
[audit procedure](../design-audit.md#1-select-and-resolve-decisions) states what
qualifies as an entry. A record that transcribes the groups or field order of
one file becomes stale with the next change and turns the audit into a
comparison against an old shape.

**Reason:** A reviewer should find the judgments needed for the assigned work
without reading unrelated decisions. Shared choices stay in one place, with
links from dependent entries. The audit indexes files and their
responsibilities; individual entries remain owned by those files. Additions
within a file therefore do not require another entry list in the audit. Adjust
physical file boundaries when related reading tasks or volume justify it, while
preserving the owning policy area and keeping names meaningful in file searches.

File boundaries follow the decisions a reviewer needs together, not a target
file size. Cross-file applicability belongs in a decision's scope; it does not
by itself identify the subject that owns the decision. Crate publication, for
example, belongs with distribution even though its implementation uses Rust.

### Compact examples in policy rules

**Decision:** Use at most one compact example per policy rule, only to clarify
a boundary.

**Reason:** An illustrative case should help distinguish the rule's application
without turning the rule into a catalogue of cases. The example does not acquire
separate normative meaning; concrete decisions retain their own scope and
rationale.

## Translation and Product Names

### Translation source and comparison chain

**Decision:** Japanese is the maintainer's source of truth. Translate through
English, then use English to check the other languages. Resolve suspected
meaning loss against Japanese and correct the complete affected chain.

**Reason:** A designated source preserves the intended meaning, and the shared
English version provides a consistent comparison point across translations.
This does not impose Japanese sentence structure or loanword conventions on
other languages.

### Per-language conventions

**Decision:** Apply these conventions to every text of the language in the
repository, including localized page data, handwritten documents, release
notes, and source comments, and extend the table when an inconsistency in a
language is settled. Quotation rows concern prose quotes; a string literal such
as `"Pyxel"` keeps its straight quotes in every language.

| Language | Convention | Basis |
| --- | --- | --- |
| All | A range of numeric or identifier values inside half-width parentheses with ASCII-only content uses the ASCII hyphen (`(0-2)`, `(CH0-CH3)`); a range in running text uses the language's range mark below; a range with a signed value, in either position, is written with the language's word for "to" (`-5 to +5`), while Japanese and Korean keep their range marks (`-5〜+5`, `-5~+5`); apostrophes are ASCII (`'`); the platform label `Web`, standing alone as a heading, tab label, or entry in a platform list, stays in Latin letters | Maintainer choice |
| English | Generic `web` is lowercase in running text and capitalized in product names; range mark `-`; entries of published release notes keep their wording | AP Stylebook, Chicago Manual of Style, and Microsoft Writing Style Guide for `web`; range mark is a maintainer choice |
| Japanese | The [typography decision](#japanese-spacing-punctuation-and-technical-spellings); range mark 〜 | Maintainer choice within common Japanese technical writing |
| Chinese | Half-width parentheses with a half-width space against adjacent Chinese text, as `scripts/format_prose` applies; range mark ～; instructions to the reader take the bare imperative for operating steps and 请 + verb for requests, including conditional instructions (要…，请…); 屏幕 for the Pyxel screen (the drawing target and its size, including the `pyxel-screen` element) and for the device display, 画面 for a page, panel, or scene of a tool or game, including the Pyxel Editor window; 瓦片 for a tile | Parentheses and spacing are the maintainer's tool-applied choice; GB/T 15834 for the range mark; terms are maintainer choices |
| Korean | 웹 for the web (`웹 버전`); range mark ~; quotation with “ ”; polite register (`…합니다` for statements, the `…세요` imperative for instructions to the reader); 화면 for the Pyxel screen while established loanwords and compounds such as 풀스크린, 스크린샷, 오프스크린, and 스크린 공간 stay; 리스트 for a Python list; 명령어 for a command-line or MML command (drawing commands are 그리기 명령); 레트로 for retro | 외래어 표기법 for 웹; 문장 부호 규정 for ~ and “ ”; register and terms are maintainer choices |
| Spanish | Quotation with « »; the reader is addressed as usted; `banco de imágenes` for an image bank; range mark `-` | RAE Ortografía for « » and `-`; register and term are maintainer choices |
| Italian | Quotation with « »; instructions to the reader use the infinitive, UI labels the imperative (`Esegui`); `banco immagini` for an image bank; range mark `-` | Maintainer choices among standard Italian forms; the imperative UI label follows the Microsoft Italian Style Guide |
| Portuguese | Quotation with “ ”; `aplicativo` for an application; `pixels` for pixels; range mark `-` | Maintainer choices among standard Brazilian Portuguese forms |
| French | Quotation with « »; instructions to the reader use the vous-imperative; `image` for an animation frame; range mark `-` | Lexique des règles typographiques for « »; Microsoft French Style Guide for the imperative; term and range mark are maintainer choices |
| German | Quotation with „ “; range mark –; instructions to the reader use the Sie-imperative; `Web-Version` for the web version | Duden for „ “, the Bis-Strich, and the hyphenated compound; Microsoft German Style Guide for the imperative |
| Russian and Ukrainian | Quotation with « »; range mark – | Правила русской орфографии и пунктуации; Український правопис |
| Turkish | Quotation with “ ”; range mark `-` | TDK Yazım Kılavuzu |

**Reason:** Each language keeps its own technical conventions, and a reader of
one language meets the same term and the same voice on every page. Where a
language offers more than one acceptable form, the recorded form is the one its
technical writing uses; recording it settles the question so that later edits do
not reopen it. The [translation chain](#translation-source-and-comparison-chain)
still decides meaning.

### Product names and author titles

**Decision:** Use these product names unchanged across languages: Pyxel, Pyxel
Cube, Pyxel Editor, Pyxel Showcase, Pyxel Code Maker, Pyxel MML Studio, Pyxel
Web Launcher, Pyxel User Examples, and Pyxel Composer. Pyxel Web or Pyxel for
Web, Pyxel MML, and Pyxel API may identify the web version, MML variant, and
public API respectively.

Write `Pyxel Editor`, not `Pyxel-Editor` or `ピクセルエディタ`. Keep author-titled
assets such as `laser-jetman` in their author's spelling.

**Reason:** Product names identify the same tools across translations and
links. An author's title is a separate proper noun, not a spelling defect to
normalize into Pyxel's product-name convention.

## Release Notes

### Entry scope and presentation

**Decision:** Include concrete features, fixes, performance changes, public API
changes, and useful dependency, runtime, toolchain, build, release-process, or
internal-structure changes. Omit test-only, policy-only, and ignore-file changes
unless they also change product, build, or release behavior.

Group changes into features or outcomes that readers can recognize. Multiple
implementation changes serving the same outcome belong in one entry; separate
files, commits, or internal fixes do not by themselves warrant separate entries.
Split entries when the outcomes are independently useful to describe.

Keep each entry on one line of at most 80 characters. Changes to an unreleased
feature fold into its introductory entry; qualifying documentation and
translation improvements form one summary entry.

**Reason:** Entries should present useful changes at comparable granularity. The
length and grouping conventions keep the list scannable, while the exceptions
retain changes that affect how the product is built or released. Specificity is
still required: grouping does not justify a generic cleanup claim.