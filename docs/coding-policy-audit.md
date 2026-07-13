# Coding Policy Audit Procedure

This file defines how to run an exhaustive audit against
`docs/coding-policy.md`. It does not define coding standards, add exceptions, or
weaken the policy. If this procedure and the policy disagree, the policy wins
and this procedure must be corrected before the audit is accepted.

An auditor reads both files in this order:

- `docs/coding-policy.md`: the source of truth for standards, scope, and
  examples.
- `docs/coding-policy-audit.md`: the execution protocol used to prove that the
  policy was checked completely and evenly.

## Operating Rules

- No sampling. Every in-scope file, normative criterion, applicable subject,
  cross-file relation, hot-path surface, and required verification gate is
  represented by an artifact row.

- No cherry-picking. A local fix is incomplete until every comparable subject
  and affected relation is checked to the same depth.

- No unstated memory. Previous conversations, old audit notes, prior summaries,
  and repository prevalence may suggest probes, but they are not evidence until
  rechecked against the frozen target and recorded in the current artifacts.

- No majority rule. Explicit policy determines departures from the relevant
  language's established idiom; otherwise that idiom determines the correct
  form. Existing repetition is used only to discover related subjects and uneven
  application.

- No example criteria. An example illustrates its parent rule and is recorded
  as example coverage; matching or differing from the example alone never
  determines a verdict.

- No summary-only evidence. Counts, search results, formatter success, test
  success, and reviewer summaries do not prove a check unless its row names what
  was inspected and why the verdict follows.

- Use precise result labels. A run is an exhaustive audit only when the
  Completion Gate is satisfied. Until then, or when the requested scope is
  intentionally narrower, report the work as a targeted review, fix pass, gate
  check, or pending audit according to the artifacts actually produced.

- The worktree under review is the audit target. Start from `git ls-files`, add
  intended untracked files, and apply the policy's Scope section to that combined
  set.

- The frozen target is immutable within a run. If its files, intended additions,
  policy, or procedure change, preserve the run and start another. Regenerate an
  artifact when only one of its recorded in-run inputs changes; a stale row is
  `pending`, never reused as proof. The final report names every preserved run
  that led to the completed one.

## When to Run

Run this procedure:

- when an exhaustive coding-policy audit is explicitly requested;
- before a release tag, as part of the release checklist;
- after a substantive revision of `docs/coding-policy.md`, on every subject
  whose verdict may change because of the revision;
- after a substantive revision of this procedure, on this procedure and every
  affected audit artifact or subject.

## Artifact Rules

Each run writes artifacts to one directory named in the final report. Every TSV
has a header, uses one row per declared unit, and has the declared column count
on every row. Text artifacts use UTF-8 without a BOM, LF line endings, and one
terminal LF. JSON objects use the declared key order and no insignificant
whitespace. TSV fields contain no literal tabs or newlines. Lists are compact
JSON arrays with no duplicate logical entries; an aligned value list retains one
value per key even when values are equal. Reference lists preserve the defining
artifact's row order, enum lists preserve this procedure's declaration order,
and other lists are sorted bytewise unless a recorded source command defines
their order.

Path values encode repository-relative Git path bytes: `/` and ASCII letters,
digits, `.`, `_`, and `-` remain literal; every other byte is `%HH` with uppercase
hex digits. Plural TSV fields are JSON arrays unless their schema says otherwise;
`policy_line` is one policy source reference and `policy_lines` is an array of
them. Artifact rows are sorted bytewise by their identifier.

Every data row has the explicit identifier declared by its artifact. Except for
the separately defined file and artifact IDs, an ID is its column name without
`_id`, followed by `-` and the lowercase SHA-256 of its natural-key JSON array.
That array uses the key values below in listed order, with no whitespace, only
required JSON escapes, and literal non-ASCII characters. Each
verdict-independent row ID is assigned before that row's verdict; finding and
review IDs are computed when those rows are created. IDs use the path-encoding
alphabet and never contain `:`.

Completeness of verdict-independent sets is validated from natural keys, never
from IDs or observed verdicts; post-verdict sets follow their declared
derivations. Natural keys are: policy source lines and normalized rule text for
criteria; policy line for policy coverage; artifact path for the manifest;
`path` for target files and scope; kind and family for relations; family for hot
paths; `cwd`, command, and trigger for commands; criterion and subject for
applicability and checks; source and target references plus finding text for
findings; criterion and subject family for balance rows; and review kind and
subject key for independent-review rows.
Missing or duplicate identifiers or natural keys, unknown criteria or subjects,
invalid enum values, empty required fields, and column-count errors are artifact
failures.

