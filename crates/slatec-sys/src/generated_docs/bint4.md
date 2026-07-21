# Purpose

BINT4 computes the B representation (T,BCOEF,N,K) of a

# Description

This canonical unsafe binding exposes original SLATEC routine `BINT4`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BINT4](https://www.netlib.org/slatec/src/bint4.f).

# Arguments

## 1. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is not specified is not specified by the problem, it is common practice to use a natural by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) spline by setting second derivatives at X(1) and X(NDATA) is anticipated, the is anticipated, the is unique for given boundary conditions. X vector of abscissae of length NDATA, distinct and in increasing order to FBCL = 2 constrain the second derivative at to FBCL to FBCR to FBCR are knot values to the right of X(NDATA) in increasing order to be supplied by the user is not specified is not specified by the problem, it is common practice to use a natural by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) spline by setting second derivatives at X(1) and X(NDATA) is anticipated, the is anticipated, the is unique for given boundary conditions. X vector of abscissae of length NDATA, distinct and in increasing order to FBCL = 2 constrain the second derivative at to FBCL to FBCR to FBCR are knot values to the right of X(NDATA) in increasing order to be supplied by the user not applicable or not stated by selected source not a workspace argument

## 2. `Y`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Y vector of ordinates of length NDATA Y vector of ordinates of length NDATA not applicable or not stated by selected source not a workspace argument

## 3. `NDATA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Parameters IBCL, IBCR, FBCL, FBCR allow the specification of the spline first or second derivative at is not specified by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) is anticipated, the number of data points, NDATA .GE. 2 to FBCR to FBCR not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IBCL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IBCR=2,FBCL=FBCR=0.0).  The spline is defined on selection parameter for left boundary condition 1 constrain the first derivative at not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IBCR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. selection parameter for right boundary condition 1 constrain first derivative at 2 constrain second derivative at not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `FBCL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. left boundary values governed by IBCL not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `FBCR`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. right boundary values governed by IBCR not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `KNTOPT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. knot selection parameter 1 sets knot multiplicity at T(4) and 3, then W(1),W(2),W(3) are knot values to not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). .LE. X .LE. T(N+1) with (ordered) interior knots at X(I)) X(1) and the knots T(N+2), T(N+3), T(N+4) X(NDATA) in increasing order.  If T(2)=T(3)=T(4)=X(1) and T(N+2)=T(N+3)=T(N+4)= X(NDATA) can be specified by KNTOPT=1.  KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the X(1) and similarly for X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+4) to the right of X(NDATA) in the work array is unique is unique for given boundary conditions. for given boundary conditions. to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets TNP)=WNP) and T(N+1+I)=w(3+I),I=1,3 where WNP),I=1,6 is supplied by the user knot array of length N+4 .LE. X .LE. T(N+1) with (ordered) interior knots at X(I)) X(1) and the knots T(N+2), T(N+3), T(N+4) X(NDATA) in increasing order.  If T(2)=T(3)=T(4)=X(1) and T(N+2)=T(N+3)=T(N+4)= X(NDATA) can be specified by KNTOPT=1.  KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the X(1) and similarly for X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+4) to the right of X(NDATA) in the work array is unique is unique for given boundary conditions. for given boundary conditions. to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets TNP)=WNP) and T(N+1+I)=w(3+I),I=1,3 where WNP),I=1,6 is supplied by the user knot array of length N+4 not applicable or not stated by selected source

## 10. `BCOEF`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). B-spline coefficient array of length N B-spline coefficient array of length N not applicable or not stated by selected source not a workspace argument

## 11. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. NDATA+2.  The knots T(1), T(2), T(3) lie to X(NDATA) in increasing order.  If X(NDATA) can be specified by KNTOPT=1.  KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+4) to the right of X(NDATA) in the work array is unique for given boundary conditions. to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets TNP)=WNP) and T(N+1+I)=w(3+I),I=1,3 where WNP),I=1,6 is supplied by the user number of coefficients, N=NDATA+2 not stated by selected source not applicable or not stated by selected source

## 12. `K`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 4) which interpolates data (X(I)),Y(I))), order of spline, K=4 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (5, *). through W(6).  In any case, the interpolation on work array of dimension at least 5*(NDATA+2) are knot are knot are knot values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing order to be supplied by the user order to be supplied by the user order to be supplied by the user through W(6).  In any case, the interpolation on work array of dimension at least 5*(NDATA+2) are knot are knot are knot values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing order to be supplied by the user order to be supplied by the user order to be supplied by the user not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper  input is a fatal error Singular system of equations is a fatal error

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `NDATA`: not a workspace argument
- `IBCL`: not a workspace argument
- `IBCR`: not a workspace argument
- `FBCL`: not a workspace argument
- `FBCR`: not a workspace argument
- `KNTOPT`: not a workspace argument
- `T`: .LE. X .LE. T(N+1) with (ordered) interior knots at X(I)) X(1) and the knots T(N+2), T(N+3), T(N+4) X(NDATA) in increasing order.  If T(2)=T(3)=T(4)=X(1) and T(N+2)=T(N+3)=T(N+4)= X(NDATA) can be specified by KNTOPT=1.  KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the X(1) and similarly for X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+4) to the right of X(NDATA) in the work array is unique is unique for given boundary conditions. for given boundary conditions. to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets TNP)=WNP) and T(N+1+I)=w(3+I),I=1,3 where WNP),I=1,6 is supplied by the user knot array of length N+4
- `BCOEF`: not a workspace argument
- `N`: NDATA+2.  The knots T(1), T(2), T(3) lie to X(NDATA) in increasing order.  If X(NDATA) can be specified by KNTOPT=1.  KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+4) to the right of X(NDATA) in the work array is unique for given boundary conditions. to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets TNP)=WNP) and T(N+1+I)=w(3+I),I=1,3 where WNP),I=1,6 is supplied by the user number of coefficients, N=NDATA+2
- `K`: not a workspace argument
- `W`: through W(6).  In any case, the interpolation on work array of dimension at least 5*(NDATA+2) are knot are knot are knot values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing order to be supplied by the user order to be supplied by the user order to be supplied by the user

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bint4`
- Original SLATEC routine: `BINT4`
- Native symbol: `bint4_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [BINT4](https://www.netlib.org/slatec/src/bint4.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
