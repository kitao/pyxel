# Coding Policy Audit Procedure

This procedure establishes complete, even coverage of the
[Pyxel Coding Policy](coding-policy.md). Read the policy first; it owns the
standards and exceptions. Correct any conflict between these documents before
accepting an audit.

## Scope and Use

Run an exhaustive audit when explicitly requested and before a release tag.
For a substantive policy or procedure revision, review both documents and the
changed obligations, affected rule families, and audit records. That revision
review does not require an exhaustive repository audit unless one is requested.

Call narrower work a targeted review, fix pass, or gate check. An exhaustive
audit remains pending until the Completion Gate below is satisfied.

- Cover every in-scope file, applicable rule, cross-file relation, hot-path
  surface, and required verification command. Do not sample.
- Check comparable subjects to comparable depth. Follow a correction through
  its whole affected family rather than stopping at the first occurrence.
- Use previous audits and conversations to find useful probes, then recheck
  against the current target. Old verdicts, repository prevalence, and summaries
  do not establish current compliance.
- Derive applicability before verdicts. Examples clarify their parent rule;
  they neither define separate criteria nor limit the subjects selected.

## Records

Keep each run in a separate directory outside the target file set and name it
in the final report. Use UTF-8 JSON or JSONL for the records below, with ordinary
JSON escaping and readable, unique identifiers. This is record format version 2;
older runs retain their original format and are not current evidence.

| Record | Required content |
| --- | --- |
| `target.json` | Format version, branch, base and head commits, file inventory, and hash of `changes.patch`. |
| `changes.patch` | Complete diff from the base to the candidate, including intended additions, deletions, modes, and binary changes. |
| `plan.json` | Target hash, criteria, scope decisions, subjects, planned commands, and expected criterion/subject pairs. |
| `checks.jsonl` | One result per expected pair: criterion, subject, verdict, rationale, evidence references, and any defect and proposed correction. |
| `commands.jsonl` | One result per planned command: identifier, target hash, exact invocation, working directory, relevant tool/environment details, exit status, and raw-log reference. |
| `review.json` | Reviewer identity, input hashes, independently derived expected sets, comparison results, review of every check, coverage balance, and record validation. |
| `result.json` | Input hashes, derived counts, unresolved defects and blockers, validation results, preceding runs, and completion status. |

A source reference names a repository-relative path and line range, or the whole
file when that is the inspected unit. A record reference names the record and
identifier; log references name the log and relevant lines. Cite deleted content
through the frozen diff. Every reference must resolve unambiguously.

Use SHA-256 of file bytes for hashes. The target hash is the hash of
`target.json`, which binds the inventory and diff. Checks and command results
record the plan hash they used; review records hash every input they inspected,
including raw logs. The result hashes all final records and logs except itself.
No record includes its own hash as an input.

## 1. Freeze the Candidate

Finish exploratory fixes, formatting, and generation before freezing. The
candidate includes all tracked files and intended additions, including files
that the policy later excludes from direct review.

- Start from `git ls-files`. Inventory each present path with its origin
  (`tracked` or `intended`), Git mode, and content hash. Hash symlink target bytes
  for symlinks and the recorded commit ID for gitlinks. Missing sparse-checkout
  or skip-worktree files block the freeze; they are not implicit deletions.
- For a release audit, use the previous release commit as the base. Otherwise
  use the requested base, the merge base with the configured upstream, or
  `HEAD` if no upstream exists. Record full commit IDs and all candidate changes
  relative to that base.
- Capture the exact bytes of both policy documents with the other files. Write
  the target and diff before deciding verdicts.

The target is immutable within a run. A change to a target file, intended file
set, policy, or procedure requires a new run; preserve the earlier run and its
findings. Changes only to audit evidence may stay in the run, but invalidate
results and reviews that depend on the changed inputs. Recheck and replace stale
results rather than carrying their verdicts forward.

## 2. Plan Complete Coverage

Build and validate `plan.json` before recording verdicts. Give criteria and
subjects stable identifiers within the plan, and derive the expected pairs from
the rules and inventories, never from completed checks.

### Criteria and File Scope

- Extract each top-level policy rule and normative introductory paragraph as a
  criterion, with its source span and full obligation. Nested authoritative
  lists belong to their parent. Account for all normative text; distinguish
  examples and headings without creating criteria for them.
- For each criterion, identify every applicable subject type: file, relation,
  hot path, process, or command. A process subject covers an obligation with no
  narrower subject, including maintenance of these documents.
- Classify every target file as included or excluded, citing the scope rule.
  Included files have the path role and all applicable content roles: source,
  test, prose, translation, release notes, policy, configuration, and structured
  data. Roles overlap and follow content, not just extensions.
- State each file criterion's selector over those roles and apply it to all
  candidate files. Record a routing subject and check for each file criterion
  to prove selection completeness; this does not prove selected files comply.
  Retain excluded files in dependency checks where the policy requires source
  and generated or distributed output to correspond.

