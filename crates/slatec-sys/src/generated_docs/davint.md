# Purpose

DAVINT integrates a function tabulated at arbitrarily spaced abscissas. The limits of integration need not coincide with the tabulated abscissas. A method of overlapping parabolas fitted to the data is used provided that there are at least 3 abscissas between the limits of integration. DAVINT also handles two special cases. If the limits of integration are equal, DAVINT returns a result of zero regardless of the number of tabulated values. If there are only two function values, DAVINT uses the trapezoid rule. Description of Parameters The user must dimension all arrays appearing in the call list X(N), Y(N)

# Description

This canonical unsafe binding exposes original SLATEC routine `DAVINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DAVINT](https://www.netlib.org/slatec/src/davint.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION array of abscissas, which must be in increasing order.

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION array of function values. i. e. , FUNC(X(I)).

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The integer number of function values supplied. GE. 2 unless XLO = XUP.

## `XLO`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION lower limit of integration.

## `XUP`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION upper limit of integration. Must have XLO. LE. XUP.

## `ANS`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Double Precision computed approximate value of integral is set to zero if IERR=2,3,4,or 5. DAVINT is documented completely in SC-M-69-335 Original program from *Numerical Integration* by Davis & Rabinowitz Adaptation and modifications by Rondall E Jones.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

A status code --Normal Code =1 Means the requested integration was performed. --Abnormal Codes =2 Means XUP was less than XLO. =3 Means the number of X(I) between XLO and XUP (inclusive) was less than 3 and neither of the two special cases described in the abstract occurred. No integration was performed. =4 Means the restriction X(I+1). GT.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::davint`
- Original SLATEC routine: `DAVINT`
- Native symbol: `davint_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64,mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [DAVINT](https://www.netlib.org/slatec/src/davint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
