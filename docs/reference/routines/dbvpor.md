# DBVPOR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBVSUP

## Description

********************************************************************** INPUT to DBVPOR (items not defined in DBVSUP comments) ********************************************************************** NOPG = 0 -- orthonormalization points not pre-assigned = 1 -- orthonormalization points pre-assigned MXNON = maximum number of orthogonalizations allowed. NDISK = 0 -- in-core storage = 1 -- disk storage. Value of NTAPE in data statement is set to 13. If another value is desired, the data statement must be changed. INTEG = type of integrator and associated test to be used to determine when to orthonormalize. 1 -- use GRAM-SCHMIDT test and DDERKF 2 -- use GRAM-SCHMIDT test and DDEABM TOL = tolerance for allowable error in orthogonalization test. NPS = 0 normalize particular solution to unit length at each point of orthonormalization. = 1 do not normalize particular solution. NTP = must be .GE. NFC*(NFC+1)/2. NFCC = 2*NFC for special treatment of a COMPLEX*16 valued problem ICOCO = 0 skip final computations (superposition coefficients and, hence, boundary problem solution) = 1 calculate superposition coefficients and obtain solution to the boundary value problem ********************************************************************** OUTPUT from DBVPOR ********************************************************************** Y(NROWY,NXPTS) = solution at specified output points. MXNON = number of orthonormalizations performed by DBVPOR. Z(MXNON+1) = locations of orthonormalizations performed by DBVPOR. NIV = number of independent vectors returned from DMGSBV. Normally this parameter will be meaningful only when DMGSBV returns with MFLAG = 2. ********************************************************************** The following variables are in the argument list because of variable dimensioning. In general, they contain no information of use to the user. The amount of storage set aside by the user must be greater than or equal to that indicated by the dimension statements. For the disk storage mode, NON = 0 and KPTS = 1, while for the in-core storage mode, NON = MXNON and KPTS = NXPTS. P(NTP,NON+1) IP(NFCC,NON+1) YHP(NCOMP,NFC+1) plus an additional column of the length NEQIVP U(NCOMP,NFC,KPTS) V(NCOMP,KPTS) W(NFCC,NON+1) COEF(NFCC) S(NFC+1) STOWA(NCOMP*(NFC+1)+NEQIVP+1) G(NCOMP) WORK(KKKWS) IWORK(LLLIWS) ********************************************************************** SUBROUTINES used by DBVPOR DLSSUD -- solves an underdetermined system of linear equations. This routine is used to get a full set of initial conditions for integration. Called by DBVPOR. DVECS -- obtains starting vectors for special treatment of COMPLEX*16 valued problems, called by DBVPOR. DRKFAB -- routine which conducts integration using DDERKF or DDEABM. DSTWAY -- storage for backup capability, called by DBVPOR and DREORT. DSTOR1 -- storage at output points, called by DBVPOR, DRKFAB, DREORT and DSTWAY. DDOT -- single precision vector inner product routine, called by DBVPOR, DCOEF, DLSSUD, DMGSBV, DBKSOL, DREORT and DPRVEC. ** NOTE ** a considerable improvement in speed can be achieved if a machine language version is used for DDOT. DCOEF -- computes the superposition constants from the boundary conditions at XFINAL. DBKSOL -- solves an upper triangular set of linear equations. **********************************************************************

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

- Canonical provider: `main-src/src/dbvpor.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbvpor.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbvpor.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbvpor.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
