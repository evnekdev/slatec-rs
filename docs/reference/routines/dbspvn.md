# DBSPVN

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of all (possibly) nonzero basis functions at X.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPVN is the BSPLVN routine of the reference. DBSPVN calculates the value of all (possibly) nonzero basis

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

- Canonical provider: `main-src/src/dbspvn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbspvn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbspvn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbspvn.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DBSPVN](https://www.netlib.org/slatec/src/dbspvn.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `T` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | IWORK is set inside the routine on are double precision knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K .LE. X .LE. T(N+1) .LE. X .LT.  T(ILEFT+1) |
| 2 | `JHIGH` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1)), where T(K) K and INDEX=1 can be faster than calling DBSPVD, but extra coding is required for derivatives order of B-spline, 1 .LE. JHIGH .LE. K |
| 3 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | highest possible order .LE. X .LE. T(N+1) |
| 4 | `INDEX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1)), where T(K) 1.  ILEFT is such that T(ILEFT) .LE. 2) and DBSPVD is set up for this purpose. Left limiting values are set up as described in DBSPVD. INDEX = 1 gives basis functions of order JHIGH = 2 denotes previous entry with work, IWORK values saved for subsequent calls to DBSPVN. 2. 1 exclusively, these are scratch variables and can be used for other purposes. |
| 5 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | 1)), where T(K) IWORK is set inside the routine on .LT. T(ILEFT+1).  A call to DINTRV(T,N+1,X,ILO,ILEFT,MFLAG) produces the proper ILEFT.  DBSPVN calculates using the basic algorithm needed in DBSPVD.  If only basis functions are are double precision argument of basis functions, |
| 6 | `ILEFT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | largest integer such that .LE. X .LT.  T(ILEFT+1) |
| 7 | `VNIKX` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are double precision vector of length K for spline values. |
| 8 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are double precision a work vector of length 2*K |
| 9 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a work parameter.  Both WORK and IWORK contain |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

Improper input is a fatal error.

### Storage and workspace requirements

`WORK`: are double precision a work vector of length 2*K

`IWORK`: a work parameter.  Both WORK and IWORK contain

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dbspvn`. Native symbol: `dbspvn_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbspvn`
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
