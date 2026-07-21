# DCOV

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either DNLS1 or DNLS1E.

## Description

1. Purpose. DCOV calculates the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either DNLS1 or DNLS1E. DCOV and DNLS1 (and DNLS1E) have compatible parameters. The required external subroutine, FCN, is the same for all three codes, DCOV, DNLS1, and DNLS1E. 2. Subroutine and Type Statements. SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO,

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DCOV](https://www.netlib.org/slatec/src/dcov.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | is the name of the user-supplied subroutine which calculates the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. If the user wants the iterates printed in DNLS1 or DNLS1E, then FCN must do the printing. See the explanation of NPRINT in DNLS1 or DNLS1E. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. |
| 2 | `IOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 2. FVEC contains the function values at X and must not be altered. FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC. Note that IFLAG will never be 3 unless IOPT=3. FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of functions. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of variables. N must not exceed M. |
| 5 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is an array of length N. On input X must contain the value at which the covariance matrix is to be evaluated. This is usually the value for X returned from a successful run of DNLS1 (or DNLS1E). The value of X will not be changed. |
| 6 | `FVEC` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is an output array of length M which contains the functions evaluated at X. |
| 7 | `R` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDR, *) | is an output array. For IOPT=1 and 2, R is an M by N array. For IOPT=3, R is an N by N array. On output, if INFO=1, the upper N by N submatrix of R contains the covariance matrix evaluated at X. |
| 8 | `LDR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable which specifies the leading dimension of the array R. For IOPT=1 and 2, must not be less than M. For IOPT=3, LDR must not be less than N. |
| 9 | `INFO` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN. Otherwise, INFO is set as follows. INFO = 0 Improper input parameters (M. LE. |
| 10 | `WA1` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION work arrays of length N. and WA3. |
| 11 | `WA2` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION work arrays of length N. and WA3. |
| 12 | `WA3` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION. |
| 13 | `WA4` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION is a work array of length M. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 Improper input parameters (M.LE.0 or N.LE.0). |
| `INFO` | `1` | 1 Successful return. The covariance matrix has been calculated and stored in the upper N by N submatrix of R. |
| `INFO` | `2` | 2 The Jacobian matrix is singular for the input value of X. The covariance matrix cannot be calculated. The upper N by N submatrix of R contains the QR factorization of the Jacobian (probably not of interest to the user). |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::least_squares::dcov`. Native symbol: `dcov_`. Declaration feature: `least-squares-covariance`. Provider feature: `least-squares-covariance`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::least_squares::dcov`
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
