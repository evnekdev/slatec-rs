# DBSKIN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute repeated integrals of the K-zero Bessel function.

## Description

The following definitions are used in DBSKIN: Definition 1 KI(0,X) = K-zero Bessel function. Definition 2 KI(N,X) = Bickley Function = integral from X to infinity of KI(N-1,t)dt for X .ge. 0 and N = 1,2,... _____________________________________________________________________ DBSKIN computes a sequence of Bickley functions (repeated integrals of the K0 Bessel function); i.e. for fixed X and N and for K=1,..., DBSKIN computes the sequence Y(K) = KI(N+K-1,X) for KODE=1 or Y(K) = EXP(X)*KI(N+K-1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). INPUT X is DOUBLE PRECISION X - Argument, X .ge. 0.0D0 N - Order of first member of the sequence N .ge. 0 KODE - Selection parameter KODE = 1 returns Y(K)= KI(N+K-1,X), K=1,M = 2 returns Y(K)=EXP(X)*KI(N+K-1,X), K=1,M M - Number of members in the sequence, M.ge.1 OUTPUT Y is a DOUBLE PRECISION VECTOR Y - A vector of dimension at least M containing the sequence selected by KODE. NZ - Underflow flag NZ = 0 means computation completed = 1 means an exponential underflow occurred on KODE=1. Y(K)=0.0D0, K=1,...,M is returned KODE=1 AND Y(K)=0.0E0, K=1,...,M IS RETURNED IERR - Error flag IERR=0, Normal return, computation completed IERR=1, Input error, no computation IERR=2, Error, no computation Algorithm termination condition not met The nominal computational accuracy is the maximum of unit roundoff (=D1MACH(4)) and 1.0D-18 since critical constants are given to only 18 digits. BSKIN is the single precision version of DBSKIN. *Long Description: Numerical recurrence on (L-1)*KI(L,X) = X(KI(L-3,X) - KI(L-1,X)) + (L-2)*KI(L-2,X) is stable where recurrence is carried forward or backward away from INT(X+0.5). The power series for indices 0,1 and 2 on 0.le.X.le.2 starts a stable recurrence for indices greater than 2. If N is sufficiently large (N.gt.NLIM), the uniform asymptotic expansion for N to INFINITY is more economical. On X.gt.2 the recursion is started by evaluating the uniform expansion for the three members whose indices are closest to INT(X+0.5) within the set N,...,N+M-1. Forward recurrence, backward recurrence or both complete the sequence depending on the relation of INT(X+0.5) to the indices N,...,N+M-1.

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
- GAMS classifications: `C10F`
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

- Canonical provider: `main-src/src/dbskin.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbskin.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbskin.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbskin.f) — `verified_cached`
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
