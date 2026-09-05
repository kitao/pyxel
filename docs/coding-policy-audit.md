# Coding Policy Audit Procedure

This procedure verifies the [Pyxel Coding Policy](coding-policy.md). The policy
alone defines the standards and exceptions; this document defines how to check
them and establish complete coverage.

It is written for AI auditors and can also be followed by human reviewers. Read
the policy first. Fix the inputs, derive coverage before judging results, and
resolve disagreements from evidence so repeated audits do not drift with the
auditor's interpretation. If the procedure contradicts the policy, correct that
conflict before accepting the audit.

## Scope

Use this procedure for an exhaustive audit required by the policy. Call narrower
work a targeted review, fix pass, or gate check. After revising either document,
review both for changed obligations, affected rule families, and corresponding
audit checks. Derive and recheck the affected subjects under the policy, using
the coverage and evidence steps below. Compare previous and revised requirements
and record the reason and practical effect of each change. Exercise both known
failures and valid cases against the revised rules and procedure, including how
omissions and conflicting verdicts are detected. Preserve earlier records with
the versions they evaluated; do not rewrite them to fit the new rules. That
revision review does not certify the repository.

Cover every applicable file, rule, relation, hot path, and verification command.
Derive what must be checked before deciding whether it passes. Previous reviews
can suggest probes, but their verdicts do not establish current compliance.

## Records

Keep each run outside the target file set and identify it by a portable run name
or relative reference in the final report. Use UTF-8 JSON or JSONL, ordinary JSON
escaping, unique identifiers, and record format version 2. Earlier formats remain
historical records.

| Record | Required content |
| --- | --- |
| `target.json` | Format version, branch, base and head commits, file inventory, and hash of `changes.patch`. |
| `changes.patch` | Complete base-to-candidate diff, including intended additions, deletions, modes, and binary changes. |
| `plan.json` | Target hash, criteria, scope decisions, subjects, commands, and expected criterion/subject pairs. |
| `checks.jsonl` | One result per expected pair: criterion, subject, verdict, rationale, evidence references, defect, and proposed correction. |
| `commands.jsonl` | Identifier, target and plan hashes, exact invocation, working directory, tool/environment details, exit status, and raw-log reference. |
| `review.json` | Reviewer identity, input hashes, independently derived expected sets, comparisons, review of every check, coverage balance, and record validation. |
| `result.json` | Input hashes, derived counts, unresolved defects and blockers, validation results, preceding runs, and completion status. |

Hash file bytes with SHA-256. The target hash is the hash of `target.json`, binding
the inventory and diff. Checks and command results identify the target and plan
hashes. Review records hash all inspected inputs, including logs; the result
hashes all final records and logs except itself. No record hashes itself.

Repository paths and working directories are relative to the repository root;
record and log paths are relative to the run directory. Keep exact invocations
and original logs for independent verification. Follow the policy's publication
rules when preparing reports; identify any redactions and retain the original
evidence for review. A report prepared for publication does not replace that
evidence.

References identify source paths and line ranges or inspected whole files;
deleted content uses the frozen diff. Record references name the record and
identifier; log references name the log and relevant lines. Every reference must
resolve unambiguously. Preserve the source, date or version, and relevant content
of maintainer decisions and historical evidence used to establish the contract;
include these records in the evidence hashes.

## 1. Prepare and Freeze

Before correcting a disagreement, apply the policy's contract and design rules.
Separate a demonstrated violation from an unresolved specification decision;
resolve the latter with the maintainer before making the dependent change.
Complete exploratory fixes, formatting, and generation, then freeze:

- Start with `git ls-files` and intended additions. Inventory every present path with
  origin (`tracked` or `intended`), Git mode, and content hash, including files
  later excluded from direct review. Hash symlink target bytes and gitlink commit
  IDs. Missing sparse-checkout or skip-worktree files block the freeze.
- Use the previous release commit as the base for a release audit. Otherwise use
  the requested base, the merge base with the configured upstream, or `HEAD` if
  no upstream exists. Record full commit IDs and every candidate change.
- Capture both documents with the candidate. Write the target and complete diff
  before planning checks.

The target is immutable within a run. Any change to its files, intended file set,
policy, or procedure starts a new run; preserve the previous run and findings.
Evidence-only changes may stay in the run, but invalidate every dependent result
and review. Recheck those inputs rather than carrying stale verdicts forward.

## 2. Plan Coverage

Build and validate `plan.json` from the policy and inventories before recording
results. Give each criterion and subject a stable identifier within the plan.

### Criteria and Subjects

- Extract every top-level policy rule and normative introductory paragraph as a
  criterion with its source span and full obligation. Nested authoritative lists
  belong to their parent. Account for all normative text; examples and headings
  neither add criteria nor limit their applicability.
