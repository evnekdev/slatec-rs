# HTRIBK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a complex Hermitian matrix from the eigenvectors of a real symmetric tridiagonal matrix output from HTRIDI.

## Description

This subroutine is a translation of a complex analogue of the ALGOL procedure TRBAK1, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine forms the eigenvectors of a COMPLEX HERMITIAN matrix by back transforming those of the corresponding real symmetric tridiagonal matrix determined by HTRIDI. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix. N is an INTEGER variable. N must be less than or equal to NM. AR and AI contain some information about the unitary transformations used in the reduction by HTRIDI in the strict lower triangle of AR and the full lower triangle of AI. The remaining upper parts of the matrices are arbitrary. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). TAU contains further information about the transformations. TAU is a one-dimensional REAL array, dimensioned TAU(2,N). M is the number of eigenvectors to be back transformed. M is an INTEGER variable. ZR contains the eigenvectors to be back transformed in its first M columns. The contents of ZI are immaterial. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). On OUTPUT ZR and ZI contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. Note that the last component of each returned vector is real and that vector Euclidean norms are preserved. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/htribk.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/htribk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::htribk`
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
