# DBOLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the problem E*X = F (in the least squares sense) with bounds on selected X values.

## Description

All INPUT and OUTPUT real variables are DOUBLE PRECISION **** The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (Here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Here NI=number of extra locations required for options 1-6; NI=0 for no options.)

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
- Safe Rust paths: `slatec::bounded_least_squares::solve_bounded_least_squares`

## Providers

- Canonical provider: `main-src/src/dbols.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbols.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbols.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbols.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [DBOLS](https://www.netlib.org/slatec/src/dbols.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDW, *) | The array W(*,*) contains the matrix [E:F] on entry. The matrix [E:F] has MROWS rows and NCOLS+1 columns. This data is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW. ge. MROWS. |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input leading dimension of `W`. It must satisfy `MDW >= MROWS`; smaller values are input errors. |
| 3 | `MROWS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The array W(*,*) contains the matrix [E:F] on entry. The matrix [E:F] has MROWS rows and NCOLS+1 columns. This data is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW. ge. MROWS. |
| 4 | `NCOLS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The array W(*,*) contains the matrix [E:F] on entry. The matrix [E:F] has MROWS rows and NCOLS+1 columns. This data is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW. ge. MROWS. |
| 5 | `BL` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds. 1. For IND(J)=1, require X(J). ge. BL(J). |
| 6 | `BU` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds. 1. For IND(J)=1, require X(J). ge. BL(J). |
| 7 | `IND` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds. 1. For IND(J)=1, require X(J). ge. BL(J). |
| 8 | `IOPT` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | This is the array where the user can specify nonstandard options for DBOLSM( ). Most of the time this feature can be ignored by setting the input value IOPT(1)=99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number Brief Statement of Purpose 1 Return to user for accumulation of blocks of least squares equations. 2 Check lengths of all arrays used in the subprogram. |
| 9 | `X` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | le. BU(J). 4. For IND(J)=4, no bounds on X(J) are required. (the values of BL(J) and BU(J) are not used. ) Values other than 1,2,3 or 4 for IND(J) are errors. |
| 10 | `RNORM` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Writable output for the minimum residual-vector length. It is meaningful when the returned `MODE` reports a solution. |
| 11 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The sign of MODE determines whether the subprogram has completed normally, or encountered an error condition or abnormal status. A value of MODE. ge. 0 signifies that the subprogram has completed normally. The value of MODE (. GE. |
| 12 | `RW` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These are working arrays with 5*NCOLS and 2*NCOLS entries. (normally the user can ignore the contents of these arrays, but they must be dimensioned properly. ) IOPT(*) CONTENTS The option array allows a user to modify internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1. |
| 13 | `IW` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | These are working arrays with 5*NCOLS and 2*NCOLS entries. (normally the user can ignore the contents of these arrays, but they must be dimensioned properly. ) IOPT(*) CONTENTS The option array allows a user to modify internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `1` | , require X(J) .ge. BL(J). (the value of BU(J) is not used.) |
| `IND` | `2` | , require X(J) .le. BU(J). (the value of BL(J) is not used.) |
| `IND` | `3` | , require X(J) .ge. BL(J) and (upper and lower bounds) the condition BL(J) .gt. BU(J) is an error. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::dbols`. Native symbol: `dbols_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::dbols`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::bounded_least_squares::solve_bounded_least_squares`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
