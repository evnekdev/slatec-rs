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

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DBOCLS](https://www.netlib.org/slatec/src/dbocls.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDW, *) | [       ] [E  :  F] The (*) after C indicates that this data can be undefined. The matrix [E:F] has MROWS rows and NCOLS+1 columns. The matrix C is placed in the first MCON rows of W(*,*) while [E:F] follows in rows MCON+1 through MCON+MROWS of W(*,*). The vector F is placed in rows MCON+1 through MCON+MROWS, column NCOLS+1. The values of MDW and NCOLS must be positive; the value of MCON must be nonnegative. An exception to this occurs when using option 1 for accumulation of blocks of equations. In that case MROWS is an OUTPUT variable only, and the matrix data for [E:F] is placed in W(*,*), one block of rows at a time. See IOPT(*) contents, option number 1, for further details. The row dimension, MDW, of the array W(*,*) must satisfy the inequality: If using option 1, MDW .ge. MCON + max(max. number of rows accumulated, NCOLS) + 1. If using option 8, MDW .ge. MCON + MROWS. Else MDW .ge. MCON + max(MROWS, NCOLS). |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `MCON` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `MROWS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `NCOLS` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `BL` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `BU` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Status argument classified by fixed-form executable read/write analysis. |
| 9 | `IOPT` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `X` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 22) for the constrained least squares problem. The value RNORMC is the Y = 0. The value RNORM is the minimum residual vector length for the |
| 11 | `RNORMC` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | 0, but in the case of inconsistent constraints this value will be nonzero. The values of X are returned in the first NVARS entries of X(*). The values of Y are returned in the last MCON entries of X(*). ---- MODE ---- The sign of MODE determines whether the subprogram has completed |
| 12 | `RNORM` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 22) for the constrained least squares problem. The value RNORMC is the |
| 14 | `RW` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 15 | `IW` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

