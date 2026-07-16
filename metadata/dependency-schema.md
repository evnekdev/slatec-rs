# Dependency metadata schema

## Purpose

This schema defines the canonical reviewable representation of SLATEC dependency, symbol, state and ownership data. It complements `metadata/routine-schema.md`; routine records describe interfaces, while dependency records describe relationships and native-build consequences.

## Files

A complete extraction may be partitioned into several TOML files, but all use the same `schema_version` and `snapshot_id`:

- nodes;
- direct edges;
- transitive closures;
- unresolved references;
- strongly connected components;
- duplicate symbols;
- ownership mappings;
- build-validation results.

## Evidence status

Allowed status values follow repository policy:

- `verified`
- `derived`
- `inferred`
- `unverified`
- `conflicting`
- `unknown`

`verified` means directly confirmed in the identified artifact. `derived` means mechanically computed from verified inputs. A successful link is verified only for the exact build fingerprint.

## Top-level fields

| Field | Type | Meaning |
|---|---|---|
| `schema_version` | integer | schema format version |
| `snapshot_id` | string | stable ID for selected source inventory |
| `generated_at` | RFC 3339 string | generation time |
| `generator` | string | tool and version |
| `source_manifest_sha256` | string | checksum of input manifest |
| `sorted` | boolean | deterministic ordering assertion |

## Nodes

Each `[[nodes]]` record contains:

- `id`: globally unique stable ID such as `routine:dqag` or `common:xercom`;
- `kind`: node kind from the extraction specification;
- `name`: original or canonical display name;
- `source_file`: optional repository-relative source path;
- `package_id`, `domain_ids`: optional classification;
- `selected`: whether this implementation is active in the current build policy;
- `evidence_status`;
- `attributes`: optional scalar metadata.

Program units with alternate entries receive distinct IDs but share a source/object owner.

## Direct edges

Each `[[edges]]` contains:

- `id`;
- `from`, `to` node IDs;
- `kind`;
- `condition`: optional source condition or build predicate;
- `link_relevant`: boolean;
- `closure_relevant`: boolean;
- `evidence_status`;
- one or more nested `[[edges.evidence]]` records.

Evidence records contain:

- `kind` such as `source_ast`, `prologue`, `object_undefined`;
- `source_id` and/or artifact path;
- `locator`;
- `snapshot_id`;
- `detail`;
- optional checksum.

Several evidence records may support or contradict one edge.

## Transitive closure

Each `[[closures]]` record contains:

- `root`;
- `edge_kinds` used;
- sorted `reachable` node IDs excluding the root unless configured otherwise;
- `max_depth`;
- `status = "derived"`;
- `direct_graph_sha256`;
- optional comparison with Netlib `tree`.

Closure must not include precision-peer, documentation-only, ownership or ordinary common-state edges unless a named specialized closure requests them.

## Unresolved references

Each `[[unresolved]]` record contains:

- `from`;
- raw `name` and normalized symbol;
- `reference_kind`;
- `classification`;
- candidate targets;
- `required_for_link`;
- evidence;
- resolution state and notes.

Allowed classifications include callback, intrinsic, runtime, external provider, optional hook, missing source, ambiguous name, parser defect and unknown.

## Strongly connected components

Each `[[sccs]]` contains:

- stable `id` derived from sorted members;
- sorted `members`;
- source files;
- incoming/outgoing component IDs;
- packages/domains represented;
- common-state nodes touched;
- `native_atomicity` recommendation;
- graph checksum.

Singletons without self-edges may be omitted from human summaries but should remain available in machine output if needed.

## Duplicate groups

Each `[[duplicates]]` contains:

- `symbol`;
- candidate defining nodes/source files/objects;
- mangled spellings observed;
- semantic relationship (`identical`, `variant`, `shim`, `unknown`);
- selected provider;
- resolution policy;
- evidence status.

## Ownership mappings

Each `[[ownership]]` maps one node/source/symbol to:

- `native_component`;
- `raw_crate`;
- optional safe wrapper crates;
- reason codes;
- confidence;
- feature predicate.

Validation invariants:

- one selected owner per source file;
- one selected provider per external implementation symbol;
- alternate entries share source/native owner;
- no selected source is compiled by two native components;
- component dependencies form a DAG after SCC collapse.

## Build validation

Each `[[builds]]` records:

- unique build fingerprint;
- compiler/version/target/flags;
- provider and feature configuration;
- component manifests;
- object symbol inventory checksum;
- link-test results;
- runtime/quick-check results;
- accidental host resolutions detected;
- validation status.

Nested link tests identify root, supplied components, result, diagnostics checksum and observed unresolved symbols.

## Determinism

Canonical exports must:

- use lowercase stable IDs;
- sort nodes by ID and edges by `(from, kind, to, condition)`;
- sort set-like arrays;
- preserve original spellings in separate fields;
- avoid timestamps in content hashes;
- include schema and input checksums;
- produce identical semantic output from identical inputs.

## Validation rules

1. All edge endpoints exist.
2. Node IDs are unique.
3. Edge IDs are unique.
4. Closure graph hash matches direct-edge input.
5. Every unresolved link-relevant reference is classified.
6. Every selected program unit has one source file.
7. Every selected source file has one native owner.
8. Every selected exported symbol has one provider.
9. SCCs recompute exactly from declared closure-relevant edges.
10. Ownership component graph is acyclic.
11. Evidence locators identify an available artifact.
12. Package/domain/crate IDs reference registered IDs or are explicitly provisional.

## Generated Markdown

For each root summary include:

- identity and selected implementation;
- direct dependencies grouped by edge kind;
- transitive native closure;
- callback and common-state requirements;
- unresolved/conflicting references;
- SCC membership;
- native/raw/safe ownership;
- validation builds and last verified result;
- sources and checksums.

Generated pages must identify the generator, graph snapshot and review status.

## Evolution

Schema changes require incrementing `schema_version` and providing a migration note. New edge/node kinds may be added, but existing meanings must not be silently broadened.
