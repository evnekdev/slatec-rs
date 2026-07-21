# DBOLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the problem E*X = F (in the least squares sense) with bounds on selected X values.

## Description

All INPUT and OUTPUT real variables are DOUBLE PRECISION **** The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS),

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

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DBOLS](https://www.netlib.org/slatec/src/dbols.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDW, *) | F] on entry. The matrix (I1) MUST BE .GE.THE NUMBER OF ROWS= (I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         1 (I1) MUST BE .GE. NCOLS+1=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         2 |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (I1) MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 |
| 3 | `MROWS` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW .ge. MROWS. Other values of MDW are errors. The values of MROWS and NCOLS must be positive. Other values are errors. There is an exception to this when using option 1 for accumulation of blocks of equations. In that case MROWS is an OUTPUT variable ONLY, and the matrix data for [E:F] is placed in W(*,*), one block of rows at a time.  MROWS contains the number of rows in the matrix after triangularizing several blocks of equations. This is an OUTPUT parameter ONLY when option 1 is used. See IOPT(*) CONTENTS for details about option 1. is an OUTPUT variable when this option is used. Its value is equal to the number of rows after the triangularization of the entire set of equations. If LP is the processing pointer for IOPT(*), the usage for the sequential processing of blocks of equations is |
| 4 | `NCOLS` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (Here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Here NI=number of extra locations required for options 1-6; NI=0 for no options.) is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW .ge. MROWS. Other values of MDW are errors. The values of MROWS and NCOLS must be positive. Other values are errors. There is an exception to this when using option 1 for accumulation of blocks of equations. In that case MROWS is an OUTPUT variable ONLY, and the matrix data for [E:F] is placed in W(*,*), one block of rows at a time.  MROWS contains the number of rows in the matrix after triangularizing several blocks of equations. This is an OUTPUT parameter ONLY when option 1 is used. See IOPT(*) CONTENTS for details about option 1. 1) for passing 1) for passing 1) for passing scaling data. The user must manage the allocation of these scaling data. The user must manage the allocation of these scaling data. The user must manage the allocation of these locations. locations. locations. 1 1 1 - - - This option allows the user to solve problems with a large number This option allows the user to solve problems with a large number This option allows the user to solve problems with a large number of rows compared to the number of variables. The idea is that the of rows compared to the number of variables. The idea is that the of rows compared to the number of variables. The idea is that the subprogram returns to the user (perhaps many times) and receives subprogram returns to the user (perhaps many times) and receives subprogram returns to the user (perhaps many times) and receives new least squares equations from the calling program unit. new least squares equations from the calling program unit. new least squares equations from the calling program unit. Eventually the user signals "that's all" and then computes the Eventually the user signals "that's all" and then computes the Eventually the user signals "that's all" and then computes the solution with one final call to subprogram DBOLS( ). The value of solution with one final call to subprogram DBOLS( ). The value of solution with one final call to subprogram DBOLS( ). The value of is equal to the number of rows after the triangularization of the entire set of equations. If LP is the processing pointer for IOPT(*), the usage for the sequential processing of blocks of equations is 1) 1) 1) = Positive scale factors for cols. of E. = Positive scale factors for cols. of E. = Positive scale factors for cols. of E. . . . CALL DBOLS() CALL DBOLS() CALL DBOLS() Use of this option adds 2 to the required length of IOPT(*) and Use of this option adds 2 to the required length of IOPT(*) and Use of this option adds 2 to the required length of IOPT(*) and to the required length of X(*). 5 - This option allows the user to provide an option array to the low-level subprogram DBOLSM(). If LP is the processing pointer for IOPT(*), 99 CALL DBOLS() CAUTION: Misuse of this option can yield some very hard -to-find bugs.  Use it with care. 99 -- There are no more options to change. Only option numbers -99, -6,-5,...,-1, 1,2,...,6, and 99 are permitted. Other values are errors. Options -99,-1,...,-6 mean that the respective options 99,1,...,6 are left at their default values. An example is the option to modify the (rank) tolerance: (I1) THE NO. OF VARIABLES MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 PROVIDED  COLUMN SCALING MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 (I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         1 |
| 5 | `BL` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are not used.) Values other than 1,2,3 or 4 for IND(J) are errors. In the case (I1) MUST BE |
| 6 | `BU` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are not used.) Values other than 1,2,3 or 4 for IND(J) are errors. In the case (I1) MUST BE |
| 7 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | 1, require X(J) .ge. BL(J). (the value of BU(J) is not used.) 2, require X(J) .le. BU(J). (the value of BL(J) is not used.) 3, require X(J) .ge. BL(J) and 4, no bounds on X(J) are required. 3 (upper and lower bounds) the condition BL(J) .gt. BU(J) is an error. 4. IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         0 (I1) MUST BE |
| 8 | `IOPT` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | 99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number   Brief Statement of Purpose CONTENTS. CONTENTS 1 Move block of equations to W(*,*) starting at the first row of W(*,*). # of rows in the block; user defined The user now calls DBOLS( ) in a loop. The value of IOPT(LP+1) directs the user's action. The value of IOPT(LP+2) points to where the subsequent rows are to be placed in W(*,*). .<LOOP . CALL DBOLS() . IF(IOPT(LP+1) .EQ. 1) THEN # OF ROWS IN THE NEW BLOCK; USER DEFINED .    PLACE NEW BLOCK OF IOPT(LP+3) ROWS IN .    W(*,*) STARTING AT ROW IOPT(LP+2). . .    IF( THIS IS THE LAST BLOCK OF EQUATIONS ) THEN 2 2 Row dimension of W(*,*) Col. dimension of W(*,*) Dimensions of BL(*),BU(*),IND(*) Dimension of X(*) Dimension of RW(*) Dimension of IW(*) Dimension of IOPT(*) . CALL DBOLS() Use of this option adds 8 to the required length of IOPT(*). 3 - This option changes the type of scaling for the data matrix E. Nominally each nonzero column of E is scaled so that the magnitude of its largest entry is equal to the value ONE. If LP is the processing pointer for IOPT(*), 3 1,2 or 3 1= Nominal scaling as noted; 2= Each nonzero column scaled to have length ONE; 3= Identity scaling; scaling effectively suppressed. . CALL DBOLS() Use of this option adds 2 to the required length of IOPT(*). 4 - This option allows the user to provide arbitrary (positive) column scaling for the matrix E. If LP is the processing pointer for IOPT(*), 4 IOFF 5 Position in IOPT(*) where option array data for DBOLSM() begins. . CALL DBOLS() Use of this option adds 2 to the required length of IOPT(*). 6 - Move the processing pointer (either forward or backward) to the location IOPT(LP+1). The processing point is moved to entry 6 in IOPT(LP).  For example to skip over locations 3,...,NCOLS+2 of IOPT(*), 6 NCOLS+3 3,...,NCOLS+2 are not defined here.) 99 CALL DBOLS() CAUTION: Misuse of this option can yield some very hard -to-find bugs.  Use it with care. 99 -- There are no more options to change. Only option numbers -99, -6,-5,...,-1, 1,2,...,6, and 99 are permitted. Other values are errors. Options -99,-1,...,-6 mean that the respective options 99,1,...,6 are left at their default values. An example is the option to modify the (rank) tolerance: 3 Option is recognized but not changed 2  Scale nonzero cols. to have length ONE 99 (I1) MUST BE .GE. THE REQD. LEN.=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         1 |
| 9 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | F (in the least  squares  sense) with bounds on selected X values. RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (Here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Here NI=number of extra locations required for options 1-6; NI=0 for no options.) .le. BU(J). 22) for the constrained least squares problem. The value RNORM is the minimum residual vector length. 1) for passing 1) for passing scaling data. The user must manage the allocation of these scaling data. The user must manage the allocation of these locations. locations. 1 1 - - This option allows the user to solve problems with a large number This option allows the user to solve problems with a large number of rows compared to the number of variables. The idea is that the of rows compared to the number of variables. The idea is that the subprogram returns to the user (perhaps many times) and receives subprogram returns to the user (perhaps many times) and receives new least squares equations from the calling program unit. new least squares equations from the calling program unit. Eventually the user signals "that's all" and then computes the Eventually the user signals "that's all" and then computes the solution with one final call to subprogram DBOLS( ). The value of solution with one final call to subprogram DBOLS( ). The value of 1) 1) = Positive scale factors for cols. of E. = Positive scale factors for cols. of E. . . CALL DBOLS() CALL DBOLS() Use of this option adds 2 to the required length of IOPT(*) and Use of this option adds 2 to the required length of IOPT(*) and PROVIDED  COLUMN SCALING MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 (I1) MUST BE .GE. THE REQD. LENGTH=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         2 |
| 10 | `RNORM` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Writable output for the minimum residual-vector length. It is meaningful when the returned `MODE` reports a solution. |
| 11 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 22) for the constrained least squares problem. The value RNORM is the minimum residual vector length. 22. 37,...,-22 come from the low-level subprogram DBOLSM(). See the section ERROR MESSAGES for DBOLSM() in the documentation for DBOLSM(). 11 to -17. The printed error message tells how long the dimension should be. If LP is the processing pointer for IOPT(*), |
| 12 | `RW` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | (I1) MUST BE .GE. 5*NCOLS=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         3 |
| 13 | `IW` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | (I1) MUST BE .GE. 2*NCOLS=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         2 |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

.<END LOOP Use of this option adds 4 to the required length of IOPT(*). (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... DBOLS(). FOR J=(I1), BOUND BL(J)=(R1) IS .GT. BU(J)=(R2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, R1=    0. IN ABOVE MESSAGE, R2=    ABOVE MESSAGE, I1=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... DBOLS(). ISCALE OPTION=(I1) MUST BE 1-3. IN ABOVE MESSAGE, I1=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... DBOLS(). EACH PROVIDED COL. SCALE FACTOR MUST BE POSITIVE. COMPONENT (I1) NOW = (R1). IN ABOVE MESSAGE, I1=        ND. .LE. MDW=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.)

### Storage and workspace requirements

`W`: F] on entry. The matrix (I1) MUST BE .GE.THE NUMBER OF ROWS= (I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         1 (I1) MUST BE .GE. NCOLS+1=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         2

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
