# GLSL Blank Line Decisions

[Audit and decision records](../design-audit.md#decision-records) · [Source Code policy](../design-policy.md#structure-and-formatting)

Read with the [shared blank-line decisions](source-code-blank-lines.md).
GLSL shader declarations and calculations.

## Shader calculation groups

**Decision:** In the [screen shaders](../../../crates/pyxel-core/src/shaders/),
separate function implementations and declaration groups from implementations
with one blank line. Keep version/precision directives, the uniform inventory,
and each macro inventory continuous. The smooth filter has separate inventories
for host-name aliases, source-size encoding, filter constants, comparison
macros, and indexed texture sampling; these serve distinct interfaces within
the filter. File boundaries do not require leading or trailing empty lines.

Within functions, use these boundaries:

| Relationship | Blank-line placement |
| --- | --- |
| Coordinates, color channels, coefficients, and the return directly evaluate one formula | None between preparation, component calculations, and result |
| A variable is filled immediately by a component inventory | None between its declaration and that inventory, or between components |
| An immediate screen check selects a sampled color or background | Keep coordinate acquisition, selection, and output continuous |
| A complete coordinate transformation precedes filtering | One blank between the transformation and filtering |
| Separate sampling passes or coordinate domains form an effect | One blank between the complete passes or domains |
| An expanded filter has independently evaluated corner cases | One blank between complete corners; use the same internal groups for every corner |

The shared coordinate/palette helpers, crisp filter, and smooth filter's
`distYCbCr`, `isPixEqual`, and `getLeftRatio` are continuous calculations.
In the retro filter, `warpScreen` groups centered-coordinate distortion and
texture-coordinate remapping separately. `getBleedingColor` groups direct
channel samples, displaced channel samples, and their common tone adjustment
separately; each RGB inventory is continuous. Vignette and scanline calculations
include their coefficients, scaling, and returns in one group. Its `main`
separates the complete coordinate warp from color selection; effect modulation
and writing the resulting color stay together inside that selection.

The smooth filter's `main` has screen admission, sampling-coordinate
construction, the nine-pixel inventory, corner-result initialization, four
corner classifications, and four corner blends. Separate these groups and
each complete corner. Within each classification, distances, gradient
selection, and the stored classification form one calculation. Within each
blend, separate eligibility calculation, origin/direction construction, and
pixel selection/mixing. Keep the line-shape alternatives with origin/direction
construction. The color accumulator starts with the first blend, and final
color output ends the last blend; neither needs a paragraph of its own.

Attribution and license blocks have their own paragraph structure. Pixel maps
introduce the sampling section or attach to their particular corner; spacing
inside those explanations is comment content, not a shader statement boundary.
The [loader](../../../crates/pyxel-core/src/graphics.rs) concatenates a version
fragment, shared code, and the selected filter. Each fragment ends with a
newline so its last directive or token remains separate from the next fragment.

**Reason:** Component-by-component gaps obscure a single formula. Complete
sampling passes and the smooth filter's expanded corner algorithms need
locatable boundaries, including matching boundaries under corner rotation.
Returning or writing the calculated value adds no new calculation stage.
Source-fragment and attribution boundaries have different roles from the
processing groups inside a function.
