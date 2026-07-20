# Batch A public raw interfaces

Batch A is the corpus-wide promotion of straightforward, non-callback SLATEC
interfaces. It provides unsafe canonical paths for the mechanically eligible
historically user-callable routines while preserving the distinction between a
source/ABI/link-validated raw declaration and a hand-reviewed or safe API.

The generated authoritative inputs and outputs are under
[`generated/raw-api/`](../../generated/raw-api/):

- `abi-classification.json` classifies every retained identity;
- `batch-a-candidates.json` records each promoted routine's source hash,
  normalized ABI fingerprint, native symbol, canonical path, and features;
- `batch-a-exclusions.json` gives every non-candidate an explicit reason;
- `batch-a-correction-report.json` lists the source-hash-guarded correction
  layer (empty when extraction required no correction); and
- `batch-a-summary.md` gives the reproducible corpus counts and policy.

## Scope and boundary

Batch A accepts scalar `INTEGER`/`REAL`/`DOUBLE PRECISION` values, supported
`LOGICAL` values, numerical arrays, leading dimensions, caller-owned workspace,
and normal scalar returns. Every promoted source has an observed native symbol,
an exact selected-provider hash, a generated canonical declaration, a coherent
`slatec-sys` feature, and an exact `slatec-src` source closure.

It deliberately excludes callbacks and other procedure arguments, CHARACTER
arguments and hidden lengths, complex returns, complex arguments, alternate
returns/ENTRY ambiguity, missing or ambiguous symbols, unresolved provider
coverage, and non-public subsidiaries or runtime support. An exclusion is a
deferment decision, not an inferred ABI.

The declarations' Rustdoc gives the parsed declaration, source, symbol, ABI
class, and conservative pointer/array safety contract. It does not invent
argument intent, extent formulas, aliasing permission, pointer retention,
error semantics, or numerical guarantees where the normalized source evidence
cannot prove them. Consult the original SLATEC prologue before calling.

## Paths and providers

Canonical modules follow the established mathematical taxonomy. Examples are
`slatec_sys::special::numerical::acosh`,
`slatec_sys::quadrature::numerical::davint`,
`slatec_sys::linear_algebra::dense::dgefa`, and
`slatec_sys::interpolation::numerical::pchsp`.

Enable a mathematical `slatec-sys` feature such as `special`, `quadrature`,
`linear-algebra`, or `interpolation`. This exposes declarations only. For the
verified source provider, select the matching `slatec-src` `batch-a-*` feature;
an application may instead supply compatible native symbols through its own
external provider. `slatec-sys/all` remains declaration-only.

Some Batch A paths re-export an existing family declaration. Other paths own a
single conditional declaration when the transitional ABI-shaped generated
module is not selected. In either configuration there is one declaration per
native symbol; compatibility and canonical paths never compile independent
duplicate `extern` items.

## Validation and regeneration

The maintained offline workflow is:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-batch-a --offline
cargo run -p slatec-tools --bin slatec-corpus -- generate-linkage-metadata --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-batch-a --offline
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-api-inventory --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-api-inventory --offline
```

The generator emits deterministic canonical compile probes and native link
probes in batches of at most 96 routines. Representative native smoke tests
exercise scalar returns, scalar outputs, array/status output, leading-dimension
matrix storage, and workspace/sequence output. Those tests are deliberately a
risk-based ABI check, not numerical validation of the full corpus.

Batch A is an intermediate raw-API stability tier: its source hash, ABI
fingerprint, canonical path, feature, and native symbol are checked, but it is
not a substitute for the authored semantic contract required by a reviewed
declaration or safe wrapper. Future batches cover callbacks and ABI-sensitive
interfaces.
