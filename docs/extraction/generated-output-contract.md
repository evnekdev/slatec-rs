# Generated output contract

## Authoritative and projected formats

Canonical JSON is the authoritative generated representation for semantic hashing and schema validation. TOML and Markdown are deterministic projections. A projection must identify the canonical JSON semantic hash it represents.

## Output tree

A run emits:

```text
manifest.json
artifacts.json
source-files.json
program-units.json
routines.json
claims.json
locators.json
conflicts.json
review-items.json
diagnostics.json
extraction-runs.json
gate-results.json
projections/
```

Large datasets may be sharded by stable ID prefix, provided a root manifest lists every shard and hash.

## Run manifest

The manifest contains:

- schema IDs and versions;
- generator components and versions;
- parser grammar/prologue rule versions;
- canonical-corpus policy hash;
- input artifact and member manifest hashes;
- configuration semantic hash;
- output file hashes;
- semantic dataset hash;
- deterministic-order assertion;
- nonsemantic audit timestamp;
- stage results and review counts.

## Ordering

- objects use lexicographically sorted keys in canonical JSON;
- entity arrays sort by stable ID;
- source spans sort by artifact/member/start/end;
- set-like arrays sort and deduplicate;
- source-order arrays such as arguments and revision entries retain explicit order indices;
- diagnostics sort by severity, source locator, rule ID, and stable diagnostic ID.

## Null and collection semantics

Outputs follow E05: omitted, unknown, not applicable, false, known empty, and partial are distinct. Bare empty arrays are invalid for evidence-sensitive collections unless accompanied by collection state.

## Diagnostics

Every diagnostic includes stable rule ID, severity, message template ID, arguments, locator, stage, and review impact. Human wording may evolve without changing semantic diagnostic identity.

## Gate results

Inventory, dependency, raw-FFI, safe-API, and release gates are calculated per applicable entity and dataset. A result lists passed rules, failed rules, blocking conflict/review IDs, and the exact validator version.

## No silent overwrite

Generation writes to a fresh run directory. Promotion of a run to `current` is an explicit atomic operation after validation. Existing authoritative output is never edited in place.

## Round-trip and traceability

Every verified or derived field resolves through claims and locators to source bytes or deterministic input claims. Generated Markdown includes source identifiers and does not become independent evidence.
