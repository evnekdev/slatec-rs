# Manual review workflow

## Review item creation

The pipeline creates a review item whenever evidence cannot safely be converted into an approved claim. Typical triggers are:

- unsupported or ambiguous syntax;
- prologue/declaration mismatch;
- unresolved function-versus-array use;
- callback signature ambiguity;
- duplicate providers;
- machine-specific implementation choice;
- character, complex, logical, common, equivalence, or alternate-return ABI risk;
- inferred argument intent or safety property;
- unresolved rights affecting generated or released material;
- parser regression or changed semantic hash.

## Review item fields

Each item contains stable ID, category, severity, affected entity/field, source locators, competing claims, parser/run IDs, gate impact, required reviewer competence, and allowed resolution kinds.

## States

- `open`;
- `in_review`;
- `resolved`;
- `deferred`;
- `superseded`;
- `reopened`.

Review state is independent of fact status and conflict state.

## Resolutions

A reviewer may:

- approve a directly supported claim;
- approve a documented inference with scope;
- reject a claim;
- create or select a conflict resolution;
- mark a construct unsupported;
- require a compiler/ABI probe;
- require upstream/rights investigation;
- defer generation for the entity.

Every resolution records reviewer identity, date, rationale, scope, evidence locators, input semantic hash, and policy version. Free-text rationale is required but does not replace structured resolution fields.

## Staleness

A resolution becomes stale when its source hash, parser major version, relevant grammar/rule hash, provider selection, target profile, or policy changes. Stale resolutions do not pass gates until revalidated.

## Two-person review

Require independent second approval for:

- safe-wrapper aliasing or ownership claims;
- callback ABI;
- COMMON/EQUIVALENCE layout exposed across FFI;
- machine-provider selection for a release;
- rights exceptions;
- manual override of a parser conflict affecting generated signatures.

## Review artifacts

The UI or report should show source context, raw bytes when needed, parser view, syntax node, all claims and evidence, conflict history, proposed generated declaration, and semantic diff from the prior approved state.

## No source editing through review

Review changes metadata, resolutions, fixture expectations, or separately tracked patches. It never edits the preserved source or silently modifies parser output.
