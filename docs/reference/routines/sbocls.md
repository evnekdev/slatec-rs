# SBOCLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the bounded and constrained least squares problem consisting of solving the equation E*X = F (in the least squares sense) subject to the linear constraints C*X = Y.

## Description

This subprogram solves the bounded and constrained least squares problem. The problem statement is:

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
- GAMS classifications: `K1A2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/sbocls.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sbocls.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sbocls.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sbocls.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SBOCLS](https://www.netlib.org/slatec/src/sbocls.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 2; dimensions (MDW, *) | *] followed by [E:F].  This must be placed in W as follows: [C  :  *] [       ] [E  :  F] The (*) after C indicates that this data can be undefined. The matrix [E:F] has MROWS rows and NCOLS+1 columns. The matrix C is F] follows in rows MCON+1 through MCON+MROWS of W(*,*). The vector F is placed in rows MCON+1 through MCON+MROWS, column NCOLS+1. The one block of rows at a time. See IOPT(*) contents, option number 1, for further details. The row dimension, MDW, of the If using option 1, (I1) MUST BE .GE. THE NUMBER OF EFFECTIVE ROWS=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 (I1) MUST BE .GE. NCOLS+ |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be positive; the value of MCON must be nonnegative. An exception to this occurs when using option 1 for accumulation of blocks of equations. In that case MROWS is an OUTPUT variable only, and the matrix data for [E:F] is placed in .ge. MCON + max(max. number of rows accumulated, NCOLS) + 1. If using option 8, .ge. MCON + MROWS. Else .ge. MCON + max(MROWS, NCOLS). Other values are errors, but this is checked only when using option=2.  The value of MROWS is an output parameter when using option number 1 for accumulating large blocks of least squares equations before solving the problem. See IOPT(*) contents for details about option 1. MCON=(I2). IN ABOVE MESSAGE, I1=         2 IN ABOVE MESSAGE, I2=         1 (I1) MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 |
| 3 | `MCON` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | RW(6*NCOLS+5*MCON) INTEGER IND(NCOLS+MCON), IOPT(17+NI), IW(2*(NCOLS+MCON)) (here NX=number of extra locations required for the options; NX=0 if no options are in use. Also NI=number of extra locations for options 1-9.) F] follows in rows MCON+1 through MCON+MROWS of W(*,*). The vector F is placed in rows MCON+1 through MCON+MROWS, column NCOLS+1. The (I2). IN ABOVE MESSAGE, I1=         2 IN ABOVE MESSAGE, I2=         3 (I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 (I1) MUST BE NONNEGATIVE. IN ABOVE MESSAGE, I1=        -1 |
| 4 | `MROWS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of rows in the least-squares block `[E:F]`. With accumulation option `IOPT(1)`, this is a writable returned count of accumulated rows; otherwise it is an input count used with `MDW`, `MCON`, and `NCOLS`. |
| 5 | `NCOLS` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | RW(6*NCOLS+5*MCON) INTEGER IND(NCOLS+MCON), IOPT(17+NI), IW(2*(NCOLS+MCON)) (here NX=number of extra locations required for the options; NX=0 if no options are in use. Also NI=number of extra locations for options 1-9.) must be positive; the value of MCON must be nonnegative. An exception to this occurs when using option 1 for accumulation of blocks of equations. In that case MROWS is an OUTPUT variable only, and the matrix data for [E:F] is placed in NCOLS) .ge. BL(J). (the value of BU(J) is not used.) NCOLS) .le. BU(J). (the value of BL(J) is not used.) NCOLS) .ge. BL(J) and .le. BU(J). 1) for passing scaling data. 1) for passing scaling data. 1) for passing scaling data. The user must manage the allocation of these locations. The user must manage the allocation of these locations. The user must manage the allocation of these locations. 1 1 1 - - - This option allows the user to solve problems with a large number This option allows the user to solve problems with a large number This option allows the user to solve problems with a large number of rows compared to the number of variables. The idea is that the of rows compared to the number of variables. The idea is that the of rows compared to the number of variables. The idea is that the subprogram returns to the user (perhaps many times) and receives subprogram returns to the user (perhaps many times) and receives subprogram returns to the user (perhaps many times) and receives new least squares equations from the calling program unit. new least squares equations from the calling program unit. new least squares equations from the calling program unit. Eventually the user signals "that's all" and a solution is then Eventually the user signals "that's all" and a solution is then Eventually the user signals "that's all" and a solution is then computed. The value of MROWS is an output variable when this computed. The value of MROWS is an output variable when this computed. The value of MROWS is an output variable when this option is used. Its value is always in the range 0 .le. MROWS option is used. Its value is always in the range 0 .le. MROWS option is used. Its value is always in the range 0 .le. MROWS .le. NCOLS+1. It is the number of rows after the .le. NCOLS+1. It is the number of rows after the .le. NCOLS+1. It is the number of rows after the triangularization of the entire set of equations. If LP is the triangularization of the entire set of equations. If LP is the triangularization of the entire set of equations. If LP is the processing pointer for IOPT(*), the usage for the sequential processing pointer for IOPT(*), the usage for the sequential processing pointer for IOPT(*), the usage for the sequential processing of blocks of equations is processing of blocks of equations is processing of blocks of equations is 1) 1) 1) = Positive scale factors for cols. of E. = Positive scale factors for cols. of E. = Positive scale factors for cols. of E. . . . CALL SBOCLS( ) CALL SBOCLS( ) CALL SBOCLS( ) Use of this option adds 2 to the required length of IOPT(*) Use of this option adds 2 to the required length of IOPT(*) Use of this option adds 2 to the required length of IOPT(*) and NCOLS to the required length of X(*). and NCOLS to the required length of X(*). and NCOLS to the required length of X(*). 5 5 5 - - - This option allows the user to provide an option array to the This option allows the user to provide an option array to the This option allows the user to provide an option array to the low-level subprogram SBOLS( ). If LP is the processing pointer low-level subprogram SBOLS( ). If LP is the processing pointer low-level subprogram SBOLS( ). If LP is the processing pointer for IOPT(*), for IOPT(*), for IOPT(*), 99 CALL SBOCLS( ) CAUTION: Misuse of this option can yield some very hard-to-find bugs. Use it with care. It is intended to be used for passing option arrays to other subprograms. 8 - This option allows the user to suppress the algorithmic feature (I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 PROVIDED COLUMN SCALING MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 (I1) THE NO. OF VARIABLES MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 |
| 6 | `BL` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | BU(J)= constraining value.) are not used.) Values other than 1,2,3 or 4 for IND(J) are errors. In the case (I1) |
| 7 | `BU` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are not used.) Values other than 1,2,3 or 4 for IND(J) are errors. In the case (I1) |
| 8 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | 1, require X(J) .ge. BL(J); 2, require X(J) .le. BU(J); 3, require X(J) .ge. BL(J) and NCOLS) are required. 3 (upper and lower bounds) the condition BL(J) .gt. BU(J) is  an  error.   The values BL(J), BU(J), J .gt. NCOLS, will be changed.  Significant changes mean that the constraints are infeasible.  (Users must make this decision themselves.) The new values for BL(J), BU(J), J .gt. NCOLS, define a region such that the perturbed problem is feasible.  If users know that their problem is feasible, this step can be skipped by using option number 8 described below. See IOPT(*) description. (I1) 4. IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         0 |
| 9 | `IOPT` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | 99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number   Brief Statement of Purpose CONTENTS 1 Move block of equations to W(*,*) starting at the first row of W(*,*). # of rows in the block; user defined The user now calls SBOCLS( ) in a loop. The value of IOPT(LP+1) directs the user's action. The value of IOPT(LP+2) points to where the subsequent rows are to be placed in W(*,*). Both of these values are first defined in the subprogram. The user changes the value of IOPT(LP+1) (to 2) as a signal that all of the rows have been processed. .<LOOP . CALL SBOCLS( ) . IF(IOPT(LP+1) .EQ. 1) THEN # OF ROWS IN THE NEW BLOCK; USER DEFINED .    PLACE NEW BLOCK OF IOPT(LP+3) ROWS IN .    W(*,*) STARTING AT ROW MCON + IOPT(LP+2). . .    IF( THIS IS THE LAST BLOCK OF EQUATIONS ) THEN 2 2 Row dimension of W(*,*) Col. dimension of W(*,*) Dimensions of BL(*),BU(*),IND(*) Dimension of X(*) Dimension of RW(*) Dimension of IW(*) Dimension of IOPT(*) . CALL SBOCLS( ) Use of this option adds 8 to the required length of IOPT(*). 3 - This option can change the type of scaling for the data matrix. Nominally each nonzero column of the matrix is scaled so that the magnitude of its largest entry is equal to the value ONE. If LP is the processing pointer for IOPT(*), 3 1,2 or 3 1= Nominal scaling as noted; 2= Each nonzero column scaled to have length ONE; 3= Identity scaling; scaling effectively suppressed. . CALL SBOCLS( ) Use of this option adds 2 to the required length of IOPT(*). 4 - This options allows the user to provide arbitrary (positive) column scaling for the matrix. If LP is the processing pointer for IOPT(*), 4 IOFF 5 Position in IOPT(*) where option array data for SBOLS( ) begins. . CALL SBOCLS( ) Use of this option adds 2 to the required length of IOPT(*). 6 - This option allows the user to provide an option array to the low-level subprogram SBOLSM( ). If LP is the processing pointer for IOPT(*), 6 Position in IOPT(*) where option array data for SBOLSM( ) begins. . CALL SBOCLS( ) Use of this option adds 2 to the required length of IOPT(*). 7 - Move the processing pointer (either forward or backward) to the location IOPT(LP+1). The processing pointer moves to locations LP+2 if option number 7 is used with the value -7.  For example to skip over locations 3,...,NCOLS+2, 7 NCOLS+3 3,...,NCOLS+2 are not defined here.) 99 CALL SBOCLS( ) CAUTION: Misuse of this option can yield some very hard-to-find bugs. Use it with care. It is intended to be used for passing option arrays to other subprograms. 8 - This option allows the user to suppress the algorithmic feature 8 . CALL SBOCLS( ) Use of this option adds 1 to the required length of IOPT(*). 9 - This option allows the user to suppress the pretriangularizing step of the least squares matrix that is done within SBOCLS( ). This is primarily a means of enhancing the subprogram efficiency and has little effect on accuracy. To suppress the step, set: 9 . CALL SBOCLS( ) Use of this option adds 1 to the required length of IOPT(*). 99 -- There are no more options to change. Only option numbers -99, -9,-8,...,-1, 1,2,...,9, and 99 are permitted. Other values are errors. Options -99,-1,...,-9 mean that the respective options 99,1,...,9 are left at their default values. An example is the option to suppress the preprocessing of constraints: 8 Option is recognized but not changed 99 CALL SBOCLS( ) (I1) MUST BE .GE. THE REQD. LEN.=(I2). IN ABOVE MESSAGE, I1=        16 IN ABOVE MESSAGE, I2=        18 |
| 10 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | F  (in the least squares sense) subject to the linear constraints Y. F (least squares sense), subject to constraints Y. In this formulation both X and Y are unknowns, and both may have bounds on any of their components.  This formulation of the problem allows the user to have equality and inequality constraints as well as simple bounds on the solution components. F Y, where E is MROWS by NCOLS, C is MCON by NCOLS. The user must have dimension statements of the form DIMENSION W(MDW,NCOLS+MCON+1), BL(NCOLS+MCON), BU(NCOLS+MCON), RW(6*NCOLS+5*MCON) INTEGER IND(NCOLS+MCON), IOPT(17+NI), IW(2*(NCOLS+MCON)) (here NX=number of extra locations required for the options; NX=0 if no options are in use. Also NI=number of extra locations for options 1-9.) .le. BU(J); NCOLS) are required. 22) for the constrained least squares problem. The value RNORMC is the Y = 0. The value RNORM is the minimum residual vector length for the 1) for passing scaling data. 1) for passing scaling data. The user must manage the allocation of these locations. The user must manage the allocation of these locations. 1 1 - - This option allows the user to solve problems with a large number This option allows the user to solve problems with a large number of rows compared to the number of variables. The idea is that the of rows compared to the number of variables. The idea is that the subprogram returns to the user (perhaps many times) and receives subprogram returns to the user (perhaps many times) and receives new least squares equations from the calling program unit. new least squares equations from the calling program unit. Eventually the user signals "that's all" and a solution is then Eventually the user signals "that's all" and a solution is then computed. The value of MROWS is an output variable when this computed. The value of MROWS is an output variable when this option is used. Its value is always in the range 0 .le. MROWS option is used. Its value is always in the range 0 .le. MROWS .le. NCOLS+1. It is the number of rows after the .le. NCOLS+1. It is the number of rows after the triangularization of the entire set of equations. If LP is the triangularization of the entire set of equations. If LP is the processing pointer for IOPT(*), the usage for the sequential processing pointer for IOPT(*), the usage for the sequential processing of blocks of equations is processing of blocks of equations is 1) 1) = Positive scale factors for cols. of E. = Positive scale factors for cols. of E. . . CALL SBOCLS( ) CALL SBOCLS( ) Use of this option adds 2 to the required length of IOPT(*) Use of this option adds 2 to the required length of IOPT(*) and NCOLS to the required length of X(*). and NCOLS to the required length of X(*). 5 5 - - This option allows the user to provide an option array to the This option allows the user to provide an option array to the low-level subprogram SBOLS( ). If LP is the processing pointer low-level subprogram SBOLS( ). If LP is the processing pointer for IOPT(*), for IOPT(*), Y and resolves infeasibilities. The steps normally done are to solve Y = 0 in a least squares sense using the stated bounds on C*X is computed using the solution X obtained. Finally the stated bounds for Y are (I1) MUST BE .GE. THE REQD.LENGTH=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 PROVIDED COLUMN SCALING MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 |
| 11 | `RNORMC` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | 0, but in the case of inconsistent constraints this value will be nonzero. The values of X are returned in the first NVARS entries of X(*). The values of Y are returned in the last MCON entries of X(*). |
| 12 | `RNORM` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Writable output for the Euclidean length of the final residual vector of the constrained least-squares problem. |
| 13 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 22) for the constrained least squares problem. The value RNORMC is the 22. 41 to -47. The printed error message tells how long the dimension should be. If LP is the processing pointer for IOPT(*), |
| 14 | `RW` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Writable real workspace with at least `6*NCOLS + 5*MCON` elements. It is scratch storage for the bounded constrained solve and native code retains no pointer. |
| 15 | `IW` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | (I1) MUST BE .GE. 2*NCOLS+2*MCON=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         4 |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