### Relations and Hot Paths

Inventory subjects with their kind, family, full membership or entry points,
governing criteria, and source references showing how completeness was derived.
A file may belong to several families. Cover all applicable relations:

- sibling naming, structure, comments, and error-message families;
- public API, binding, stub, and implementation counterparts;
- translated content and its source chain;
- exception groups, with the exact convention and policy reason;
- changes and their dependent tests, documentation, release notes, generated
  outputs, and distributed artifacts;
- the policy and this procedure, including changed obligations.

Include every policy-listed hot-path family and all its implementation surfaces.
Trace entry points and executed paths, naming actual cost risks found there.
A family with no implementation still needs an explicit absence subject and
repository-wide evidence. Examples in the policy are not a complete risk list.

### Commands and Expected Checks

Declare each required or targeted command with its identifier, governing rule
or procedure step, exact invocation, working directory, and trigger. Include
native and WASM lint in every exhaustive run, tests after code changes, and the
applicable formatting, generation, parsing, consistency, and diff checks from
the policy. Determine change-based triggers from the frozen diff. Declare
required manual cases and their expected observations as process subjects.

Derive one expected pair for every applicable criterion and subject. Include
routing checks, relation and hot-path checks, process obligations, and triggered
commands. If a declared subject type has no concrete subject, add an absence
check with evidence; absence passes only if it satisfies the rule. Missing
selection or membership evidence remains pending rather than silently removing
a check. A local file check never replaces a cross-file check.

## 3. Inspect and Record Results

Inspect every selected source and relation directly. Record one of these
verdicts for each expected pair:

- `pass`: current evidence establishes the obligation;
- `fix`: a specific policy or procedure obligation is violated;
- `pending`: evidence, execution, or judgment remains unresolved.

Applicability is settled in the plan, so there is no `not applicable` verdict.
Resolve semantic judgments from the rule, current source, and comparable sites;
do not invent an exception to close a finding.

Evidence identifies what was inspected and why the verdict follows. Relation
checks name peers and dependencies; performance checks include measurement or
executed-path analysis. A defect names the source or record, violated criterion,
and concrete correction. Bare statements such as “checked” or “no issue” are
insufficient.

Successful commands can prove their specific obligations, such as formatting or
a test run. They do not establish unrelated naming, prose, design, or coverage
quality. Search results help locate subjects; counts and summaries do not
replace source evidence. Evidence chains must end in frozen source, the target
inventory or diff, or current command logs, without cycles.

Run every triggered command and preserve its output. Record an unrun command
with its blocker and a failed command with its exit status; neither passes.
Bind each result to the frozen target and plan. Record required manual cases,
what was actually exercised, and observations separately from automated results.
If any check or command changes the target, follow the new-run rule above.

For every criterion and subject family, compare review depth with all applicable
peer families under that or analogous rules, or justify that no comparator exists.
Record the compared families and supporting checks. Fix counts need not match;
a family with no findings still needs evidence of comparable inspection.
Unexplained differences in depth require further inspection, not a summary
claim that the family was checked before.

## 4. Review Independently

A reviewer who did not produce the primary audit records independently derives
the criteria, file scope, subjects, expected pairs, and coverage comparisons from
the frozen source before opening the primary plan, verdicts, or findings. Verify
the target inventory independently against tracked paths, intended additions,
and deletions. Record these expected sets, then compare them with the primary
records. Review every result and its source evidence; sampling cannot clear
this step.

The reviewer also checks coverage balance and record integrity: required fields,
unique identifiers and logical keys, exact expected sets, valid verdicts,
references, evidence chains, input hashes, and command outcomes. Logical keys
are rule source spans for criteria, paths for files, kind and family for other
subjects, working directory/invocation/trigger for commands, and criterion/subject
pairs for checks. Different identifiers cannot conceal a duplicate or omission.

Record a verdict and rationale for every reviewed check and coverage comparison,
and explicit results for expected-set comparisons and record validation.
Actionable findings require correction regardless of severity. Corrected evidence
requires renewed review of its dependents; a changed target requires a new run.
If an independent reviewer is unavailable, the audit remains pending.

When work is delegated, supply both documents, the frozen target, complete
inventories, assigned expected pairs, and their dependencies. The lead reads
every returned result, validates its evidence, and resolves disagreements from
the source. Delegated summaries do not substitute for records or independent
review.

## Completion Gate

Write `result.json` from the final records. Mark it `complete` only when:

- A fresh inventory and diff from the live worktree exactly match the freeze.
- Target inventory, criteria, scope, subjects, expected pairs, and coverage
  comparisons match the independently derived sets, and every expected check
  and comparison passes.
- Every required command and manual check has current passing evidence, every
  required review passes, and no defect or blocker remains unresolved.
- All required records and logs exist, references and evidence chains resolve,
  input hashes are current, validation reports no errors, and summary counts
  agree with the underlying records.

Otherwise report `pending`, name the remaining work, and preserve the evidence.