An evidence reference is one of:

- `source:<path>:<line>` or `source:<path>:<first>-<last>`;
- `source:<path>` for a whole-file check;
- `row:<artifact>:<row_id>` for a TSV row;
- `artifact:<artifact>`, optionally followed by `:<line>` or
  `:<first>-<last>`, for whole-artifact or raw-line evidence.

Source lines are one-based and ranges are inclusive. Paths use the encoding above;
`artifact` is a required artifact filename. A deleted path is cited through its
hunk in `current-diff.patch`, not as current source.

Every reference resolves to exactly one current source location, artifact
location, or row. The evidence graph is acyclic and terminates in current source,
a frozen-target root artifact, or a successful target-bound command row. Broken,
cyclic, or unterminated evidence blocks completion.

`manifest.tsv` binds manifested artifacts to their exact inputs and the frozen
target. A manifested artifact is stale when its current hash, input hashes, or
`target_sha256` differs from the manifest. The independent review is stale when
its target or manifest hash differs; the summary is stale when any of its three
hashes differs. Immediately before completion, the target snapshot and diff are
independently re-derived from the live worktree; comparing stored artifacts with
one another is insufficient. No artifact hash, fingerprint, or verdict is an
input to its own computation. Identifiers only locate units derived from natural
keys; an observed identifier set never proves its own completeness.

## Required Artifacts

- `target_files.tsv`
  - Columns: `file_id`, `path`, `source`, `git_mode`, `content_sha256`.
  - Contains every tracked path not deleted by the frozen diff, plus every
    intended untracked file, whether the policy later includes or excludes it.
    A missing sparse- or skip-worktree path blocks the freeze rather than being
    treated as a deletion. `source` is `tracked` or `untracked_intended`.
  - `file_id` equals the encoded `path`. `git_mode` is the six-digit mode the
    frozen target tree would record. `content_sha256` hashes the working-tree
    bytes, the link-target bytes for a symlink, or the hexadecimal bytes of the
    recorded commit ID for a gitlink.

- `freeze.json`
  - Contains exactly: `schema_version`, `branch`, `base_commit`,
    `head_commit`, `policy_sha256`, `procedure_sha256`,
    `target_files_sha256`, `diff_sha256`, `target_sha256`, and `artifact_dir`.
  - `schema_version` is the integer `1` for this schema and increments whenever a
    required artifact's fields or meaning change. `branch` is the short branch
    name using the path-byte encoding above, or null; commits are full object
    IDs; digests are lowercase SHA-256 strings; and `artifact_dir` is the absolute
    run-directory path.
  - `base_commit` is the previous release commit for a release audit; otherwise
    it is the explicitly requested comparison base, the merge base of `HEAD` and
    its configured upstream when no base was requested, or `HEAD` when no
    upstream exists.
  - `target_sha256` is the SHA-256 of a UTF-8 JSON array containing, in order,
    `schema_version`, `branch`, `base_commit`, `head_commit`, `policy_sha256`,
    `procedure_sha256`, and `target_files_sha256`. The serialization uses no
    whitespace and only required JSON escapes.
  - The object records the target before any verdict is written.

- `current-diff.patch`
  - Contains the exact diff against `base_commit`, including intended untracked
    files as added-file patches, in Git's binary-capable full-index format.
    The tracked diff comes first; added-file patches follow in encoded-path order.
  - Its SHA-256 equals `freeze.json.diff_sha256`.

