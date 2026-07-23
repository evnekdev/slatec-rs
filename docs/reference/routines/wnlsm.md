# WNLSM

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to WNNLS

## Description

This is a companion subprogram to WNNLS. The documentation for WNNLS has complete usage instructions. In addition to the parameters discussed in the prologue to subroutine WNNLS, the following work arrays are used in subroutine WNLSM (they are passed through the calling sequence from WNNLS for purposes of variable dimensioning). Their contents will in general be of no interest to the user. IPIVOT(*) An array of length N. Upon completion it contains the pivoting information for the cols of W(*,*). ITYPE(*) An array of length M which is used to keep track of the classification of the equations. ITYPE(I)=0 denotes equation I as an equality constraint. ITYPE(I)=1 denotes equation I as a least squares equation. WD(*) An array of length N. Upon completion it contains the dual solution vector. H(*) An array of length N. Upon completion it contains the pivot scalars of the Householder transformations performed in the case KRANK.LT.L. SCALE(*) An array of length M which is used by the subroutine to store the diagonal matrix of weights. These are used to apply the modified Givens transformations. Z(*),TEMP(*) Working arrays of length N. D(*) An array of length N that contains the column scaling for the matrix (E). (A)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `WNNLS`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/wnlsm.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/wnlsm.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/wnlsm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/wnlsm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Approximation](../families/approximation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `W` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (MDW, *) | Upon completion it contains the pivoting information for the cols of W(*,*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MME` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MA` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IPIVOT(*) An array of length N. | IPIVOT(*) An array of length N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `L` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Upon completion it contains the pivot scalars of the Householder transformations performed in the case KRANK.LT.L. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PRGOPT` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPIVOT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IPIVOT(*) An array of length N. | IPIVOT(*) An array of length N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ITYPE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | ITYPE(*) An array of length M which is used to keep track of the classification of the equations. | ITYPE(*) An array of length M which is used to keep track of the classification of the equations. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WD` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WD(*) An array of length N. | WD(*) An array of length N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | H(*) An array of length N. | H(*) An array of length N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SCALE` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | SCALE(*) An array of length M which is used by the subroutine to store the diagonal matrix of weights. | SCALE(*) An array of length M which is used by the subroutine to store the diagonal matrix of weights. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Z` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Z(*),TEMP(*) Working arrays of length N. | Z(*),TEMP(*) Working arrays of length N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TEMP` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Z(*),TEMP(*) Working arrays of length N. | Z(*),TEMP(*) Working arrays of length N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | D(*) An array of length N that contains the column scaling for the matrix (E). | D(*) An array of length N that contains the column scaling for the matrix (E). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
