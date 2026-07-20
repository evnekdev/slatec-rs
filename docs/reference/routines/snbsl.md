# SNBSL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a real band system using the factors computed by SNBCO or SNBFA.

## Description

SNBSL solves the real band system A * X = B or TRANS(A) * X = B using the factors computed by SNBCO or SNBFA. On Entry ABE REAL(LDA, NC) the output from SNBCO or SNBFA. NC must be .GE. 2*ML+MU+1 . LDA INTEGER the leading dimension of the array ABE . N INTEGER the order of the original matrix. ML INTEGER number of diagonals below the main diagonal. MU INTEGER number of diagonals above the main diagonal. IPVT INTEGER(N) the pivot vector from SNBCO or SNBFA. B REAL(N) the right hand side vector. JOB INTEGER = 0 to solve A*X = B . = nonzero to solve TRANS(A)*X = B , where TRANS(A) is the transpose. On Return B the solution vector X . Error Condition A division by zero will occur if the input factor contains a zero on the diagonal. Technically, this indicates singularity, but it is often caused by improper arguments or improper setting of LDA. It will not occur if the subroutines are called correctly and if SNBCO has set RCOND .GT. 0.0 or SNBFA has set INFO .EQ. 0 . To compute INVERSE(A) * C where C is a matrix with P columns CALL SNBCO(ABE,LDA,N,ML,MU,IPVT,RCOND,Z) IF (RCOND is too small) GO TO ... DO 10 J = 1, P CALL SNBSL(ABE,LDA,N,ML,MU,IPVT,C(1,J),0) 10 CONTINUE

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

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/snbsl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/snbsl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/snbsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/snbsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::snbsl`
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
