# Reviewed raw scalar special foundations

R2B promotes 40 ABI-simple, historically user-callable scalar functions to
stable canonical `slatec-sys` paths. They are unsafe direct bindings to the
selected SLATEC sources, not safe numerical wrappers.

| Group | Canonical module | Public feature |
| --- | --- | --- |
| Elementary | `slatec_sys::special::elementary` | `special-elementary` |
| Gamma | `slatec_sys::special::gamma` | `special-gamma` |
| Beta | `slatec_sys::special::beta` | `special-beta` |
| Error | `slatec_sys::special::error` | `special-error` |

`special` enables all four reviewed groups and the existing unpromoted special
family gates. The complete, source-hash guarded candidate disposition is
[`special-foundations-report.json`](../../generated/raw-api/special-foundations-report.json).

## Direct calls and provider selection

`slatec-sys` is declaration-only. Select a compatible native provider through
`slatec-src` or make the final link yourself; enabling `special-*` or `all`
does not download, build, or select Fortran.

```rust
let mut x = 1.0_f64;
let value = unsafe { slatec_sys::special::error::derf(&mut x) };
```

The functions use the observed GNU MinGW profile. Each Fortran scalar input is
passed by address, must be non-null, aligned, and readable for the entire
call; these scalar functions do not retain the pointer. The reviewed ABI
returns `f32` or `f64` directly. Routine Rustdoc gives the exact argument
contract, source link, native symbol, domain, and error behavior.

Callers must observe each routine's documented mathematical domain. In
particular, the incomplete beta functions require `0 <= X <= 1` and positive
shape parameters, beta functions require positive parameters, and gamma/log
gamma functions must avoid poles and source-specific range failures. These
legacy sources use XERROR/XERMSG for documented domain, range, and precision
conditions; a raw call does not translate those reports into a Rust `Result`.

FNLIB initialization, saved coefficients, and legacy error controls are
process-global. `GAMR` and `DGAMR` additionally change XERROR controls while
evaluating their result. This raw interface does not serialize calls, so a
consumer that may overlap calls must provide process-wide synchronization.

## The `all` declaration aggregate

`slatec-sys/all` directly enables every authored public mathematical family
aggregate. It is intentionally declaration-only: it excludes provider,
backend, profile-only, ABI-shaped generated, and test-only switches. Its
deterministic coverage check is
[`all-feature-coverage.json`](../../generated/raw-api/all-feature-coverage.json);
the current report has no missing public family.

`all` is useful for a consumer that supplies a complete external native
provider. It does not promote private ABI-shaped declaration paths, and it does
not make unreviewed routines stable. The canonical-path compile test imports a
representative reviewed path from every public family.
