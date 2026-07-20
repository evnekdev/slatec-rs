# CORTB

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

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

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Eigenvalue problems](../families/eigenvalue-problems.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `NM` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LOW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LOW and IGH are two INTEGER variables determined by the balancing subroutine CBAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IGH` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LOW and IGH are two INTEGER variables determined by the balancing subroutine CBAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AR` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, *) | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AI` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, *) | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ORTR` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ORTR and ORTI contain further information about the unitary transformations used in the reduction by CORTH. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ORTI` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ORTR and ORTI contain further information about the unitary transformations used in the reduction by CORTH. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | M is the number of columns of Z=(ZR,ZI) to be back transformed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ZR` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, *) | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ZI` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, *) | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::cortb`. Native symbol: `cortb_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cortb`
- Compatibility aliases: `slatec_sys::eigen::numerical::cortb`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
