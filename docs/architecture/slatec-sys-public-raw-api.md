# `slatec-sys` public raw API

## Goal and evidence boundary

`slatec-sys` is the unsafe, profile-specific Rust boundary for feasible SLATEC
program units. It does not download, compile, or bundle Fortran. A callable
routine is only stable after the raw API inventory records a selected provider,
source hash, native symbol, reviewed declaration, canonical Rust path, feature
and provider feature, routine and argument documentation, and the required
validation evidence.

The generator creates exactly one record for every retained catalogue identity:
[`generated/raw-api/routine-status.json`](../../generated/raw-api/routine-status.json).
It replaces the ambiguous historical aggregate with separate,
reproducible coverage metrics in
[`coverage-summary.json`](../../generated/raw-api/coverage-summary.json).

The currently supported ABI is the observed GNU Fortran
`x86_64-w64-mingw32` profile, exposed by
`ffi-profile-gnu-mingw-x86_64`. It is not a portability promise. `INTEGER` and
`LOGICAL` are `i32`; hidden CHARACTER lengths use `usize`; and the observed
lowercase-underscore symbol spelling is part of the profile evidence.

## Status model

Every record has one `raw_api_state`. The finite states distinguish reviewed
drivers and subsidiaries from generated candidates, ABI-validated generated
declarations, source-present-but-unbound routines, callback and special-return
ABI blockers, conflicting/ambiguous/missing symbols, non-callable subsidiaries,
runtime support, block data, documentation/tooling entries, external
dependencies, and catalogue-only identities. The machine-readable enumeration
is `reviewed_public_driver`, `reviewed_public_subsidiary`,
`batch_a_public_driver`, `batch_b_public_driver`, `batch_c_public_driver`,
`generated_candidate`, `generated_abi_validated`, `source_present_unbound`,
`unsupported_callback_abi`, `unsupported_complex_return_abi`,
`unsupported_character_return_abi`, `unsupported_entry_or_alternate_return`,
`conflicting_interface`, `ambiguous_symbol`, `missing_symbol`,
`not_independently_callable`, `runtime_or_machine_support`, `block_data`,
`documentation_or_tooling`, `catalogue_only`, and `external_dependency`.

`generated_abi_validated` says that a declaration shape belongs to a successful
compiler/profile batch. It does **not** say that its callback lifetime, pointer
semantics, workspace, error handling, or public documentation have been
reviewed. Only the hash-guarded authored correction layer can emit
`reviewed_public_driver` or `reviewed_public_subsidiary`.

`batch_a_public_driver` is a separate, generated stability tier for a
historically user-callable, non-callback numerical ABI. It has an exact source
hash, parsed ABI fingerprint, observed symbol, canonical path, provider
closure, generated argument/Safety contract, and bulk compile/link evidence.
It is not hand-reviewed semantic documentation and is never counted as a
`reviewed_*` declaration. The full policy and regeneration workflow are in
[Batch A raw interfaces](../api/raw-batch-a.md).

`batch_b_public_driver` is the matching generated stability tier for
callback-bearing historically public routines whose outer ABI and callback ABI
are both reconstructed from selected fixed-form source. It records callback
fingerprints, forwarding evidence, canonical paths, features, source hashes,
compile probes, and native link probes. It is still an unsafe raw tier: it does
not create safe Rust closures, user-data trampolines, panic containment, or
full semantic argument review. See
[Batch B callback-bearing raw interfaces](../api/raw-batch-b-callbacks.md).

`batch_c_public_driver` is the generated stability tier for complex numerical
and simple flag-bearing interfaces. Promotion requires independent evidence for
every complex, fixed `CHARACTER*1`, `LOGICAL`, and callback constituent, plus a
selected source hash, unique symbol, canonical path, exact provider closure,
generated unsafe documentation, and bulk compile/link evidence. Complex returns
use only the compiler-probed GNU MinGW convention; raw logical values remain
explicit `i32` ABI values rather than Rust `bool`. See
[Batch C complex and flag-bearing interfaces](../api/raw-batch-c-complex-character-logical.md).

