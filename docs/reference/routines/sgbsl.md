# SGBSL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve the real band system A*X=B or TRANS(A)*X=B using the factors computed by SGBCO or SGBFA.

## Description

SGBSL solves the real band system A * X = B or TRANS(A) * X = B using the factors computed by SBGCO or SGBFA. On Entry ABD REAL(LDA, N) the output from SBGCO or SGBFA. LDA INTEGER the leading dimension of the array ABD . N INTEGER the order of the original matrix. ML INTEGER number of diagonals below the main diagonal. MU INTEGER number of diagonals above the main diagonal. IPVT INTEGER(N) the pivot vector from SBGCO or SGBFA. B REAL(N) the right hand side vector. JOB INTEGER = 0 to solve A*X = B , = nonzero to solve TRANS(A)*X = B , where TRANS(A) is the transpose. On Return B the solution vector X . Error Condition A division by zero will occur if the input factor contains a zero on the diagonal. Technically, this indicates singularity, but it is often caused by improper arguments or improper setting of LDA . It will not occur if the subroutines are called correctly and if SBGCO has set RCOND .GT. 0.0 or SGBFA has set INFO .EQ. 0 . To compute INVERSE(A) * C where C is a matrix with P columns CALL SBGCO(ABD,LDA,N,ML,MU,IPVT,RCOND,Z) If (RCOND is too small) GO TO ... DO 10 J = 1, P CALL SGBSL(ABD,LDA,N,ML,MU,IPVT,C(1,J),0) 10 CONTINUE

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
- GAMS classifications: `D2A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/sgbsl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sgbsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sgbsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::banded::sgbsl`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
