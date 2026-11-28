# Design Audit Procedure

The [Design Policy](design-policy.md) defines the governing standards.
[Decision records](#decision-records) specify concrete choices under those
standards, with their rationale and affected subjects. This procedure defines
how to select and verify those decisions, update them when warranted, and check
a candidate against them and the policy without omissions.

It is written for AI auditors and can also be followed by human reviewers. Read
the policy and the decisions relevant to the assigned scope, establish the
inputs, derive coverage before judging compliance, and resolve disagreements
from evidence, so repeated audits do not drift with the auditor's
interpretation. If the procedure or a decision contradicts the policy, resolve
the conflict through [revision review](#revisions-to-governing-documents) before
applying the affected criteria.

## Modes

| Mode | When | Required |
| --- | --- | --- |
| Gate check | Every change | The [triggered commands](#commands) |
| Targeted review | Every change except mechanical formatting or regeneration that preserves established behavior, meaning, and expected results | Gate check; decision selection for the changed files and their relations; a [findings record](#audit-records) |
| Exhaustive audit | Before a release tag, or when explicitly requested | Complete coverage, the full record set, and independent review |

Report the mode performed and keep the [records for that mode](#audit-records).
For corrections and refinements made during an exhaustive audit, record their
before-and-after evidence in the exhaustive records alone. Revisions to the
governing documents also follow the [revision
procedure](#revisions-to-governing-documents).

In targeted reviews and exhaustive audits, each reviewer first records their
own assessment from current requirements, source, and evidence of use. Follow
[§5](#5-review-independently) for the cases requiring independent review.
Distinguish adopted maintainer decisions from reviewers' recommendations.
Previous reviews can suggest probes, but their verdicts do not establish
current compliance. After recording the initial assessment, check any
additional cases they identify against the candidate.

## 1. Select and Resolve Decisions

Derive the review topics and their complete affected families from the policy
and the requested scope, including unchanged files, generated outputs,
translations, tests, and embedded content. Existing decisions do not define the
universe of choices that need review.

Pyxel distributes separately authored applications in
`python/pyxel/examples/apps/`. Exclude their contents from project-wide checks,
checks of all examples, and consistency edits. Check the applications' behavior
and dependencies as supplied. Review explanatory accuracy when a specific
concern arises. Editing a supplied application requires an explicit maintainer
request.

Check the [decision index](#decision-records) against the files in the decision
directory. For a targeted review, select files by their subjects and the
affected families. Within each selected file, account for every entry and
follow dependencies across files. Record the selected entries, the excluded
files, and the excluded entries from selected files. A single reason may cover
a group when it explains every exclusion in it. For an exhaustive audit, derive
every entry from all the files and select them all.

Review the rationale of each selected entry once for its family, then verify
its application to every member. Resolve the distinctions and uncovered cases
across that family.

Check in both directions: from decisions to all affected subjects, and from
candidate files and relationships to the decisions or policy rules that govern
them. Inspect source, released contracts, and original maintainer decisions;
neither the current implementation nor an earlier verdict proves a choice
correct. When inspection cannot settle a premise, use a bounded reproduction to
compare concrete alternatives, and treat it as evidence rather than an adopted
decision. Resolve the choice and its affected family before including the change
in the candidate; a passing test run does not decide what the contract should
be.

Consult the governing policy when a choice is missing, conflicting, or
unsupported, or when comparing alternatives reveals a better way to serve the
intended task. Record compliance with the current rule and the case for an
improvement separately. Follow the [revision
procedure](#revisions-to-governing-documents) for proposed amendments. Use
the cases below to decide whether to correct, add, consolidate, or keep an
entry.

- Correct an entry when its premise was mistaken, evidence was overlooked, its
  scope conflates distinct cases, or an adopted contract change makes it stale.
  Correcting a misreading needs no accompanying source change.
- Add an entry when a concrete choice needs rationale beyond a direct
  application of the policy and recording it will prevent repeated
  interpretation or keep an affected family consistent. Do not record every
  mechanical correction or merely transcribe the current groups or stages of a
  file or function; those belong to the code and its headings. When a source
  comment already states the local rationale and the policy settles the case,
  keep the inspection result in the audit records instead of adding an entry.
- Consolidate duplicate or conflicting entries at their owning policy section,
  preserving meaningful distinctions and updating affected references.
- Keep a supported decision when the candidate violates it; correct the
  candidate. Preference alone justifies neither a finding nor a new decision.

Each entry states the applicable scope and conditions, the resulting choice, and
sufficient rationale with references that let a reviewer inspect its basis,
using examples to clarify those conditions rather than to substitute for them.
A separate **Boundary** paragraph is optional when limits on a decision need
more explanation.
Specify the required form independently of the candidate's current form or diff;
whether a correction is needed is an application result, not the decision. For
terminology and orthography, the required form comes from the language's
standard or a recorded convention. Frequency alone does not establish it.

Preserve original maintainer answers in the audit records; keep incident
narratives out of the decision's explanation. Resolve public-contract changes or
unresolved specifications with the maintainer under the policy before making
dependent changes.

## 2. Prepare the Candidate

Before adopting a correction, establish why it follows from the reviewed
decisions and policy, including intended distinctions between comparable sites.
Use targeted checks that can establish the property being changed, then run the
triggered commands on the settled candidate; repeat them when changed inputs
or unresolved failures require it, rather than after every intermediate
adjustment.

When refining a design, identify who uses it, the task they need to complete,
and the behavior to preserve. Trace that task through the current design and
the alternatives, including relevant support code and assets, even when the
current design meets its requirements. For a readability or simplification
change, follow the values, operations, and dependencies the reader needs to
understand. Compare the result, effort, and trade-offs for that reader.
Equivalent behavior alone does not establish a readability improvement. Adopt a
change when the evidence supports its benefit and the result to preserve.
Record the comparison in the targeted findings or the exhaustive audit checks.

### Commands

These requirements apply to every mode. For an exhaustive audit before a
release tag, use the previous release commit as the comparison base. Otherwise
use the requested base, or the merge base with the configured upstream if none
was requested. Fall back to `HEAD` when there is no upstream.

Treat the release version update and the commit adding its validated web wheel
as one candidate for the gate check. Run the required commands on the candidate
containing both. For the exhaustive audit, verify the candidate to be tagged.

Before any verification command that builds a wheel, retain the contents or
SHA-256 hashes of the tracked files the build regenerates. The frozen
candidate's matching records suffice. After the build, require byte-for-byte
agreement with those pre-build contents.

| Trigger | Required action |
| --- | --- |
| Every change | Run `git diff --check <base> --` on the complete candidate, including staged changes. Use a temporary index to include any intended additions that are still untracked. |
| Code or formatter-managed document changes | Run `make format` after the changes and before committing them. |
| Changes affecting lint inputs or configuration, or an exhaustive audit | Run `make lint` and `make lint-wasm` warning-free. Clippy warnings fail the check; each suppression needs a specific justification. |
| Changes to code, files read by the tests, or test configuration, or an exhaustive audit | Run `make test` before claiming completion; when it stops at an earlier suite, run the remaining recipe lines of `make test` before claiming their result. |
| Changes to code for GL, shaders, windows, or audio or input devices | Exercise the changed behavior on each affected platform. For native checks, run `make install` and `scripts/check_window_startup`, then use an example that exercises the affected rendering, window, sound, or input. Record the startup output and the observation separately. |
| Changes compiled into the wheel that alter browser-specific behavior or whose effect requires a browser to verify | Run `make run-wasm` on the complete candidate. Exercise the affected behavior in a browser with the development wheel built in `dist/`, using an automated case where it establishes the result. Record the wheel's SHA-256 and the observed result. |
| Changes under `wasm/` or `web/` to files a browser loads | Use the Showcase started by `make run-wasm` when the page also needs the development wheel. Otherwise start it with `scripts/start_showcase`. In a browser, open each changed page and one page that loads each other changed file, start an example on each opened page that runs one, and record the browser console output and the `pyxel.js` URL of each started example. |
| Changes to the `app2html` template | Open a page generated by `pyxel app2html` in a browser with its `pyxel.js` reference replaced by the runtime served by `scripts/start_showcase`, and record the `pyxel.js` URL and the browser console output. |
| Documentation or structured-data changes | Run applicable parsers, generators, and consistency checks. Verify generation at its source; do not hand-edit output. |
| Base-stub docstring changes | Edit the source data and regenerate with `scripts/generate_pyi_docstrings`. |
| Changes affecting generated or distributed artifacts | Regenerate affected build outputs from their sources and verify their correspondence. In ordinary tests, compare actual results with the [committed references](design-decisions/testing.md#screenshot-capture-and-actual-launch-coverage) without updating the references. Prepare affected distributions through their normal build process. For a release web wheel, follow the [retention rule](design-decisions/public-contract-distribution.md#web-wheel-retention). |

Include every automated test in `make test` except the [windowed startup
check](design-decisions/testing.md#division-between-make-test-and-make-run),
which runs under `make run`. Investigate failures, and fix or remove tests that
do not establish their claimed protection. In a targeted review or exhaustive
audit, record each command with its trigger, governing rule, invocation, working
directory, exit status, and log. A failed command and an unrun command both fail
their check, and an environment blocker is recorded as such. A successful
command proves only its specific obligation.

### Freezing an exhaustive audit target

For an exhaustive audit, freeze the target before planning: inventory every
tracked path and intended addition with its origin, Git mode, and content hash
(symlink target bytes, gitlink commit IDs); a sparse or skip-worktree checkout
blocks the freeze. Record the full base and head commits and the complete
base-to-candidate diff, and capture the policy, procedure, and decision files
with the candidate. Use the [comparison base](#commands) for the diff. The
target is immutable within a run; any change to its files, intended file set,
or governing documents starts a new run and preserves the previous records.
Evidence-only additions may stay in the run but invalidate every dependent
result, which is rechecked rather than carried forward.

In a new run, derive coverage and judge the new target. To reuse a recorded
observation, cite its original evidence and hash. Verify matching identities
and SHA-256 hashes for the subject, the applicable rule spans, and all inputs
on which the observation depends. For a relation, include its membership. For a
manual case, also require the same build hash and matching device and operating
conditions. Judge the observation anew. Repeat the check if a match cannot be
established or new evidence calls the observation into question. Run the
[triggered commands](#commands) and their automated checks on the settled
candidate. The same reuse conditions apply to observations from manual cases
required by the command table.

## 3. Plan Coverage

For an exhaustive audit, build the plan from the policy and the inventories
before recording results, giving each criterion and subject a stable identifier.

- Criteria are the policy sections: each top-level rule and normative
  introductory paragraph belongs to its section's criterion with its source
  span. Account for every non-blank policy line as a criterion, an example of
  one, or non-normative structure, so an omitted requirement cannot disappear;
  an example does not limit the applicability of its rule. Decision entries
  supply concrete requirements under those criteria, including requirements
  expressed in tables and explanatory paragraphs, and cannot override the
  policy.
- Subjects are files, relations, hot paths, processes, and commands. Inventory
  every tracked file and intended addition with its path role and all
  applicable content roles: source, test, prose, translation, release notes,
  policy, configuration, and structured data, derived from contents rather than
  extensions. Files marked `binary` by `.gitattributes`, `*.tmx`, `*.bdf`,
  `Cargo.lock`, `*-lock.json`, `web/styles.css`, and Markdown beginning
  `<!-- This file is generated` receive no direct text-style review. Retain
  their dependency checks. Define each criterion's selector over the applicable
  roles, apply it to the complete inventory, and record the routing. Verify
  that every file received every applicable criterion. Selection does not
  establish compliance. Use a process subject for obligations without a
  narrower subject, including manual cases with their expected observations and
  maintenance of the standards themselves.
- Relations cover every policy-governed relationship: sibling conventions;
  public interfaces and their implementations, stubs, reference data, and
  generated descriptions; translations; error families; changes and their
  dependent tests, documentation, release notes, generated outputs, and
  distributed artifacts; and the standards and their verification. Derive
  sibling families from directory, naming pattern, and shared role; a file may
  belong to several families. A subject with no decision entry needs its direct
  policy basis or its unresolved choice identified; absence is neither an
  exemption nor evidence of compliance. Prefer a mechanical relation check where
  one exists or can be added to `make test`.
- Hot paths are the performance-sensitive families, inventoried with their
  entry points and concrete cost risks. Cover per-pixel blits and primitive
  drawing, per-pixel Cube rasterization and shading, per-sample voice
  synthesis, and per-frame MML and BGM voice updates. Cover per-frame Cube
  scene traversal, motion sampling, vertex transforms, collision, and BVH
  queries. Include PyO3 argument marshaling and return paths, SIMD, and
  multi-threaded sections. An absent family needs repository-wide evidence of
  its absence. Measurements can extend the list.
- Expected pairs: one check for each applicable criterion and file, the routing
  check, one for each relation and its governing criteria, one for each hot
  path, one for each process obligation, and one for each command. Map the
  selected decision entries onto those pairs; an entry mapped to no pair remains
  pending. A declared subject type with no instance requires an absence check,
  and a file check never replaces a relation check.

For a targeted review, record the changed files, their relations, and the
[decision selection](#1-select-and-resolve-decisions) in `findings.md`.
Include obligations that follow directly from the policy.

For delegated work, supply the governing documents, the assigned scope,
inputs, checks, and dependencies. In an exhaustive audit, identify the frozen
target, inventories, and assigned pairs. The lead verifies the returned evidence
and resolves disagreements from the source. Independent review remains required
where specified below.

## 4. Inspect and Verify

Inspect each selected subject and relation. Record a result for each selected
obligation in a targeted review and for each expected pair in an exhaustive
audit. Use `pass` when direct evidence establishes the obligation, `fix` for a
specific violation, and `pending` for a missing judgment or unresolved
evidence, execution, or decision. If an obligation was omitted or misrouted,
correct the plan and recheck the affected results. Generated records preserve
each judgment and its evidence.

Each result states the expected property, what was inspected and observed, and
why the verdict follows, identifying the decision choices and boundaries
applied. A generic assurance, evidence about another property, a count, or a
search result does not establish compliance. Relation checks identify peers and
dependencies; performance checks include measurement or executed-path analysis;
manual cases record what was exercised and observed, separately from automated
results, and identify the build they exercised. Defects identify the violated
rule, the affected source or record, and the concrete correction. Evidence
chains end in the inspected source, inventory or diff, recorded decisions or
historical evidence, or current command logs, without cycles. Historical
evidence establishes past behavior or intent. Verify current compliance against
the candidate, using the frozen target for an exhaustive audit.

Check changed implementation, documentation, and tests against the contract
under the policy, including defaults, accepted inputs, and intended differences
between surfaces; agreement among edited files does not establish correctness.
If inspection undermines a decision, return to [decision
resolution](#1-select-and-resolve-decisions). For changes to a frozen target,
follow the [new-run rules](#freezing-an-exhaustive-audit-target).

Read one file once for all of its criteria rather than once per criterion, and
record the verdicts together; a result names the rules of its section that it
applied, and a `fix` names the violated rule. For comments, inspect interface
summaries, definition-group headings, and local explanations against the
complete peer family, checking the members a heading describes, its position,
and its vocabulary; reviewing only edited comments does not establish these
properties. For release notes, verify each independent change against the code
diff from the previous release under the adopted entry scope and presentation.
For layout-only corrections, compare non-blank source, comments, and literal
contents, and check whitespace inside embedded programs, markup, or data
according to its consuming language; blanket whitespace removal is not evidence
of equivalent behavior, and these preservation checks stay distinct from the
review of readability and grouping.

Never create an exception to close a finding or treat an unexplained difference
as a defect. When judgments differ, record the interpretations and resolve them
from the governing policy and evidence; a non-deterministic rule is escalated to
the maintainer, not settled by repetition or majority.

Inspection order may vary. Use the same method and depth wherever the same
obligation applies, including families with no findings. If a platform or
representation requires a different method, show that it detects the same
failure with equivalent coverage and record why the method differs. An
unexplained imbalance keeps the affected checks pending. Where individual steps
leave a user task's outcome uncertain, verify their interaction. Extend
inspection to close a specific evidence gap or assess a [design
alternative](#2-prepare-the-candidate) with a concrete benefit.

## 5. Review Independently

Independent review is required for an exhaustive audit, a governing-document
revision, and a correction or refinement that applies one interpretation across
several files. Use a reviewer who did not produce the primary records. Before
opening the primary plan or verdicts, this reviewer derives the applicable
criteria, subjects, relationships, and required checks from the governing
documents and source. Record a verdict and its basis for every check in scope,
then compare coverage and judgments with the primary records.

- For a correction or refinement that applies one interpretation across
  several files, inspect the affected family before and after the change. For
  a correction, establish the original violation and the result's compliance.
  For a refinement, compare costs and benefits to the intended user and verify
  that the result meets existing requirements.
- For a governing-document revision, follow the [revision
  procedure](#revisions-to-governing-documents). Identify the reviewed inputs
  by hash and record the checks in `review.json`.
- For an exhaustive audit, use the frozen target. Verify the file inventory
  against tracked paths, intended additions, and deletions. Check the decision
  inventory, the mapping in both directions, and every expected pair. Record
  the comparison and validation in `review.json`.

Validate the required fields, references, evidence chains, hashes, command
outcomes, coverage, and consistent inspection depth across families in the
records for the mode. For an exhaustive audit, compare exact sets by logical
keys as well as identifiers: rule spans, paths, subject kinds and families,
command triggers, invocations and working directories, and criterion–subject
pairs. Correct all actionable violations regardless of severity. Record
recommendations separately. Recheck affected evidence and revised records
before closing the review. Changes to a frozen target follow the [new-run
rules](#freezing-an-exhaustive-audit-target). Work requiring independent review
remains pending until that review passes.

## Completion

Mark an exhaustive audit `complete` only when a fresh inventory and diff match
the frozen target; every decision entry is accounted for, every selected choice
has passing evidence, and every subject has a decision or direct policy basis;
every expected check, command, and manual case has current passing evidence with
no unresolved defect, decision, or blocker; the independent review passes; and
all records validate. Otherwise report `pending` and identify the remaining
work.

A targeted review is complete when its decision selection is accounted for,
every selected obligation has passing evidence, and every finding is resolved.
Its findings record must show that the triggered commands, required manual
cases, and any required independent review passed.

## Revisions to Governing Documents

Identify the defect or the improvement a concrete alternative would deliver.
Explain why the change belongs at the proposed layer and what applying the
current requirement correctly would achieve. Correct implementation and review
failures under the standards that already settle them. Prepare the proposal
separately from the candidate. The current standards apply during evaluation.

Compare the proposed and current requirements, record the reason and practical
effect of each change, and assess the consequences for the procedure and the
complete body of decisions and verification, reviewing the affected
obligations, choices, implementation families, and checks through
[coverage planning](#3-plan-coverage) and [inspection](#4-inspect-and-verify).
Reread the entire policy for contradictions, gaps, and uneven detail, comparing
analogous rules and their verification. Exercise both known failures and valid
cases against the revised rules, including how omissions and conflicting
verdicts are detected; for source-layout conditions, include valid compact cases
and inspect the complete resulting source for readability, which a clean diff or
formatter result does not establish. Preserve earlier records with the versions
they evaluated; do not rewrite them to fit the new rules.

Present the complete reviewed diff, its rationale, impact assessment, and
verification evidence for explicit maintainer approval before adoption.
Authorization to investigate or prepare a revision does not approve the
resulting changes; a changed proposal needs renewed review and approval. Apply
only the approved diff. Reassess the affected obligations under the adopted
revision, freezing a new target for any exhaustive audit. Keep dependent work
pending while continuing work that the current standards already settle.

## Decision Records

Use these files to locate decisions for the assigned scope. The files group
judgments by policy area and review subject, then by the interface where that
separates the work. Within a file, read the applicable entries and their
referenced dependencies.

| File | Read for |
| --- | --- |
| [Performance](design-decisions/source-code-performance.md) | Executed cost, numeric representations, arithmetic widths, and resource ownership |
| [Naming](design-decisions/source-code-naming.md) | Naming families and correspondence across interfaces |
| [Structure and formatting](design-decisions/source-code-structure-and-formatting.md) | Definition order, runtime and editor/widget settings, configuration groups, file representation, local outputs and versioned deliverables, and localized page data |
| [Blank lines](design-decisions/source-code-blank-lines.md) | Formatting ownership, processing groups, declaration inventories, comment boundaries, and the language-specific boundaries of Rust, Python, Web, GLSL, configuration, shell scripts, and tests |
| [Comments](design-decisions/source-code-comments.md) | Comment forms, concise rationale, group headings, and UI interface summaries |
| [Cross-file consistency](design-decisions/source-code-cross-file-consistency.md) | Message families, idiom exceptions, coordinate correspondence, and examples |
| [Rust/Python boundary](design-decisions/public-contract-rust-python.md) | Exception conversion, resource access, native failures, and reentrant Python calls |
| [Python contracts](design-decisions/public-contract-python.md) | Python APIs and stubs, key constants and combined modifiers, app packaging and CLI, resources and capture, audio, editors, and Cube behavior |
| [Web contracts](design-decisions/public-contract-web.md) | Web runtime and tool behavior, compatibility routes, and the local Showcase server |
| [Distribution contracts](design-decisions/public-contract-distribution.md) | Rust crate publication, Python and web compatibility, SDL2 linkage, web wheel installation, retention and release procedure, release versions, optional dependencies, and executable export |
| [Documentation](design-decisions/documentation.md) | Reader scope, generated and handwritten documents, contributor workflows and commands, design document organization, translations and language conventions, typography, names, and release notes |
| [Testing](design-decisions/testing.md) | Automated and manual coverage, native and browser checks, extensionless Python tools, repository scripts, mechanical correspondence checks, numerical expectations, and audio concurrency |

This is a file index, not the coverage inventory. Shared source choices also
apply to JavaScript, HTML, CSS, shaders, and other source or configuration
formats where relevant, and a file containing markup, scripts, and user-facing
prose can require several decision files. Unrecorded choices still require
review under the policy.

## Audit Records

Keep each run outside the target file set and identify it by a portable run name
in the final report. Use UTF-8 for text records and ordinary escaping and unique
identifiers in JSON and JSONL records.

| Record | Used in | Required content |
| --- | --- | --- |
| `findings.md` | Targeted review | Changed files and relations, excluded decision files and entries with reasons, each selected decision or direct policy obligation with its verdict and evidence, each finding with its correction or pending reason, each triggered command with its trigger, governing rule, invocation, working directory, exit status, and log, each required manual case with its build, action, expected and observed result, and any required independent review |
| `target.json` | Exhaustive audit | Branch, base and head commits, file inventory with origins, modes, and hashes, and the hash of `changes.patch` |
| `changes.patch` | Exhaustive audit | Complete base-to-candidate diff, including intended additions, deletions, modes, and binary changes |
| `plan.json` | Exhaustive audit | Target hash, criteria with source spans, decision inventory and mappings, subject inventory with roles, routing, and exclusions, commands, and expected pairs |
| `checks.jsonl` | Exhaustive audit | One result per expected pair: criterion, subject, verdict, rationale, evidence references, defect, and correction, identifying the target and plan hashes |
| `commands.jsonl` | Exhaustive audit | Identifier, trigger, governing rule, invocation, working directory, environment details, exit status, log reference, and the target and plan hashes |
| `review.json` | Exhaustive audit and revision review | Reviewer identity, hashes of every inspected input including logs, independently derived expected sets and verdicts, comparison with the primary records, and validation results |
| `result.json` | Exhaustive audit | Hashes of all final records and logs except itself, derived counts, unresolved defects and blockers, preceding runs, and completion status |

Hash file bytes with SHA-256; the target hash is the hash of `target.json` and
binds the inventory and diff, and no record hashes itself. Repository paths are
relative to the repository root and record paths to the run directory.
References identify source paths and line ranges or inspected whole files
(deleted content through the frozen diff), record identifiers, or log lines, and
must resolve unambiguously. Preserve the source, date, and content of maintainer
decisions and historical evidence used to establish a contract, and include them
in the evidence hashes. Follow the policy's [Reader and Prose
standards](design-policy.md#reader-and-prose) for published reports. Identify
redactions and retain the original evidence and audit records.