- `manifest.tsv`
  - Columns: `artifact_id`, `path`, `sha256`, `input_artifact_ids`,
    `input_sha256s`, `target_sha256`.
  - Contains one row for every required artifact except itself,
    `independent_review.tsv`, and `audit_summary.json`.
  - `artifact_id` is the artifact filename and `path` is its path relative to the
    run directory.
  - Input ID and hash arrays have equal length and preserve dependency order.
  - The dependency graph is acyclic and contains these direct edges:
    - `target_files.tsv` and `current-diff.patch` are roots;
    - `freeze.json` depends on both roots;
    - `criteria.tsv` and `policy_coverage.tsv` depend on `freeze.json`;
    - `scope.tsv` depends on `target_files.tsv`, `criteria.tsv`, and
      `policy_coverage.tsv`;
    - `relations.tsv`, `hot_paths.tsv`, and `commands.tsv` depend on
      `criteria.tsv` and `scope.tsv`, with change-driven inventories also
      depending on `current-diff.patch`;
    - `applicability.tsv` depends on `criteria.tsv`, `scope.tsv`, `relations.tsv`,
      `hot_paths.tsv`, and `commands.tsv`;
    - `checks.tsv` depends on `applicability.tsv` and `commands.tsv`;
    - `coverage_balance.tsv` depends on `criteria.tsv`, `applicability.tsv`, and
      `checks.tsv`;
    - `findings.tsv` depends on `checks.tsv` and `coverage_balance.tsv`.
  - Each row also names every additional direct artifact input it actually uses.

- `criteria.tsv`
  - Columns: `criterion_id`, `policy_lines`, `subject_types`,
    `surface_kinds`, `file_selector`, `rule_text`.
  - Contains one row for each top-level rule bullet and each normative
    introductory paragraph in `docs/coding-policy.md`.
  - Nested authoritative enumerations belong to their parent criterion.
    `e.g.` bullets and hypothetical anti-patterns never create criteria.
  - `subject_types` is a non-empty JSON array drawn from `file`, `relation`,
    `hot_path`, `process`, and `command`.
  - `file` governs a file surface; `relation` compares files; `hot_path` governs
    a policy-listed performance family; `process` governs a repository-wide or
    audit obligation with no narrower subject; and `command` requires execution.
    Every applicable type is listed.
  - `surface_kinds` is a JSON array used only by `file` criteria and drawn
    from `path`, `source`, `test`, `prose`, `translation`,
    `release_notes`, `policy`, `configuration`, and `structured_data`.
    It is non-empty exactly when `subject_types` contains `file`.
  - `file_selector` is empty for a non-file criterion. Otherwise it states the
    rule-derived predicate that selects governed content within the candidate
    surface kinds. It is `all` only where the rule governs every candidate file;
    examples and hand-picked repository paths cannot define it.
  - `rule_text` is the normative text with internal whitespace collapsed to one
    ASCII space. Any text change changes the criterion ID; a semantic addition,
    removal, split, or merge changes the criterion set.

- `policy_coverage.tsv`
  - Columns: `coverage_id`, `policy_line`, `line_text`, `coverage_kind`,
    `criterion_ids`, `rationale`.
  - Contains exactly one row for every non-blank policy line; `line_text` is the
    exact line content.
  - `coverage_kind` is `criterion`, `example`, or `structural`.
    Criterion and example rows name their parent criteria. Structural rows have
    an empty `criterion_ids` array and explain why the line imposes no
    requirement.

- `scope.tsv`
  - Columns: `file_id`, `path`, `source`, `scope_status`,
    `surface_kinds`, `reason`, `policy_lines`.
  - Contains exactly the files in `target_files.tsv`, with the same file IDs,
    paths, and sources.
  - `source` is `tracked` or `untracked_intended`; `scope_status` is
    `included` or `excluded`.
  - Every included file's `surface_kinds` contains `path` plus every applicable
    kind. Excluded files have an empty surface list. Every row cites the governing
    policy lines and explains its inclusion or exclusion.
  - `path` applies to every included file; `source` to syntax, structure,
    identifiers, comments, or other non-prose content; `test` to verification
    code or data; `prose` to documentation, descriptive metadata, or user-facing
    natural language other than code comments; `translation` to content with
    language variants; `release_notes` to changelog entries; `policy` to these
    two policy documents; `configuration` to configuration content; and
    `structured_data` to machine-parsed non-code data. Kinds overlap and follow
    content and role, not filename extension alone.

- `relations.tsv`
  - Columns: `relation_id`, `relation_kind`, `family`, `member_file_ids`,
    `criterion_ids`, `policy_lines`, `rationale`, `derivation_refs`.
  - Expands every cross-file concern before checking begins. A file may belong
    to any number of relations.
  - `member_file_ids` contains unique IDs from included `scope.tsv` rows. It is
    empty only for a policy-mandated family with no current member, proved by
    `derivation_refs`.
  - `relation_kind` is `sibling`, `mirror`, `translation`, `exception`, or
    `change_dependency`.
  - `criterion_ids` contains every relation criterion governing the family;
    `policy_lines` cites those rules.
  - Required relations include applicable naming and error-message families,
    API/binding/stub mirrors, translated content, policy exception groups,
    release notes against code changes, and tests against changed behavior.
  - A change-dependency relation cites deleted or renamed inputs through the
    frozen diff; its member IDs still name only current included files.
  - An exception relation cites the exact policy lines, convention, and reason.
    Repository prevalence is not a rationale.