- Map every non-blank policy line in the plan to its criterion, an example of a
  criterion, or non-normative structure. Explain structural classifications so
  an omitted requirement cannot disappear from the coverage check.
- Assign applicable subject types: file, relation, hot path, process, or command.
  Use a process subject for obligations without a narrower subject, including
  maintenance of the standards and verification process.
- Classify every file as included or excluded under the policy. Included files
  have a path role and all applicable content roles: source, test, prose,
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
verification. A file may belong to several families.

Inventory every implementation surface of each policy-listed hot-path family,
with entry points and concrete cost risks. An absent family needs a subject with
repository-wide evidence of its absence.

### Commands and Expected Pairs

List each required or targeted command with its identifier, governing policy
rule, invocation, working directory, and trigger. Include both lint targets in
every exhaustive run. Derive change-triggered checks from the frozen diff under
the policy. Record manual cases and expected observations as process subjects.

Derive exactly one expected pair for each applicable criterion and subject,
including routing, relation, process, hot-path, and command checks. A declared
subject type with no instance requires an absence check; it passes only when
absence satisfies the rule. Missing selection or membership evidence remains
pending. A local file check never replaces a relation check.

## 3. Inspect and Verify

Inspect every selected source and relation directly. Record one verdict for each
expected pair:

- `pass`: evidence establishes the obligation;
- `fix`: a specific obligation is violated;
- `pending`: evidence, execution, or a required decision is unresolved.

Applicability was settled in the plan; there is no `not applicable` verdict.
Apply the policy to the source and comparable subjects. Never create an exception
to close a finding or treat an unexplained difference as a defect.

Each result states what was inspected and why the verdict follows. Relation
checks identify peers and dependencies; performance checks include measurement
or executed-path analysis. Defects identify the violated rule, affected source
or record, and concrete correction. Evidence chains end in frozen source, the
target inventory or diff, recorded decisions or historical evidence, or current
command logs, without cycles. Historical evidence establishes past behavior or
intent; current compliance still requires inspection of the frozen target.

Check changed implementation, documentation, and tests against the contract under
the policy, including defaults, accepted inputs, and intended differences between
surfaces. Agreement among edited files does not establish correctness. Record
unresolved contract decisions as pending. When judgments differ, record the
competing interpretations and resolve them from the governing rule and evidence;
repeating the audit or taking a majority vote does not resolve the disagreement.

Run every triggered command and preserve its output. Record failed commands with
exit status and unrun commands with their blockers; neither passes. Record what
manual cases actually exercised and observed, separately from automated results.
A successful command proves only its specific obligation. Counts, search results,
and summaries do not substitute for direct inspection.

Compare inspection depth across every criterion and subject family and their
applicable peers; identify the comparisons and evidence, or justify the absence
of peers. Finding counts need not match. A family with no findings still needs
evidence of comparable inspection. Unexplained imbalance remains pending.

## 4. Review Independently

A reviewer who did not produce the primary records first derives criteria, scope,
subjects, expected pairs, and coverage comparisons from the frozen source,
without opening the primary plan, verdicts, or findings. Verify the inventory
against tracked paths, intended additions, and deletions. Record the expected
sets, then compare them with the primary records and review every result and its
evidence.

Validate required fields, references, evidence chains, hashes, command outcomes,
coverage balance, and exact expected sets. Check logical keys as well as IDs:
rule spans for criteria; paths for files; kind and family for other subjects;
working directory, invocation, and trigger for commands; criterion/subject pairs
for checks. Renaming an ID cannot conceal duplication or omission.

Record a verdict and rationale for each reviewed check and coverage comparison,
and explicit results for expected-set comparisons and record validation. Correct
all actionable findings, regardless of severity. Apply the new-run and evidence
invalidation rules after corrections. Without an independent reviewer, the audit
remains pending.

For delegated work, supply both documents, the frozen target, complete inventories,
assigned pairs, and dependencies. The lead validates every returned result and
its evidence and resolves disagreements from the source. Delegated summaries
replace neither records nor independent review.

## Completion

Derive `result.json` from the final records. Mark the audit `complete` only when:

- A fresh live inventory and diff exactly match the frozen target.
- Independently derived criteria, scope, subjects, pairs, and coverage comparisons
  match the final records, and every expected check and review passes.
- Every required command and manual case has current passing evidence, with no
  unresolved defect, decision, or blocker.
- All required records and logs exist; references, evidence chains, and hashes
  validate; derived counts match the records; and validation reports no errors.

Otherwise report `pending`, identify the remaining work, and preserve the evidence.
