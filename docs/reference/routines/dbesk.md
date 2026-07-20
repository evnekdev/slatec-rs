# DBESK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.

## Description

Abstract **** a double precision routine **** DBESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,..,N for real X .GT. 0.0D0 and non-negative orders FNU. If FNU .LT. NULIM, orders FNU and FNU+1 are obtained from DBSKNU to start the recursion. If FNU .GE. NULIM, the uniform asymptotic expansion is used for orders FNU and FNU+1 to start the recursion. NULIM is 35 or 70 depending on whether N=1 or N .GE. 2. Under and overflow tests are made on the leading term of the asymptotic expansion before any extensive computation is done. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic. Description of Arguments Input X,FNU are double precision X - X .GT. 0.0D0 FNU - order of the initial K function, FNU .GE. 0.0D0 KODE - a parameter to indicate the scaling option KODE=1 returns Y(I)= K/sub(FNU+I-1)/(X), I=1,...,N KODE=2 returns Y(I)=EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N N - number of members in the sequence, N .GE. 1 Output Y is double precision Y - a vector whose first N components contain values for the sequence Y(I)= k/sub(FNU+I-1)/(X), I=1,...,N or Y(I)=EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N depending on KODE NZ - number of components of Y set to zero due to underflow with KODE=1, NZ=0 , normal return, computation completed NZ .NE. 0, first NZ components of Y set to zero due to underflow, Y(I)=0.0D0, I=1,...,NZ Error Conditions Improper input arguments - a fatal error Overflow - a fatal error Underflow with KODE=1 - a non-fatal error (NZ .NE. 0)

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

- Canonical provider: `main-src/src/dbesk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbesk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbesk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbesk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::special::numerical::dbesk`
- Current legacy Rust paths: `none`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
