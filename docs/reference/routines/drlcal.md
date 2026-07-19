# DRLCAL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Internal routine for DGMRES.

## Description

This routine calculates the scaled residual RL from the V(I)'s. *Usage: INTEGER N, KMP, LL, MAXL DOUBLE PRECISION V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL DRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. KMP :IN Integer The number of previous V vectors the new vector VNEW must be made orthogonal to. (KMP .le. MAXL) LL :IN Integer The current dimension of the Krylov subspace. MAXL :IN Integer The maximum dimension of the Krylov subspace. V :IN Double Precision V(N,LL) The N x LL array containing the orthogonal vectors V(*,1) to V(*,LL). Q :IN Double Precision Q(2*MAXL) A double precision array of length 2*MAXL containing the components of the Givens rotations used in the QR decomposition of HES. It is loaded in DHEQR and used in DHELS. RL :OUT Double Precision RL(N) The residual vector RL. This is either SB*(B-A*XL) if not preconditioning or preconditioning on the right, or SB*(M-inverse)*(B-A*XL) if preconditioning on the left. SNORMW :IN Double Precision Scale factor. PROD :IN Double Precision The product s1*s2*...*sl = the product of the sines of the Givens rotations used in the QR factorization of the Hessenberg matrix HES. R0NRM :IN Double Precision The scaled norm of initial residual R0.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2A4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/drlcal.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/drlcal.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/drlcal.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
