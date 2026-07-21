# Purpose

All INPUT and OUTPUT real variables are DOUBLE PRECISION **** This subprogram solves the bounded and constrained least squares problem. The problem statement is: Solve E*X = F (least squares sense), subject to constraints C*X=Y. In this formulation both X and Y are unknowns, and both may have bounds on any of their components. This formulation of the problem allows the user to have equality and inequality constraints as well as simple bounds on the solution components. This constrained linear least squares subprogram solves E*X=F subject to C*X=Y, where E is MROWS by NCOLS, C is MCON by NCOLS. The user must have dimension statements of the form DIMENSION W(MDW,NCOLS+MCON+1), BL(NCOLS+MCON), BU(NCOLS+MCON), X(2*(NCOLS+MCON)+2+NX), RW(6*NCOLS+5*MCON) INTEGER IND(NCOLS+MCON), IOPT(17+NI), IW(2*(NCOLS+MCON)) (here NX=number of extra locations required for the options; NX=0 if no options are in use. Also NI=number of extra locations for options 1-9.)

# Description

This canonical unsafe binding exposes original SLATEC routine `DBOCLS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBOCLS](https://www.netlib.org/slatec/src/dbocls.f).

# Arguments

## `W`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (MDW, *).

The array W contains the (possibly null) matrix \[C:*\] followed by \[E:F\]. This must be placed in W as follows: \[C : *\] = \[ \] \[E : F\] The (*) after C indicates that this data can be undefined. The matrix \[E:F\] has MROWS rows and NCOLS+1 columns. The matrix C is placed in the first MCON rows of W(*,*) while \[E:F\] follows in rows MCON+1 through MCON+MROWS of W(*,*). The vector F is placed in rows MCON+1 through MCON+MROWS, column NCOLS+1. The values of MDW and NCOLS must be positive; the value of MCON must be nonnegative.

## `MDW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

ge. MCON + max(max. number of rows accumulated, NCOLS) + 1. If using option 8,. MCON + MROWS. Else.

## `MCON`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array W contains the (possibly null) matrix \[C:*\] followed by \[E:F\]. This must be placed in W as follows: \[C : *\].

## `MROWS`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of rows in the least-squares block `\[E:F\]`. With accumulation option `IOPT(1)`, this is a writable returned count of accumulated rows; otherwise it is an input count used with `MDW`, `MCON`, and `NCOLS`.

## `NCOLS`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array W contains the (possibly null) matrix \[C:*\] followed by \[E:F\]. This must be placed in W as follows: \[C : *\].

## `BL`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds on the unknowns X and Y. The first NVARS entries of IND(*), BL(*) and BU(*) specify bounds on X; the next MCON entries specify bounds on Y. 1. For IND(J)=1, require X(J). ge.

## `BU`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds on the unknowns X and Y. The first NVARS entries of IND(*), BL(*) and BU(*) specify bounds on X; the next MCON entries specify bounds on Y. 1. For IND(J)=1, require X(J). ge.

## `IND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

These arrays contain the information about the bounds that the solution values are to satisfy. The value of IND(J) tells the type of bound and BL(J) and BU(J) give the explicit values for the respective upper and lower bounds on the unknowns X and Y. The first NVARS entries of IND(*), BL(*) and BU(*) specify bounds on X; the next MCON entries specify bounds on Y. 1. For IND(J)=1, require X(J). ge.

## `IOPT`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

This is the array where the user can specify nonstandard options for DBOCLS( ). Most of the time this feature can be ignored by setting the input value IOPT(1)=99. Occasionally users may have needs that require use of the following subprogram options. For details about how to use the options see below: IOPT(*) CONTENTS. Option Number Brief Statement of Purpose 1 Return to user for accumulation of blocks of least squares equations. The values of IOPT(*) are changed with this option.

## `X`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

le. BU(J); IF J. gt. NCOLS, Y(J-NCOLS). ge. BL(J) and Y(J-NCOLS).

## `RNORMC`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

The array X(*) contains a solution (if MODE. ge. 0 or. eq. -22) for the constrained least squares problem. The value RNORMC is the minimum residual vector length for the constraints C*X - Y = 0.

## `RNORM`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Writable output for the Euclidean length of the final residual vector of the constrained least-squares problem.

## `MODE`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The sign of MODE determines whether the subprogram has completed normally, or encountered an error condition or abnormal status. A value of MODE. ge. 0 signifies that the subprogram has completed normally. The value of mode (. 0) is the number of variables in an active status: not at a bound nor at the value zero, for the case of free variables.

## `RW`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Writable real workspace with at least `6*NCOLS + 5*MCON` elements. It is scratch storage for the bounded constrained solve and native code retains no pointer.

## `IW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

These are working arrays. (normally the user can ignore the contents of these arrays. ) IOPT(*) CONTENTS The option array allows a user to modify some internal variables in the subprogram without recompiling the source code. A central goal of the initial software design was to do a good job for most people. Thus the use of options will be restricted to a select group of users. The processing of the option array proceeds as follows: a pointer, here called LP, is initially set to the value 1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `1` | , require X(J) .ge. BL(J); |
| `IND` | `>0` | NCOLS, Y(J-NCOLS) .ge. BL(J). (the value of BU(J) is not used.) |
| `IND` | `2` | , require X(J) .le. BU(J); |
| `IND` | `>0` | NCOLS, Y(J-NCOLS) .le. BU(J). (the value of BL(J) is not used.) |
| `IND` | `3` | , require X(J) .ge. BL(J) and (upper and lower bounds) the condition BL(J) .gt. BU(J) |
| `IND` | `>0` | NCOLS, will be changed. Significant changes mean that the constraints are infeasible. (Users must make this decision themselves.) NCOLS, define a region such that the perturbed problem is feasible. If users know that their problem is feasible, this step can be skipped by using option number 8 described below. See IOPT(*) description. |

# Workspace and array requirements

- `W`: not a workspace argument
- `BL`: not a workspace argument
- `BU`: not a workspace argument
- `IND`: not a workspace argument
- `IOPT`: not a workspace argument
- `X`: not a workspace argument
- `RW`: Writable real workspace with at least `6*NCOLS + 5*MCON` elements. It is scratch storage for the bounded constrained solve and native code retains no pointer.
- `IW`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::dbocls`
- Original SLATEC routine: `DBOCLS`
- Native symbol: `dbocls_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DBOCLS](https://www.netlib.org/slatec/src/dbocls.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
