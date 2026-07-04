# Coding Policy Audit Procedure

This file defines how to run an exhaustive audit against
`docs/coding-policy.md`. It does not define coding standards, add exceptions, or
weaken the policy. If this procedure and the policy disagree, the policy wins
and this procedure must be corrected before the audit is accepted.

An auditor reads both files in this order:

- `docs/coding-policy.md`: the source of truth for standards, scope, examples,
  and completion requirements.
- `docs/coding-policy-audit.md`: the execution protocol used to prove that the
  policy was checked completely and evenly.

## Operating Rules

- No sampling. Every in-scope file, policy rule, sibling group, cross-file
  dependency, hot-path surface, and required verification gate is represented by
  an artifact row.

- No cherry-picking. A local fix is incomplete until comparable files and the
  affected group rows are checked to the same depth.

- No unstated memory. Previous conversations, old audit notes, and prior
  summaries may suggest probes, but they are not evidence until rechecked
  against the current worktree and recorded in the current artifacts.

- No summary-only evidence. Counts, grep success, formatter success, or reviewer
  summaries do not prove a cell unless the underlying row names what was
  inspected and why the verdict follows.

- Use precise result labels. A run is an exhaustive audit only when the
  Completion Gate is satisfied. Until then, or when the requested scope is
  intentionally narrower, report the work as a targeted review, fix pass, gate
  check, or pending audit according to the artifacts actually produced.

- The worktree under review is the audit target. Start from `git ls-files`, then
  add any untracked files intended for the audited change set. Apply the
  policy's Scope section to that combined file set.

- All artifacts are regenerated after meaningful fixes. Stale rows are treated
  as `pending`, not reused as proof.

## When to Run

Run this procedure:

- when an exhaustive coding-policy audit is explicitly requested;
- before a release tag, as part of the release checklist;
- after a substantive revision of `docs/coding-policy.md`, on every file whose
  verdict may change because of the revision;
- after a substantive revision of this procedure, on this procedure file and
  any affected audited files.

## Artifact Rules

Each audit run writes artifacts to a single run directory. The directory can be
temporary, but it must be named in the final report. Every TSV has a header row,
uses one row per checked unit, and has the declared column count on every row.
Fields contain no literal tabs or newlines; do not use CSV quoting as an escape
mechanism. Lists inside a field use JSON arrays or a stable comma-separated
format.

Every countable row has a stable key. The key is either an explicit id column or
the declared natural key for that artifact, such as `path` plus `criterion_id`
in `file_matrix.tsv`. Fields named `criterion_id` or `criterion_ids` reference
existing `criteria.tsv` rows. Missing keys, duplicate keys, unknown criteria,
invalid enum values, empty evidence, broken evidence references, missing
expected keys, unexpected keys, column-count mismatches, and row-count
mismatches are audit failures. Expected matrix keys are derived before verdicts
are written, not inferred from the produced rows afterward.

## Required Artifacts

- `criteria.tsv`
  - Columns: `criterion_id`, `policy_lines`, `policy_path`,
    `criterion_type`, `rule_text`, `example_families`.
  - Contains one row for every policy rule and every required example family.
  - `criterion_type` is one of `file`, `cross_file`, `group`, `process`, or
    `command`.

- `policy_coverage.tsv`
  - Columns: `policy_line`, `line_text`, `coverage_kind`, `criterion_ids`,
    `evidence`.
  - Maps every non-blank line in `docs/coding-policy.md` to one or more
    criteria, or marks it as structural text.
  - `coverage_kind` is `criterion` or `structural`. Criterion rows name one or
    more criteria; structural rows explain why no criterion is needed.
  - Structural coverage is allowed only for headings or explanatory text that
    imposes no checkable requirement.
  - A policy line with neither a criterion nor a structural explanation blocks
    completion.

- `scope.tsv`
  - Columns: `path`, `source`, `scope_status`, `reason`, `policy_lines`.
  - Contains every tracked file plus every untracked file intended for the
    audited change set.
  - `source` is `tracked` or `untracked_intended`; `scope_status` is
    `included` or `excluded`.
  - Exclusions cite the policy's Scope section and the concrete reason.

