# DSUDS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBVSUP

## Description

DSUDS solves the underdetermined system of linear equations A Z = B where A is NEQ by NUK and NEQ .LE. NUK. in particular, if rank A equals IRA, a vector X and a matrix U are determined such that X is the UNIQUE solution of smallest length, satisfying A X = B, and the columns of U form an orthonormal basis for the null space of A, satisfying A U = 0 . Then all solutions Z are given by Z = X + C(1)*U(1) + ..... + C(NUK-IRA)*U(NUK-IRA) where U(J) represents the J-th column of U and the C(J) are arbitrary constants. If the system of equations are not compatible, only the least squares solution of minimal length is computed. DSUDS is an interfacing routine which calls subroutine DLSSUD for the solution. DLSSUD in turn calls subroutine DORTHR and possibly subroutine DOHTRL for the decomposition of A by orthogonal transformations. In the process, DORTHR calls upon subroutine DCSCAL for scaling. ********************************************************************

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

- Canonical provider: `main-src/src/dsuds.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dsuds.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dsuds.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dsuds.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