option=2.  The value of MROWS is an output parameter when using option number 1 for accumulating large blocks of least squares equations before solving the problem. See IOPT(*) contents for details about option 1. ------------------ BL(*),BU(*),IND(*) ------------------ These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the IND(J)=3 (upper and lower bounds) the condition BL(J) .gt. BU(J) changed.  Significant changes mean that the constraints are infeasible.  (Users must make this decision themselves.) The new values for BL(J), BU(J), J .gt. NCOLS, define a region such that the perturbed problem is feasible.  If users know that their problem is feasible, this step can be skipped by using option number 8 described below. See IOPT(*) description. ------- IOPT(*) ------- This is the array where the user can specify nonstandard options for DBOCLS( ). Most of the time this feature can be ignored by setting the input value IOPT(1)=99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number   Brief Statement of Purpose ------ ------   ----- --------- -- ------- 1         Return to user for accumulation of blocks of least squares equations.  The values of IOPT(*) are changed with this option. The changes are updates to pointers for placing the rows of equations into position for processing. 2         Check lengths of all arrays used in the subprogram. 3         Column scaling of the data matrix, [C]. [E] 4         User provides column scaling for matrix [C]. [E] 5         Provide option array to the low-level subprogram SBOLS( ). 6         Provide option array to the low-level subprogram SBOLSM( ). 7         Move the IOPT(*) processing pointer. 8         Do not preprocess the constraints to resolve infeasibilities. 9         Do not pretriangularize the least squares matrix. 99         No more options to change. ---- X(*) ---- This array is used to pass data associated with options 4,5 and 6. Ignore this parameter (on input) if no options are used. Otherwise see below: IOPT(*) CONTENTS. value of MODE .ge. 0 signifies that the subprogram has completed normally. The value of mode (.ge. 0) is the number of variables in an active status: not at a bound nor at the value zero, for the case of free variables. A negative value of MODE will be one of the cases (-57)-(-41), (-37)-(-22), (-19)-(-2). Values .lt. -1 correspond to an abnormal completion of the subprogram. These SBOLSM(), and SBOLS().  An approximate solution will be returned to the user only when max. iterations is reached, MODE=-22. ----------- RW(*),IW(*) ----------- These are working arrays.  (normally the user can ignore the contents of these arrays.) IOPT(*) CONTENTS ------- -------- The option array allows a user to modify some internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1. At the pointer position the option number is extracted and used for locating other information that allows for options to be changed. The portion of the array IOPT(*) that is used for each option is fixed; the user and the subprogram both know how many locations are needed for each option. The value of LP is updated for each option based on the amount of storage in IOPT(*) that is subprogram on the contents of the option array. Nevertheless it is still possible to give the subprogram optional input that is meaningless. For example option 4 uses the locations X(NCOLS+IOFF),...,X(NCOLS+IOFF+NCOLS-1) for passing scaling data. The user must manage the allocation of these locations. .<END LOOP Use of this option adds 4 to the required length of IOPT(*). message tells how long the dimension should be. If LP is the processing pointer for IOPT(*), IOPT(LP)=2 IOPT(LP+1)=Row dimension of W(*,*) IOPT(LP+2)=Col. dimension of W(*,*) IOPT(LP+3)=Dimensions of BL(*),BU(*),IND(*) IOPT(LP+4)=Dimension of X(*) IOPT(LP+5)=Dimension of RW(*) IOPT(LP+6)=Dimension of IW(*) IOPT(LP+7)=Dimension of IOPT(*) . CALL DBOCLS( ) Use of this option adds 8 to the required length of IOPT(*). that the respective options 99,1,...,9 are left at their default values. An example is the option to suppress the preprocessing of constraints: IOPT(1)=-8 Option is recognized but not changed IOPT(2)=99 CALL DBOCLS( ) ----- -------- --- -------- WARNING in... DBOCLS(). THE ROW DIMENSION OF W(,)=(I1) MUST BE .GE. THE NUMBER OF EFFECTIVE ROWS=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 WARNING IN... DBOCLS(). THE COLUMN DIMENSION OF W(,)=(I1) MUST BE .GE. NCOLS+ MCON+1=(I2). IN ABOVE MESSAGE, I1=         2 IN ABOVE MESSAGE, I2=         3 WARNING IN... DBOCLS(). THE DIMENSIONS OF THE ARRAYS BL(),BU(), AND IND()=(I1) MUST BE .GE. NCOLS+MCON=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 WARNING IN... DBOCLS(). THE DIMENSION OF X()=(I1) MUST BE .GE. THE REQD.LENGTH=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 WARNING IN... DBOCLS(). THE . DBOCLS() THE DIMENSION OF IW()=(I1) MUST BE .GE. 2*NCOLS+2*MCON=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         4 WARNING IN... DBOCLS(). THE DIMENSION OF IOPT()=(I1) MUST BE .GE. THE REQD. LEN.=(I2). IN ABOVE MESSAGE, I1=        16 IN ABOVE MESSAGE, I2=        18 WARNING IN... DBOCLS(). ISCALE OPTION=(I1) MUST BE 1-3. IN ABOVE MESSAGE, I1=         0 WARNING IN... DBOCLS(). OFFSET PAST X(NCOLS) (I1) FOR USER-PROVIDED COLUMN SCALING MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 WARNING IN... DBOCLS(). EACH PROVIDED COL. SCALE FACTOR MUST BE POSITIVE. COMPONENT (I1) NOW = (R1). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, R1=    0. WARNING IN... DBOCLS(). THE OPTION NUMBER=(I1) IS NOT DEFINED. IN ABOVE MESSAGE, I1=      1001 WARNING IN... DBOCLS(). NO. OF ROWS=(I1) MUST BE .GE. 0 .AND. .LE. MDW-MCON=(I2). IN ABOVE MESSAGE, I1=         2 IN ABOVE MESSAGE, I2=         1 WARNING IN... DBOCLS(). MDW=(I1) MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 WARNING IN... DBOCLS(). MCON=(I1) MUST BE NONNEGATIVE. IN ABOVE MESSAGE, I1=        -1 WARNING IN... DBOCLS(). NCOLS=(I1) THE NO. OF VARIABLES MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 WARNING IN... DBOCLS(). FOR J=(I1), IND(J)=(I2) MUST BE 1-4. IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         0 WARNING IN... DBOCLS(). FOR J=(I1), BOUND BL(J)=(R1) IS .GT. BU(J)=(R2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, R1=     .1000000000E+01 IN ABOVE MESSAGE, R2=    0. LINEAR CONSTRAINTS, SNLA REPT. SAND82-1517, AUG., (1982).

### Storage and workspace requirements

`W`: [       ] [E  :  F] The (*) after C indicates that this data can be undefined. The matrix [E:F] has MROWS rows and NCOLS+1 columns. The matrix C is placed in the first MCON rows of W(*,*) while [E:F] follows in rows MCON+1 through MCON+MROWS of W(*,*). The vector F is placed in rows MCON+1 through MCON+MROWS, column NCOLS+1. The values of MDW and NCOLS must be positive; the value of MCON must be nonnegative. An exception to this occurs when using option 1 for accumulation of blocks of equations. In that case MROWS is an OUTPUT variable only, and the matrix data for [E:F] is placed in W(*,*), one block of rows at a time. See IOPT(*) contents, option number 1, for further details. The row dimension, MDW, of the array W(*,*) must satisfy the inequality: If using option 1, MDW .ge. MCON + max(max. number of rows accumulated, NCOLS) + 1. If using option 8, MDW .ge. MCON + MROWS. Else MDW .ge. MCON + max(MROWS, NCOLS).

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