- `group_inventory.tsv`
  - Columns: `path`, `group_id`, `group_kind`, `peer_paths`,
    `singleton_reason`.
  - Assigns every included file to a sibling group or records why it is a
    singleton.
  - Groups are derived from directory, naming pattern, shared role, mirrored
    API, translated content, generated/manual relationship, or domain
    convention.

- `hot_path_inventory.tsv`
  - Columns: `hot_path_id`, `policy_lines`, `hot_path_kind`, `paths`,
    `entry_points`, `cost_risks_checked`, `verdict`, `evidence_ref`.
  - Enumerates every implementation surface for each hot-path family listed in
    `docs/coding-policy.md`.
  - `cost_risks_checked` names the concrete risks inspected, such as allocation,
    copying, conversion, bounds-check, dispatch, SIMD, inlining, or lock costs.
  - A hot-path family with no current implementation surface records a row with
    repository-wide evidence for that absence; silence is not evidence.

- `cross_dependencies.tsv`
  - Columns: `dependency_id`, `dependency_kind`, `paths`, `criterion_ids`,
    `policy_lines`, `derivation`.
  - Expands every cross-file concern into concrete file paths before checking
    begins.
  - `criterion_ids` is non-empty and names the exact criteria the dependency
    exercises.
  - Required families include, when present: sibling files, binding/API/stub
    mirrors, file and directory naming patterns, HTML-to-i18n key sets,
    translation source chains, localized `languages` arrays, generated/manual
    pairs, widget conventions, proper-noun usage, release notes against code
    diffs, and test coverage against changed behavior.

- `file_matrix.tsv`
  - Columns: `path`, `criterion_id`, `verdict`, `evidence`, `evidence_ref`.
  - Contains every included file crossed with every criterion.
  - For each included file, the `criterion_id` set matches `criteria.tsv`
    exactly: no omissions, extras, or duplicates.
  - If a criterion has no local surface in a file, use `pass` only when the
    evidence explains why the file cannot violate the criterion and names the
    artifact row that carries the authoritative check.

- `cross_matrix.tsv`
  - Columns: `dependency_id`, `criterion_id`, `verdict`, `evidence`,
    `evidence_ref`.
  - Contains every cross-file dependency crossed with the criteria it exercises.
  - For each dependency, the `criterion_id` set matches that dependency's
    `cross_dependencies.tsv` row exactly.

- `group_uniformity.tsv`
  - Columns: `group_id`, `criterion_id`, `verdict`, `evidence`,
    `evidence_ref`.
  - Contains every group crossed with every group criterion, plus any file or
    cross-file criterion whose evidence depends on sibling comparison.
  - Records whether each group was checked evenly and whether comparable files
    were fixed to the same degree.

- `process_matrix.tsv`
  - Columns: `gate_id`, `criterion_id`, `policy_lines`, `verdict`, `evidence`,
    `evidence_ref`.
  - Contains every process and command criterion from `criteria.tsv`.
  - Records repository-level checks such as policy self-applicability, release
    note evaluation, generated-file exclusions, command selection, and rerun
    requirements.

- `findings.tsv`
  - Columns: `finding_id`, `source_artifact`, `source_row`, `severity`,
    `path_or_dependency`, `criterion_id`, `finding`, `proposed_fix`.
  - `severity` is one of `blocker`, `high`, `medium`, or `low`. Use `blocker`
    when completion cannot be trusted, `high` for broad or user-visible policy
    risk, `medium` for localized policy violations, and `low` for narrow
    cleanup. Severity is not based on fix size.
  - Contains every `fix`, `review`, or `pending` verdict ever discovered in the
    run.
  - A non-`pass` matrix row without a corresponding finding row is an artifact
    failure. Fixed findings remain in this file and are resolved through
    `classifications.tsv`.

- `finding_distribution.tsv`
  - Columns: `probe_family`, `criterion_ids`, `finding_ids`, `finding_count`,
    `coverage_evidence_ref`, `imbalance_verdict`, `rationale`.
  - Summarizes the finding distribution across every named probe in the Minimum
    Probe Families. A top-level family row does not replace its named probes.
  - `coverage_evidence_ref` names the concrete artifact rows or commands that
    performed the comparable probe; this is required even when `finding_count`
    is zero.
  - `imbalance_verdict` is `pass`, `review`, or `pending`.

