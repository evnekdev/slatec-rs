# `slatec-sys` public raw API

## Purpose and evidence boundary

`slatec-sys` is the unsafe, provider-neutral Rust declaration layer for feasible
SLATEC program units. It does not download, compile, bundle, select, or link a
Fortran implementation. Provider selection belongs to `slatec-src` or to the
final application.

The authoritative inventory contains exactly one record for each of the 1,517
retained identities. It distinguishes 812 canonical public raw routines from
compatibility aliases, generated candidates, provider subsidiaries, support
units, and permanent exclusions. Compatibility aliases are paths, not new
functions. Current counts and evidence are generated under
[`generated/raw-api`](../../generated/raw-api/) and
[`generated/public-api`](../../generated/public-api/).

## Supported ABI profile

The currently validated native profile is GNU Fortran on
`x86_64-pc-windows-gnu`, selected by `ffi-profile-gnu-mingw-x86_64`.
`INTEGER` and `LOGICAL` use 32-bit storage, `LOGICAL` is not Rust `bool`, hidden
`CHARACTER` lengths use `usize`, and observed native names use the
lowercase-underscore convention. Complex argument and return layouts and
callback signatures are accepted only when compiler and selected-source
evidence agree. This profile is not a portability promise.

## One declaration owner

Every public native symbol has exactly one authoritative Rust `extern`
declaration in a private implementation module. User-facing mathematical
modules and deprecated compatibility namespaces only re-export that item:

```text
private authoritative declaration
            -> canonical mathematical re-export
            -> optional deprecated compatibility re-export
```

Feature combinations control reachability, never declaration ownership. The
same symbol cannot be rendered independently by a canonical module, an
ABI-shaped compatibility module, or a compile probe. The generated
[`ffi-declaration-ownership.json`](../../generated/public-api/ffi-declaration-ownership.json)
records the declaration path, canonical path, aliases, ABI fingerprint, feature
gates, and declaration count. Validation rejects duplicate link names,
incompatible signatures, canonical or compatibility `extern` blocks, and
aliases that inflate routine counts.

Complete routine documentation is attached to the authoritative declaration
and rendered through the canonical re-export. Compatibility paths carry only
their replacement guidance; they do not maintain a second ABI contract.

## Canonical mathematical namespace

Preferred paths describe mathematics, storage, or problem structure:

```text
slatec_sys::blas::{level1,level2,level3}
slatec_sys::linear_algebra::{dense,banded,packed,sparse,eigen}
slatec_sys::roots::{scalar,polynomial}
slatec_sys::nonlinear
slatec_sys::least_squares
slatec_sys::optimization
slatec_sys::quadrature
slatec_sys::ode
slatec_sys::dae
slatec_sys::pde::fishpack
slatec_sys::fftpack
slatec_sys::interpolation
slatec_sys::approximation
slatec_sys::special
slatec_sys::statistics
slatec_sys::integral_equations
slatec_sys::runtime
```

Storage classes `dense`, `banded`, `packed`, and `sparse` are siblings.
Eigenvalue problems live under `linear_algebra::eigen`; meaningful subfamilies
are introduced only when the corpus supports them. Empty navigation layers such
as `numerical`, `general`, or `misc` are not canonical concepts.

Examples include:

```rust
slatec_sys::linear_algebra::eigen::imtql2
slatec_sys::special::bessel::besj0
slatec_sys::quadrature::avint
slatec_sys::pde::fishpack::pois3d
```

Every renamed pre-release path remains a deprecated re-export when it was an
established user-visible path. A compatibility path always resolves to the same
Rust item and native symbol as the canonical path.

## Public status and role model

User-facing reports use stable status terms:

- `canonical-public`: callable raw routine with a canonical path and complete
  required structural evidence;
- `compatibility-alias`: deprecated path to a canonical item;
- `internal-subsidiary`: provider routine not promoted as a principal API;
- `support-routine`: runtime, error, machine, or block-data support;
- `historical-program`: demonstration or historical driver not exposed as a
  callable raw function;
- `missing-symbol`: retained identity with no verified native symbol;
- `unsupported-abi`: callable intent exists but the ABI cannot be represented
  confidently on the supported profile.

Internal generator provenance may retain finer historical classifications, but
those values are not public API taxonomy.

Roles are recorded separately as historically user-callable driver, expert
public primitive, internal subsidiary, shared utility, runtime support, or not
independently callable. Drivers are prioritized in family navigation.
Subsidiaries remain visually and structurally separate and are never hidden
merely because no safe wrapper uses them.

