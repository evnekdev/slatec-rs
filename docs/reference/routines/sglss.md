# SGLSS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a linear least squares problems by performing a QR factorization of the matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. If M.GE.N, the least squares solution is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factorization). If the matrix A is determined to be rank deficient, that is the rank of A is less than MIN(M,N), then the minimal length least squares solution is computed. SGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes LLSIA and ULSIA are recommended. SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. ****************************************************************** * * * WARNING - All input arrays are changed on exit. * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
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

- Canonical provider: `main-src/src/sglss.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sglss.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sglss.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sglss.f) — `verified_cached`
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
