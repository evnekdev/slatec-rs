# Family features and native providers

The safe API and its implementation provider are selected independently. A
narrow family feature compiles only its safe Rust module and activates the
reviewed raw declarations and native dependency closure for that family.

## Public families

Broad aliases expand to the narrow features below; there is no feature per
routine.

| Domain alias | Narrow features |
| --- | --- |
| `blas` | `blas-level1`, `blas-level2`, `blas-level3` |
| `special` | `special-elementary`, `special-gamma`, `special-beta`, `special-error`, `special-airy`, `special-bessel`, `special-integrals`, `special-polynomials`, `special-scalar-expanded`, `batch-a-special` |
| `quadrature` | `quadrature-basic`, `quadrature-breakpoints`, `quadrature-weighted`, `quadrature-oscillatory`, `quadrature-fourier`, `quadrature-nonadaptive`, `batch-b-quadrature` |
| `roots` | `roots-scalar`, `roots-polynomial` |
| `nonlinear` | `nonlinear-easy`, `nonlinear-expert`, `nonlinear-jacobian-check` |
| `ode` | `ode-sdrive-expert`, `batch-b-ode` |
| DAE | `dassl` |
| Cartesian PDE | `fishpack-cartesian-2d` |
| Structured 3D FISHPACK system | `fishpack-pois3d` |
| `optimization` | `optimization-linear-programming-in-memory` |
| `least-squares` | `least-squares-nonlinear-easy`, `least-squares-nonlinear-expert`, `least-squares-covariance`, `least-squares-linear-nonnegative`, `least-squares-linear-bounded`, `least-squares-linear-bounded-constrained` |
| Batch A raw domains | `batch-a-linear-algebra`, `batch-a-eigen`, `batch-a-approximation`, `batch-a-statistics`, plus the Batch A members of `special`, `quadrature`, `nonlinear`, `ode`, `fftpack`, `fishpack`, and `interpolation` |
| Batch B raw domains | `batch-b-quadrature`, `batch-b-linear-algebra`, `batch-b-ode` |

`roots-polynomial` remains an explicit deferred empty family: no polynomial
root wrapper is exposed until its interface validation gate is cleared.
The optimization alias is deliberately limited to resident-memory
`SPLP`/`DSPLP`; external paging and Fortran-unit lifecycle management remain
deferred.


## Capability layers

- Core-only APIs use `core` and caller-owned slices. They compile with
  `--no-default-features` plus the relevant family and an explicit backend.
- `alloc` enables Rust's standalone `alloc` crate. It does not enable or
  require `std`; a `no_std` consumer supplies an allocator.
- `std` enables `alloc` and is required by current callback containment,
  process-global native serialization, and FNLIB error-state handling.

The Rust API can therefore be `no_std` compatible while the selected native
backend remains hosted. No bare-metal execution has been validated.

## Concurrency and storage policy

Hosted safe wrappers that enter legacy SLATEC runtime state are serialized by a
process-wide runtime guard. Existing `no_std`/`alloc` BLAS and Jacobian-check
features intentionally do not acquire that hosted guard; they are classified
as `BackendDependent`, not thread-safe. The classification is per safe
function in [`generated/safe-api/concurrency-index.json`](../../generated/safe-api/concurrency-index.json).

Every native argument has an explicit storage record. The core API does not
silently transpose, repack, densify, or materialize strided matrix data. Any
future ecosystem adapter must be optional and verify the exact native layout,
mutability, and leading-dimension rules. See the
[runtime concurrency and storage policy](../architecture/runtime-concurrency-and-storage-policy.md).

## Backends and offline policy

Exactly one backend must be selected whenever a numerical family is enabled.

- `prebuilt` means a target-specific packaged archive requiring no network or
  compiler. It currently fails with a rights-blocked diagnostic: no historical
  SLATEC archive is published until source and binary redistribution are
  cleared.
