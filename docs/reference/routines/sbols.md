# SBOLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the problem E*X = F (in the least squares sense) with bounds on selected X values.

## Description

The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Here NI=number of extra locations required for options 1-6; NI=0 for no options.)

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
- Safe Rust paths: `slatec::bounded_least_squares::solve_bounded_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/sbols.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sbols.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sbols.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sbols.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [SBOLS](https://www.netlib.org/slatec/src/sbols.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 2; dimensions (MDW, *) | Workspace argument classified by fixed-form executable read/write analysis. |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `MROWS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `NCOLS` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `BL` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `BU` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Status argument classified by fixed-form executable read/write analysis. |
| 8 | `IOPT` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 9 | `X` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 22) for the constrained least squares problem. The value RNORM is the minimum residual vector length. ---- MODE ---- The sign of MODE determines whether the subprogram has completed |
| 10 | `RNORM` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 22) for the constrained least squares problem. The value RNORM is the minimum residual vector length. ---- MODE ---- The sign of MODE determines whether the subprogram has completed |
| 12 | `RW` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 13 | `IW` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

to this when using option 1 for accumulation of blocks of equations. In that case MROWS is an OUTPUT variable ONLY, and the matrix data for [E:F] is placed in W(*,*), one block of rows at a time.  MROWS contains the number of rows in the matrix after triangularizing several blocks of equations. This is an OUTPUT parameter ONLY when option 1 is used. See IOPT(*) CONTENTS for details about option 1. ------------------ BL(*),BU(*),IND(*) ------------------ These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the IND(J)=3 (upper and lower bounds) the condition BL(J) .gt. BU(J) ------- IOPT(*) ------- This is the array where the user can specify nonstandard options for SBOLSM( ). Most of the time this feature can be ignored by setting the input value IOPT(1)=99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number   Brief Statement of Purpose ------ ------   ----- --------- -- ------- 1         Return to user for accumulation of blocks of least squares equations. 2         Check lengths of all arrays used in the subprogram. 3         Standard scaling of the data matrix, E. 4         User provides column scaling for matrix E. 5         Provide option array to the low-level subprogram SBOLSM( ). 6         Move the IOPT(*) processing pointer. 99         No more options to change. ---- X(*) ---- This array is used to pass data associated with option 4. Ignore this parameter if this option is not used. Otherwise see below: IOPT(*) CONTENTS. value of MODE .ge. 0 signifies that the subprogram has completed normally. The value of MODE (.GE. 0) is the number of variables in an active status: not at a bound nor at the value ZERO, for the case of free variables. A negative value of MODE will be one of the cases -37,-36,...,-22, or -17,...,-2. Values .lt. -1 correspond to an abnormal completion of the subprogram. To MESSAGES for SBOLS( ). AN approximate solution will be returned to the user only when max. iterations is reached, MODE=-22. Values for MODE=-37,...,-22 come from the low-level subprogram documentation for SBOLSM(). ----------- RW(*),IW(*) ----------- These are working arrays with 5*NCOLS and 2*NCOLS entries. (normally the user can ignore the contents of these arrays, but they must be dimensioned properly.) IOPT(*) CONTENTS ------- -------- The option array allows a user to modify internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1. This value is updated as each option is processed. At the pointer position the option number is extracted and used for locating other information that allows for options to be changed. The portion of the array IOPT(*) that is used for each option is fixed; the user and the subprogram both know how many locations done by the subprogram on the contents of the option array. Nevertheless it is still possible to give the subprogram optional input that is meaningless. For example option 4 uses the locations X(NCOLS+IOFF),...,X(NCOLS+IOFF+NCOLS-1) for passing scaling data. The user must manage the allocation of these locations. .<END LOOP Use of this option adds 4 to the required length of IOPT(*). message tells how long the dimension should be. If LP is the processing pointer for IOPT(*), IOPT(LP)=2 IOPT(LP+1)=Row dimension of W(*,*) IOPT(LP+2)=Col. dimension of W(*,*) IOPT(LP+3)=Dimensions of BL(*),BU(*),IND(*) IOPT(LP+4)=Dimension of X(*) IOPT(LP+5)=Dimension of RW(*) IOPT(LP+6)=Dimension of IW(*) IOPT(LP+7)=Dimension of IOPT(*) . CALL SBOLS() Use of this option adds 8 to the required length of IOPT(*). that the respective options 99,1,...,6 are left at their default values. An example is the option to modify the (rank) tolerance: IOPT(1)=-3 Option is recognized but not changed IOPT(2)=2  Scale nonzero cols. to have length ONE IOPT(3)=99 ----- -------- --- ------- WARNING IN... SBOLS(). MDW=(I1) MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). NCOLS=(I1) THE NO. OF VARIABLES MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). FOR J=(I1), IND(J)=(I2) MUST BE 1-4. IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). FOR J=(I1), BOUND BL(J)=(R1) IS .GT. BU(J)=(R2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, R1=    0. IN ABOVE MESSAGE, R2=    ABOVE MESSAGE, I1=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). ISCALE OPTION=(I1) MUST BE 1-3. IN ABOVE MESSAGE, I1=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). OFFSET PAST X(NCOLS) (I1) FOR USER-PROVIDED  COLUMN SCALING MUST BE POSITIVE. IN ABOVE MESSAGE, I1=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). EACH PROVIDED COL. SCALE FACTOR MUST BE POSITIVE. COMPONENT (I1) NOW = (R1). IN ABOVE MESSAGE, I1=        ND. .LE. MDW=(I2). IN ABOVE MESSAGE, I1=         1 IN ABOVE MESSAGE, I2=         0 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS().THE ROW DIMENSION OF W(,)=(I1) MUST BE .GE.THE NUMBER OF ROWS= (I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         1 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). THE COLUMN DIMENSION OF W(,)=(I1) MUST BE .GE. NCOLS+1=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         2 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS().THE DIMENSIONS OF THE ARRAYS BL(),BU(), AND IND()=(I1) MUST BE .GE. NCOLS=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         1 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). THE DIMENSION OF X()=(I1) MUST BE .GE. THE REQD. LENGTH=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         2 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS(). THE DIMENSION OF RW()=(I1) MUST BE .GE. 5*NCOLS=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         3 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS() THE DIMENSION OF IW()=(I1) MUST BE .GE. 2*NCOLS=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         2 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.) WARNING IN... SBOLS() THE DIMENSION OF IOPT()=(I1) MUST BE .GE. THE REQD. LEN.=(I2). IN ABOVE MESSAGE, I1=         0 IN ABOVE MESSAGE, I2=         1 (NORMALLY A RETURN TO THE USER TAKES PLACE FOLLOWING THIS MESSAGE.)

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::sbols`. Native symbol: `sbols_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::sbols`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::bounded_least_squares::solve_bounded_least_squares_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
