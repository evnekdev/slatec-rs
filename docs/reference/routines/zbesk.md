# ZBESK

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute a sequence of the Bessel functions K(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.

## Description

A DOUBLE PRECISION ROUTINE*** On KODE=1, ZBESK computes an N member sequence of complex Bessel functions CY(L)=K(FNU+L-1,Z) for real nonnegative orders FNU+L-1, L=1,...,N and complex Z.NE.0 in the cut plane -pi<arg(Z)<=pi where Z=ZR+i*ZI. On KODE=2, CBESJ returns the scaled functions CY(L) = exp(Z)*K(FNU+L-1,Z), L=1,...,N which remove the exponential growth in both the left and right half planes as Z goes to infinity. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f64`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B4`
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

- Canonical provider: `main-src/src/zbesk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/zbesk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/zbesk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/zbesk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [ZBESK](https://www.netlib.org/slatec/src/zbesk.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `ZR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION real part of nonzero argument Z. |
| 2 | `ZI` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION imag part of nonzero argument Z. |
| 3 | `FNU` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION initial order, FNU>=0. |
| 4 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | A parameter to indicate the scaling option 1 returns CY(L)=K(FNU+L-1,Z), L=1,. ,N =2 returns CY(L)=K(FNU+L-1,Z)*EXP(Z), L=1,. ,N. |
| 5 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of terms in the sequence, N>=1. |
| 6 | `CYR` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | DOUBLE PRECISION real part of result vector. |
| 7 | `CYI` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | DOUBLE PRECISION imag part of result vector. |
| 8 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of underflows set to zero NZ=0 Normal return NZ>0 CY(L)=0 for NZ values of L (if Re(Z)>0 then CY(L)=0 for L=1,. ,NZ; in the complementary half plane the underflows may not be in an uninterrupted sequence). |
| 9 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (abs(Z) too small and/or FNU+N-1 too large) 3 Precision warning - COMPUTATION COMPLETED (Result has half precision or less because abs(Z) or FNU+N-1 is large) 4 Precision error - NO COMPUTATION (Result has no precision because abs(Z) or FNU+N-1 is too large) 5 Algorithmic error - NO COMPUTATION (Termination condition not met). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | Normal return for NZ values of L (if Re(Z)>0 for L=1,...,NZ; in the complementary half plane the underflows may not be in an uninterrupted sequence) |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::bessel::zbesk`. Native symbol: `zbesk_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::bessel::zbesk`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