## Canonical namespace

The long-term taxonomy is:

```text
blas                         linear_algebra::{dense,banded,packed,sparse}
eigen                        roots::{scalar,polynomial}
nonlinear                    least_squares
optimization                 quadrature
ode                           dae
pde::fishpack                fftpack
interpolation                approximation
special                      statistics
integral_equations           runtime
```

`module-taxonomy.json` maps every retained identity to one intended module.
R1 creates only the namespaces needed for reviewed paths:

```rust
slatec_sys::roots::scalar::{fzero, dfzero}
slatec_sys::pde::fishpack::{hwscrt, pois3d}
```

R2A additionally establishes stable public BLAS paths:

```rust
slatec_sys::blas::level1::daxpy
slatec_sys::blas::level2::dgemv
slatec_sys::blas::level3::dgemm
```

R2B establishes reviewed scalar special-function paths without changing the
existing safe facade:

```rust
slatec_sys::special::elementary::dlnrel
slatec_sys::special::gamma::dgamma
slatec_sys::special::beta::dbetai
slatec_sys::special::error::derfc
```

R2C adds the reviewed real FNLIB Airy drivers under one canonical module:

```rust
slatec_sys::special::airy::{ai, aie, bi, bie, dai, daie, dbi, dbie}
```

The earlier `slatec_sys::families::special_airy::*` items remain compatibility
re-exports of those declarations. Complex Amos Airy drivers and Airy
subsidiaries are reported explicitly, but are not promoted by the real-scalar
review.

Batch A extends the existing taxonomy with generated canonical numerical
submodules where required, including `linear_algebra::dense`, `eigen::numerical`,
`quadrature::numerical`, `interpolation::numerical`, `fftpack::numerical`,
`pde::fishpack::numerical`, `ode::numerical`, `approximation::numerical`,
`statistics::numerical`, and `special::numerical`. The direct paths are stable
within the Batch A evidence boundary; `generated::*` remains transitional.

Batch B adds generated callback namespaces where the callback ABI has been
source-reconstructed:

```rust
slatec_sys::quadrature::callbacks::qk15
slatec_sys::linear_algebra::sparse::callbacks::scg
slatec_sys::ode::callbacks::derkf
```

These paths are stable within the Batch B evidence boundary. They remain raw
FFI declarations: a Rust callback must be ABI-compatible, must not unwind
across native code, and must manage any captured state outside the declaration.

Batch C adds `complex` submodules inside the established mathematical taxonomy,
including `linear_algebra::dense::complex`, `special::complex`,
`nonlinear::complex`, and `pde::fishpack::complex`, while complex BLAS entries
remain at their conventional BLAS levels. The private generated owner is not a
user-facing `batch_c` namespace.

The complete feasible BLAS set is generated from the source-hash-guarded
family-review policy rather than copied into hand-written `extern` blocks.
Each canonical item re-exports the single ABI-shaped generated declaration;
`slatec_sys::families::blas_level{1,2,3}` are compatibility re-exports of
that same item. The public surface therefore has no duplicate declarations.
The companion [`blas-family-report.json`](../../generated/raw-api/blas-family-report.json)
records all retained BLAS classifications, including the Batch C promotion of
complex-return functions, non-BLAS multiprecision subsidiaries, and
catalogue-only entries.

The legacy `slatec_sys::roots::{fzero,dfzero}`,
`slatec_sys::fishpack_cartesian_2d::hwscrt`, and
`slatec_sys::fishpack_pois3d::pois3d` paths are compatibility re-exports. A
routine has one declaration: compatibility paths re-export it and never add a
second `extern` item.

Drivers are organized before subsidiaries. The recorded roles are historically
user-callable driver, expert public primitive, internal subsidiary, shared
utility, runtime support, and not independently callable. Future promoted
subsidiaries belong below an explicit `subsidiary` namespace rather than beside
principal drivers.

