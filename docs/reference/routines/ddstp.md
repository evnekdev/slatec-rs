# DDSTP

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

DDSTP performs one step of the integration of an initial value problem for a system of ordinary differential equations.

## Description

Communication with DDSTP is done with the following variables: YH An N by MAXORD+1 array containing the dependent variables and their scaled derivatives. MAXORD, the maximum order used, is currently 12 for the Adams methods and 5 for the Gear methods. YH(I,J+1) contains the J-th derivative of Y(I), scaled by H**J/factorial(J). Only Y(I), 1 .LE. I .LE. N, need be set by the calling program on the first entry. The YH array should not be altered by the calling program. When referencing YH as a 2-dimensional array, use a column length of N, as this is the value used in DDSTP. DFDY A block of locations used for partial derivatives if MITER is not 0. If MITER is 1 or 2 its length must be at least N*N. If MITER is 4 or 5 its length must be at least (2*ML+MU+1)*N. YWT An array of N locations used in convergence and error tests SAVE1 SAVE2 Arrays of length N used for temporary storage. IPVT An integer array of length N used by the linear system solvers for the storage of row interchange information. A A block of locations used to store the matrix A, when using the implicit method. If IMPL is 1, A is a MATDIM by N array. If MITER is 1 or 2 MATDIM is N, and if MITER is 4 or 5 MATDIM is 2*ML+MU+1. If IMPL is 2 its length is N. If IMPL is 3, A is a MATDIM by NDE array. JTASK An integer used on input. It has the following values and meanings: .EQ. 0 Perform the first step. This value enables the subroutine to initialize itself. .GT. 0 Take a new step continuing from the last. Assumes the last step was successful and user has not changed any parameters. .LT. 0 Take a new step with a new value of H and/or MINT and/or MITER. JSTATE A completion code with the following meanings: 1 The step was successful. 2 A solution could not be obtained with H .NE. 0. 3 A solution was not obtained in MXTRY attempts. 4 For IMPL .NE. 0, the matrix A is singular. On a return with JSTATE .GT. 1, the values of T and the YH array are as of the beginning of the last step, and H is the last step size attempted.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `ode-dae-families`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/ddstp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddstp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddstp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddstp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `EPS` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FA` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `HMAX` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IMPL` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If IMPL is 1, A is a MATDIM by N array. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JACOBN` | callback | `INTEGER` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MATDIM` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If IMPL is 1, A is a MATDIM by N array. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MAXORD` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Communication with DDSTP is done with the following variables: YH An N by MAXORD+1 array containing the dependent variables and their scaled derivatives. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MINT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | 0 Take a new step with a new value of H and/or MINT and/or MITER. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MITER` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DFDY A block of locations used for partial derivatives if MITER is not 0. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ML` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If MITER is 4 or 5 its length must be at least (2*ML+MU+1)*N. | If MITER is 4 or 5 its length must be at least (2*ML+MU+1)*N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MU` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If MITER is 4 or 5 its length must be at least (2*ML+MU+1)*N. | If MITER is 4 or 5 its length must be at least (2*ML+MU+1)*N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Communication with DDSTP is done with the following variables: YH An N by MAXORD+1 array containing the dependent variables and their scaled derivatives. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NDE` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If IMPL is 3, A is a MATDIM by NDE array. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YWT` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | YWT An array of N locations used in convergence and error tests SAVE1 SAVE2 Arrays of length N used for temporary storage. | YWT An array of N locations used in convergence and error tests SAVE1 SAVE2 Arrays of length N used for temporary storage. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `UROUND` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `USERS` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AVGH` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AVGORD` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | YH(I,J+1) contains the J-th derivative of Y(I), scaled by H**J/factorial(J). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `HUSED` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JTASK` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | JTASK An integer used on input. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MNTOLD` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MTROLD` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NFE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NJE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NQUSED` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NSTEP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | 1, the values of T and the YH array are as of the beginning of the last step, and H is the last step size attempted. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | YH(I,J+1) contains the J-th derivative of Y(I), scaled by H**J/factorial(J). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YH` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (N, *) | Communication with DDSTP is done with the following variables: YH An N by MAXORD+1 array containing the dependent variables and their scaled derivatives. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (MATDIM, *) | When referencing YH as a 2-dimensional array, use a column length of N, as this is the value used in DDSTP. | When referencing YH as a 2-dimensional array, use a column length of N, as this is the value used in DDSTP. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CONVRG` | unavailable | `LOGICAL` (`explicit`) | `*mut crate::FortranLogical` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DFDY` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (MATDIM, *) | DFDY A block of locations used for partial derivatives if MITER is not 0. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (13, 12) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FAC` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `HOLD` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPVT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IPVT An integer array of length N used by the linear system solvers for the storage of row interchange information. | IPVT An integer array of length N used by the linear system solvers for the storage of row interchange information. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JSTATE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | JSTATE A completion code with the following meanings: 1 The step was successful. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JSTEPL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NQ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NWAIT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RC` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RMAX` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SAVE1` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | YWT An array of N locations used in convergence and error tests SAVE1 SAVE2 Arrays of length N used for temporary storage. | YWT An array of N locations used in convergence and error tests SAVE1 SAVE2 Arrays of length N used for temporary storage. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SAVE2` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | YWT An array of N locations used in convergence and error tests SAVE1 SAVE2 Arrays of length N used for temporary storage. | YWT An array of N locations used in convergence and error tests SAVE1 SAVE2 Arrays of length N used for temporary storage. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TQ` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (3, 12) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TREND` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ISWFLG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MTRSV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MXRDSV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `unsupported-abi`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
