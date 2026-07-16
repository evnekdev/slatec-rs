# Conflict representation

## Principle

Conflicts are first-class evidence. They are not represented by overwriting an earlier value, choosing the newest file by directory order, or replacing several claims with `unknown`.

## Conflict group

A conflict group contains:

```text
id
subject_id
field_path
claim_ids[]
conflict_kind
conflict_state
impact[]
resolution_ids[]
```

Conflict kinds include:

- `different_values`
- `different_providers`
- `scope_disagreement`
- `revision_disagreement`
- `prologue_vs_source`
- `source_vs_object`
- `documentation_vs_runtime`
- `rights_conflict`
- `identity_ambiguity`
- `parser_disagreement`

## Claim preservation

Every competing value remains in its own claim with its own source, locator, retrieval status, fact status, and provenance confidence. A conflict group does not alter the historical claims.

## Resolution record

A resolution contains:

```text
id
conflict_id
scope
selected_claim_ids[]
rejected_claim_ids[]
resolution_kind
rationale
evidence_ids[]
decided_by
decided_at
review_state
policy_version
```

Resolution kinds:

- `evidence_dominates`
- `selected_for_corpus`
- `selected_for_target`
- `selected_for_historical_profile`
- `merge_preserving_candidates`
- `deferred`
- `invalid_claim`

A policy-specific resolution must not mark the conflict `resolved_globally`.

## Impact and gates

`impact` identifies affected uses:

- `inventory`
- `provenance`
- `dependency_graph`
- `native_build`
- `raw_ffi`
- `safe_api`
- `documentation`
- `redistribution`

An open conflict affecting source signature, provider selection, symbol definition, ABI, mutable state, machine constants, required dependencies, or rights blocks the corresponding generation/release gate.

## Duplicate providers

Duplicate symbols are represented as provider claims and a conflict group. Exactly one provider may be selected for a named build unless the symbols are intentionally renamed or hidden. The resolver must never use filesystem order, link order, or first match as an undocumented policy.

## Contradictory dependency evidence

Prologue calls, parsed source calls, Netlib trees, object undefined symbols, and linker observations remain separate evidence kinds. A reconciled edge records all supporting and contradicting claims. Absence from one source is not automatically a contradiction unless that source asserts complete coverage.