## Documentation and correction contract

Every reviewed raw declaration must have a one-sentence purpose, original
routine name, precision, mathematical operation, selected source/provider,
supported ABI profile, native symbol, complete argument list, I/O and mutation
semantics, array extents and leading dimensions, workspace rules, callback
contract where relevant, status meanings, global-state notes, and a `# Safety`
section. Pointer arguments explicitly state nullability, readable/writable
extent, direction, aliasing, and retention. Missing intent is recorded as
unavailable rather than inferred from an argument name.

Documentation fields record evidence as `source_prologue`,
`executable_declaration`, `selected_provider_metadata`,
`compiler_observation`, `authored_override`, `safe_wrapper_audit`, or
`unavailable`. The deterministic correction registry is
[`metadata/raw-api-corrections.json`](../../metadata/raw-api-corrections.json).
It is intentionally small, keyed by stable identity and selected source SHA-256,
and generation rejects it when the source hash or executable argument order
changes.

The deterministic documentation audit reports public extern declarations,
routine and argument documentation, Safety sections, source links, ABI-profile
statements, and exact missing fields. The raw API review queue is therefore a
work plan, not generated prose with guessed semantics.

## Features, providers, and validation

The raw declaration feature in `slatec-sys`, the native closure feature in
`slatec-src`, and an optional safe facade feature in `slatec` are joined in
[`feature-provider-map.json`](../../generated/raw-api/feature-provider-map.json).
Reviewed records fail validation when any required feature is absent. A raw
consumer may use `slatec-sys` with an explicit external provider; `slatec-sys`
does not select a backend implicitly.

Batch A records the same raw/provider relationship using coherent
`batch-a-*` closures. Those closure features are provider-only selectors; they
do not turn the declaration-only `slatec-sys/all` feature into a native backend.
Batch B records `batch-b-*` declaration features with matching coherent
provider features for the source families that own the callbacks. The initial
provider mapping is `batch-b-quadrature` to `slatec-src/quadrature`,
`batch-b-linear-algebra` to `slatec-src/linear-algebra`, and `batch-b-ode` to
`slatec-src/ode`.
Batch C uses five coherent mathematical declaration/provider pairs:
`batch-c-blas`, `batch-c-linear-algebra`, `batch-c-special`,
`batch-c-nonlinear`, and `batch-c-fishpack`. Their exact source closures are
generated from candidate roots and compiler-observed dependencies.

The public `all` feature directly names every authored public mathematical
family aggregate. It is declaration-only: provider/backend, profile-only,
ABI-shaped generated, raw-family, compatibility, and test-only switches are
not direct members. The generated
[`all-feature-coverage.json`](../../generated/raw-api/all-feature-coverage.json)
checks that the registry and Cargo feature graph have no missing or unexpected
aggregate members. A reviewed record also states whether its declaration
feature is transitively reached by `all`.

