# Normalized evidence model

## Purpose

The project currently uses similar words in different contexts: source retrieval, direct factual verification, provenance confidence, provider relationships, documentation coverage, rights review, and build validation. E05 assigns one purpose to each dimension and defines a common field-level envelope.

## Evidence-bearing entities

The model applies to:

- source artifacts and archive members;
- selected and alternate source providers;
- program units, arguments, common blocks, block-data units, entries, and symbols;
- package, family, domain, and ownership classifications;
- dependency nodes and edges;
- machine-provider profiles and build results;
- rights and attribution claims;
- patches, normalized views, generated metadata, bindings, and releases.

## Common entity header

Every machine record should carry:

```text
id
schema_id
schema_version
snapshot_id
created_by
created_at
source_record_ids[]
review_state
```

`created_at` records the extraction event but must not participate in semantic content hashes. `snapshot_id` identifies the complete selected input and policy configuration.

## Field evidence envelope

A scalar or collection whose value is evidence-dependent should be represented conceptually as:

```text
value
fact_status
claims[]
resolution
review
```

Each claim contains:

```text
claim_id
statement_kind
value
source_record_id
locator
retrieval_status
extraction
confidence
notes
```

The exact storage shape may use nested TOML tables, normalized claim tables, or generated references, but the semantics must remain the same.

### Statement kinds

- `source_fact` — directly stated or present in an inspected artifact.
- `mechanical_result` — deterministic output from declared verified inputs.
- `interpretation` — reasoned classification not directly stated.
- `project_decision` — selected policy or design choice.
- `reported_external_claim` — a claim made by a source but not independently verified.

A source statement that software is GPL, public domain, thread-safe, or part of a package is still a `reported_external_claim` until the claim's scope and authority are established.

## Dimensions

### Fact status

Applies to one value or relationship assertion. It answers: **how is this claim supported?**

### Retrieval status

Applies to an artifact or exact source locator. It answers: **what bytes or representation were actually obtained and inspected?**

### Provenance confidence

Applies to historical origin, package membership, authorship linkage, or revision lineage. It answers: **how strongly is this attribution established?**

### Relationship confidence

Applies to duplicate, relocated, corrected, upstream, alternate, generated, or unknown relationships. It answers: **how strongly is this relationship demonstrated?**

### Coverage status

Applies to extraction completeness. It answers: **how much of the expected scope was processed and reviewed?**

### Conflict state

Applies to a group of incompatible claims or providers. It answers: **is disagreement present and has it been resolved for a named policy?**

### Review state

Applies to human review. It answers: **has a qualified reviewer accepted the machine or research result for its declared use?**

### Rights status

Applies to an artifact or distribution unit. It answers: **what rights evidence and packaging gate apply?** It must use the E04 vocabulary and must not be converted into fact confidence.

### Build validation status

Applies only to an exact build fingerprint. It answers: **did compilation, linkage, symbol checks, quick checks, or runtime probes pass for this configuration?**

## Inference rules

Fields that may be inferred include package/family attribution, candidate domain, argument intent, renamed-routine linkage, and likely duplicate relationships, but only when:

- the schema permits inference for that field;
- supporting claim IDs are listed;
- the method is stated;
- `fact_status = "inferred"`;
- the value cannot drive binding generation unless a separate gate explicitly permits it.

The following may not be inferred for production FFI:

- source signature and program-unit kind;
- argument order, type, rank, character length, callback ABI, or alternate return;
- selected provider and source preimage;
- exported symbol spelling;
- mutable global state and required block data;
- workspace size or persistence requirement;
- machine constants;
- successful linkage or runtime behavior;
- redistribution permission.

## FFI evidence gates

A field may drive generated raw FFI only when:

1. its source artifact is checksum-pinned and retrieval is complete;
2. its declaration is `verified` from parsed source;
3. its locator resolves to the selected provider;
4. no unresolved conflict affects the field;
5. parser output has passed required human review or an approved deterministic validation;
6. ABI-sensitive properties have a matching compiler/target probe where required.

Safe-wrapper generation additionally requires verified semantic evidence for argument intent, aliasing, workspace rules, callback behavior, error behavior, retained state, and thread safety. Absence of evidence must result in an unsafe or unavailable API, not an optimistic safe wrapper.

## Collection semantics

A collection field must carry a `collection_state`:

- `known_values` — listed elements are the complete known result for the declared scope.
- `known_empty` — the declared scope was fully inspected and no elements were found.
- `partial` — listed values are incomplete.
- `unknown` — extraction or evidence has not established the values.
- `not_applicable` — the field does not apply to this entity.

An empty array without `collection_state` is invalid. `known_empty` requires verified or derived complete-coverage evidence.

## Deterministic derivation

Every mechanical result records:

- tool and semantic version;
- parser or algorithm version;
- input artifact and manifest hashes;
- configuration hash;
- extraction timestamp;
- deterministic ordering rules;
- output semantic hash;
- warnings and unparsed material.

A derived result becomes stale when any declared input, parser semantic version, schema major version, or configuration changes.
