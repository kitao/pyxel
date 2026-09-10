# Design Audit Procedure

The [Design Policy](design-policy.md) defines the governing standards.
[Decision records](#decision-records) specify concrete choices under those
standards, with their rationale and affected subjects. This procedure defines
how to select and verify those decisions, update them when warranted, and check
the candidate against them and the policy without omissions.

It is written for AI auditors and can also be followed by human reviewers. Read
the policy and the decisions relevant to the assigned scope. Fix the inputs,
derive coverage before judging compliance, and resolve disagreements from
evidence so repeated audits do not drift with the auditor's interpretation. If
the procedure or a decision contradicts the policy, resolve the conflict through
[revision review](#revisions-to-governing-documents) before applying the affected
criteria.

## Scope

Run an exhaustive audit when explicitly requested and before a release tag. The
[command requirements](#commands-and-expected-pairs) apply to ordinary changes
as well as exhaustive audits, according to their triggers. The complete
coverage, version-2 records, independent review, and completion procedure are
required for an exhaustive audit. Call narrower work a targeted review, fix
pass, or gate check; use the decision-selection and evidence steps for its
stated scope.

When revising the policy, procedure, or decision records, apply the
[revision review](#revisions-to-governing-documents) before relying on the
revised criteria. Also require [independent review](#4-review-independently)
for governing-document revisions and audit corrections that apply a shared
interpretation across multiple files, limited to the affected obligations and
families.

Cover every applicable file, rule, relation, hot path, and verification
command. Derive what must be checked before deciding whether it passes.
Previous reviews can suggest probes, but their verdicts do not establish
current compliance.

## 1. Prepare and Freeze

### Select and Review Decisions

Derive the review topics and their complete affected families from the policy
and the requested source scope, including unchanged files and embedded content.
Existing decisions do not define the universe of choices that need review.

Then use the [decision index](#decision-records) to locate relevant records and
inventory the entries in `design-decisions/` by file and section. Read their
scope to select the entries relevant to the requested audit, including shared
interfaces and dependencies. Account for every entry as selected or outside the
scope with a reason; an exhaustive repository audit selects them all. Check
that every decision file is present in the file index; derive its entries from
the file itself so an unlisted entry cannot disappear.

Preserve the evidence gathered here using the
[audit record conventions](#audit-records).

For each selected entry, identify its concrete choices and boundaries,
governing policy rules, evidence, and complete affected family. In the primary
review, review the rationale once for that family, then verify its application
across all members. Assign shared entries explicitly and share their evidence;
each auditor need only read the decisions and policy sections governing the
assigned scope.

Work through each source-derived topic across its family, using the reviewed
records and resolving missing or conflicting choices under the policy. Settle
the topic's distinctions and uncovered cases together rather than accumulating
unrelated local edits.

Check in both directions: from decisions to all affected subjects, and from
candidate files and relationships to the decisions or policy rules that govern
them. This finds stale or incomplete decision scopes as well as choices missing
from the records. Inspect source, released contracts, and original maintainer
decisions; neither the current implementation nor an earlier verdict proves the
choice correct. When inspection cannot settle a premise, use a bounded
reproduction or provisional implementation to compare concrete alternatives.
Treat that work as evidence, not an adopted decision. Resolve the choice and
its affected family before including the change in the candidate; a full test
run does not decide what the contract should be.

### Resolve and Update Decisions

Consult the governing policy when a choice is missing, conflicting, or
unsupported. The following conditions justify proposing a revision, not adopting
it; use the [revision procedure](#revisions-to-governing-documents) for any change
to a governing document:

- Correct an existing entry when its premise was mistaken, evidence was overlooked,
  its scope conflates distinct cases, or an adopted contract change makes it stale.
  No new source change is needed to correct an earlier misreading.
- Add an entry when a concrete choice needs rationale beyond a direct application
  of the policy, and recording it will prevent repeated interpretation or keep an
  affected family consistent. Do not record every mechanical correction.
- Consolidate duplicate or conflicting entries at their owning policy section
  and language. Preserve meaningful distinctions and update affected references.
- Keep a supported decision when the candidate violates it; correct the candidate.
  Preference alone justifies neither a finding nor a new decision. Do not change
  the policy or invent an exception to accommodate a local implementation.

Each entry states the applicable scope and conditions, the resulting choice,
and sufficient rationale, with references that let a reviewer inspect its basis.
Use examples to clarify those conditions, not to substitute for them. If the
source comment already states the complete local rationale and the policy
directly settles its placement, keep the inspection result in the audit records
rather than duplicating it as another decision.
Specify the required form independently of the candidate's current form or diff;
whether a correction is needed is an application result, not the decision.
Preserve original maintainer answers and historical evidence in the audit
records; keep incident narratives and the sequence of past reversals out of the
decision's explanation. Resolve public-contract changes or unresolved
specifications with the maintainer under the policy before making dependent
changes.

### Apply and Freeze the Candidate

Before adopting a correction into the candidate, establish why it follows from
the reviewed decisions and policy, including intended distinctions between
comparable sites. During candidate preparation, use targeted checks that can
establish the property being changed. Run the complete triggered gates on the
settled candidate; repeat them when changed inputs or unresolved failures
require it, rather than after every intermediate formatting adjustment.

Apply supported corrections, complete the required
[formatting and generation](#commands-and-expected-pairs), then freeze the
candidate using the [audit record format](#audit-records):

- Start with `git ls-files` and intended additions. Inventory every present path with
  origin (`tracked` or `intended`), Git mode, and content hash, including files
  later excluded from direct review. Hash symlink target bytes and gitlink commit
  IDs. Missing sparse-checkout or skip-worktree files block the freeze.
- Use the previous release commit as the base for a release audit. Otherwise use
  the requested base, the merge base with the configured upstream, or `HEAD` if
  no upstream exists. Record full commit IDs and every candidate change.
- Capture the policy, procedure, and decision files with the
  candidate. Write the target and complete diff before planning checks.

The target is immutable within a run. Any change to its files, intended file
set, policy, procedure, or decision records starts a new run; preserve the
previous run and findings. Evidence-only changes may stay in the run, but
invalidate every dependent result and review. Recheck those inputs rather than
carrying stale verdicts forward.

## 2. Plan Coverage

Build and validate [`plan.json`](#audit-records) from the policy and
inventories before recording results. Give each criterion and subject a stable
identifier within the plan.

### Criteria and Subjects

- Extract every top-level policy rule and normative introductory paragraph as a
  criterion with its source span and full obligation. Nested authoritative lists
  belong to their parent. Account for all normative text; examples and headings
  neither add criteria nor limit their applicability.
  Decision records supply concrete requirements and rationale under those
  criteria. Include their applicable choices in the checks; they cannot override
  the governing policy.
- Map every non-blank policy line in the plan to its criterion, an example of a
  criterion, or non-normative structure. Explain structural classifications so
  an omitted requirement cannot disappear from the coverage check.
- Bind the decision inventory to the frozen files. Map every selected choice and
  boundary to its policy criteria, affected subjects, and expected checks. Include
  requirements expressed in tables or explanatory paragraphs, not just text
  labeled `Decision`. An unmapped choice remains pending.
- Assign applicable subject types: file, relation, hot path, process, or command.
  Use a process subject for obligations without a narrower subject, including
  maintenance of the standards and verification process.
- Inventory every git-tracked file and intended addition. Files marked `binary`
  by `.gitattributes` receive no direct text-style review. Exclude the following
  generated or tool-maintained files from direct style review as well:
  - `*.tmx` and `*.bdf`;
  - `Cargo.lock` and `*-lock.json`;
  - `web/styles.css`;
  - Markdown whose first line begins with `<!-- This file is generated`.
  These exclusions do not remove the policy's dependency checks. Record each
  classification and preserve the complete target inventory.
- Included files have a path role and all applicable content roles: source, test, prose,
  translation, release notes, policy, configuration, and structured data.
  Derive overlapping roles from contents, not file extensions.
- For each file criterion, define its selector over those roles and apply it to
  the complete inventory. Add a routing check to prove selection completeness.
  Selection does not establish compliance. Keep excluded files in the dependency
  checks required by the policy.

Inventory relations by kind, family, full membership, governing criteria, and
source evidence for their selection. Cover all policy-governed relationships:
sibling conventions; public interfaces and their implementations; translations;
exception groups; changes and dependent tests, documentation, release notes,
generated outputs, and distributed artifacts; and the standards and their
verification. Derive sibling groups from directory, naming pattern, and shared
role; a file may belong to several families. Map applicable decision records to
these subjects using the reviewed scope and implementation references,
including affected interfaces and dependencies. For a subject with no decision
entry, identify its direct policy basis or the unresolved choice; absence is
neither an exemption nor evidence of compliance.

Inventory every implementation surface of these performance-sensitive families,
with entry points and concrete cost risks:

- per-pixel blits and primitive drawing;
- per-pixel 3D rasterization and shading;
- per-sample voice synthesis;
- per-frame MML and BGM voice updates;
- per-frame 3D collision and BVH queries;
- PyO3 argument marshaling and return paths;
- SIMD and multi-threaded sections.

An absent family needs repository-wide evidence of its absence. Extend this
inventory when measurements establish another performance-critical family; the
list does not limit the policy's performance-review obligation.

### Commands and Expected Pairs

Use mechanical checks where possible. List each required or targeted command
with its identifier, governing policy rule, invocation, working directory, and
trigger. These requirements also apply outside an exhaustive audit:

| Trigger | Required action |
| --- | --- |
| Code or formatter-managed document changes | Run `make format` after the changes and before committing them. |
| Changes affecting lint inputs or configuration, or an exhaustive audit | Run `make lint` and `make lint-wasm` warning-free. Clippy warnings fail the check; each suppression needs a specific justification. |
| Code changes | Run `make test` before claiming completion. |
| Documentation or structured-data changes | Run applicable parsers, generators, consistency checks, and `git diff --check`. Verify generation at its source; do not hand-edit output. |
| Base-stub docstring changes | Edit the source data and regenerate with `scripts/generate_pyi_docstrings`. |
| Changes affecting generated or distributed artifacts | Regenerate them and verify their required correspondence with the source. |

Every automated test must be included in `make test`.

Derive additional targeted checks from the frozen diff and decision mappings.
Record manual cases and expected observations as process subjects. Investigate
failures, fix or remove tests that do not establish their claimed protection,
and report unrun checks with their blockers rather than treating them as
passing.

Derive exactly one expected pair for each applicable criterion and subject,
including routing, relation, process, hot-path, and command checks. A declared
subject type with no instance requires an absence check; it passes only when
absence satisfies the rule. Missing selection or membership evidence remains
pending. A local file check never replaces a relation check.

For delegated work, supply the policy, procedure, relevant decision records,
the frozen target, complete inventories, assigned pairs, and dependencies. The
lead validates every returned result and its evidence and resolves
disagreements from the source. Delegated summaries replace neither records nor
independent review.

## 3. Inspect and Verify

Inspect every selected source and relation directly. Record
[one result](#audit-records) for each expected pair:

- `pass`: evidence establishes the obligation;
- `fix`: a specific obligation is violated;
- `pending`: evidence, execution, or a required decision is unresolved.

Applicability was settled in the plan; there is no `not applicable` verdict.
Apply the policy to the source and comparable subjects. Never create an
exception to close a finding or treat an unexplained difference as a defect.

Each result states the expected property, what was inspected and observed,
and why the verdict follows. A generic assurance or evidence about another
property does not establish compliance. Record generation must preserve
individual judgments and their evidence; missing judgments remain `pending`.
Relation checks identify peers and dependencies; performance checks include
measurement or executed-path analysis. Defects identify the violated rule,
affected source or record, and concrete correction. Evidence chains end in frozen source, the
target inventory or diff, recorded decisions or historical evidence, or current
command logs, without cycles. Historical evidence establishes past behavior or
intent; current compliance still requires inspection of the frozen target.

Check changed implementation, documentation, and tests against the contract
under the policy, including defaults, accepted inputs, and intended differences
between surfaces. Agreement among edited files does not establish correctness.
Each result identifies the applicable decision choices and boundaries and
states whether they are satisfied, violated, or unresolved, and why. If
inspection undermines a decision, return to
[Resolve and Update Decisions](#resolve-and-update-decisions) and apply the
run-invalidation rules. When judgments differ, record the interpretations and
resolve them from the governing policy and evidence; repeating the audit or
taking a majority vote does not resolve the disagreement.

For comments, inspect interface summaries, definition-group headings, and local
explanations separately. Check the members a heading actually describes, its
position, and its vocabulary against the complete peer family. Distinguish a
missing responsibility group from a self-contained declaration that needs no
extra label. Reviewing only edited comments does not establish these properties.

For release notes, verify each independent change against the code diff from
the previous release and apply the adopted entry scope and presentation.

For layout-only corrections, compare non-blank source, comments and literal
contents, and use applicable parsed or structural checks. Check whitespace
inside embedded programs, markup or data according to its consuming language;
blanket whitespace removal is not evidence of equivalent behavior. Keep these
preservation checks distinct from the review of readability and grouping.

Run every triggered command and preserve its output. Record failed commands
with exit status and unrun commands with their blockers; neither passes. Record
what manual cases actually exercised and observed, separately from automated
results. A successful command proves only its specific obligation. Counts,
search results, and summaries do not substitute for direct inspection.

Compare inspection depth across every criterion and subject family and their
applicable peers; identify the comparisons and evidence, or justify the absence
of peers. Finding counts need not match. A family with no findings still needs
evidence of comparable inspection. Unexplained imbalance remains pending.

## 4. Review Independently

A reviewer who did not produce the primary records first derives
[criteria, scope](#2-plan-coverage), subjects, expected pairs, and coverage
comparisons from the frozen policy, decision records, and source, without
opening the primary plan, verdicts, or findings. Verify the inventory against
tracked paths, intended additions, and deletions. Independently check the
decision-entry inventory, each choice's coverage, and the reverse mapping from
subjects to decisions or direct policy rules. Independently inspect each
expected pair and record its verdict and basis before reading the primary
judgments. For corrections under existing standards, establish both the
violation in the previous source and compliance of the result, preserving
justified differences. Then compare
coverage and judgments with the primary records and review every result and
its evidence.

Validate [required fields](#audit-records), references, evidence chains,
hashes, command outcomes, coverage balance, and exact expected sets. Check
logical keys as well as IDs: rule spans for criteria; paths for files; kind and
family for other subjects; working directory, invocation, and trigger for
commands; criterion/subject pairs for checks. Renaming an ID cannot conceal
duplication or omission.

Record a verdict and rationale for each reviewed check and coverage comparison,
and explicit results for expected-set comparisons and record validation.
Correct all actionable findings, regardless of severity. Apply the new-run and
evidence invalidation rules after corrections. Without an independent reviewer,
the work requiring that review remains pending.

Checks of independent review itself remain pending until that work is done.
Preserve the reviewed submission and record the completed review against it;
use that evidence to resolve those checks. Independently review the resulting
record changes before completion. This keeps the evidence chain acyclic
without assuming a review's own result or omitting its completion checks.

## Completion

Derive [`result.json`](#audit-records) from the final records. Mark the audit
`complete` only when:

- A fresh live inventory and diff exactly match the frozen target.
- Independently derived criteria, scope, subjects, pairs, and coverage comparisons
  match the final records, and every expected check and review passes.
- Every decision entry is accounted for, every selected choice and boundary has
  passing evidence, and every subject has a decision or direct policy basis.
- Every required command and manual case has current passing evidence, with no
  unresolved defect, decision, or blocker.
- All required records and logs exist; references, evidence chains, and hashes
  validate; derived counts match the records; and validation reports no errors.

Otherwise report `pending`, identify the remaining work, and preserve the
evidence.

## Revisions to Governing Documents

Identify the defect in the governing documents, why correcting their
application alone would not resolve it, and why the revision belongs at the
proposed layer. Correct implementation or review failures under supported
standards. Prepare the proposal separately from the audited candidate, keeping
current standards in force during evaluation.

Before applying a policy revision, compare the proposed and current
requirements and assess its consequences for the procedure and the complete
body of decisions and verification. Reread the entire policy for contradictions,
gaps, and uneven detail, and compare analogous rules and their verification.

For revisions to any governing document, compare previous and revised
requirements and record the reason and practical effect of each change. Review
the affected obligations, choices, implementation families, and audit checks
through [coverage planning](#2-plan-coverage) and [inspection](#3-inspect-and-verify).
A policy revision requires reassessing the procedure and the complete body of
decisions under the revised criteria; do not carry their earlier verdicts into
the new audit.

Exercise both known failures and valid cases against the revised rules and
procedure, including how omissions and conflicting verdicts are detected.
For source-layout conditions, include valid compact cases and inspect the
complete resulting source for readability; a clean diff or formatter result
does not establish that quality.
Preserve earlier records with the versions they evaluated; do not rewrite them
to fit the new rules. Revision review does not certify the repository.

Present the complete reviewed diff, its rationale, impact assessment, and
verification evidence for explicit maintainer approval before adoption.
Authorization to investigate or prepare a revision does not approve the
resulting changes. A changed proposal needs renewed review and approval;
apply only the approved diff, then follow the candidate-invalidation rules.
Keep unresolved dependent work pending while continuing work that the current
standards already settle.

## Decision Records

Use these files to locate decisions for the assigned scope. The files group
judgments by policy area and review subject, then by the interface or language
where that separates the work. Within a file, read the applicable entries and
their referenced dependencies.

| File | Read for |
| --- | --- |
| [Performance](design-decisions/source-code-performance.md) | Executed cost, numeric representations, arithmetic widths, and resource ownership |
| [Naming](design-decisions/source-code-naming.md) | Naming families and correspondence across interfaces |
| [Structure and formatting](design-decisions/source-code-structure-and-formatting.md) | Definition order, settings and configuration groups, and file representation |
| [Blank lines: shared](design-decisions/source-code-blank-lines.md) | Formatting ownership, processing groups, comment boundaries, and cross-language constant catalogues |
| [Blank lines: Python](design-decisions/source-code-blank-lines-python.md) | Python programs, embedded Python, editor construction, and tools |
| [Blank lines: Rust](design-decisions/source-code-blank-lines-rust.md) | Rust engine and binding groups, with their directly related test cases |
| [Blank lines: Web](design-decisions/source-code-blank-lines-web.md) | JavaScript, HTML, CSS, and Web runtime groups |
| [Blank lines: GLSL](design-decisions/source-code-blank-lines-glsl.md) | GLSL shader declarations and calculations |
| [Blank lines: configuration](design-decisions/source-code-blank-lines-configuration.md) | Configuration records, shell stages, and Makefile content |
| [Blank lines: tests](design-decisions/source-code-blank-lines-tests.md) | Rust, Python, and Web test fixtures, cases, and state lifecycles |
| [Comments](design-decisions/source-code-comments.md) | Comment forms, concise rationale, and UI interface descriptions |
| [Cross-file consistency](design-decisions/source-code-cross-file-consistency.md) | Message families, idiom exceptions, coordinate correspondence, examples, and Python node identity |
| [Rust/Python boundary](design-decisions/public-contract-rust-python.md) | Exception conversion, resource access, and reentrant Python calls |
| [Python contracts](design-decisions/public-contract-python.md) | Python APIs and stubs, CLI, resources, audio, editors, and Cube behavior |
| [Web contracts](design-decisions/public-contract-web.md) | User-visible behavior of the Web tools |
| [Distribution contracts](design-decisions/public-contract-distribution.md) | Rust crate publication, Python and Web compatibility, SDL2 linkage, wheel installation, release versions, and optional dependencies |
| [Documentation](design-decisions/documentation.md) | Reader scope, sources, translations, typography, names, and release notes |
| [Testing](design-decisions/testing.md) | Automated/manual coverage, numerical expectations, and audio concurrency |

This is a file index, not the coverage inventory or a list of supported
languages. Shared source choices also apply to JavaScript, HTML, CSS, shaders,
and other source or configuration formats where relevant. A file containing
markup, scripts, and user-facing prose can require several decision files.
Unrecorded choices still require review under the policy; derive entry coverage
from the actual decision files through [coverage planning](#2-plan-coverage).

## Audit Records

Keep each run outside the target file set and identify it by a portable run
name or relative reference in the final report. Use UTF-8 JSON or JSONL,
ordinary JSON escaping, unique identifiers, and record format version 2.
Earlier formats remain historical records.

| Record | Required content |
| --- | --- |
| `target.json` | Format version, branch, base and head commits, file inventory, and hash of `changes.patch`. |
| `changes.patch` | Complete base-to-candidate diff, including intended additions, deletions, modes, and binary changes. |
| `plan.json` | Target hash, criteria, decision inventory and mappings, scope decisions, subjects, commands, and expected criterion/subject pairs. |
| `checks.jsonl` | One result per expected pair: criterion, subject, verdict, rationale, evidence references, defect, and proposed correction. |
| `commands.jsonl` | Identifier, target and plan hashes, exact invocation, working directory, tool/environment details, exit status, and raw-log reference. |
| `review.json` | Reviewer identity, input hashes, independently derived expected sets and judgments, comparisons, review of every check, coverage balance, and record validation. |
| `result.json` | Input hashes, derived counts, unresolved defects and blockers, validation results, preceding runs, and completion status. |

Hash file bytes with SHA-256. The target hash is the hash of `target.json`,
binding the inventory and diff. Checks and command results identify the target
and plan hashes. Review records hash all inspected inputs, including logs; the
result hashes all final records and logs except itself. No record hashes
itself.

Repository paths and working directories are relative to the repository root;
record and log paths are relative to the run directory. Keep exact invocations
and original logs for independent verification. Follow the policy's publication
rules when preparing reports; identify any redactions and retain the original
evidence for review. A report prepared for publication does not replace that
evidence.

References identify source paths and line ranges or inspected whole files;
deleted content uses the frozen diff. Record references name the record and
identifier; log references name the log and relevant lines. Every reference
must resolve unambiguously. Preserve the source, date or version, and relevant
content of maintainer decisions and historical evidence used to establish the
contract; include these records in the evidence hashes.
