# Purpose

SUBROUTINE TO CALCULATE NORMALIZED LEGENDRE POLYNOMIALS (XNRMP is single-precision version) DXNRMP calculates normalized Legendre polynomials of varying order and fixed argument and degree. The order MU and degree

# Description

This canonical unsafe binding exposes original SLATEC routine `DXNRMP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DXNRMP](https://www.netlib.org/slatec/src/dxnrmp.f).

# Arguments

## 1. `NU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J.M. Smith, F.W.J. Olver, and D.W. Lozier, Extended-Range Arithmetic and Normalized Legendre Polynomials, ACM Transactions on Mathematical Soft- ware, 93-105, March 1981, for a complete description of the algorithm and special arithmetic. Also see program comments in DXSET. The normalized Legendre polynomials are multiples of the associated Legendre polynomials of the first kind where the normalizing coefficients are chosen so as to make the integral from -1 to 1 of the square of each function equal to 1. See E. Jahnke, F. Emde and F. Losch, Tables of Higher Functions, McGraw-Hill, New York, 1960, p. 121. The input values to DXNRMP are NU, MU1, MU2, DARG, and MODE. These must satisfy 1. NU .GE. 0 specifies the degree of the normalized Legendre polynomial that is wanted. MU1, negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J.M. Smith, F.W.J. Olver, and D.W. Lozier, Extended-Range Arithmetic and Normalized Legendre Polynomials, ACM Transactions on Mathematical Soft- ware, 93-105, March 1981, for a complete description of the algorithm and special arithmetic. Also see program comments in DXSET. The normalized Legendre polynomials are multiples of the associated Legendre polynomials of the first kind where the normalizing coefficients are chosen so as to make the integral from -1 to 1 of the square of each function equal to 1. See E. Jahnke, F. Emde and F. Losch, Tables of Higher Functions, McGraw-Hill, New York, 1960, p. 121. The input values to DXNRMP are NU, MU1, MU2, DARG, and MODE. These must satisfy 1. NU .GE. 0 specifies the degree of the normalized Legendre polynomial that is wanted. MU1, not applicable or not stated by selected source not a workspace argument

## 2. `MU1`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. order normalized Legendre polynomial that is wanted. order normalized Leg- endre polynomial that is wanted. + 1, ..., MU2. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `MU2`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. order normalized Leg- endre polynomial that is wanted. MU1 + 1 and DX = DARG or COS(DARG) according not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `DARG`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. MU1, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1.0D0 .LE. DARG .LE. 1.0D0 specifies that 3.14159... .LT. DARG .LT. 3.14159... spec- ifies that Normalized Legendre(NU, MU, COS(DARG)) is wanted for MU = MU1, MU1 + 1, ..., MU2. The output of DXNRMP consists of the two vectors DPN and IPN and the error estimate ISIG. The computed values are stored as extended-range numbers such that 1 or 2. Finally, ISIG is an estimate of the number of decimal digits lost through rounding errors in the computation. For example if DARG is accurate to 12 significant decimals, then the computed function values are accurate to 12 - ISIG significant decimals (except in neighborhoods of zeros). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `DPN`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). NORMALIZED LEGENDRE(NU,MU1,DX) NORMALIZED LEGENDRE(NU,MU1+1,DX) . . NORMALIZED LEGENDRE(NU,MU2,DX) is DPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I) .NE. 0 the corre- sponding value of the normalized Legendre polynomial cannot be represented in double-precision because of overflow or under- flow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case that IPN(I) is nonzero, the user could rewrite his/her program to use extended range arithmetic. The interpretation of (DPN(I),IPN(I)) can be changed to range subroutine DXCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL DXCON(DPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, DPN(I), IPN(I) 10 FORMAT(1X, D30.18 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J NORMALIZED LEGENDRE(NU,MU1,DX) NORMALIZED LEGENDRE(NU,MU1+1,DX) . . NORMALIZED LEGENDRE(NU,MU2,DX) is DPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I) .NE. 0 the corre- sponding value of the normalized Legendre polynomial cannot be represented in double-precision because of overflow or under- flow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case that IPN(I) is nonzero, the user could rewrite his/her program to use extended range arithmetic. The interpretation of (DPN(I),IPN(I)) can be changed to range subroutine DXCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL DXCON(DPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, DPN(I), IPN(I) 10 FORMAT(1X, D30.18 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J not applicable or not stated by selected source not a workspace argument

## 7. `IPN`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). NORMALIZED LEGENDRE(NU,MU1,DX) NORMALIZED LEGENDRE(NU,MU1+1,DX) . . NORMALIZED LEGENDRE(NU,MU2,DX) is DPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When 0 the value of the normalized Legendre polynomial is range subroutine DXCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL DXCON(DPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, DPN(I), IPN(I) 10 FORMAT(1X, D30.18 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J IPN(2) = ... = IPN(J) = 0. Because of the change of representation caused by calling DXCON, (DPN(I), J+1, J+2, ... cannot be used in subsequent extended-range computations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ISIG`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Writable output estimate of decimal digits lost to rounding in the extended-range normalized-Legendre calculation. Subtract it from the significant digits in the input argument to estimate retained result precision away from zeros. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an error indicator. If no errors are detected, 0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. 212 or 213, invalid input was provided to DXNRMP. 201,202,203, or 204, invalid input was provided to DXSET. 205 or 206, an internal consistency error occurred in DXSET (probably due to a software malfunction in the range number was detected in DXADJ. range number was detected in DXC210. is an error indicator. If no errors are detected, 0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. 212 or 213, invalid input was provided to DXNRMP. 201,202,203, or 204, invalid input was provided to DXSET. 205 or 206, an internal consistency error occurred in DXSET (probably due to a software malfunction in the range number was detected in DXADJ. range number was detected in DXC210. not applicable or not stated by selected source not a workspace argument

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
- `DARG`: not a workspace argument
- `MODE`: not a workspace argument
- `DPN`: not a workspace argument
- `IPN`: not a workspace argument
- `ISIG`: not a workspace argument
- `IERROR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dxnrmp`
- Original SLATEC routine: `DXNRMP`
- Native symbol: `dxnrmp_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [DXNRMP](https://www.netlib.org/slatec/src/dxnrmp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
