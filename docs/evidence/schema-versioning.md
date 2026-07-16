# Schema versioning and migration

## Version model

Every schema has a stable `schema_id` and semantic version:

```text
schema_id = "slatec-rs/evidence"
schema_version = "1.0.0"
```

- **major**: incompatible meaning, required-field, identity, or representation change;
- **minor**: backward-compatible optional field or enum extension;
- **patch**: clarification or validation correction that does not alter valid data meaning.

Generated datasets also declare `producer_schema_version` and the exact schema document hash.

## Compatibility rules

- Readers must reject unsupported major versions.
- Readers may accept a newer minor version only when unknown fields are preserved or explicitly ignored without changing meaning.
- Unknown enum values must not be coerced to `unknown`; retain the raw value and fail the affected gate.
- Existing field meanings must never be broadened silently.
- Deprecated fields remain readable for at least one major migration cycle when practical.

## Migration records

Each migration contains:

```text
from_version
to_version
migration_id
tool_version
lossless
manual_review_required
field_mappings[]
semantic_changes[]
```

Migration output records the input and output semantic hashes. A lossy migration must preserve the original dataset and emit a review queue.

## Required initial migrations

The first implementation must migrate legacy metadata by:

1. mapping generic `evidence_status` to `fact_status`;
2. mapping contextual `source_access` and retrieval strings to `retrieval_status`;
3. replacing ambiguous `confidence` with a named confidence dimension;
4. adding `collection_state` to all arrays;
5. converting broad source URLs into claim records and locators;
6. adding schema, snapshot, parser, and extraction-run identity;
7. preserving unrecognized legacy values rather than discarding them.

## Missing-value semantics

- field omitted: not supplied by this schema producer or not yet migrated;
- `unknown`: property applies but no defensible value exists;
- `not_applicable`: property does not apply;
- `known_empty`: complete inspection established no members;
- empty string: invalid unless the field explicitly permits it;
- empty array: invalid without `collection_state`;
- `false`: an asserted boolean value requiring evidence, not a default for missing.

## Staleness

A record becomes stale when:

- its source artifact or selected provider changes;
- a cited member hash changes;
- parser semantics change;
- the schema major version changes;
- a relevant conflict is opened or resolution invalidated;
- an ABI or build claim is used outside its fingerprint;
- rights evidence or release scope changes.

Stale records remain auditable but cannot pass generation gates until regenerated or reviewed.
