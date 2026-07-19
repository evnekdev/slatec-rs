# DWNLSM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DWNNLS

## Description

This is a companion subprogram to DWNNLS. The documentation for DWNNLS has complete usage instructions. In addition to the parameters discussed in the prologue to subroutine DWNNLS, the following work arrays are used in subroutine DWNLSM (they are passed through the calling sequence from DWNNLS for purposes of variable dimensioning). Their contents will in general be of no interest to the user. Variables of type REAL are DOUBLE PRECISION. IPIVOT(*) An array of length N. Upon completion it contains the pivoting information for the cols of W(*,*). ITYPE(*) An array of length M which is used to keep track of the classification of the equations. ITYPE(I)=0 denotes equation I as an equality constraint. ITYPE(I)=1 denotes equation I as a least squares equation. WD(*) An array of length N. Upon completion it contains the dual solution vector. H(*) An array of length N. Upon completion it contains the pivot scalars of the Householder transformations performed in the case KRANK.LT.L. SCALE(*) An array of length M which is used by the subroutine to store the diagonal matrix of weights. These are used to apply the modified Givens transformations. Z(*),TEMP(*) Working arrays of length N. D(*) An array of length N that contains the column scaling for the matrix (E). (A)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DWNNLS`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dwnlsm.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dwnlsm.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dwnlsm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dwnlsm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
