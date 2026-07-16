# Linker test policy

## Test levels

1. `object-link`: link one root object with an explicit object set;
2. `archive-link`: link against proposed static archives in declared order;
3. `component-link`: link proposed native components and external providers;
4. `ffi-link`: link the generated raw-FFI probe;
5. `runtime-smoke`: execute a non-destructive call or quick check where possible.

## Fingerprint

Every result is scoped to source snapshot, provider policy, patch set, compiler and version, target, flags, archive tool, linker and version, library search paths, environment allowlist, component manifests, and command hashes.

## Minimal-root tests

Generate one test per selected public root plus targeted tests for callbacks, COMMON state, BLOCK DATA, machine constants, error hooks, character arguments, complex returns, and alternate returns. Tests must not rely on an umbrella executable that accidentally supplies unrelated symbols.

## Failure and success interpretation

- Failure records all unresolved symbols and diagnostics; it does not automatically identify the missing source provider.
- Success is valid only when the resolved-provider inventory is controlled or audited.
- Success with undeclared host/system resolution is `contaminated`, not verified.
- Runtime success does not establish thread safety.

## Archive-order tests

Where cycles or traditional linkers make order relevant, test the documented order, repeated archives or linker groups, and direct object linkage. Record the least permissive reproducible recipe rather than assuming modern linker behavior.

## Determinism

Remove volatile paths from the semantic result while preserving raw logs separately. Repeated tests under the same fingerprint must yield the same provider map and status.