- `classifications.tsv`
  - Columns: `finding_id`, `disposition`, `design_intent`, `rationale`,
    `action_ref`.
  - Resolves every finding as `fixed`, `accepted`, or `deferred_blocker`.
  - `accepted` covers both false positives and true findings kept by a
    documented rationale; the rationale requirements are defined in the
    classification phase.
  - A `deferred_blocker` disposition blocks completion.

- `command_evidence.tsv`
  - Columns: `command_id`, `cwd`, `command`, `trigger`, `exit_status`,
    `key_output`, `artifact_ref`.
  - Records formatter, lint, test, structured-file, and targeted verification
    commands.
  - `exit_status` is a process exit code when the command ran, or `not_run` when
    user direction or the environment prevents execution. `not_run` rows name
    the blocker in `key_output`, and the related process gate remains `pending`.

- `current-diff.patch`
  - The exact diff being audited after the final fix batch, including intended
    untracked files as added-file patches.

- `audit_summary.json`
  - Contains counts for criteria, policy coverage, included scope, excluded
    scope, expected matrix rows and keys, actual matrix rows and keys, findings
    by disposition, hot-path inventory, cross and group criterion-pair coverage,
    findings by probe family, command results, and artifact-level errors.

## Verdicts

Every artifact verdict is one of:

- `pass`: the criterion is satisfied, with concrete evidence.
- `fix`: the criterion is violated and the file or group must change.
- `review`: the criterion may be violated and needs semantic judgment.
- `pending`: required evidence, verification, artifact generation, or review is
  incomplete.

Do not use `not applicable` as a verdict. If the local file has no surface for a
criterion, record `pass` with evidence that names the absence and points to the
cross-file, group, process, or command row that performs the real check.

Evidence is substantive only when it:

- names what was inspected;
- explains why the verdict follows;
- covers both the rule body's intent and the example family's concrete
  patterns;
- is specific to the file, dependency, group, or process gate;
- avoids bare phrases such as "no issue", "checked", or "matches policy".

Search output, file inventories, formatter success, test success, and reviewer
summaries are probes or command evidence, not findings by themselves. A finding
names the inspected line or artifact row, the policy criterion, and, for
cross-file or group concerns, the peer or dependency evidence that makes the
issue actionable.

`evidence_ref` holds one or more references separated by semicolons. Each
reference is a `path:line` or `path:first-last` location, a bare `path` for a
whole-file check, or an artifact-row reference — `process:<gate_id>`,
`cross:<dependency_id>`, `group:<group_id>`, `command:<command_id>`, or
`hotpath:<hot_path_id>` — naming the row that carries the authoritative check.
Any other form, or a reference to a row that does not exist, is a broken
evidence reference. After fixes, line references are rechecked against the
current file content before completion is claimed.

## Minimum Probe Families

The criteria extracted from the policy are authoritative. This section is a
minimum guard against common omissions; if the policy later adds a category, the
new category is still required through `policy_coverage.tsv`.

Artifacts must contain explicit rows for at least these probe families:

- Source code: hot-path inventory, hot-path performance, symbol naming, file and
  directory naming, ordering, comments, formatting, sibling-group consistency,
  exception groups, parallel mirrors, and `.pyi` effective defaults.

- Comments: English-only comments, intent-bearing comments, required headers for
  long blocks, group separator symmetry, label-vs-sentence punctuation,
  documentation-comment bans, domain convention comments, and self-contained
  wording.

- Testing: behavior that needs unit coverage, behavior left to screenshot or
  manual coverage, test claims that can actually fail, deterministic assertions,
  and test inclusion in `make test`.

- Documentation: natural prose, Japanese spacing, Japanese technical loanword
  spellings, parenthesis width, translation source chains, target-language
  conventions, product names, other proper nouns, and context labels that stand
  in for product names.

