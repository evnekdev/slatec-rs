# DGAUS8

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Integrate a real function of one variable over a finite interval using an adaptive 8-point Legendre-Gauss algorithm. Intended primarily for high accuracy integration or integration of smooth functions.

## Description

Abstract *** a DOUBLE PRECISION routine *** DGAUS8 integrates real functions of one variable over finite intervals using an adaptive 8-point Legendre-Gauss algorithm. DGAUS8 is intended primarily for high accuracy integration or integration of smooth functions. The maximum number of significant digits obtainable in ANS is the smaller of 18 and the number of digits carried in double precision arithmetic.

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
- GAMS classifications: `H2A1A1`
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

- Canonical provider: `main-src/src/dgaus8.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dgaus8.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dgaus8.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dgaus8.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DGAUS8](https://www.netlib.org/slatec/src/dgaus8.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FUN` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are DOUBLE PRECISION * name of external function to be integrated.  This name must be in an EXTERNAL statement in the calling program. must be a DOUBLE PRECISION function of one DOUBLE PRECISION argument.  The value of the argument to FUN is the variable of integration which ranges from A to B. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are DOUBLE PRECISION * lower limit of integration negative value for ERR causes an estimate of the absolute error in ANS to be returned in ERR.  Note that B. are too nearly equal to allow normal integration.  ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are DOUBLE PRECISION * upper limit of integration (may be less than A) are too nearly equal to allow normal integration.  ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance. |
| 4 | `ERR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are DOUBLE PRECISION * is a requested pseudorelative error tolerance.  Normally pick a value of ABS(ERR) so that DTOL .LT. ABS(ERR) .LE. 1.0D-3 where DTOL is the larger of 1.0D-18 and the double precision unit roundoff D1MACH(4).  ANS will normally have no more error than ABS(ERR) times the integral of the absolute value of FUN(X).  Usually, smaller values of ERR yield more accuracy and require more function evaluations. must be a variable (not a constant) in this case. Note also that the user must reset the value of ERR before making any more calls that use the variable ERR. are double precision * will be an estimate of the absolute error in ANS if the is unchanged if is unchanged if negative.)  The estimated |
| 5 | `ANS` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are double precision * computed value of integral |
| 6 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a status code --Normal codes 1 ANS most likely meets requested error tolerance, |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

not be used as a correction to the computed integral.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dgaus8`. Native symbol: `dgaus8_`. Declaration feature: `quadrature`. Provider feature: `quadrature-direct`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::dgaus8`
- Public declaration feature: `quadrature`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
