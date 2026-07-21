# BSQAD

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the integral of a K-th order B-spline using the B-representation.

## Description

BSQAD computes the integral on (X1,X2) of a K-th order B-spline using the B-representation (T,BCOEF,N,K). Orders K as high as 20 are permitted by applying a 2, 6, or 10 point Gauss formula on subintervals of (X1,X2) which are formed by included (distinct) knots. If orders K greater than 20 are needed, use BFQAD with F(X) = 1.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::interpolation::bspline::BSpline::integrate`

## Providers

- Canonical provider: `main-src/src/bsqad.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bsqad.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bsqad.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bsqad.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [BSQAD](https://www.netlib.org/slatec/src/bsqad.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `T` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | knot array of length N+K |
| 2 | `BCOEF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | B-spline coefficient array of length N |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | length of coefficient array |
| 4 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order of B-spline, 1 .LE. K .LE. 20 |
| 5 | `X1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | end points of quadrature interval in T(K) .LE. X .LE. T(N+1) |
| 6 | `X2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | end points of quadrature interval in T(K) .LE. X .LE. T(N+1) |
| 7 | `BQUAD` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | integral of the B-spline over (X1,X2) |
| 8 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | work vector of length 3*K |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: work vector of length 3*K

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::bsqad`. Native symbol: `bsqad_`. Declaration feature: `quadrature`. Provider feature: `quadrature-direct`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::bsqad`
- Public declaration feature: `quadrature`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::interpolation::bspline::BSpline::integrate`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