Validation levels are separate: source/hash and catalogue checks, compiler and
symbol observation, declaration signature audit, compile-only canonical-path
imports, native link validation, runtime validation, and safe-wrapper tests.
The R1 command is:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-api-inventory --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-api-inventory --offline
```

Validation rejects a reviewed entry lacking its source hash, unique observed
symbol, executable argument order, ABI/profile evidence, unique canonical path,
raw and provider feature, complete docs, or required link/runtime status.
Batch A has an analogous validator for its narrower generated-contract bar,
including candidate source-hash stability, canonical-path uniqueness, feature
coverage, exact provider-closure membership, and every generated Rustdoc
argument/Safety/source-link marker.
Batch B validation adds callback-specific invariants: every promoted routine
must have at least one callback ABI fingerprint, callback evidence must come
from direct or forwarded source calls, unresolved or conflicting callback
signatures are excluded, and the generated compile/link probes must import or
reference every promoted canonical path.
Batch C additionally requires controlled and selected-source compiler probes
for complex layout and returns, character hidden lengths, and logical values.
Its validator rejects long strings, unresolved combinations, missing provider
closure members, incompatible fingerprints, and duplicate canonical paths.

## Stability and transition policy

For `0.1.x`, reviewed canonical paths and routine names do not move. Features
are additive where practical and path corrections retain compatibility
re-exports. Evidence-proven ABI corrections may change a signature because an
incorrect FFI declaration is a safety bug. The ABI-shaped
`slatec_sys::generated` namespace remains available during 0.x, but is
transitional, unstable, and never the sole stable path for a promoted routine.

An excluded identity has a machine-readable reason. The R2 promotion queue is
evidence-ranked in `promotion-priority.json`; R3 is reserved for callback
drivers, R4 for exceptional return and CHARACTER/complex interfaces, and R5
for the publication freeze audit. `roots-family-report.json` is the required
pilot gap report; HWSCRT and POIS3D remain tracked reviewed drivers with their
selected closures and native regression evidence.

## BLAS R2A evidence and ABI boundary

The reviewed BLAS policy covers 121 historically user-callable subroutines
whose GNU Fortran declaration shapes and native symbols were validated in the
numeric-scalar, numeric-array, scalar-function, complex-argument, and
CHARACTER ABI batches. The policy is keyed to a manifest of selected provider
source hashes; any source drift stops regeneration before a reviewed path can
be emitted.

BLAS character options are native one-byte CHARACTER buffers followed by the
compiler-observed trailing hidden length values (`FortranCharacterLength`), in
visible argument order. `Complex32` documents the selected GNU Fortran
COMPLEX storage record. The three complex-return functions are deliberately
outside the R2A reviewed tier; Batch C later declares them after establishing
the correct Rust return ABI with controlled and selected-source probes.
Routine documentation is generated from verified prologue facts plus a small
authored BLAS operation template, and its audit checks every exported routine,
argument, source link, ABI statement, and `# Safety` section.

## Scalar special foundations R2B evidence and ABI boundary

The source-hash-guarded R2B policy promotes exactly 40 feasible, historically
user-callable scalar routines in four groups: elementary (12), gamma (18),
beta (6), and error (4). Each re-exports one validated generated declaration
under `slatec_sys::special::{elementary,gamma,beta,error}`; the former
`families::special_*` paths remain compatibility re-exports, not second FFI
declarations. The candidate report records every target routine, source hash,
provider, symbol, prior family path, safe-wrapper audit, disposition, and
deferment reason.

These are direct scalar-return functions for the supported GNU MinGW ABI;
their scalar input pointers are non-null, readable, non-retained, and
read-only. Documentation derives domain and XERROR facts from selected source
prologues plus focused authored metadata. FNLIB saves initialization state and
legacy error controls are process-global; the raw binding deliberately makes
no thread-safety claim. `GAMR`/`DGAMR` additionally use XGETF/XSETF/XERCLR and
must be serialized with other consumers of the legacy error runtime. Other
special domains, complex returns, callback-bearing interfaces, and unresolved
semantics remain unpromoted.

## Real Airy R2C evidence and ABI boundary

The source-hash-guarded R2C policy promotes exactly eight historically
user-callable real FNLIB scalar functions: single- and double-precision Ai,
Bi, and their exponentially scaled forms. Each accepts one real scalar by
address and returns its real result directly under the observed GNU MinGW ABI.
The generated Rustdoc records the selected source and symbol, scaling formula,
XERROR range behavior for unscaled functions, FNLIB global-state caveat, and
the complete raw pointer contract.

The canonical `special::airy` declarations re-export the one existing
compiler-observed `families::special_airy` declaration; no duplicate `extern`
block is introduced. `airy-family-report.json` covers the eight reviewed real
drivers alongside CAIRY/CBIRY/ZAIRY/ZBIRY and direct Airy helpers, which remain
deferred pending their complex or subsidiary-contract review. The direct raw
link probe imports all eight real symbols, while the safe facade keeps its
existing public `slatec::special::airy` paths and serializes FNLIB calls.