- `source-build` compiles from `SLATEC_SOURCE_CACHE` and never accesses the
  network. Populate that cache explicitly with
  `slatec-corpus acquire-provider-sources`; the build fails with the exact
  command when the cache is absent or invalid.
- `system` links an explicitly located validated static archive using
  `SLATEC_SYSTEM_LIB_DIR` and optional `SLATEC_SYSTEM_LIB_NAME`. The supported
  GNU profile also requires `SLATEC_SYSTEM_RUNTIME_LIB_DIR` containing static
  `libgfortran.a` and `libquadmath.a`.
- `external-backend` emits no native build or link instruction. It is intended
  for applications whose final link is managed outside Cargo.
- `bundled` is a deprecated compatibility alias with exactly the offline
  `source-build` meaning. It never downloads source.

Incompatible combinations fail in `slatec-src/build.rs` with a direct error.
`slatec-src` itself is a `#![no_std]` provider-selection crate and exposes no
numerical API. Its build script contains no network client.

The source builder currently supports the validated GNU Fortran 14.2.0
`x86_64-w64-mingw32` profile. Other compiler versions are not claimed as
validated merely because their target string matches. Other targets must select
`system` or `external-backend`. No native backend is selected by default while
prebuilt publication is blocked.

Explicit acquisition from a repository checkout:

```text
cargo run -p slatec-tools --bin slatec-corpus -- acquire-provider-sources --output-dir evidence/provider-sources
set SLATEC_SOURCE_CACHE=evidence/provider-sources
cargo build --offline --no-default-features --features std,source-build,special-gamma --target x86_64-pc-windows-gnu
```

The source build statically links `libgfortran` and `libquadmath`. The tested
consumer has no GNU runtime DLL imports, but static `libquadmath` introduces
LGPL redistribution obligations and `libgfortran` remains governed by its exact
GPL/GCC Runtime Library Exception notices. See `generated/licensing/`.

## Static retention

Every physical selected source is compiled to a separate object and archived
without whole-archive linking. The platform linker therefore extracts only
objects referenced by the program and their transitive support closure. The
compiler-observed closure and eleven native binary checks are recorded under
`generated/linkage/`. Narrow examples check their own family roots and reject
unrelated public family roots while permitting required support symbols.

The source-backend native-link audit supersedes a feature-name-only claim: it
records every archive member, its source/program units/symbol references, and
the symbols and selected members in raw and safe release probes. In particular,
`slatec-sys/all` is declaration-only; pairing it with `slatec`'s `full` source
provider still must not retain SLATEC implementation code when no routine is
called. See [native link granularity](../architecture/native-link-granularity.md).

## Examples

```toml
[dependencies]
slatec = { version = "0.1", default-features = false, features = ["std", "source-build", "special-gamma"] }
```

For a `no_std` consumer that supplies the final native link:

```toml
[dependencies]
slatec = { version = "0.1", default-features = false, features = ["external-backend", "blas-level1"] }
```

## Raw API feature reconciliation

`slatec-sys` is usable directly with its reviewed raw-family feature and a
matching `slatec-src` provider feature, or with an externally managed native
link. The feature name names the Rust declaration surface; the provider
feature selects the source closure. Neither fact alone implies that a
generated declaration has been semantically reviewed.

R1 records all three layers—`slatec-sys`, `slatec-src`, and the optional safe
`slatec` facade—in `generated/raw-api/feature-provider-map.json`. Generation
rejects a reviewed raw routine that lacks its `slatec-sys` gate or a usable
`slatec-src` provider feature. It also makes the direct/raw relationship
explicit: use `raw-family-roots-scalar` with `roots-scalar`,
`raw-family-fishpack-cartesian-2d` with `fishpack-cartesian-2d`, and
`raw-family-fishpack-pois3d` with `fishpack-pois3d`.

The aggregate `raw-ffi-*` features enable only the unstable generated layer.
They are not a promise of canonical family paths or provider closure. See
[the raw API architecture](../architecture/slatec-sys-public-raw-api.md) for
the reviewed-path and compatibility policy.