- Release notes: user benefit, maintainer breadcrumb, sub-change splitting,
  release-relative grouping, unshipped-change folding, category-specific
  wording, line length, diff verification, and documentation wording bundles.

- Verification: scope exclusions, generated or toolchain-output files, code-side
  aspects of prose files, formatter/lint/test triggers, targeted structured-file
  checks, and rerun requirements.

- Policy-document conventions: section placement, incident folding,
  authoritative enumerations, `e.g.` sub-bullet usage, language-specific rule
  statements, and whole-file balance after policy revisions.

- Enumeration freshness: every authoritative enumeration in the policy (hot
  paths, adopted spellings, product names, scope exclusions) is checked against
  the current repository for members the code has added but the list has not.
  Policy-named sibling and exception group examples are checked for current
  paths and rationales, but exhaustive group coverage comes from
  `group_inventory.tsv`, not from the examples.

## Phases

Run phases in order. A phase cannot close with `pending`, stale artifacts,
schema errors, or row-count errors. `fix` and `review` verdicts move to
classification and repair; verification and completion remain blocked until they
are resolved.

1. Freeze the audit target.
   - Record the current branch, comparison base commit, head commit,
     `git status --short`, and artifact directory.
   - Write `current-diff.patch`, including tracked changes and intended
     untracked files.
   - Add intended untracked files to `scope.tsv` even before they are staged.

2. Extract criteria from the policy.
   - Read `docs/coding-policy.md` from top to bottom.
   - Create criteria for each normative bullet and each required example
     family.
   - Map every non-blank policy line in `policy_coverage.tsv`.
   - Treat headings and explanatory prose as structural only when they do not
     impose a checkable requirement.

3. Build scope and groups.
   - Enumerate tracked files with `git ls-files`.
   - Add intended untracked files from `git status --short`.
   - Apply the policy's Scope exclusions and record every exclusion reason.
   - Assign every included file to a group or singleton row.

4. Build cross-file dependencies before judging files.
   - Expand every dependency family into concrete paths.
   - Include both obvious mirrors and indirect consistency surfaces, such as
     API signatures, localized strings, proper nouns, release notes, and tests
     for changed behavior.
   - If a dependency family is suspected but not yet enumerated, mark the
     related rows `pending`.

5. Build the hot-path inventory.
   - Create one row for every implementation surface of each policy-listed hot
     path family.
   - For each row, inspect the operation shape and name the concrete cost risks
     checked before recording a performance verdict.
   - A file-matrix performance pass may cite `hot_path_inventory.tsv`, but it
     does not replace the inventory row.

6. Fill the file matrix.
   - Cross every included file with every criterion.
   - Inspect the file itself for local surfaces: symbol names, path names,
     ordering, comments, formatting, tests, documentation prose, release-note
     relevance, and verification impact.
   - Record a verdict and evidence for every cell before aggregating results.

7. Fill the cross-file and group matrices.
   - Check naming agreement, sibling style, exception-group rules, parallel
     mirrors, generated/manual relationships, translation chains, and API/stub
     defaults.
   - Re-check group uniformity after every fix that touches one member of a
     group.
   - A local pass does not override a failing cross-file or group row.

8. Probe for imbalance.
   - Build or update `finding_distribution.tsv`.
   - Compare finding distribution across structurally similar groups and
     criteria.
   - Compare distribution across the Minimum Probe Families; a concentration in
     comments, documentation, or any other family does not prove other families
     clean by itself.
   - When one group produces findings and a comparable group produces none,
     re-run the same probes on the zero-finding group before accepting the
     result.
   - When one probe family produces findings and a related family produces none,
     cite the file, cross-file, group, process, or command rows that prove the
     zero-finding family was checked to comparable depth.
   - Treat unexplained imbalance as `pending`.

9. Classify and fix findings.
   - Fix clear violations.
   - For `review` findings, decide whether the current state follows policy
     intent or needs a change.
   - An `accepted` disposition requires a policy-derived rationale, or a design
     rationale that does not contradict the policy, in `classifications.tsv`. A
     category name alone is not evidence.
   - Do not add reusable acceptance categories here. If a recurring pattern
     should be generally allowed, update `docs/coding-policy.md` first;
     otherwise keep the rationale local to the finding.

