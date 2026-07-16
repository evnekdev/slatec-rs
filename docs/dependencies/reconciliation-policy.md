# Dependency reconciliation policy

## Core rule

Disagreement is a result, not a parser failure. Preserve every claim and create an explicit reconciliation record.

## Matching

Normalize Fortran identifiers case-insensitively while preserving spelling. Resolve references in this order:

1. selected program unit or entry in the canonical corpus;
2. explicit alias/rename record;
3. selected external package provider;
4. callback formal or forwarded callback;
5. intrinsic or compiler/runtime service;
6. optional/site hook;
7. ambiguous or unresolved reference.

No directory order, archive order, or first matching symbol may resolve ambiguity.

## Comparison classes

For each caller/reference compare:

- parsed source versus prologue;
- parsed source versus `tree1`;
- derived closure versus `tree`;
- selected closure versus plus-dependency inventory;
- parsed references versus object undefined symbols;
- selected providers versus linker-observed providers.

Classify outcomes as `agree`, `source_only`, `document_only`, `tree_only`, `object_only`, `linker_only`, `provider_mismatch`, `conditional_difference`, `callback_classification_difference`, `runtime_lowering`, `snapshot_difference`, `parser_gap`, or `unresolved`.

## Conflict rules

Create a conflict group when incompatible claims affect direct linkage, provider identity, signature, state, or ownership. A conflict contains all claims, impact gates, review state, and optional scoped resolution. A resolution for one target or canonical profile does not erase alternate claims.

## Inference rules

- Prologue-only dependencies may be shown as documentary but cannot be marked link-required without corroboration.
- Object-only undefined symbols may become link-required for the exact build after classification.
- A successful link may verify provider sufficiency for that fingerprint, but does not prove source-level completeness or portability.
- A missing object reference does not disprove a source reference if optimization or dead-code elimination could remove it.
- `tree` and plus-dependency products cannot define direct edges.

## Unresolved references

Every unresolved reference records raw spelling, normalized spelling, source location, reference kind, candidate targets, classification, required-for-link state, evidence, and review action. Unknown and ambiguous are never silently converted into callbacks or runtimes.
