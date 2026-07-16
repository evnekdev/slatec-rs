# Status vocabularies

## 1. Fact verification status

Field: `fact_status`

| Value | Meaning |
|---|---|
| `verified` | Directly confirmed in the cited inspected artifact or exact reproducible experiment. |
| `derived` | Mechanically computed from verified inputs by a declared deterministic method. |
| `inferred` | Supported interpretation, but not directly stated or mechanically proven. |
| `unverified` | Candidate value exists but its cited content was not adequately inspected. |
| `conflicting` | Two or more relevant claims cannot simultaneously be accepted. |
| `unknown` | No defensible value has been established. |
| `not_applicable` | The property does not apply to the entity. |

`not_applicable` is a semantic value, not a synonym for missing.

## 2. Source retrieval status

Field: `retrieval_status`

| Value | Meaning |
|---|---|
| `not_attempted` | No retrieval attempt recorded. |
| `located` | A URL or repository path was identified. |
| `metadata_verified` | Index metadata was inspected, but target bytes were not obtained. |
| `retrieved` | Bytes were obtained but not content-inspected. |
| `checksum_verified` | Retrieved bytes match an expected or recorded checksum. |
| `inspected` | Relevant content was opened or parsed. |
| `fully_inventoried` | All members/files in the declared scope were enumerated and hashed. |
| `failed` | Retrieval failed; diagnostics are recorded. |
| `superseded` | Record retained, but another retrieval record replaces it. |

These values are not a strict linear scale. For example, `inspected` does not imply `fully_inventoried`.

## 3. Provenance confidence

Field: `provenance_confidence`

| Value | Meaning |
|---|---|
| `verified` | Exact lineage, authorship, or package membership is directly established. |
| `high` | Multiple strong primary indicators agree; exact identity may remain unproven. |
| `medium` | Credible evidence exists, but alternatives remain plausible. |
| `low` | Weak naming, co-location, or contextual evidence only. |
| `conflicting` | Relevant provenance sources disagree. |
| `unknown` | No defensible provenance attribution. |

Directory co-location alone cannot exceed `low`.

## 4. Relationship confidence

Field: `relationship_status`

Canonical values:

- `verified-identical`
- `verified-different`
- `likely-identical`
- `possible-duplicate`
- `alternate-implementation`
- `relocated-copy`
- `corrected-copy`
- `independent-upstream-package`
- `documentation-only`
- `generated-dependency-product`
- `machine-specific-alternative`
- `unknown`
- `unresolved`

The relationship kind and proof status should be separate in future machine schemas. For compatibility, existing combined values remain readable during migration.

## 5. Coverage status

Field: `coverage_status`

| Value | Meaning |
|---|---|
| `none` | No items in the declared scope processed. |
| `sampled` | Representative items inspected; not intended as complete. |
| `partial` | A known subset processed. |
| `complete_unreviewed` | All expected items processed mechanically. |
| `complete_reviewed` | Complete output reviewed against declared checks. |
| `complete_verified` | Completeness and correctness validated reproducibly. |
| `unknown_scope` | The expected population is itself unknown. |

## 6. Conflict state

Field: `conflict_state`

- `none_detected`
- `open`
- `resolved_for_policy`
- `resolved_globally`
- `deferred`
- `invalidated`

`resolved_for_policy` means a provider or value is selected for one named corpus/build policy while alternatives remain preserved.

## 7. Human review state

Field: `review_state`

- `unreviewed`
- `machine_checked`
- `review_requested`
- `reviewed`
- `approved_for_extraction`
- `approved_for_ffi`
- `approved_for_release`
- `rejected`
- `stale`

Approval is use-specific. `approved_for_extraction` does not imply FFI or release approval.

## 8. Rights status

Use the E04 values without reinterpretation:

- `explicit-permissive`
- `public-domain-stated`
- `government-work-stated`
- `redistribution-permitted`
- `no-explicit-license-found`
- `conflicting`
- `requires-review`

Rights status does not establish factual correctness, numerical quality, or provenance.

## 9. State transitions

Permitted transitions are evidence events, not editorial upgrades:

- `unverified -> verified` only after inspecting a cited artifact.
- `verified -> derived` is invalid; create a new derived claim instead.
- `inferred -> verified` requires a new direct claim and resolution record.
- any status may become `conflicting` when contrary evidence appears.
- `conflicting -> verified` is invalid without retaining the conflict group and recording its resolution.
- review approvals become `stale` when relevant inputs, parser semantics, schema major version, or policy change.
- retrieval records are append-only; a new retrieval supersedes rather than mutates historical bytes.

## Legacy mapping

Existing generic `evidence_status` fields map to `fact_status`. Existing `source_access` maps to `retrieval_status` only after contextual migration. Generic `confidence` must be assigned explicitly to provenance, relationship, ownership, or another named dimension; ambiguous values fail validation.