- `hot_paths.tsv`
  - Columns: `hot_path_id`, `family`, `member_file_ids`, `entry_points`,
    `criterion_ids`, `cost_risks`, `derivation`, `derivation_refs`.
  - Contains exactly one row for every policy-listed hot-path family and
    aggregates every implementation surface in that row.
  - `criterion_ids` contains every hot-path criterion governing the family.
  - `member_file_ids` contains unique IDs from included scope rows and is empty
    only in an absence row.
  - `cost_risks` names every concrete cost implied by the rule and inspected
    implementation; it is not limited to examples or a fixed category list.
  - `entry_points` and `derivation_refs` contain evidence references;
    `derivation` explains how they establish completeness.
  - A family with no implementation surface has an explicit absence row with
    repository-wide derivation evidence.

- `commands.tsv`
  - Columns: `command_id`, `criterion_ids`, `cwd`, `command`, `trigger`,
    `target_sha256`, `exit_status`, `key_output`.
  - Declares every required and targeted command before it runs.
  - `cwd` is `.` or an encoded repository-relative directory; `command` records
    the exact non-interactive invocation.
  - `criterion_ids` contains every command criterion the invocation checks. It
    may be empty only for a procedure-level validation command; such a command
    remains subject to its trigger and the Completion Gate.
  - `exit_status` is a non-negative integer or `not_run`; zero passes and any
    other integer fails. A `not_run` row records the blocker and cannot clear a
    command criterion.
  - A successful command is evidence only when its target digest matches
    `freeze.json.target_sha256`.

- `applicability.tsv`
  - Columns: `check_id`, `criterion_id`, `subject_ref`,
    `derivation_refs`.
  - Contains the complete expected check set, derived before verdicts.
  - `subject_ref` is `file:<file_id>`, `relation:<relation_id>`,
    `routing:<criterion_id>`, `hot_path:<hot_path_id>`,
    `process:<criterion_id>`, `command:<command_id>`, or
    `absence:<criterion_id>:<subject_type>`.
  - File candidates are the intersections of criterion and included-file surface
    kinds. The selector produces the file checks from those candidates, and every
    file criterion also produces one routing check whose derivation proves that
    the selector was applied exhaustively. Relation, hot-path, and command checks
    come from the corresponding `criterion_ids`; each process criterion produces
    one process check.
  - Each declared non-file subject type with no concrete subject produces one
    absence check. Its derivation proves the absence; it passes only when absence
    satisfies the rule. A criterion missing any required applicability row, or a
    row not produced by these rules, is an artifact failure.

- `checks.tsv`
  - Columns: `check_id`, `criterion_id`, `subject_ref`, `verdict`,
    `evidence`, `evidence_refs`.
  - Contains exactly one row for every `applicability.tsv` row, with the same
    ID, criterion, and subject.
  - A local check does not replace a relation or hot-path check, and one
    subject's pass does not override another subject's non-pass.
  - A routing check proves selection completeness only; it never proves that a
    selected file satisfies the criterion.

- `findings.tsv`
  - Columns: `finding_id`, `source_ref`, `severity`, `target_ref`,
    `criterion_ids`, `finding`, `proposed_fix`.
  - Contains one or more rows for every current `fix` verdict in `checks.tsv` or
    `coverage_balance.tsv`, one row per distinct target defect, and no rows from
    any other verdict.
  - `source_ref` names the originating fix row; `target_ref` names the defective
    source or artifact; and `criterion_ids` includes the source criterion and any
    other directly violated criterion.
  - `severity` is `blocker`, `high`, `medium`, or `low`; severity never
    changes whether a policy finding must be resolved.

