# DBESI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute an N member sequence of I Bessel functions I/SUB(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/SUB(ALPHA+K-1)/(X), K=1,...,N for nonnegative ALPHA and X.

## Description

Abstract **** a double precision routine **** DBESI computes an N member sequence of I Bessel functions I/sub(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,...,N for nonnegative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity, and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane. For values not covered by one of these formulae, the order is incremented by an integer so that one of these formulae apply. Backward recursion is used to reduce orders by integer values. The asymptotic expansion for X to infinity is used only when the entire sequence (specifically the last member) lies within the region covered by the expansion. Leading terms of these expansions are used to test for over or underflow where appropriate. If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all are set to zero. An overflow cannot occur with scaling. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic. Description of Arguments Input X,ALPHA are double precision X - X .GE. 0.0D0 ALPHA - order of first member of the sequence, ALPHA .GE. 0.0D0 KODE - a parameter to indicate the scaling option KODE=1 returns Y(K)= I/sub(ALPHA+K-1)/(X), K=1,...,N KODE=2 returns Y(K)=EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,...,N N - number of members in the sequence, N .GE. 1 Output Y is double precision Y - a vector whose first N components contain values for I/sub(ALPHA+K-1)/(X) or scaled values for EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,...,N depending on KODE NZ - number of components of Y set to zero due to underflow, NZ=0 , normal return, computation completed NZ .NE. 0, last NZ components of Y set to zero, Y(K)=0.0D0, K=N-NZ+1,...,N. Error Conditions Improper input arguments - a fatal error Overflow with KODE=1 - a fatal error Underflow - a non-fatal error(NZ .NE. 0)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbesi.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbesi.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbesi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbesi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
