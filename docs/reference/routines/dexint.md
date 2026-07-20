# DEXINT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute an M member sequence of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0.

## Description

DEXINT computes M member sequences of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0. The exponential integral is defined by E(N,X)=integral on (1,infinity) of EXP(-XT)/T**N where X=0.0 and N=1 cannot occur simultaneously. Formulas and notation are found in the NBS Handbook of Mathematical Functions (ref. 1). The power series is implemented for X .LE. XCUT and the confluent hypergeometric representation E(A,X) = EXP(-X)*(X**(A-1))*U(A,A,X) is computed for X .GT. XCUT. Since sequences are computed in a stable fashion by recurring away from X, A is selected as the integer closest to X within the constraint N .LE. A .LE. N+M-1. For the U computation, A is further modified to be the nearest even integer. Indices are carried forward or backward by the two term recursion relation K*E(K+1,X) + X*E(K,X) = EXP(-X) once E(A,X) is computed. The U function is computed by means of the backward recursive Miller algorithm applied to the three term contiguous relation for U(A+K,A,X), K=0,1,... This produces accurate ratios and determines U(A+K,A,X), and hence E(A,X), to within a multiplicative constant C. Another contiguous relation applied to C*U(A,A,X) and C*U(A+1,A,X) gets C*U(A+1,A+1,X), a quantity proportional to E(A+1,X). The normalizing constant C is obtained from the two term recursion relation above with K=A. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic. Description of Arguments Input * X and TOL are double precision * X X .GT. 0.0 for N=1 and X .GE. 0.0 for N .GE. 2 N order of the first member of the sequence, N .GE. 1 (X=0.0 and N=1 is an error) KODE a selection parameter for scaled values KODE=1 returns E(N+K,X), K=0,1,...,M-1. =2 returns EXP(X)*E(N+K,X), K=0,1,...,M-1. M number of exponential integrals in the sequence, M .GE. 1 TOL relative accuracy wanted, ETOL .LE. TOL .LE. 0.1 ETOL is the larger of double precision unit roundoff = D1MACH(4) and 1.0D-18 Output * EN is a double precision vector * EN a vector of dimension at least M containing values EN(K) = E(N+K-1,X) or EXP(X)*E(N+K-1,X), K=1,M depending on KODE NZ underflow indicator NZ=0 a normal return NZ=M X exceeds XLIM and an underflow occurs. EN(K)=0.0D0 , K=1,M returned on KODE=1 IERR error flag IERR=0, normal return, computation completed IERR=1, input error, no computation IERR=2, error, no computation algorithm termination condition not met

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
- GAMS classifications: `C5`
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

- Canonical provider: `main-src/src/dexint.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dexint.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dexint.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dexint.f) — `verified_cached`
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