- `coverage_balance.tsv`
  - Columns: `balance_id`, `criterion_id`, `subject_family`, `check_ids`,
    `comparator_balance_ids`, `verdict`, `rationale`, `evidence_refs`.
  - Contains one row for every criterion and subject family represented in
    `applicability.tsv`. Subject families are `file:<surface_kind>`,
    `routing`, `relation:<relation_kind>`, `hot_path:<family>`, `process`, and
    `command`, plus `absence:<subject_type>`.
  - `verdict` is `pass`, `fix`, `review`, or `pending`.
  - Comparators are other families for the same criterion and structurally
    analogous criterion-family rows. The comparator list is empty only when no
    such row exists and the rationale proves why.
  - Fix counts need not match. A zero-fix family passes only when its
    checks and evidence show depth comparable to its named comparator rows.

- `independent_review.tsv`
  - Columns: `review_id`, `reviewer`, `review_kind`, `subject_key`, `source_ref`,
    `target_sha256`, `manifest_sha256`, `verdict`, `evidence_refs`.
  - `review_kind` is `expected_key`, `check`, `balance`, or `artifact`;
    `subject_key` is a compact JSON array beginning with the artifact filename,
    followed by that unit's natural-key values. The whole-manifest key is
    `["manifest.tsv"]`; each manifest-row key adds its artifact path.
  - Contains one row for every independently expected natural key in target
    files, policy extraction, scope, subject inventories, and applicability; one
    row for every check and balance row; and artifact rows for the manifest as a
    whole and for every manifest row.
  - `source_ref` resolves to the corresponding current row or artifact. It is
    empty only when an expected unit is missing, which requires a non-pass
    verdict.
  - Every row names the independent reviewer and the reviewed target and manifest
    hashes. `verdict` is `pass`, `fix`, or `pending`.

- `audit_summary.json`
  - Contains exactly: `schema_version`, `target_sha256`, `manifest_sha256`,
    `review_sha256`, `expected_checks`, `actual_checks`, `verdict_counts`,
    `finding_counts`, `command_counts`, `balance_counts`, `review_counts`,
    `validation_errors`, `stale_artifacts`, and `completion_status`.
  - `schema_version` and `target_sha256` equal the freeze; `manifest_sha256` and
    `review_sha256` are the lowercase SHA-256 hashes of the current files.
  - `verdict_counts` has exactly `pass`, `fix`, `review`, and `pending`;
    `finding_counts` has `blocker`, `high`, `medium`, and `low`;
    `command_counts` has `passed`, `failed`, and `not_run`;
    `balance_counts` has `pass`, `fix`, `review`, and `pending`;
    and `review_counts` has `pass`, `fix`, and `pending`.
  - `validation_errors` has exactly `schema`, `identifier`, `enum`, `reference`,
    `expected_key`, `dependency`, `evidence_cycle`, and
    `evidence_termination`.
  - `expected_checks`, `actual_checks`, every count member, and every validation
    member are non-negative integers. `stale_artifacts` is an array of artifact
    IDs.
  - `completion_status` is `complete` or `pending` and is derived from the
    source artifacts; it never overrides them.

## Verdicts and Evidence

Every `checks.tsv.verdict` is one of:

- `pass`: the governed check obligation is satisfied, with substantive current
  evidence;
- `fix`: the governed policy or procedure obligation is violated;
- `review`: semantic judgment is required;
- `pending`: required evidence, execution, or review is incomplete.

Coverage-balance verdicts use the same four meanings for their audit obligation.

Do not use `not applicable`. Applicability is decided before verdicts, and only
applicable criterion/subject pairs enter `applicability.tsv`.

Evidence is substantive only when it:

- names the exact subject and surface inspected;
- explains why the verdict follows from the rule rather than an example;
- records measured or mechanical support where the rule requires it;
- names peers or dependencies for relation verdicts;
- avoids bare phrases such as "no issue", "checked", or "matches policy".

Search output, inventories, formatter success, test success, finding counts, and
reviewer summaries are probes or command evidence, not findings by themselves. A
finding identifies the current source or artifact row, criterion, actionable
defect, and relevant peer or dependency evidence.

## Phases

Run phases in order. Missing expected rows, invalid references, stale artifacts,
or schema errors block the current phase. Non-pass verdicts proceed through
balance and internal resolution, but block independent review and final
validation until resolved.

1. Freeze the target.
   - Record every target file's mode and content digest, the branch, base and
     head commits, policy and procedure digests, artifact directory, and exact
     diff.
   - Write `target_files.tsv`, `freeze.json`, and `current-diff.patch` before
     verdicts.

