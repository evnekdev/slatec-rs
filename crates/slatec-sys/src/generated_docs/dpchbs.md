# Purpose

Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR DPCHBS computes the B-spline representation of the PCH function determined by N,X,F,D. To be compatible with the rest of PCHIP, DPCHBS includes INCFD, the increment between successive values of

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHBS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHBS](https://www.netlib.org/slatec/pchip/dpchbs.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN  is the number of data points, N.ge.2 .  (not checked) X(N-1)). X(N-1)). X(1))  . Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used in a parametric setting. are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). IN  is the real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N.   (not checked) nmax, the dimension of X, must be .ge.N. (X(2)-X(1))  ; X(N-1)). X(N-1)). (X(N)-X(N-1)); X(1))  . X(1))  . Here M=NDIM=2*N. Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used This option is provided for improved efficiency when used in a parametric setting. in a parametric setting. values and the boundary knots set as indicated above. If KNOTYP.LT.0, it is assumed that T was set by a previous call to DPCHBS.  (This routine does **not** verify that T forms a legitimate knot sequence.) are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) 1,...,N .  (not checked) 1,...,N .  (not checked) 3. INCFD.GT.0 .  (not checked) 3. INCFD.GT.0 .  (not checked) 4. KNOTYP.LE.2 .  (error return if not) 4. KNOTYP.LE.2 .  (error return if not) IN  is the real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N.   (not checked) nmax, the dimension of X, must be .ge.N. (X(2)-X(1))  ; X(N-1)). X(N-1)). (X(N)-X(N-1)); X(1))  . X(1))  . Here M=NDIM=2*N. Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used This option is provided for improved efficiency when used in a parametric setting. in a parametric setting. values and the boundary knots set as indicated above. If KNOTYP.LT.0, it is assumed that T was set by a previous call to DPCHBS.  (This routine does **not** verify that T forms a legitimate knot sequence.) are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) 1,...,N .  (not checked) 1,...,N .  (not checked) 3. INCFD.GT.0 .  (not checked) 3. INCFD.GT.0 .  (not checked) 4. KNOTYP.LE.2 .  (error return if not) 4. KNOTYP.LE.2 .  (error return if not) not applicable or not stated by selected source not a workspace argument

## 3. `F`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). IN  is the real array of dependent variable values. 1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of F, must be .ge.N. subscripted arrays. and D-arrays. The output is the B-representation for the function:  NKNOTS, T, Fortran 77, in the strict sense, but it works on all systems on which DPCHBS has been tested. IN  is the real array of dependent variable values. 1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of F, must be .ge.N. subscripted arrays. and D-arrays. The output is the B-representation for the function:  NKNOTS, T, Fortran 77, in the strict sense, but it works on all systems on which DPCHBS has been tested. not applicable or not stated by selected source not a workspace argument

## 4. `D`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). IN  is the real array of derivative values at the data points. 1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of D, must be .ge.N. subscripted arrays. Fortran 77, in the strict sense, but it works on all systems on which DPCHBS has been tested. IN  is the real array of derivative values at the data points. 1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of D, must be .ge.N. subscripted arrays. Fortran 77, in the strict sense, but it works on all systems on which DPCHBS has been tested. not applicable or not stated by selected source not a workspace argument

## 5. `INCFD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ...) DOUBLE PRECISION  X(nmax), F(INCFD,nmax), D(INCFD,nmax), IN  is the increment between successive values in F and D. This argument is provided primarily for 2-D applications. It may have the value 1 for one-dimensional applications, are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) patible with the rest of PCHIP. 900410  Converted prologue to SLATEC 4.0 format. 900410  Added calls to XERMSG and changed constant 3. to 3 to reduce single/double differences. 900411  Added reference. 900430  Produced double precision version. 900501  Corrected declarations. 930317  Minor cosmetic changes.  (FNF) 930514  Corrected problems with dimensioning of arguments and clarified DESCRIPTION.  (FNF) 930604  Removed  NKNOTS from DPCHKT call list.  (FNF) ...) DOUBLE PRECISION  X(nmax), F(INCFD,nmax), D(INCFD,nmax), IN  is the increment between successive values in F and D. This argument is provided primarily for 2-D applications. It may have the value 1 for one-dimensional applications, are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) patible with the rest of PCHIP. 900410  Converted prologue to SLATEC 4.0 format. 900410  Added calls to XERMSG and changed constant 3. to 3 to reduce single/double differences. 900411  Added reference. 900430  Produced double precision version. 900501  Corrected declarations. 930317  Minor cosmetic changes.  (FNF) 930514  Corrected problems with dimensioning of arguments and clarified DESCRIPTION.  (FNF) 930604  Removed  NKNOTS from DPCHKT call list.  (FNF) not applicable or not stated by selected source not a workspace argument

## 6. `KNOTYP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN  is a flag to control the knot sequence. The knot sequence T is normally computed from X by putting a double knot at each X and setting the end knot pairs Quadruple knots at X(1) and X(N).  (default) Replicate lengths of extreme subintervals: Periodic placement of boundary knots: is an input variable, and an IN  is a flag to control the knot sequence. The knot sequence T is normally computed from X by putting a double knot at each X and setting the end knot pairs Quadruple knots at X(1) and X(N).  (default) Replicate lengths of extreme subintervals: Periodic placement of boundary knots: is an input variable, and an not applicable or not stated by selected source not a workspace argument

## 7. `NKNOTS`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INOUT  is the number of knots. If KNOTYP.GE.0, then NKNOTS will be set to NDIM+4. is an input variable, and an NDIM+4 = 2*N+4 .  (error return if not) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `T`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, (X(2)-X(1))  ; (X(2)-X(1))  ; X(N-1)). X(N-1)). (X(N)-X(N-1)); (X(N)-X(N-1)); X(1))  . X(1))  . Here M=NDIM=2*N. Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used This option is provided for improved efficiency when used in a parametric setting. in a parametric setting. INOUT  is the array of 2*N+4 knots for the B-representation. If KNOTYP.GE.0, T will be returned by DPCHBS with the T(2*k) = X(k), k=1,...,N .  (not checked) Indicates this applies only if KNOTYP.LT.0 . Portability: Argument INCFD is used only to cause the compiler to generate efficient code for the subscript expressions (1+(I-1)*INCFD) . The normal usage, in which DPCHBS is called with one-dimensional BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, (X(2)-X(1))  ; (X(2)-X(1))  ; X(N-1)). X(N-1)). (X(N)-X(N-1)); (X(N)-X(N-1)); X(1))  . X(1))  . Here M=NDIM=2*N. Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used This option is provided for improved efficiency when used in a parametric setting. in a parametric setting. INOUT  is the array of 2*N+4 knots for the B-representation. If KNOTYP.GE.0, T will be returned by DPCHBS with the T(2*k) = X(k), k=1,...,N .  (not checked) Indicates this applies only if KNOTYP.LT.0 . Portability: Argument INCFD is used only to cause the compiler to generate efficient code for the subscript expressions (1+(I-1)*INCFD) . The normal usage, in which DPCHBS is called with one-dimensional not applicable or not stated by selected source not a workspace argument

## 9. `BCOEF`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). OUT  is the array of 2*N B-spline coefficients. NDIM, KORD. Caution: Since it is assumed that the input PCH function has been computed by one of the other routines in the package PCHIP, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NDIM`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. KORD, IERR) OUT  is the dimension of the B-spline space.  (Set to 2*N.) KORD, IERR) OUT  is the dimension of the B-spline space.  (Set to 2*N.) not applicable or not stated by selected source not a workspace argument

## 11. `KORD`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT  is the order of the B-spline.  (Set to 4.) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT  is an error flag. Normal return: 0  (no errors). "Recoverable" errors: 4  if KNOTYP.GT.2 . 5  if KNOTYP.LT.0 and NKNOTS.NE.(2*N+4). not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `INCFD`: not a workspace argument
- `KNOTYP`: not a workspace argument
- `NKNOTS`: not a workspace argument
- `T`: not a workspace argument
- `BCOEF`: not a workspace argument
- `NDIM`: not a workspace argument
- `KORD`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchbs`
- Original SLATEC routine: `DPCHBS`
- Native symbol: `dpchbs_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [DPCHBS](https://www.netlib.org/slatec/pchip/dpchbs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
