# DCOV

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either DNLS1 or DNLS1E.

## Description

1. Purpose. DCOV calculates the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either DNLS1 or DNLS1E. DCOV and DNLS1 (and DNLS1E) have compatible parameters. The required external subroutine, FCN, is the same for all three codes, DCOV, DNLS1, and DNLS1E. 2. Subroutine and Type Statements. SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION FCN is the name of the user-supplied subroutine which calculates the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. If the user wants the iterates printed in DNLS1 or DNLS1E, then FCN must do the printing. See the explanation of NPRINT in DNLS1 or DNLS1E. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N DOUBLE PRECISION X(N),FVEC(M) FJAC and LDFJAC may be ignored , if IOPT=1. DOUBLE PRECISION FJAC(LDFJAC,N) , if IOPT=2. DOUBLE PRECISION FJAC(N) , if IOPT=3. If IFLAG=0, the values in X and FVEC are available for printing in DNLS1 or DNLS1E. IFLAG will never be zero when FCN is called by DCOV. The values of X and FVEC must not be changed. RETURN If IFLAG=1, calculate the functions at X and return this vector in FVEC. RETURN If IFLAG=2, calculate the full Jacobian at X and return this matrix in FJAC. Note that IFLAG will never be 2 unless IOPT=2. FVEC contains the function values at X and must not be altered. FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC. Note that IFLAG will never be 3 unless IOPT=3. FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1B1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::least_squares::covariance_from_expert_fit, slatec::least_squares::estimate_covariance, slatec::least_squares::estimate_covariance_finite_difference`

## Providers

- Canonical provider: `main-src/src/dcov.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dcov.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dcov.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dcov.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Approximation](../families/approximation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `FCN` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | The required external subroutine, FCN, is the same for all three codes, DCOV, DNLS1, and DNLS1E. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IOPT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FVEC` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDR, *) | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA1` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA2` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA3` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA4` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::least_squares::dcov`. Native symbol: `dcov_`. Feature: `least-squares-covariance`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::least_squares::dcov`
- Compatibility aliases: `none`
- Public declaration feature: `least-squares-covariance`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::least_squares::covariance_from_expert_fit`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
