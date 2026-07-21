# BINTK

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the B-representation of a spline which interpolates given data.

## Description

Written by Carl de Boor and modified by D. E. Amos BINTK is the SPLINT routine of the reference. BINTK produces the B-spline coefficients, BCOEF, of the B-spline of order K with knots T(I), I=1,...,N+K, which

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E1A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::interpolation::bspline::BSpline::interpolate_with_knots`

## Providers

- Canonical provider: `main-src/src/bintk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bintk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bintk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bintk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [BINTK](https://www.netlib.org/slatec/src/bintk.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1,...,N.  The spline or any of its derivatives can be evaluated by calls to BVALU. The I-th equation of the linear system A*BCOEF = B for the coefficients of the interpolant enforces interpolation at 1,...,N.  Hence, B(I) = Y(I), all I, and A is a band matrix with 2K-1 bands if A is invertible. The matrix A is generated row by row and stored, diagonal by diagonal, in the rows of Q, with the main diagonal going into row K. The banded system is then solved by a call to BNFAC (which constructs the triangular factorization for A and stores it again in Q), followed by a call to BNSLV (which then obtains the solution BCOEF by substitution). BNFAC does no pivoting, since the total positivity of the matrix A makes this unnecessary.  The linear system to be solved is (theoretically) invertible if and only if vector of length N containing data point abscissa in strictly increasing order. K knots (not nec- essarily X(I)) values) interior to (X(1),X(N)) 1,...,N with the same abscissa can be obtained by loading YY into BCOEF and then executing |
| 2 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1,...,N.  The spline or any of its derivatives can be evaluated by calls to BVALU. The I-th equation of the linear system A*BCOEF = B for the coefficients of the interpolant enforces interpolation at corresponding vector of length N containing data point ordinates. |
| 3 | `T` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | .LT. X(I)) .LT. T(I+K),        all I. Equality is permitted on the left for I=1 and on the right knot vector of length N+K since T(1),..,T(K) .LE. X(1) and T(N+1),..,T(N+K) |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are used at X(1) or X(N).  Otherwise, violation of this condition is certain to lead to an error. K knots (not nec- K knots (not nec- essarily X(I)) values) interior to (X(1),X(N)) essarily X(I)) values) interior to (X(1),X(N)) number of data points, N .GE. K |
| 5 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are used at X(1) or X(N).  Otherwise, violation of this condition is certain to lead to an error. order of the spline, K .GE. 1 |
| 6 | `BCOEF` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a vector of length N containing the B-spline coefficients |
| 7 | `Q` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a work vector of length (2*K-1)*N, containing the triangular factorization of the coefficient matrix of the linear system being solved.  The coefficients for the interpolant of an 1,N,K-1,K-1,BCOEF) |
| 8 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | work vector of length 2*K |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

Improper  input is a fatal error Singular system of equations is a fatal error

### Storage and workspace requirements

`WORK`: work vector of length 2*K

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::bintk`. Native symbol: `bintk_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::bintk`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::interpolation::bspline::BSpline::interpolate_with_knots`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
