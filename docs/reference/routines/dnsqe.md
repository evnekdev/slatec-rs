# DNSQE

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

An easy-to-use code to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method.

## Description

1. Purpose. The purpose of DNSQE is to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method. This is done by using the more general nonlinear equation solver DNSQ. The user must provide a subroutine which calculates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD1 and HYBRJ1. 2. Subroutine and Type Statements. SUBROUTINE DNSQE(FCN,JAC,IOPT,N,X,FVEC,TOL,NPRINT,INFO, * WA,LWA) INTEGER IOPT,N,NPRINT,INFO,LWA DOUBLE PRECISION TOL DOUBLE PRECISION X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC 3. Parameters. Parameters designated as input parameters must be specified on entry to DNSQE and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNSQE. FCN is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an external statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) Calculate the functions at X and return this vector in FVEC. RETURN

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::nonlinear::solve_system`

## Providers

- Canonical provider: `main-src/src/dnsqe.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnsqe.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnsqe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnsqe.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::nonlinear::dnsqe`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `slatec::nonlinear::solve_system`
- Exclusion or deferment reason: `pre-existing declaration remains deferred until the R1 source-hash, argument-documentation, and ABI review gate passes`
<!-- raw-api-status:end -->
