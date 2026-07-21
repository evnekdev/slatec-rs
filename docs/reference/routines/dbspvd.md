# DBSPVD

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value and all derivatives of order less than NDERIV of all basis functions which do not vanish at X.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPVD is the BSPLVD routine of the reference. DBSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. ILEFT is input such that T(ILEFT) .LE. X .LT. T(ILEFT+1). A call to INTRV(T,N+1,X, ILO,ILEFT,MFLAG) will produce the proper ILEFT. The output of DBSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. These basis functions have indices ILEFT-K+I, I=1,K, K .LE. ILEFT .LE. N. The nonzero part of the I-th basis function lies in (T(I),T(I+K)), I=1,N). If X=T(ILEFT+1) then VNIKX contains left limiting values (left derivatives) at T(ILEFT+1). In particular, ILEFT = N produces left limiting values at the right end point X=T(N+1). To obtain left limiting values at T(I), I=K+1,N+1, set X= next lower distinct knot, call INTRV to get ILEFT, set X=T(I), and then call DBSPVD.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbspvd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbspvd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbspvd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbspvd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DBSPVD](https://www.netlib.org/slatec/src/dbspvd.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `T` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K |
| 2 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order of the B-spline, K .GE. 1 |
| 3 | `NDERIV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of derivatives = NDERIV-1, 1 .LE. NDERIV .LE. K |
| 4 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | argument of basis functions, T(K) .LE. X .LE. T(N+1) |
| 5 | `ILEFT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | largest integer such that T(ILEFT) .LE. X .LT.  T(ILEFT+1) |
| 6 | `LDVNIK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | leading dimension of matrix VNIKX Output     VNIKX,WORK are double precision |
| 7 | `VNIKX` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDVNIK, *) | matrix of dimension at least (K,NDERIV) contain- ing the nonzero basis functions at X and their derivatives columnwise. |
| 8 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | a work vector of length (K+1)*(K+2)/2 |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: a work vector of length (K+1)*(K+2)/2

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dbspvd`. Native symbol: `dbspvd_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbspvd`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