2. Extract the policy.
   - Build `criteria.tsv` from normative units only.
   - Build `policy_coverage.tsv`, classifying every non-blank line as criterion,
     example, or structural.

3. Build scope.
   - Classify every target snapshot row.
   - Apply policy exclusions and assign all applicable surface kinds.

4. Build subject inventories.
   - Expand all cross-file relations.
   - Enumerate every hot-path surface.
   - Declare required and targeted commands.

5. Derive applicability.
   - Generate `applicability.tsv` from criteria and subject inventories.
   - Validate the complete expected check set before writing verdicts.

6. Perform checks and run commands.
   - Inspect every applicable subject and write one `checks.tsv` row per
     expected check.
   - Inspect source directly; searches and automation remain probes.
   - Execute every triggered command and record its target-bound result.
   - Run `make format` after code or formatter-managed document changes.
   - Run `make lint` and `make lint-wasm` in every exhaustive run.
   - Run `make test` after code changes.
   - Run targeted parsers and validators for changed structured files, applicable
     prose or documentation tests, and `git diff --check`.
   - If a command changes the frozen target, preserve the run and start another.

7. Check audit balance.
   - Compare each criterion-family row with every comparator it names.
   - Re-run the same probes on an unexplained zero-fix family and keep the
     balance row `pending` until the difference is justified.

8. Resolve reviews and record findings.
   - Resolve every `review` as `pass` or `fix` from current evidence, without
     inventing exceptions. Keep missing evidence or blocked execution `pending`.
   - Generate `findings.tsv` from the resulting `fix` rows.
   - Correct an audit-evidence defect in place and regenerate every dependent
     artifact. To correct the frozen target, preserve the current pending run and
     start a new run against the changed target.
   - Repeat the affected phases until every check and balance row is `pass` and
     `findings.tsv` is empty.

9. Run independent review.
    - A reviewer who produced none of the audited artifacts independently derives
      policy, inventory, and applicability natural keys from the frozen source
      before opening the run's verdicts or findings.
    - Give that reviewer both documents, the frozen target, complete inventories,
      applicability, checks, findings, balance evidence, and manifest for
      comparison and source review. Record every required review row in
      `independent_review.tsv`.
    - The reviewer validates every expected key, verdict, and evidence chain;
      review sampling cannot clear this gate.
    - Every actionable review result is `fix` regardless of severity. Propagate
      source defects to checks and findings, and artifact defects to validation
      errors.
    - Correct audit evidence in place; for a target correction, preserve the run
      and start another. Repeat review after either change.
    - If independent review is unavailable, completion remains `pending`.

10. Validate completion.
    - Validate every schema, enum, identifier, natural key, reference, dependency,
      evidence chain, count, and manifest hash from its source artifact.
    - Re-derive the target file snapshot and diff from the live worktree and
      require exact equality with the freeze.
    - Require the independent-review set to be exact, every review verdict to be
      `pass`, and every row to name the current target and manifest hashes.
    - Write `audit_summary.json` only after the independent review is clean.

## Delegation Rules

When an audit is split, the lead auditor gives each auditor both documents, the
frozen target, complete subject inventories, exact assigned check IDs, and all
dependencies needed by those checks.

Delegated summaries are not evidence. The lead reads every returned row,
validates its references, rejects copied or shallow evidence, and resolves
conflicting verdicts by re-reading the subjects.

## Completion Gate

The audit is complete only when:

- the live target snapshot and diff exactly match the freeze;
- every required artifact exists, every manifested artifact matches its schema
  and manifest inputs, and every artifact is bound to the frozen target;
- policy coverage, scope, subject inventories, and applicability contain exactly
  their independently derived expected natural keys;
- `checks.tsv` contains exactly the applicability set and every verdict is
  `pass`;
- every `coverage_balance.tsv` verdict is `pass`;
- `findings.tsv` is empty;
- `independent_review.tsv` contains exactly its independently derived expected
  natural keys, every verdict is `pass`, and its target and manifest hashes are
  current;
- every evidence reference resolves, the evidence graph is acyclic, and every
  chain terminates in current source, a frozen-target root artifact, or successful
  target-bound command evidence;
- every triggered command succeeded against the frozen target;
- `audit_summary.json` matches the current target, manifest, and review hashes,
  reports matching check counts, zero validation errors, zero stale artifacts, and
  `completion_status: complete`.

Do not claim completion before every condition is proved. If any condition is
uncertain, the verdict is `pending`.
