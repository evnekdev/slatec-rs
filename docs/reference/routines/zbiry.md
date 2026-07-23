# ZBIRY

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the Airy function Bi(z) or its derivative dBi/dz for complex argument z. A scaling option is available to help avoid overflow.

## Description

A DOUBLE PRECISION ROUTINE*** On KODE=1, ZBIRY computes the complex Airy function Bi(z) or its derivative dBi/dz on ID=0 or ID=1 respectively. On KODE=2, a scaling option exp(abs(Re(zeta)))*Bi(z) or exp(abs(Re(zeta)))*dBi/dz is provided to remove the exponential behavior in both the left and right half planes where zeta=(2/3)*z**(3/2). The Airy functions Bi(z) and dBi/dz are analytic in the whole z-plane, and the scaling option does not destroy this property.

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
- GAMS classifications: `C10D`
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

- Canonical provider: `main-src/src/zbiry.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/zbiry.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/zbiry.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/zbiry.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [ZBIRY](https://www.netlib.org/slatec/src/zbiry.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `ZR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION real part of argument Z. |
| 2 | `ZI` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION imag part of argument Z. |
| 3 | `ID` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Order of derivative, ID=0 or ID=1. |
| 4 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | A parameter to indicate the scaling option 1 returns BI=Bi(z) on ID=0 BI=dBi/dz on ID=1 at z=Z =2 returns BI=exp(abs(Re(zeta)))*Bi(z) on ID=0 BI=exp(abs(Re(zeta)))*dBi/dz on ID=1 at z=Z where zeta=(2/3)*z**(3/2). |
| 5 | `BIR` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION real part of result. |
| 6 | `BII` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION imag part of result. |
| 7 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (Re(Z) too large with KODE=1) 3 Precision warning - COMPUTATION COMPLETED (Result has less than half precision) 4 Precision error - NO COMPUTATION (Result has no precision) 5 Algorithmic error - NO COMPUTATION (Termination condition not met). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::zbiry`. Native symbol: `zbiry_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::zbiry`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
