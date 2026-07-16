# Deterministic extraction toolchain architecture

**Task:** E06  
**Scope:** implementation-ready specification; no parser code is implemented here.

## Architectural decision

The extraction system is a staged, content-addressed pipeline. Each stage consumes immutable manifests, emits immutable outputs plus diagnostics, and records the exact tool version, configuration hash, schema version, and input semantic hashes. A downstream stage must never infer that an upstream stage succeeded merely because output files exist.

The canonical input is the checksum-pinned source corpus selected by E03. The preserved archive and raw extracted members remain unchanged. Normalized parser views, parsed records, reviewed resolutions, patches, and generated outputs are separate layers.

## Required stages

1. **Acquire** — obtain an artifact, record URL, redirects, timestamp, size, media type, and SHA-256.
2. **Verify** — compare the artifact against the policy-pinned checksum and expected size.
3. **Inspect archive** — enumerate members without extraction; reject unsafe paths, escaping links, devices, and unsupported entry kinds.
4. **Extract raw members** — copy regular members byte-for-byte into a content-addressed evidence store.
5. **Build source-file manifest** — assign stable IDs, raw hashes, path identities, encoding observations, and corpus-selection state.
6. **Create parser views** — normalize line endings only; retain a reversible raw-byte to parser-position map.
7. **Lex fixed-form source** — classify physical lines, continuation, labels, comments, strings, Hollerith constants, and statement boundaries.
8. **Parse program units** — produce a lossless syntax representation and exact spans for declarations and executable statements.
9. **Parse prologues** — recognize dialects, preserve raw fields, and attach field-level locators.
10. **Extract semantics** — derive routines, arguments, calls, function references, callbacks, state, entries, alternate returns, and ABI-relevant facts.
11. **Reconcile claims** — preserve contradictory evidence, form conflict groups, and apply only approved scoped resolutions.
12. **Validate** — run structural schema validation and cross-record semantic validation.
13. **Queue manual review** — emit deterministic review items for unsupported syntax, ambiguity, conflicts, and generation gates.
14. **Export** — write canonical JSON plus deterministic TOML/Markdown projections and manifests.

## Components

| Component | Responsibility | Must not do |
|---|---|---|
| `artifact-acquirer` | retrieval metadata and archive bytes | choose a canonical provider |
| `archive-inspector` | safe member inventory and extraction | rewrite names or contents |
| `source-manifest` | source identities and hashes | parse Fortran semantics |
| `fixed-form-lexer` | physical/logical statement construction | infer types or intent |
| `fortran-parser` | lossless program-unit syntax | discard unsupported statements |
| `prologue-parser` | dialect recognition and raw field capture | treat documentation as executable truth |
| `semantic-extractor` | typed facts and dependency candidates | resolve conflicts silently |
| `evidence-reconciler` | claims, conflicts, review states | alter source text |
| `validator` | schema, references, hashes, gates | repair invalid records automatically |
| `exporter` | canonical ordering and projections | add unstated facts |

## Content-addressed runs

A run ID is derived from:

- tool name and version;
- schema version;
- parser grammar version;
- canonical-corpus policy hash;
- configuration semantic hash;
- sorted input artifact/member hashes;
- declared environment-affecting options.

Timestamps are retained for audit but excluded from semantic run IDs.

## Failure model

Stages return one of `success`, `success_with_review_items`, or `failed`. Unsupported syntax is not necessarily a pipeline failure if exact raw spans are retained, but it must lower coverage and create review items. Checksum mismatch, unsafe archive paths, non-resolving verified locators, duplicate selected providers, or nondeterministic output are fatal.

## Restart and cache policy

A stage may reuse prior output only when its complete input fingerprint and implementation fingerprint match. Cache hits are recorded. Partial output from a failed stage is never promoted to an authoritative manifest.

## Parallelism

Files may be parsed in parallel. Output aggregation must sort by stable IDs and source positions so thread scheduling cannot affect bytes. Conflict IDs and diagnostic IDs must be derived from stable inputs, not insertion order.
