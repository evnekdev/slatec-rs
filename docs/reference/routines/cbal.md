# CBAL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Balance a complex general matrix and isolate eigenvalues whenever possible.

## Description

This subroutine is a translation of the ALGOL procedure CBALANCE, which is a complex version of BALANCE, NUM. MATH. 13, 293-304(1969) by Parlett and Reinsch. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 315-326(1971). This subroutine balances a COMPLEX matrix and isolates eigenvalues whenever possible. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR and AI, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM. AR and AI contain the real and imaginary parts, respectively, of the complex matrix to be balanced. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). On OUTPUT AR and AI contain the real and imaginary parts, respectively, of the balanced matrix. LOW and IGH are two INTEGER variables such that AR(I,J) and AI(I,J) are equal to zero if (1) I is greater than J and (2) J=1,...,LOW-1 or I=IGH+1,...,N. SCALE contains information determining the permutations and scaling factors used. SCALE is a one-dimensional REAL array, dimensioned SCALE(N). Suppose that the principal submatrix in rows LOW through IGH has been balanced, that P(J) denotes the index interchanged with J during the permutation step, and that the elements of the diagonal matrix used are denoted by D(I,J). Then SCALE(J) = P(J), for J = 1,...,LOW-1 = D(J,J) J = LOW,...,IGH = P(J) J = IGH+1,...,N. The order in which the interchanges are made is N to IGH+1, then 1 to LOW-1. Note that 1 is returned for IGH if IGH is zero formally. The ALGOL procedure EXC contained in CBALANCE appears in CBAL in line. (Note that the ALGOL roles of identifiers K,L have been reversed.) Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C1A`
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

- Canonical provider: `lin/cbal.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cbal.f) — `verified_cached`
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
