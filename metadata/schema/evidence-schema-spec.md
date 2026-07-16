# Evidence schema specification

## Normative validation architecture

The future implementation should parse TOML or YAML into a representation-neutral data model and validate it against JSON Schema Draft 2020-12. A second semantic validator must perform checks that JSON Schema cannot reliably express across files or generated graphs.

No validator is implemented by E05.

## Schema modules

Recommended schema IDs:

- `slatec-rs/evidence`
- `slatec-rs/artifact`
- `slatec-rs/source-file`
- `slatec-rs/program-unit`
- `slatec-rs/routine`
- `slatec-rs/dependency`
- `slatec-rs/provenance`
- `slatec-rs/rights`
- `slatec-rs/conflict`
- `slatec-rs/patch`
- `slatec-rs/build`
- `slatec-rs/release`

## Required definitions

The evidence schema must define:

- stable ID pattern;
- semantic schema version;
- SHA-256 pattern;
- RFC 3339 timestamps;
- fact, retrieval, provenance, relationship, coverage, conflict, review, rights, and build statuses;
- statement kinds;
- source locator union;
- extraction-run record;
- field claim;
- collection state;
- conflict group and resolution;
- generator gate result.

## Structural invariants

1. IDs are unique within their namespace.
2. Every claim has exactly one statement kind and fact status.
3. `verified` claims have at least one resolving locator to inspected evidence.
4. `derived` claims identify a deterministic run and verified input claims.
5. `inferred` claims list supporting claims and method.
6. `conflicting` fields reference an open or resolved conflict group.
7. `known_empty` collections have complete-coverage evidence.
8. `not_applicable` values do not carry ordinary values.
9. rights records use only E04 status values.
10. timestamps are excluded from semantic-hash canonicalization.

## Cross-record semantic validation

The project validator must verify:

- all referenced artifact, member, claim, run, conflict, policy, package, domain, and provider IDs exist;
- artifact/member hashes agree with manifests;
- source spans resolve and context hashes match;
- selected providers agree with canonical-corpus policy;
- duplicate definitions have explicit resolutions;
- generated FFI fields meet evidence gates;
- safe-wrapper fields meet stronger semantic gates;
- open rights conflicts block release scopes;
- build claims match the exact compiler, target, flags, providers, and source snapshot;
- deterministic sort order and semantic hashes reproduce.

## Gate profiles

### Inventory gate

Permits verified, derived, inferred, unverified, conflicting, and unknown records, provided status and provenance are explicit.

### Dependency-analysis gate

Requires checksum-pinned source, parser identity, resolving locators, and explicit unresolved references. Inferred edges may appear but cannot be treated as link-required without corroboration.

### Raw-FFI gate

Requires verified selected declarations and no open signature/provider/ABI conflict. Compiler-sensitive details require an applicable ABI probe.

### Safe-API gate

Requires raw-FFI gate plus reviewed intent, aliasing, workspace, callback, state, error, and thread-safety evidence. Unknown safety-relevant properties block safe generation.

### Release gate

Requires approved source/provider manifest, complete provenance, applicable rights decisions, notice bundle, patch manifest, reproducible build record, and no open release-impact conflict.

## Canonical serialization

For semantic hashing:

- encode through a documented canonical JSON representation;
- sort object keys lexicographically;
- sort set-like arrays by stable ID;
- preserve order-sensitive arrays explicitly;
- normalize no source text as part of metadata serialization;
- omit volatile run timestamps from semantic hashes while retaining them in records;
- include schema, policy, parser, configuration, and input manifest hashes.

## Migration validation

A migration test corpus must include:

- legacy generic evidence statuses;
- located-but-uninspected sources;
- empty arrays with ambiguous meaning;
- inferred package ownership;
- conflicting providers;
- source/prologue dependency disagreement;
- machine-provider alternatives;
- unresolved rights;
- stale build fingerprints.

Each migration fixture must assert preserved claims, explicit ambiguity, and stable output on repeated runs.
