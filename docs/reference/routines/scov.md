# SCOV

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either SNLS1 or SNLS1E.

## Description

1. Purpose. SCOV calculates the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either SNLS1 or SNLS1E. SCOV and SNLS1 (and SNLS1E) have compatible parameters. The required external subroutine, FCN, is the same for all three codes, SCOV, SNLS1, and SNLS1E. 2. Subroutine and Type Statements. SUBROUTINE SCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO REAL X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
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
- Safe Rust paths: `slatec::least_squares::covariance_from_expert_fit_f32, slatec::least_squares::estimate_covariance_f32, slatec::least_squares::estimate_covariance_finite_difference_f32`

## Providers

- Canonical provider: `main-src/src/scov.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/scov.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/scov.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/scov.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SCOV](https://www.netlib.org/slatec/src/scov.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | supplied subroutine which calculates the functions.  If the user wants to supply the Jacobian must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N REAL X(N),FVEC(M) 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) is set as follows. |
| 2 | `IOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions.  See the explanation must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N REAL X(N),FVEC(M) 1. 2. 3. 2.  FVEC contains the function values at X and must not be altered.  FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN 3.  FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN is an input variable which specifies how the Jacobian will 2 or 3, then the user must supply the Jacobian, as well as the function values, through the 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) 1, the code will approximate the Jacobian by forward differencing. 1 and 2, R is an M by N array. 3, R is an N by N array.  On output, if INFO=1, 1 and 2, 3, LDR must not be less than N. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of functions. contains the functions evaluated at X. 3, LDR must not be less than N. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 2. 3. is a positive integer input variable set to the number of variables.  N must not exceed M. contain the value at which the covariance matrix is to be evaluated.  This is usually the value for X returned from a successful run of SNLS1 (or SNLS1E).  The value of X will not be changed. contains the covariance contains the covariance matrix evaluated at X. matrix evaluated at X. contains the QR contains the QR factorization of the Jacobian (probably not of factorization of the Jacobian (probably not of interest to the user). interest to the user). |
| 5 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contain the value contain the value at which the covariance matrix is to be evaluated.  This is at which the covariance matrix is to be evaluated.  This is usually the value for X returned from a successful run of usually the value for X returned from a successful run of SNLS1 (or SNLS1E).  The value of X will not be changed. SNLS1 (or SNLS1E).  The value of X will not be changed. |
| 6 | `FVEC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the functions evaluated at X. |
| 7 | `R` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDR, *) | 1 and 2, R is an M by N array. contains the covariance matrix evaluated at X. 1 and 2, contains the QR factorization of the Jacobian (probably not of interest to the user). |
| 8 | `LDR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable which specifies 3, LDR must not be less than N. |
| 9 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable.  If the user has terminated execution, INFO is set to the (negative) value of IFLAG.  See is set as follows. 0 Improper input parameters (M.LE.0 or N.LE.0). 1 Successful return.  The covariance matrix has been calculated and stored in the upper N by N submatrix of R. 2 The Jacobian matrix is singular for the input value of X.  The covariance matrix cannot be calculated. |
| 10 | `WA1` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a work array of length N. |
| 11 | `WA2` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a work array of length N. |
| 12 | `WA3` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a work array of length N. |
| 13 | `WA4` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a work array of length M. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::least_squares::scov`. Native symbol: `scov_`. Declaration feature: `least-squares-covariance`. Provider feature: `least-squares-covariance`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::least_squares::scov`
- Public declaration feature: `least-squares-covariance`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::least_squares::covariance_from_expert_fit_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
