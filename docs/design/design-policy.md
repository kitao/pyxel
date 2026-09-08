# Pyxel Design Policy

This policy defines the governing principles and standards for reading, changing, and reviewing Pyxel.

Concrete choices applying these standards and their rationale are recorded in [Design Decisions](design-decisions/).

## Principles

- **Design.** Keep Pyxel approachable. Preserve its deliberate constraints and separation of basic and advanced capabilities: they support learning and discovery. Keep differences that serve distinct users or purposes.
- **Clarity.** Keep what readers need to understand, use, or maintain the project. Prefer direct code and concise explanations; preserve necessary conditions and rationale.
- **Evidence.** Make decisions reproducible from the rules and evidence. Explain judgments through the relevant rule, source, and comparable sites. Personal taste and majority usage do not settle a disagreement.
- **Consistency.** Apply a correction to the whole affected family, including its public interfaces, translations, generated outputs, and tests.
- **Performance.** Correctness comes first. Demonstrated hot-path cost can justify a departure from language idiom; elsewhere, prefer idiomatic, readable code.
- **Self-applicability.** These standards govern every applicable part of the project, including code, documentation, tests, tooling, and the policy itself.

## Public Contract

- Preserve the maintainer's intended public contract and released compatibility. Changes to public behavior or meaning beyond the agreed scope require the maintainer's decision before changing implementation, documentation, or tests.
- No single representation establishes the intended contract. Resolve disagreements from the maintainer's decisions and evidence of behavior and meaning. Documented requirements, accepted inputs, and editor controls may differ by design.
- Public descriptions must explain the behavior users need to rely on, including what happens when arguments are omitted. Internal representations must not obscure that behavior.
- Evaluate changes to failure handling against the existing contract, callers' recovery needs, and the added code and runtime cost.

## Source Code

### Performance

- Review cost across all performance-sensitive paths. Remove avoidable cost demonstrated by measurement or analysis of the executed path. Justify added complexity by the cost it removes; a suspicious-looking construct alone is not evidence.
- Outside demonstrated performance-sensitive paths, prefer readability over micro-optimization.

### Naming

- Follow the language's case conventions, lint rules, and idiomatic abbreviations. Keep a clear local name unless it misleads, describes an obsolete concept, or conflicts with comparable sites.
- File and directory names use consistent base names, separators, and role suffixes within each sibling group. Public URLs, generated paths, and author-titled assets retain their established spelling unless all affected references change together.
- Use consistent terminology for the same concept. Exact mirrors share names; distinct roles or language boundaries may use different names when their correspondence is clear. Preserve symmetric verb families and avoid redundant owner/type names.

### Structure and Formatting

- Make meaningful groups and their relationships clear. Preserve required declarations and order-dependent behavior. Formatting must not fragment a coherent group or introduce unrelated changes.

### Comments

- Explain non-obvious intent, encodings, numerical assumptions, workarounds, or non-local invariants; do not narrate the code.
- Keep a shared rationale at the site that owns the decision. Dependent sites may identify the relevant invariant or point to that explanation when needed for local understanding.
- Use the shortest explanation that preserves the reason. Comparable operations receive comparable detail; complexity, not file length, determines how much is needed.
- A group label is useful only when language structure does not already identify the group.
- Explain the current reason without requiring knowledge of a past incident. Avoid self-referential or tautological explanations.

### Cross-file Consistency

- Preserve intentional parallel structure and assess every applicable family, including overlapping groups. Fix shared defects across all affected members; repository-wide prevalence does not determine the correct form.
- Preserve differences justified by external interfaces or distinct readers and purposes. A local preference does not justify departing from the governing conventions.
- Related errors and warnings must express the same failure consistently and follow the host language's conventions. Distinct failure kinds need not use identical messages or treatment.

## Testing

Test code follows the source standards above. Match verification to the behavior and qualities it can establish.

- Derive expected results from the agreed contract or independently established properties. Changing implementation, documentation, and assertions together does not establish that their shared expectation is correct.
- Choose coverage by the failure it detects. Add a test when existing coverage cannot reliably detect the failure or isolate an important contract. Do not repeat the same assertion at another layer without a distinct reason. A theoretically visible or audible failure is not proof that an existing reference case exercises it.
- Use human observation for appearance, sound quality, interaction feel, and device-specific behavior that automated tests cannot establish. An unperformed manual check provides no coverage evidence.
- A test's name and comments describe what its assertions establish. Tests that cannot fail for the claimed reason, merely repeat implementation steps, or add no protection beyond existing cases do not establish useful coverage.
- Pin deterministic discrete results and exact-output contracts exactly. Use an explicit numerical tolerance only when justified by precision or the algorithm. Accept alternative outcomes only for genuine nondeterminism, with its cause stated.

## Documentation

### Reader and Prose

- Write for the page's intended reader and task, using words they can understand. Introductory material gives enough context and a small usable example to take the next step; introduce advanced or internal details only when they help that step.
- Keep each explanation in one authoritative place and link to it when needed. Remove repetition and unnecessary qualification while preserving prerequisites, units, limits, and consequences of destructive operations.
- Make instructions usable independently of a contributor's machine or private tools. Keep personal paths and private environment details out of published documentation and reports.
- Use natural technical writing in each language. Choose established technical spellings and keep terminology consistent across related pages. Translate meaning rather than the source language's sentence structure. Prose conventions must not alter code literals.

### Translation and Product Names

- Translations must preserve the intended source meaning across the whole language chain. Each language uses its own technical conventions and established loanwords; conditions, placeholders, and behavior must remain consistent.
- Preserve product-name spelling and casing in every language. A descriptive label may replace a product name where the context makes its reference clear. Other proper nouns keep their author's spelling, spacing, hyphens, and casing.

## Release Notes

- Record a concrete user benefit or a useful investigation clue. A maintenance entry names the affected component and change; generic cleanup claims are insufficient.
- Evaluate independent changes separately, relative to the previous release. Use consistent, natural verbs and specificity within each change category.

## Verification

### Scope

- Cover the complete requested scope and every applicable aspect of its subjects: code, comments, prose, translations, configuration, tests, paths, and their relationships. An exhaustive repository audit includes every repository file and intended addition, subject to justified exclusions from direct inspection.
- An exclusion removes direct inspection of the excluded aspect, not dependency checks. Changes affecting a generated document, package, or other distributed artifact require verification of its correspondence with the source.

### Acceptance

- Complete the verification appropriate to each change before claiming completion. Required checks must pass. Investigate failures; do not waive them as flaky or count unperformed checks as successful. Report environment blockers explicitly.
- When standards or verification procedures change, reassess the obligations and their verification. Earlier results do not establish compliance with revised criteria, and a revision review does not certify the repository.
- An exhaustive audit must establish compliance across every applicable rule and subject without sampling. Unresolved findings or unperformed checks keep the audit pending.

## Maintaining This Policy

- Keep this policy limited to governing principles and standards. Do not amend it to accommodate a local implementation or accumulate case-specific decisions and incident histories.
- Preserve agreed requirements when shortening or reorganizing them. Changes to requirements need the maintainer's decision. A policy revision changes the basis of the audit procedure and the complete body of decisions and verification; its consequences must be assessed as a whole.
- Keep authoritative requirements at their owning layer. Examples clarify a boundary; they do not create separate requirements.
- Judge amendments by the coherence and completeness of the policy as a whole. Preserve necessary conditions and rationale; neither word count nor a local improvement establishes that a revision is sound.