### Batch A raw provider features

Batch A adds coherent `batch-a-*` declaration features in `slatec-sys` and
matching source-closure features in `slatec-src`. They cover the generated
canonical paths for straightforward historical numerical drivers; there is no
feature per routine. They may be selected directly for a raw-only application,
or through the relevant broad mathematical feature. The source closure is
exactly derived from the promoted public symbols and their observed native
dependencies. As with every raw declaration feature, `slatec-sys` alone
selects no provider. See [`raw-batch-a.md`](raw-batch-a.md).

### Batch B callback raw provider features

Batch B adds coherent callback-bearing declaration features in `slatec-sys`:
`batch-b-quadrature`, `batch-b-linear-algebra`, and `batch-b-ode`. The matching
verified source-provider features are the existing family closures in
`slatec-src`: `quadrature`, `linear-algebra`, and `ode`. Broad mathematical
aliases include the relevant Batch B declaration features, while
`slatec-sys/all` remains declaration-only and provider-neutral.

These interfaces are raw callback ABIs, not safe closure wrappers. They
stabilize canonical paths such as
`slatec_sys::quadrature::callbacks::dqk15`,
`slatec_sys::linear_algebra::sparse::callbacks::scg`, and
`slatec_sys::ode::callbacks::derkf` within the generated evidence boundary.
See [`raw-batch-b-callbacks.md`](raw-batch-b-callbacks.md).

### Reviewed BLAS raw features

R2A adds matching public raw aliases in `slatec-sys`: `blas` expands to
`blas-level1`, `blas-level2`, and `blas-level3`, each of which selects the
matching `raw-family-blas-level*` declaration gates. The same level names in
`slatec-src` select generated, symbol-closed source sets for every promoted
raw BLAS routine, not only the routines used by the safe facade. `slatec`
uses the same feature names and retains its safe wrapper subset; it also hosts
the direct raw link and runtime smoke tests when a native provider is selected.

Direct consumers can instead depend on `slatec-sys` and provide the selected
profile's native symbols themselves. The reconciliation report records this
distinction per routine; source-provider selection is never an implicit side
effect of enabling `slatec-sys`.

## Safe-facade operation granularity

Family features are not per-routine link switches. They expose coherent safe
surfaces and source closures, while a real operation call selects only its
checked wrapper and genuine native closure on the supported source-build
profile. `full` remains clean when no operation is called. The deterministic
safe/raw comparison reports, including link maps and Rust-symbol categories,
are generated under `generated/native-link/`; see the
[safe-facade policy](../architecture/safe-facade-link-granularity.md).

### Reviewed scalar-special raw features and `all`

R2B promotes the scalar elementary, gamma, beta, and error foundations under
`slatec_sys::special::{elementary,gamma,beta,error}`. The public
`special-elementary`, `special-gamma`, `special-beta`, and `special-error`
aliases select their corresponding reviewed declarations; `special` includes
all four. The matching `slatec-src` feature selects the source closure, while
the safe `slatec` feature continues to select only its existing safe facade.
The former `families::special_*` raw paths remain compatibility re-exports.

`slatec-sys/all` is the declaration-only aggregate of every public
mathematical family. It intentionally selects no provider or backend and does
not promise stability for any unreviewed generated declaration. The generated
[`all-feature-coverage.json`](../../generated/raw-api/all-feature-coverage.json)
proves direct coverage of the maintained family registry and reports its
transitive declaration closure.

### Reviewed real Airy raw feature

`special-airy` selects the reviewed real FNLIB Airy declarations at
`slatec_sys::special::airy` and the matching `slatec-src/special-airy` source
closure. It is declaration-only in `slatec-sys`; a direct caller selects a
provider explicitly. The safe `slatec/special-airy` feature keeps the existing
`slatec::special::airy` API and uses those canonical raw re-exports. Complex
Amos Airy interfaces and Airy subsidiaries do not become public stable paths
through this feature.
