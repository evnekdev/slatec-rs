# DBOCLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the bounded and constrained least squares problem consisting of solving the equation E*X = F (in the least squares sense) subject to the linear constraints C*X = Y.

## Description

All INPUT and OUTPUT real variables are DOUBLE PRECISION **** This subprogram solves the bounded and constrained least squares problem. The problem statement is: Solve E*X = F (least squares sense), subject to constraints C*X=Y. In this formulation both X and Y are unknowns, and both may have bounds on any of their components. This formulation of the problem allows the user to have equality and inequality constraints as well as simple bounds on the solution components. This constrained linear least squares subprogram solves E*X=F subject to C*X=Y, where E is MROWS by NCOLS, C is MCON by NCOLS. The user must have dimension statements of the form DIMENSION W(MDW,NCOLS+MCON+1), BL(NCOLS+MCON), BU(NCOLS+MCON), X(2*(NCOLS+MCON)+2+NX), RW(6*NCOLS+5*MCON) INTEGER IND(NCOLS+MCON), IOPT(17+NI), IW(2*(NCOLS+MCON)) (here NX=number of extra locations required for the options; NX=0 if no options are in use. Also NI=number of extra locations for options 1-9.)

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
- GAMS classifications: `K1A2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares`

## Providers

- Canonical provider: `main-src/src/dbocls.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbocls.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbocls.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbocls.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [DBOCLS](https://www.netlib.org/slatec/src/dbocls.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDW, *) | The array W contains the (possibly null) matrix [C:*] followed by [E:F]. This must be placed in W as follows: [C : *] = [ ] [E : F] The (*) after C indicates that this data can be undefined. The matrix [E:F] has MROWS rows and NCOLS+1 columns. The matrix C is placed in the first MCON rows of W(*,*) while [E:F] follows in rows MCON+1 through MCON+MROWS of W(*,*). The vector F is placed in rows MCON+1 through MCON+MROWS, column NCOLS+1. The values of MDW and NCOLS must be positive; the value of MCON must be nonnegative. |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ge. MCON + max(max. number of rows accumulated, NCOLS) + 1. If using option 8,. MCON + MROWS. Else. |
| 3 | `MCON` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The array W contains the (possibly null) matrix [C:*] followed by [E:F]. This must be placed in W as follows: [C : *]. |
| 4 | `MROWS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of rows in the least-squares block `[E:F]`. With accumulation option `IOPT(1)`, this is a writable returned count of accumulated rows; otherwise it is an input count used with `MDW`, `MCON`, and `NCOLS`. |
| 5 | `NCOLS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The array W contains the (possibly null) matrix [C:*] followed by [E:F]. This must be placed in W as follows: [C : *]. |
| 6 | `BL` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds on the unknowns X and Y. The first NVARS entries of IND(*), BL(*) and BU(*) specify bounds on X; the next MCON entries specify bounds on Y. 1. For IND(J)=1, require X(J). ge. |
| 7 | `BU` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds on the unknowns X and Y. The first NVARS entries of IND(*), BL(*) and BU(*) specify bounds on X; the next MCON entries specify bounds on Y. 1. For IND(J)=1, require X(J). ge. |
| 8 | `IND` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds on the unknowns X and Y. The first NVARS entries of IND(*), BL(*) and BU(*) specify bounds on X; the next MCON entries specify bounds on Y. 1. For IND(J)=1, require X(J). ge. |
| 9 | `IOPT` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | This is the array where the user can specify nonstandard options for DBOCLS( ). Most of the time this feature can be ignored by setting the input value IOPT(1)=99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number Brief Statement of Purpose 1 Return to user for accumulation of blocks of least squares equations. The values of IOPT(*) are changed with this option. |
| 10 | `X` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | le. BU(J); IF J. gt. NCOLS, Y(J-NCOLS). ge. BL(J) and Y(J-NCOLS). |
| 11 | `RNORMC` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | The array X(*) contains a solution (if MODE. ge. 0 or. eq. -22) for the constrained least squares problem. The value RNORMC is the minimum residual vector length for the constraints C*X - Y = 0. |
| 12 | `RNORM` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Writable output for the Euclidean length of the final residual vector of the constrained least-squares problem. |
| 13 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The sign of MODE determines whether the subprogram has completed normally, or encountered an error condition or abnormal status. A value of MODE. ge. 0 signifies that the subprogram has completed normally. The value of mode (. 0) is the number of variables in an active status: not at a bound nor at the value zero, for the case of free variables. |
| 14 | `RW` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Writable real workspace with at least `6*NCOLS + 5*MCON` elements. It is scratch storage for the bounded constrained solve and native code retains no pointer. |
| 15 | `IW` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | These are working arrays. (normally the user can ignore the contents of these arrays. ) IOPT(*) CONTENTS The option array allows a user to modify some internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `1` | , require X(J) .ge. BL(J); |
| `IND` | `>0` | NCOLS, Y(J-NCOLS) .ge. BL(J). (the value of BU(J) is not used.) |
| `IND` | `2` | , require X(J) .le. BU(J); |
| `IND` | `>0` | NCOLS, Y(J-NCOLS) .le. BU(J). (the value of BL(J) is not used.) |
| `IND` | `3` | , require X(J) .ge. BL(J) and (upper and lower bounds) the condition BL(J) .gt. BU(J) |
| `IND` | `>0` | NCOLS, will be changed. Significant changes mean that the constraints are infeasible. (Users must make this decision themselves.) NCOLS, define a region such that the perturbed problem is feasible. If users know that their problem is feasible, this step can be skipped by using option number 8 described below. See IOPT(*) description. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::dbocls`. Native symbol: `dbocls_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::dbocls`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
