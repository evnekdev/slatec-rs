# BVPOR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to BVSUP

## Description

********************************************************************** INPUT to BVPOR (items not defined in BVSUP comments) ********************************************************************** NOPG = 0 -- Orthonormalization points not pre-assigned = 1 -- Orthonormalization points pre-assigned MXNON = Maximum number of orthogonalizations allowed. NDISK = 0 -- IN-CORE storage = 1 -- DISK storage. Value of NTAPE in data statement is set to 13. If another value is desired, the data statement must be changed. INTEG = Type of integrator and associated test to be used to determine when to orthonormalize. 1 -- Use GRAM-SCHMIDT test and DERKF 2 -- Use GRAM-SCHMIDT test and DEABM TOL = Tolerance for allowable error in orthogonalization test. NPS = 0 Normalize particular solution to unit length at each point of orthonormalization. = 1 Do not normalize particular solution. NTP = Must be .GE. NFC*(NFC+1)/2. NFCC = 2*NFC for special treatment of a complex valued problem ICOCO = 0 Skip final computations (superposition coefficients and ,hence, boundary problem solution) = 1 Calculate superposition coefficients and obtain solution to the boundary value problem ********************************************************************** OUTPUT from BVPOR ********************************************************************** Y(NROWY,NXPTS) = Solution at specified output points. MXNON = Number of orthonormalizations performed by BVPOR. Z(MXNON+1) = Locations of orthonormalizations performed by BVPOR. NIV = Number of independent vectors returned from MGSBV. Normally this parameter will be meaningful only when MGSBV returns with MFLAG = 2. ********************************************************************** The following variables are in the argument list because of variable dimensioning. In general, they contain no information of use to the user. The amount of storage set aside by the user must be greater than or equal to that indicated by the dimension statements. For the DISK storage mode, NON = 0 and KPTS = 1, while for the IN-CORE storage mode, NON = MXNON and KPTS = NXPTS. P(NTP,NON+1) IP(NFCC,NON+1) YHP(NCOMP,NFC+1) plus an additional column of the length NEQIVP U(NCOMP,NFC,KPTS) V(NCOMP,KPTS) W(NFCC,NON+1) COEF(NFCC) S(NFC+1) STOWA(NCOMP*(NFC+1)+NEQIVP+1) G(NCOMP) WORK(KKKWS) IWORK(LLLIWS) ********************************************************************** Subroutines used by BVPOR LSSUDS -- Solves an underdetermined system of linear equations. This routine is used to get a full set of initial conditions for integration. Called by BVPOR SVECS -- Obtains starting vectors for special treatment of complex valued problems , called by BVPOR RKFAB -- Routine which conducts integration using DERKF or DEABM STWAY -- Storage for backup capability, called by BVPOR and REORT STOR1 -- Storage at output points, called by BVPOR, RKFAB, REORT and STWAY. SDOT -- Single precision vector inner product routine, called by BVPOR, SCOEF, LSSUDS, MGSBV, BKSOL, REORT and PRVEC. ** NOTE ** A considerable improvement in speed can be achieved if a machine language version is used for SDOT. SCOEF -- Computes the superposition constants from the boundary conditions at Xfinal. BKSOL -- Solves an upper triangular set of linear equations. **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `BVSUP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/bvpor.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bvpor.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bvpor.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bvpor.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
