# DDSTP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

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

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
