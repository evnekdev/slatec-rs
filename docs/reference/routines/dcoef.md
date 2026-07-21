# DCOEF

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBVSUP

## Description

********************************************************************** INPUT to DCOEF ********************************************************************** YH = matrix of homogeneous solutions. YP = vector containing particular solution. NCOMP = number of components per solution vector. NROWB = first dimension of B in calling program. NFC = number of base solution vectors. NFCC = 2*NFC for the special treatment of COMPLEX*16 valued equations. Otherwise, NFCC=NFC. NIC = number of specified initial conditions. B = boundary condition matrix at X = XFINAL. BETA = vector of nonhomogeneous boundary conditions at X = XFINAL. 1 - nonzero particular solution INHOMO = 2 - zero particular solution 3 - eigenvalue problem RE = relative error tolerance. AE = absolute error tolerance. BY = storage space for the matrix B*YH CVEC = storage space for the vector BETA-B*YP WORK = double precision array of internal storage. Dimension must be GE NFCC*(NFCC+4) IWORK = integer array of internal storage. Dimension must be GE 3+NFCC ********************************************************************** OUTPUT from DCOEF ********************************************************************** COEF = array containing superposition constants. IFLAG = indicator of success from DSUDS in solving the boundary equations. = 0 boundary equations are solved. = 1 boundary equations appear to have many solutions. = 2 boundary equations appear to be inconsistent. = 3 for this value of an eigenparameter, the boundary equations have only the zero solution. ********************************************************************** Subroutine DCOEF solves for the superposition constants from the linear equations defined by the boundary conditions at X = XFINAL. B*YP + B*YH*COEF = BETA **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBVSUP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dcoef.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dcoef.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dcoef.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dcoef.f) — `verified_cached`
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
| `YH` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (NCOMP, *) | ********************************************************************** INPUT to DCOEF ********************************************************************** YH = matrix of homogeneous solutions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YP` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | YP = vector containing particular solution. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NCOMP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NCOMP = number of components per solution vector. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NROWB` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NROWB = first dimension of B in calling program. | NROWB = first dimension of B in calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NFC` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NFC = number of base solution vectors. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NIC` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NIC = number of specified initial conditions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (NROWB, *) | NROWB = first dimension of B in calling program. | NROWB = first dimension of B in calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BETA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | BETA = vector of nonhomogeneous boundary conditions at X = XFINAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `COEF` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Dimension must be GE 3+NFCC ********************************************************************** OUTPUT from DCOEF ********************************************************************** COEF = array containing superposition constants. | Dimension must be GE 3+NFCC ********************************************************************** OUTPUT from DCOEF ********************************************************************** COEF = array containing superposition constants. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INHOMO` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | 1 - nonzero particular solution INHOMO = 2 - zero particular solution 3 - eigenvalue problem RE = relative error tolerance. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RE` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | 1 - nonzero particular solution INHOMO = 2 - zero particular solution 3 - eigenvalue problem RE = relative error tolerance. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AE` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | AE = absolute error tolerance. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BY` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (NFCC, *) | BY = storage space for the matrix B*YH CVEC = storage space for the vector BETA-B*YP WORK = double precision array of internal storage. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CVEC` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | BY = storage space for the matrix B*YH CVEC = storage space for the vector BETA-B*YP WORK = double precision array of internal storage. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | BY = storage space for the matrix B*YH CVEC = storage space for the vector BETA-B*YP WORK = double precision array of internal storage. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Dimension must be GE NFCC*(NFCC+4) IWORK = integer array of internal storage. | Dimension must be GE NFCC*(NFCC+4) IWORK = integer array of internal storage. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IFLAG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IFLAG = indicator of success from DSUDS in solving the boundary equations. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NFCC` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NFCC = 2*NFC for the special treatment of COMPLEX*16 valued equations. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
