# Purpose

All INPUT and OUTPUT real variables are DOUBLE PRECISION **** The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (Here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Here NI=number of extra locations required for options 1-6; NI=0 for no options.)

# Description

This canonical unsafe binding exposes original SLATEC routine `DBOLS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBOLS](https://www.netlib.org/slatec/src/dbols.f).

# Arguments

## `W`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (MDW, *).

The array W(*,*) contains the matrix \[E:F\] on entry. The matrix \[E:F\] has MROWS rows and NCOLS+1 columns. This data is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW. ge. MROWS.

## `MDW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input leading dimension of `W`. It must satisfy `MDW >= MROWS`; smaller values are input errors.

## `MROWS`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array W(*,*) contains the matrix \[E:F\] on entry. The matrix \[E:F\] has MROWS rows and NCOLS+1 columns. This data is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW. ge. MROWS.

## `NCOLS`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array W(*,*) contains the matrix \[E:F\] on entry. The matrix \[E:F\] has MROWS rows and NCOLS+1 columns. This data is placed in the array W(*,*) with E occupying the first NCOLS columns and the right side vector F in column NCOLS+1. The row dimension, MDW, of the array W(*,*) must satisfy the inequality MDW. ge. MROWS.

## `BL`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds. 1. For IND(J)=1, require X(J). ge. BL(J).

## `BU`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds. 1. For IND(J)=1, require X(J). ge. BL(J).

## `IND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds. 1. For IND(J)=1, require X(J). ge. BL(J).

## `IOPT`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

This is the array where the user can specify nonstandard options for DBOLSM( ). Most of the time this feature can be ignored by setting the input value IOPT(1)=99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number Brief Statement of Purpose 1 Return to user for accumulation of blocks of least squares equations. 2 Check lengths of all arrays used in the subprogram.

## `X`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

le. BU(J). 4. For IND(J)=4, no bounds on X(J) are required. (the values of BL(J) and BU(J) are not used. ) Values other than 1,2,3 or 4 for IND(J) are errors.

## `RNORM`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Writable output for the minimum residual-vector length. It is meaningful when the returned `MODE` reports a solution.

## `MODE`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The sign of MODE determines whether the subprogram has completed normally, or encountered an error condition or abnormal status. A value of MODE. ge. 0 signifies that the subprogram has completed normally. The value of MODE (. GE.

## `RW`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These are working arrays with 5*NCOLS and 2*NCOLS entries. (normally the user can ignore the contents of these arrays, but they must be dimensioned properly. ) IOPT(*) CONTENTS The option array allows a user to modify internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1.

## `IW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

These are working arrays with 5*NCOLS and 2*NCOLS entries. (normally the user can ignore the contents of these arrays, but they must be dimensioned properly. ) IOPT(*) CONTENTS The option array allows a user to modify internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `1` | , require X(J) .ge. BL(J). (the value of BU(J) is not used.) |
| `IND` | `2` | , require X(J) .le. BU(J). (the value of BL(J) is not used.) |
| `IND` | `3` | , require X(J) .ge. BL(J) and (upper and lower bounds) the condition BL(J) .gt. BU(J) is an error. |

# Workspace and array requirements

- `W`: not a workspace argument
- `MDW`: not a workspace argument
- `BL`: not a workspace argument
- `BU`: not a workspace argument
- `IND`: not a workspace argument
- `IOPT`: not a workspace argument
- `X`: not a workspace argument
- `RW`: not a workspace argument
- `IW`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::dbols`
- Original SLATEC routine: `DBOLS`
- Native symbol: `dbols_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DBOLS](https://www.netlib.org/slatec/src/dbols.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
