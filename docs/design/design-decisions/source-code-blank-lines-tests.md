# Tests Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

Read with the [shared blank-line decisions](source-code-blank-lines.md).
These decisions apply to Rust, Python, and Web tests and their support code.

## Cases and observations

**Decision:** Apply the shared processing groups to test bodies. Separate
prepared cases and observable interaction or lifecycle phases. Keep each
phase's action and immediate observations close together; repeated use of the
same fixture does not merge distinct phases.

Keep short input/call/assertion sequences and related assertion inventories
compact. Component checks, endpoints, expected-value calculations, and repeated
parameter values do not each need a paragraph. A large setup, execution, or
output inspection can have its own paragraph even when the whole test checks
one result. Neither mandatory arrange/act/assert separation nor continuous
formatting of an entire test case applies.

**Reason:** Readers need to locate what is exercised and the evidence about
it. Test-case identity alone says nothing about the amount of code readers
must navigate. Distinct phases need boundaries; splitting each assertion or
component hides their relationship.

## Fixtures and support code

**Decision:** Separate substantial fixture construction or mock-environment
setup from the trials that consume it. Keep fields, captured values, argument
lists, and parameterized case inventories together within their groups.
A helper call or local variable alone does not establish a setup paragraph.

Saving state, exercising behavior, inspecting results, and restoring state
follow the same processing groups as production code. A shared resource
lifecycle does not require all those steps to be continuous. A `try`, `finally`,
`with`, `yield`, `await`, or assertion does not itself introduce a paragraph.
Short cleanup stays with the operation it completes; longer teardown and
subsequent outcome inspection can form separate paragraphs.

Local definitions retain their language's implementation boundaries. Compact
Web mock interfaces follow the [Web fixture decisions](source-code-blank-lines-web.md#javascript-implementation-boundaries).
Embedded programs retain their own content and formatting ownership.

**Reason:** Fixtures and test runners perform ordinary processing work. Their
paragraphs should expose that work, while keeping cleanup and observations
understandable in relation to the operation being tested.