SBOLSM(), and SBOLS().  An approximate solution will be returned .<END LOOP Use of this option adds 4 to the required length of IOPT(*). WARNING IN... WARNING IN... WARNING IN... WARNING IN... SBOCLS(). THE . WARNING IN... WARNING IN... SBOCLS(). ISCALE OPTION=(I1) MUST BE 1-3. IN ABOVE MESSAGE, I1=         0 WARNING IN... WARNING IN... SBOCLS(). EACH PROVIDED COL. SCALE FACTOR MUST BE POSITIVE. COMPONENT (I1) NOW = (R1). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, R1=    0. WARNING IN... SBOCLS(). THE OPTION NUMBER=(I1) IS NOT DEFINED. IN ABOVE MESSAGE, I1=      1001 WARNING IN... WARNING IN... WARNING IN... WARNING IN... WARNING IN... WARNING IN... SBOCLS(). FOR J=(I1), BOUND BL(J)=(R1) IS .GT. BU(J)=(R2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, R1=     .1000000000E+01 IN ABOVE MESSAGE, R2=    0. LINEAR CONSTRAINTS, SNLA REPT. SAND82-1517, AUG. (1982).

### Storage and workspace requirements

`W`: *] followed by [E:F].  This must be placed in W as follows: [C  :  *] [       ] [E  :  F] The (*) after C indicates that this data can be undefined. The matrix [E:F] has MROWS rows and NCOLS+1 columns. The matrix C is F] follows in rows MCON+1 through MCON+MROWS of W(*,*). The vector F is placed in rows MCON+1 through MCON+MROWS, column NCOLS+1. The one block of rows at a time. See IOPT(*) contents, option number 1, for further details. The row dimension, MDW, of the If using option 1, (I1) MUST BE .GE. THE NUMBER OF EFFECTIVE ROWS=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         2 (I1) MUST BE .GE. NCOLS+

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::sbocls`. Native symbol: `sbocls_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::sbocls`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