## Generated and reviewed declarations

Compiler-observed generated candidates are factual ABI evidence, not a claim of
semantic review. A routine becomes canonical public only after its selected
provider, source hash, unique symbol, normalized signature, canonical path,
feature, provider mapping, documentation, and required structural/link evidence
pass validation.

`slatec_sys::generated` is a transitional ABI-shaped compatibility namespace.
It remains available during the 0.x transition, but its paths are unstable
unless separately promoted to a canonical mathematical module. It must never
be the only path for a routine described as stable.

## Documentation contract

Every canonical public raw routine records:

- one-sentence purpose, meaningful description, original routine name,
  precision, mathematical operation, selected provider, source hash, native
  symbol, feature, supported ABI profile, and ABI fingerprint;
- an ordered argument record with Fortran type, Rust raw type, scalar/array
  shape, dimensions, direction, semantic description, ranges or options,
  relationships, leading-dimension rules, workspace rules, and nullability;
- a return-value section for functions and a callback contract where relevant;
- mutation, aliasing, pointer-retention, global-state, serialization, status,
  and error-code obligations; and
- a `Safety` section.

Compiler/interface facts are kept separate from source-prologue semantics.
Unknown intent, aliasing, retention, workspace, or dimension relationships are
reported explicitly and are never guessed from an argument name.

Documentation quality is not a boolean. The generated levels are
`complete_structured`, `complete_unstructured`, `purpose_only`,
`argument_contract_incomplete`, `mangled_source_prologue`,
`subsidiary_minimal`, `support_unit_minimal`, and `unavailable`. Public routines
must have a meaningful description and a structured row for every argument;
remaining source-semantic gaps stay visible in the review queue.

Narrow authored corrections live in source-hash-guarded metadata. Generation
rejects an override when the selected source hash changes. Large generated Rust
or Markdown outputs are never edited by hand.

## Features and providers

Public `slatec-sys` features use mathematical family names and expose
declarations only. `slatec-sys/all` is the additive declaration aggregate and
does not select a provider. `slatec-src` owns provider selection and the Cargo
`links = "slatec"` namespace; `slatec-sys` has no build script or `links`
identity. A final application can instead provide compatible native symbols
through an external backend.

The feature/provider reconciliation joins raw declaration features, provider
closure features, optional safe-facade features, and aggregate membership. It
rejects missing or wrong closures, duplicate providers, orphan declaration
features, and provider features with no public routine.

## Validation levels

Validation evidence is deliberately separated:

1. catalogue identity, selected provider, and source-hash validation;
2. compiler-observed program unit, ABI category, argument order, and symbol;
3. unique authoritative extern ownership and ABI fingerprint;
4. compile-only canonical and compatibility path checks across narrow, legacy,
   combined, all-feature, and no-default-feature configurations;
5. native link probes for provider-backed declarations;
6. representative runtime tests and safe-wrapper regressions;
7. routine, argument, callback, return, and Safety documentation audits;
8. catalogue/export/family/alphabetical-index reconciliation;
9. deterministic same-root and clean-root regeneration; and
10. package-content, publication-graph, license-boundary, and downstream
    consumer checks.

No native command is reported as passed unless it ran in the stated
environment. The GNU MinGW profile has the strongest native validation;
portable CI exercises declaration, documentation, package, and provider-neutral
checks without pretending to validate native numerical behavior.

## Exclusions

Every non-public retained identity has a terminal, machine-readable disposition
and a reopen condition. Exclusions distinguish subsidiaries and support units
from missing symbols, callback or special-return ABI blockers, entry or
alternate-return interfaces, compiler-specific behavior, Fortran I/O, external
dependencies, demonstrations, and catalogue-only records. New source,
provider, compiler, role, or ABI evidence may reopen a decision; routine counts
do not.

## Stability and release policy

For `0.1.x`, canonical reviewed paths and routine names should not move.
Features are additive where practical, and path corrections retain deprecated
compatibility re-exports. Evidence-proven signature corrections are permitted
because an incorrect unsafe declaration is a safety and correctness bug.
Generated implementation paths are not stable merely because they compile.

Initial-release readiness requires all retained identities to reconcile, every
canonical path to resolve to one authoritative declaration, all public argument
rows and Safety contracts to pass audit, provider and package graphs to be
coherent, generated artifacts to reproduce deterministically, package contents
to exclude unresolved source and binary redistribution, and the documented
portable and GNU MinGW validation matrices to pass. The release process does
not imply that all raw routines have safe wrappers or runtime tests.