10. Regenerate artifacts after fixes.
    - Rebuild criteria if the policy changed.
    - Rebuild scope if files were added, removed, generated, or renamed.
    - Rebuild hot-path inventory, file, cross-file, group, process, findings,
      finding distribution, classifications, and command artifacts after every
      meaningful fix batch.
    - Validate regenerated artifacts against the Artifact Rules.

11. Run verification commands.
    - Run `make format` after code or formatter-managed document changes.
    - Run `make lint` and `make lint-wasm` after code or web changes.
    - Run `make test` after code changes.
    - Run targeted checks for touched structured files, such as `jq empty` for
      JSON.
    - Run prose or documentation tests when touched documents are covered by
      them.
    - Run `git diff --check`.
    - Record command names, working directories, exit statuses, and key output
      in `command_evidence.tsv`.
    - If a required command is forbidden by the user or unavailable in the
      current environment, record the gate as `pending` with the reason; do not
      infer `pass` from file inspection or neighboring commands.

12. Run independent review.
    - Give the reviewer the full policy, this procedure, the complete file
      list, all cross-file dependencies, the current diff, and all artifacts.
    - Ask for actionable findings in the edited files and in the audit design.
    - If the reviewer finds a blocker, high, or medium issue, fix it,
      regenerate artifacts, and repeat independent review.
    - If no independent reviewer is available, record `pending`; do not claim
      completion.

## Delegation Rules

When the audit is split across multiple auditors, the lead auditor gives each
auditor:

- the full text of `docs/coding-policy.md`;
- this procedure;
- the complete scope list;
- the full criteria list;
- the complete group inventory;
- the complete cross-dependency inventory;
- the assigned artifact rows, not a vague file category;
- the current diff.

Delegated work is not accepted from summaries. The lead auditor reads the
delegated rows, checks the evidence, and marks weak or copied evidence
`pending`. If assignments overlap, conflicting verdicts are resolved by
re-reading the files and updating the artifacts.

## Completion Gate

The audit is complete only when all conditions are true:

- `policy_coverage.tsv` covers every non-blank line of
  `docs/coding-policy.md`.
- `scope.tsv` contains every tracked file and every intended untracked file,
  either included or excluded with a policy-backed reason.
- `group_inventory.tsv` assigns every included file to a group or singleton.
- `file_matrix.tsv` covers every included-file and criterion pair exactly once;
  its row count equals included-file count multiplied by criteria count, and
  every included file has exactly the criterion set from `criteria.tsv`.
- `hot_path_inventory.tsv` covers every policy-listed hot-path family, and every
  row verdict is `pass`.
- `cross_matrix.tsv` covers every dependency and exact dependency/criterion pair
  declared by `cross_dependencies.tsv` exactly once.
- `group_uniformity.tsv` covers every group and every required group/criterion
  pair exactly once, plus any file or cross-file criterion routed to a group
  row.
- `process_matrix.tsv` covers every repository-level and command-level policy
  requirement, and every process or command criterion in `criteria.tsv`.
- `command_evidence.tsv` records every required and targeted command with its
  working directory, exit status, and key output.
- `current-diff.patch` includes every changed tracked file and every intended
  untracked file.
- Every non-`pass` matrix verdict has a corresponding row in `findings.tsv`.
- Every row in `findings.tsv` has a resolved row in `classifications.tsv`.
- `finding_distribution.tsv` covers every named probe in the Minimum Probe
  Families, and every `imbalance_verdict` is `pass`.
- `classifications.tsv` contains no `deferred_blocker`.
- `audit_summary.json` reports zero row-count mismatches, zero column-count
  mismatches, zero invalid enum values, zero duplicate keys, zero missing
  expected keys, zero unexpected keys, zero empty evidence fields, zero broken
  evidence references, and zero stale artifacts.
- Required commands have passed, or a documented reproducible environment
  failure is followed by a successful approved rerun.
- Independent review reports no blocker, high, or medium actionable findings.

Do not claim completion before the gate is satisfied. If any condition is
uncertain, the verdict is `pending`.
