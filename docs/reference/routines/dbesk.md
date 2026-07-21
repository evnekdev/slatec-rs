# DBESK

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.

## Description

Abstract **** a double precision routine **** DBESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,..,N for real X .GT. 0.0D0 and non-negative orders FNU. If FNU .LT. NULIM, orders FNU and FNU+1 are obtained from DBSKNU to start the recursion. If FNU .GE. NULIM, the uniform asymptotic expansion is used for orders FNU and FNU+1 to start the recursion. NULIM is 35 or 70 depending on whether N=1 or N .GE. 2. Under and overflow tests are made on the leading term of the asymptotic expansion before any extensive computation is done. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B3`
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

- Canonical provider: `main-src/src/dbesk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbesk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbesk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbesk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DBESK](https://www.netlib.org/slatec/src/dbesk.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | X .GT. 0.0D0 1)/(X), I=1,...,N 1)/(X), I=1,...,N depending on KODE |
| 2 | `FNU` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | order of the initial K function, FNU .GE. 0.0D0 1)/(X), I=1,...,N 1)/(X), I=1,...,N 1)/(X), I=1,...,N  or 1)/(X), I=1,...,N depending on KODE |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a parameter to indicate the scaling option 1)/(X), I=1,...,N 1)/(X), I=1,...,N 1, |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of members in the sequence, N .GE. 1 Output     Y is double precision |
| 5 | `Y` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 1)/(X), I=1,...,N 1)/(X), I=1,...,N a vector whose first N components contain values for the sequence 1)/(X), I=1,...,N  or 1)/(X), I=1,...,N depending on KODE 0.0D0, I=1,...,NZ |
| 6 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of components of Y set to zero due to 0   , normal return, computation completed NZ .NE. 0, first NZ components of Y set to zero |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::bessel::dbesk`. Native symbol: `dbesk_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::bessel::dbesk`
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
