# DPFQAD

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the integral on (X1,X2) of a product of a function F and the ID-th derivative of a B-spline, (PP-representation).

## Description

Abstract **** a double precision routine **** DPFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a B-spline, using the PP-representation (C,XI,LXI,K). (X1,X2) is normally a sub- interval of XI(1) .LE. X .LE. XI(LXI+1). An integration routine, DPPGQ8 (a modification of GAUS8), integrates the product on subintervals of (X1,X2) formed by the included break points. Integration outside of (XI(1),XI(LXI+1)) is permitted provided F is defined. The maximum number of significant digits obtainable in DBSQAD is the smaller of 18 and the number of digits carried in double precision arithmetic.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dpfqad.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpfqad.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpfqad.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dpfqad.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [DPFQAD](https://www.netlib.org/slatec/src/dpfqad.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Required synchronous double-precision scalar integrand callback `F(X)`. It receives only a readable scalar pointer, returns directly, has no user-data pointer, must not retain the pointer, and must not unwind through Fortran. |
| 2 | `LDC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input leading dimension of the Fortran column-major coefficient matrix `C`; it must be at least `K`. |
| 3 | `C` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDC, *) | Readable column-major matrix of right Taylor derivatives, with at least `LDC * LXI` double-precision elements. |
| 4 | `XI` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Readable breakpoint vector with at least `LXI + 1` elements; it defines the piecewise-polynomial intervals. |
| 5 | `LXI` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input number of polynomial pieces; it must be positive. |
| 6 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input B-spline order; it must be positive. |
| 7 | `ID` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input derivative order, constrained to `0..K-1`; zero selects the spline itself. |
| 8 | `X1` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input first integration endpoint. The routine reverses the sign when `X1 > X2`. |
| 9 | `X2` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input second integration endpoint. |
| 10 | `TOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input requested quadrature tolerance. The selected source requires it between the double-precision machine floor and `0.1`. |
| 11 | `QUAD` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Writable output integral of `F(X)` times the selected B-spline derivative. |
| 12 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable status: `1` means normal completion and `2` means some subinterval did not meet `TOL`. Invalid arguments enter the legacy error path. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

Improper input is a fatal error. Some quadrature does not meet the requested tolerance.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dpfqad`. Native symbol: `dpfqad_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature-callbacks`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dpfqad`
- Public declaration feature: `quadrature-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
