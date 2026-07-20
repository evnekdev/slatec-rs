# CORTB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a complex general matrix from eigenvectors of upper Hessenberg matrix output from CORTH.

## Description

This subroutine is a translation of a complex analogue of the ALGOL procedure ORTBAK, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). This subroutine forms the eigenvectors of a COMPLEX GENERAL matrix by back transforming those of the corresponding upper Hessenberg matrix determined by CORTH. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable. LOW and IGH are two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix. AR and AI contain information about the unitary transformations used in the reduction by CORTH in their strict lower triangles. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH). ORTR and ORTI contain further information about the unitary transformations used in the reduction by CORTH. Only elements LOW through IGH are used. ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and ORTI(IGH). M is the number of columns of Z=(ZR,ZI) to be back transformed. M is an INTEGER variable. ZR and ZI contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first M columns. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). On OUTPUT ZR and ZI contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. ORTR and ORTI have been altered. Note that CORTB preserves vector Euclidean norms. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/cortb.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cortb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::cortb`
- Current legacy Rust paths: `none`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
