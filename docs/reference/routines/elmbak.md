# ELMBAK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a real general matrix from the eigenvectors of the upper Hessenberg matrix output from ELMHES.

## Description

This subroutine is a translation of the ALGOL procedure ELMBAK, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). This subroutine forms the eigenvectors of a REAL GENERAL matrix by back transforming those of the corresponding upper Hessenberg matrix determined by ELMHES. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. LOW and IGH are two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix. A contains the multipliers which were used in the reduction by ELMHES in its lower triangle below the subdiagonal. A is a two-dimensional REAL array, dimensioned A(NM,IGH). INT contains information on the rows and columns interchanged in the reduction by ELMHES. Only elements LOW through IGH are used. INT is a one-dimensional INTEGER array, dimensioned INT(IGH). M is the number of columns of Z to be back transformed. M is an INTEGER variable. Z contains the real and imaginary parts of the eigenvectors to be back transformed in its first M columns. Z is a two-dimensional REAL array, dimensioned Z(NM,M). On OUTPUT Z contains the real and imaginary parts of the transformed eigenvectors in its first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C4`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/elmbak.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/elmbak.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
