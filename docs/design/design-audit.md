# Design Audit Procedure

The [Design Policy](design-policy.md) defines the governing standards.
[Decision records](#decision-records) specify concrete choices under those
standards, with their rationale and affected subjects. This procedure defines
how to select and verify those decisions, update them when warranted, and check
a candidate against them and the policy without omissions.

It is written for AI auditors and can also be followed by human reviewers. Read
the policy and the decisions relevant to the assigned scope, fix the inputs,
derive coverage before judging compliance, and resolve disagreements from
evidence, so repeated audits do not drift with the auditor's interpretation. If
the procedure or a decision contradicts the policy, resolve the conflict through
[revision review](#revisions-to-governing-documents) before applying the affected
criteria.

## Modes

| Mode | When | Required |
| --- | --- | --- |
| Gate check | Every change | The [triggered commands](#commands) |
| Targeted review | Every change other than formatter output or regenerated artifacts | Gate check; decision selection for the changed files and their relations; a [findings record](#audit-records) |
| Exhaustive audit | Before a release tag, or when explicitly requested | Complete coverage, the full record set, and independent review |

Call narrower work by its mode. Work without its record is not a completed
review of that mode. Previous reviews can suggest probes, but their verdicts do
not establish current compliance.

## 1. Select and Resolve Decisions

Derive the review topics and their complete affected families from the policy
and the requested scope, including unchanged files, generated outputs,
translations, tests, and embedded content. Existing decisions do not define the
universe of choices that need review.

Use the [decision index](#decision-records) to locate relevant records and read
the applicable entries with their referenced dependencies. Account for every
entry as selected or outside the scope with a reason; an exhaustive audit
selects them all. Check that every decision file appears in the index, and
derive the entries from the files themselves so an unlisted entry cannot
disappear. Review an entry's rationale once for its family, then verify its
application to every member; settle a topic's distinctions and uncovered cases
across the family rather than accumulating unrelated local edits.

Check in both directions: from decisions to all affected subjects, and from
candidate files and relationships to the decisions or policy rules that govern
them. Inspect source, released contracts, and original maintainer decisions;
neither the current implementation nor an earlier verdict proves a choice
correct. When inspection cannot settle a premise, use a bounded reproduction to
compare concrete alternatives, and treat it as evidence rather than an adopted
decision. Resolve the choice and its affected family before including the change
in the candidate; a passing test run does not decide what the contract should
be.

When a choice is missing, conflicting, or unsupported, consult the governing
policy. These conditions justify proposing a revision through the
[revision procedure](#revisions-to-governing-documents), not adopting it:

- Correct an entry when its premise was mistaken, evidence was overlooked, its
  scope conflates distinct cases, or an adopted contract change makes it stale.
  Correcting a misreading needs no accompanying source change.
- Add an entry when a concrete choice needs rationale beyond a direct
  application of the policy and recording it will prevent repeated
  interpretation or keep an affected family consistent. Do not record every
  mechanical correction, and do not record the groups or stages of one file or
  function; those belong to the code and its headings. When a source comment
  already states the local rationale and the policy settles the case, keep the
  inspection result in the audit records instead of adding an entry.
- Consolidate duplicate or conflicting entries at their owning policy section,
  preserving meaningful distinctions and updating affected references.
- Keep a supported decision when the candidate violates it; correct the
  candidate. Preference alone justifies neither a finding nor a new decision.

Each entry states the applicable scope and conditions, the resulting choice, and
sufficient rationale with references that let a reviewer inspect its basis,
using examples to clarify those conditions rather than to substitute for them.
Specify the required form independently of the candidate's current form or diff;
whether a correction is needed is an application result, not the decision. For
terminology and orthography, the required form comes from the language's
standard or a recorded convention; how often a form already occurs is not
evidence.
Preserve original maintainer answers in the audit records; keep incident
narratives out of the decision's explanation. Resolve public-contract changes or
unresolved specifications with the maintainer under the policy before making
dependent changes.

## 2. Prepare the Candidate

Before adopting a correction, establish why it follows from the reviewed
decisions and policy, including intended distinctions between comparable sites.
Use targeted checks that can establish the property being changed, then run the
complete triggered gates on the settled candidate; repeat them when changed
inputs or unresolved failures require it, rather than after every intermediate
adjustment.

### Commands

These requirements apply to every mode:

| Trigger | Required action |
| --- | --- |
| Code or formatter-managed document changes | Run `make format` after the changes and before committing them. |
| Changes affecting lint inputs or configuration, or an exhaustive audit | Run `make lint` and `make lint-wasm` warning-free. Clippy warnings fail the check; each suppression needs a specific justification. |
| Code changes | Run `make test` before claiming completion; when it stops at an earlier suite, run the remaining recipe lines of `make test` before claiming their result. |
| Changes to the GL context, shaders, or platform window or input code | Run `make run` on each affected platform; its startup check reports the installed build it exercised, and the record includes that line. |
| Changes to the web runtime (`wasm/`) or the launcher, showcase, or tool pages | Serve the pages locally (`make run-wasm`, or `scripts/start_showcase` with the tracked wheel) and start an example in a browser; the record names the served runtime origin and the console state. |
| Documentation or structured-data changes | Run applicable parsers, generators, consistency checks, and `git diff --check`. Verify generation at its source; do not hand-edit output. |
| Base-stub docstring changes | Edit the source data and regenerate with `scripts/generate_pyi_docstrings`. |
| Changes affecting generated or distributed artifacts | Regenerate them and verify their required correspondence with the source. |

Every automated test must be included in `make test`. Investigate failures, and
fix or remove tests that do not establish their claimed protection. In a
targeted review or exhaustive audit, record each command with its trigger,
governing rule, invocation, working directory, exit status, and log. A failed
command and an unrun command both fail their check, and an environment blocker
is recorded as such. A successful command proves only its specific obligation.

### Freezing an exhaustive audit target

For an exhaustive audit, freeze the target before planning: inventory every
tracked path and intended addition with its origin, Git mode, and content hash
(symlink target bytes, gitlink commit IDs); a sparse or skip-worktree checkout
blocks the freeze. Record the full base and head commits and the complete
base-to-candidate diff, and capture the policy, procedure, and decision files
with the candidate. Use the previous release commit as the base for a release
audit; otherwise the requested base, the merge base with the configured
upstream, or `HEAD`. The target is immutable within a run; any change to its
files, intended file set, or governing documents starts a new run and preserves
the previous records. Evidence-only additions may stay in the run but invalidate
every dependent result, which is rechecked rather than carried forward.

## 3. Plan Coverage

Build the plan from the policy and the inventories before recording results,
giving each criterion and subject a stable identifier.

- Criteria are the policy sections: each top-level rule and normative
  introductory paragraph belongs to its section's criterion with its source
  span. Account for every non-blank policy line as a criterion, an example of
  one, or non-normative structure, so an omitted requirement cannot disappear;
  an example does not limit the applicability of its rule.
  Decision entries supply concrete requirements under those criteria, including
  requirements expressed in tables and explanatory paragraphs, and cannot
  override the policy.
- Subjects are files, relations, hot paths, processes, and commands. Inventory
  every tracked file and intended addition with its path role and all applicable
  content roles: source, test, prose, translation, release notes, policy,
  configuration, and structured data, derived from contents rather than
  extensions. Files marked `binary` by `.gitattributes`, `*.tmx`, `*.bdf`,
  `Cargo.lock`, `*-lock.json`, `web/styles.css`, and Markdown beginning `<!--
  This file is generated` receive no direct text-style review; they keep their
  dependency checks. Define each criterion's selector over those roles, apply it
  to the complete inventory, and record the routing; a routing check verifies
  that every file received every applicable criterion; selection does not
  establish compliance. Use a process subject for obligations without a narrower
  subject, including manual cases with their expected observations and
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
- Hot paths are the performance-sensitive families, inventoried with their entry
  points and concrete cost risks: per-pixel blits and primitive drawing;
  per-pixel 3D rasterization and shading; per-sample voice synthesis; per-frame
  MML and BGM voice updates; per-frame 3D collision and BVH queries; PyO3
  argument marshaling and return paths; SIMD and multi-threaded sections. An
  absent family needs repository-wide evidence of its absence, and measurements
  can extend the list.
- Expected pairs: one check for each applicable criterion and file, the routing
  check, one for each relation and its governing criteria, one for each hot
  path, one for each process obligation, and one for each command. Map the
  selected decision entries onto those pairs; an entry mapped to no pair remains
  pending. A declared subject type with no instance requires an absence check,
  and a file check never replaces a relation check.

For a targeted review, the plan is the list of changed files, their relations,
and the decision entries selected for them; record it in the findings record.

For delegated work, supply the policy, procedure, relevant decision records,
the frozen target, inventories, assigned pairs, and dependencies. The lead
validates every returned result and its evidence and resolves disagreements
from the source; delegated summaries replace neither records nor independent
review.

## 4. Inspect and Verify

Inspect every selected subject and relation directly, and record one result per
expected pair: `pass` when evidence establishes the obligation, `fix` when a
specific obligation is violated, or `pending` when evidence, execution, or a
required decision is unresolved. Applicability was settled in the plan; there is
no `not applicable` verdict. Generated records preserve individual judgments and
their evidence; a missing judgment remains `pending`.

Each result states the expected property, what was inspected and observed, and
why the verdict follows, identifying the decision choices and boundaries
applied. A generic assurance, evidence about another property, a count, or a
search result does not establish compliance. Relation checks identify peers and
dependencies; performance checks include measurement or executed-path analysis;
manual cases record what was exercised and observed, separately from automated
results, and name the installed build they exercised. Defects identify the violated rule, the affected source or record, and
the concrete correction. Evidence chains end in frozen source, the target
inventory or diff, recorded decisions or historical evidence, or current command
logs, without cycles. Historical evidence establishes past behavior or intent;
current compliance still requires inspection of the frozen target.

Check changed implementation, documentation, and tests against the contract
under the policy, including defaults, accepted inputs, and intended differences
between surfaces; agreement among edited files does not establish correctness.
If inspection undermines a decision, return to [decision
resolution](#1-select-and-resolve-decisions) and apply the new-run rules.

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
the maintainer, not settled by repetition or majority. Compare inspection depth
across subject families: finding counts need not match, but a family with no
findings still needs evidence of comparable inspection, and unexplained
imbalance remains pending.

## 5. Review Independently

An exhaustive audit, any governing-document revision, and a correction that
applies one shared interpretation across several files require a reviewer who
did not produce the primary records; for a revision or such a correction, the
review is limited to the affected obligations and families, and a correction's
review is appended to the findings record. The reviewer first derives criteria,
subjects, expected pairs, and coverage comparisons from the frozen policy,
decision records, and source without opening the primary plan or verdicts;
verifies the inventory against tracked paths, intended additions, and deletions;
checks the decision-entry inventory and the mapping in both directions; inspects
each expected pair and records a verdict with its basis; and only then compares
coverage and judgments with the primary records. For corrections under existing
standards, establish both the violation in the previous source and the
compliance of the result.

Validate required fields, references, evidence chains, hashes, command outcomes,
coverage balance, and exact expected sets, checking logical keys as well as
identifiers (rule spans for criteria, paths for files, kind and family for other
subjects, trigger, invocation, and working directory for commands, and the
criterion and subject of each check) so that renaming an identifier cannot
conceal duplication or omission. Correct all actionable findings regardless of
severity, apply the new-run rules after corrections, and record the completed
review against the reviewed submission; the review's own checks remain pending
until that work is done, and the resulting record changes are reviewed before
completion. Without an independent reviewer, the work requiring that review
remains pending.

## Completion

Mark an exhaustive audit `complete` only when a fresh inventory and diff match
the frozen target; every decision entry is accounted for, every selected choice
has passing evidence, and every subject has a decision or direct policy basis;
every expected check, command, and manual case has current passing evidence with
no unresolved defect, decision, or blocker; the independent review passes; and
all records validate. Otherwise report `pending` and identify the remaining
work. A targeted review is complete when its gate check passes, its findings
record lists every selected decision with a verdict and every finding with its
resolution, and any independent review it requires passes.

## Revisions to Governing Documents

Identify the defect in the governing documents, why correcting their
application alone would not resolve it, and why the revision belongs at the
proposed layer. Correct implementation or review failures under supported
standards. Prepare the proposal separately from the audited candidate, keeping
current standards in force during evaluation.

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
only the approved diff, then follow the new-run rules. Keep dependent work
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
| [Structure and formatting](design-decisions/source-code-structure-and-formatting.md) | Definition order, settings and configuration groups, and file representation |
| [Blank lines](design-decisions/source-code-blank-lines.md) | Formatting ownership, processing groups, declaration inventories, comment boundaries, and the language-specific boundaries of Rust, Python, Web, GLSL, configuration, and tests |
| [Comments](design-decisions/source-code-comments.md) | Comment forms, concise rationale, group headings, and UI interface summaries |
| [Cross-file consistency](design-decisions/source-code-cross-file-consistency.md) | Message families, idiom exceptions, coordinate correspondence, examples, and Python node identity |
| [Rust/Python boundary](design-decisions/public-contract-rust-python.md) | Exception conversion, resource access, native failures, and reentrant Python calls |
| [Python contracts](design-decisions/public-contract-python.md) | Python APIs and stubs, CLI, resources and capture, audio, editors, and Cube behavior |
| [Web contracts](design-decisions/public-contract-web.md) | User-visible behavior of the web tools and compatibility routes |
| [Distribution contracts](design-decisions/public-contract-distribution.md) | Rust crate publication, Python and web compatibility, SDL2 linkage, wheel installation, release versions, optional dependencies, and executable export |
| [Documentation](design-decisions/documentation.md) | Reader scope, sources of generated text, organization of design documents, translations and per-language conventions, typography, names, and release notes |
| [Testing](design-decisions/testing.md) | Automated/manual coverage, mechanical correspondence checks, numerical expectations, and audio concurrency |

This is a file index, not the coverage inventory. Shared source choices also
apply to JavaScript, HTML, CSS, shaders, and other source or configuration
formats where relevant, and a file containing markup, scripts, and user-facing
prose can require several decision files. Unrecorded choices still require
review under the policy.

## Audit Records

Keep each run outside the target file set, identify it by a portable run name in
the final report, and use UTF-8 JSON or JSONL with ordinary escaping and unique
identifiers.

| Record | Used in | Required content |
| --- | --- | --- |
| `findings.md` | Targeted review | Changed files and relations, the selected decision entries with verdicts, each finding with its correction or pending reason, each gate command with its trigger, governing rule, invocation, working directory, exit status, and log, and any required independent review |
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
in the evidence hashes. Follow the policy's publication rules when preparing a
report, identifying any redactions and retaining the original evidence; a report
does not replace the records.
