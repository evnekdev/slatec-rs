# Purpose

SUBROUTINE TO CALCULATE NORMALIZED LEGENDRE POLYNOMIALS (DXNRMP is double-precision version) XNRMP calculates normalized Legendre polynomials of varying order and fixed argument and degree. The order MU and degree

# Description

This canonical unsafe binding exposes original SLATEC routine `XNRMP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [XNRMP](https://www.netlib.org/slatec/src/xnrmp.f).

# Arguments

## 1. `NU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J.M. Smith, F.W.J. Olver, and D.W. Lozier, Extended-Range Arithmetic and Normalized Legendre Polynomials, ACM Transactions on Mathematical Soft- ware, 93-105, March 1981, for a complete description of the algorithm and special arithmetic. Also see program comments in XSET. The normalized Legendre polynomials are multiples of the associated Legendre polynomials of the first kind where the normalizing coefficients are chosen so as to make the integral from -1 to 1 of the square of each function equal to 1. See E. Jahnke, F. Emde and F. Losch, Tables of Higher Functions, McGraw-Hill, New York, 1960, p. 121. The input values to XNRMP are NU, MU1, MU2, SARG, and MODE. These must satisfy 1. NU .GE. 0 specifies the degree of the normalized Legendre polynomial that is wanted. MU1, ed for MU = MU1, MU1 + 1, ..., MU2. The output of XNRMP consists of the two vectors SPN and IPN and the error estimate ISIG. The computed values are stored as extended-range numbers such that negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J.M. Smith, F.W.J. Olver, and D.W. Lozier, Extended-Range Arithmetic and Normalized Legendre Polynomials, ACM Transactions on Mathematical Soft- ware, 93-105, March 1981, for a complete description of the algorithm and special arithmetic. Also see program comments in XSET. The normalized Legendre polynomials are multiples of the associated Legendre polynomials of the first kind where the normalizing coefficients are chosen so as to make the integral from -1 to 1 of the square of each function equal to 1. See E. Jahnke, F. Emde and F. Losch, Tables of Higher Functions, McGraw-Hill, New York, 1960, p. 121. The input values to XNRMP are NU, MU1, MU2, SARG, and MODE. These must satisfy 1. NU .GE. 0 specifies the degree of the normalized Legendre polynomial that is wanted. MU1, ed for MU = MU1, MU1 + 1, ..., MU2. The output of XNRMP consists of the two vectors SPN and IPN and the error estimate ISIG. The computed values are stored as extended-range numbers such that not applicable or not stated by selected source not a workspace argument

## 2. `MU1`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. order normalized Legendre polynomial that is wanted. order normalized Leg- endre polynomial that is wanted. + 1, ..., MU2. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `MU2`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. order normalized Leg- endre polynomial that is wanted. MU1 + 1 and X = SARG or COS(SARG) according not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `SARG`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. MU1, ed for MU = MU1, MU1 + 1, ..., MU2. The output of XNRMP consists of the two vectors SPN and IPN and the error estimate ISIG. The computed values are stored as extended-range numbers such that not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1.0 .LE. SARG .LE. 1.0 specifies that 3.14159... .LT. SARG .LT. 3.14159... spec- 1 or 2. Finally, ISIG is an estimate of the number of decimal digits lost through rounding errors in the computation. For example if SARG is accurate to 12 significant decimals, then the computed function values are accurate to 12 - ISIG significant decimals (except in neighborhoods of zeros). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `SPN`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). NORMALIZED LEGENDRE(NU,MU1,X) NORMALIZED LEGENDRE(NU,MU1+1,X) . . NORMALIZED LEGENDRE(NU,MU2,X) is SPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I) .NE. 0 the corre- sponding value of the normalized Legendre polynomial cannot be represented in single-precision because of overflow or under- flow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case range subroutine XCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL XCON(SPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, SPN(I), IPN(I) 10 FORMAT(1X, E30.8 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J NORMALIZED LEGENDRE(NU,MU1,X) NORMALIZED LEGENDRE(NU,MU1+1,X) . . NORMALIZED LEGENDRE(NU,MU2,X) is SPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I) .NE. 0 the corre- sponding value of the normalized Legendre polynomial cannot be represented in single-precision because of overflow or under- flow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case range subroutine XCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL XCON(SPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, SPN(I), IPN(I) 10 FORMAT(1X, E30.8 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J not applicable or not stated by selected source not a workspace argument

## 7. `IPN`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). NORMALIZED LEGENDRE(NU,MU1,X) NORMALIZED LEGENDRE(NU,MU1+1,X) . . NORMALIZED LEGENDRE(NU,MU2,X) is SPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When 0 the value of the normalized Legendre polynomial is cision if it has a wider exponent range. If double precision fails, the user could rewrite his/her program to use extended- range arithmetic. The interpretation of (SPN(I),IPN(I)) can be changed to range subroutine XCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL XCON(SPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, SPN(I), IPN(I) 10 FORMAT(1X, E30.8 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J IPN(2) = ... = IPN(J) = 0. Because of the change of representation caused by calling XCON, (SPN(I), J+1, J+2, ... cannot be used in subsequent extended-range computations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ISIG`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Writable output estimate of decimal digits lost to rounding in the extended-range normalized-Legendre calculation. Subtract it from the significant digits in the input argument to estimate retained result precision away from zeros. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an error indicator. If no errors are detected, 0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. 112 or 113, invalid input was provided to XNRMP. 101,102,103, or 104, invalid input was provided to XSET. 105 or 106, an internal consistency error occurred in XSET (probably due to a software malfunction in the range number was detected in XADJ. range number was detected in XC210. is an error indicator. If no errors are detected, 0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. 112 or 113, invalid input was provided to XNRMP. 101,102,103, or 104, invalid input was provided to XSET. 105 or 106, an internal consistency error occurred in XSET (probably due to a software malfunction in the range number was detected in XADJ. range number was detected in XC210. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NU`: not a workspace argument
- `MU1`: not a workspace argument
- `MU2`: not a workspace argument
- `SARG`: not a workspace argument
- `MODE`: not a workspace argument
- `SPN`: not a workspace argument
- `IPN`: not a workspace argument
- `ISIG`: not a workspace argument
- `IERROR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::xnrmp`
- Original SLATEC routine: `XNRMP`
- Native symbol: `xnrmp_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [XNRMP](https://www.netlib.org/slatec/src/xnrmp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
