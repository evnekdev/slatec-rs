# SOS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a square system of nonlinear equations.

## Description

SOS solves a system of NEQ simultaneous nonlinear equations in NEQ unknowns. That is, it solves the problem F(X)=0 where X is a vector with components X(1),...,X(NEQ) and F is a vector of nonlinear functions. Each equation is of the form F (X(1),...,X(NEQ))=0 for K=1,...,NEQ. K The algorithm is based on an iterative method which is a variation of Newton's method using Gaussian elimination in a manner similar to the Gauss-Seidel process. Convergence is roughly quadratic. All partial derivatives required by the algorithm are approximated by first difference quotients. The convergence behavior of this code is affected by the ordering of the equations, and it is advantageous to place linear and mildly nonlinear equations first in the ordering. Actually, SOS is merely an interfacing routine for calling subroutine SOSEQS which embodies the solution algorithm. The purpose of this is to add greater flexibility and ease of use for the prospective user. SOSEQS calls the accompanying routine SOSSOL, which solves special triangular linear systems by back-substitution. The user must supply a function subprogram which evaluates the K-th equation only (K specified by SOSEQS) for each call to the subprogram. SOS represents an implementation of the mathematical algorithm described in the references below. It is a modification of the code SOSNLE written by H. A. Watts in 1973. **********************************************************************

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sos.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sos.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sos.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sos.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
