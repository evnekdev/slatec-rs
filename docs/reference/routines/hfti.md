# HFTI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a linear least squares problems by performing a QR factorization of the matrix using Householder transformations.

## Description

DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. The problem data consists of an M by N matrix A, an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below. The NB column vectors of B represent right-side vectors for NB distinct linear least squares problems. This set of problems can also be written as the matrix least squares problem AX = B, where X is the N by NB solution matrix. Note that if B is the M by M identity matrix, then X will be the pseudo-inverse of A. This subroutine first transforms the augmented matrix (A B) to a matrix (R C) using premultiplying Householder transformations with column interchanges. All subdiagonal elements in the matrix R are zero and its diagonal elements satisfy ABS(R(I,I)).GE.ABS(R(I+1,I+1)), I = 1,...,L-1, where L = MIN(M,N). The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. Then a solution of minimum Euclidean length is computed using the first KRANK rows of (R C). To be specific we suggest that the user consider an easily computable matrix norm, such as, the maximum of all column sums of magnitudes. Now if the relative uncertainty of B is EPS, (norm of uncertainty/ norm of B), it is suggested that TAU be set approximately equal to EPS*(norm of A). The user must dimension all arrays appearing in the call list.. A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N). This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/hfti.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/hfti.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/hfti.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/hfti.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::hfti`
- Current legacy Rust paths: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
