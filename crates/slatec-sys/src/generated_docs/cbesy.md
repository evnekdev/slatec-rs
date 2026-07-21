# Purpose

On KODE=1, CBESY computes an N member sequence of complex Bessel functions CY(L)=Y(FNU+L-1,Z) for real nonnegative orders FNU+L-1, L=1,...,N and complex Z in the cut plane -pi<arg(Z)<=pi. On KODE=2, CBESY returns the scaled functions CY(L) = exp(-abs(Y))*Y(FNU+L-1,Z), L=1,...,N, Y=Im(Z) which remove the exponential growth in both the upper and lower half planes as Z goes to infinity. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

# Description

This canonical unsafe binding exposes original SLATEC routine `CBESY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBESY](https://www.netlib.org/slatec/src/cbesy.f).

# Arguments

## 1. `Z`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. b,b+1, b+2,... where b>0.  A scaling option is available to help avoid overflow. On KODE=2, CBESY returns the scaled functions Nonzero argument of type COMPLEX 1 too large) 1 is large) 1 is too large) H(2,a,z))/(2*i) H(2,a,z))/(2*i) where the Hankel functions are computed as described in CBESH. where the Hankel functions are computed as described in CBESH. For negative orders, the formula For negative orders, the formula = Y(a,z)*cos(a*pi) + J(a,z)*sin(a*pi) can be used.  However, for large orders close to half odd integers the function changes radically.  When a is a large positive half odd integer, the magnitude of Y(-a,z)=J(a,z)* sin(a*pi) is a large negative power of ten.  But when a is not a half odd integer, Y(a,z) dominates in magnitude with a large positive power of ten and the most that the second term can be reduced is by unit roundoff from the coefficient. Thus,  wide changes can occur within unit roundoff of a large half odd integer.  Here, large means a>abs(z). In most complex variable computation, one must evaluate ele- 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component.  In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P. b,b+1, b+2,... where b>0.  A scaling option is available to help avoid overflow. On KODE=2, CBESY returns the scaled functions Nonzero argument of type COMPLEX 1 too large) 1 is large) 1 is too large) H(2,a,z))/(2*i) H(2,a,z))/(2*i) where the Hankel functions are computed as described in CBESH. where the Hankel functions are computed as described in CBESH. For negative orders, the formula For negative orders, the formula = Y(a,z)*cos(a*pi) + J(a,z)*sin(a*pi) can be used.  However, for large orders close to half odd integers the function changes radically.  When a is a large positive half odd integer, the magnitude of Y(-a,z)=J(a,z)* sin(a*pi) is a large negative power of ten.  But when a is not a half odd integer, Y(a,z) dominates in magnitude with a large positive power of ten and the most that the second term can be reduced is by unit roundoff from the coefficient. Thus,  wide changes can occur within unit roundoff of a large half odd integer.  Here, large means a>abs(z). In most complex variable computation, one must evaluate ele- 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component.  In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P. not applicable or not stated by selected source not a workspace argument

## 2. `FNU`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1,Z) for real nonnegative 1, L=1,...,N and complex Z in the cut plane Initial order of type REAL, FNU>=0 1,Z), L=1,...,N =2  returns 1,Z)*exp(-abs(Y)), L=1,...,N where Y=Im(Z) 1 too large) 1 is large) 1 is too large) 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), MAX(1,ABS(EXPONENT OF not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, CBESY computes an N member sequence of complex A parameter to indicate the scaling option 1  returns 2 (the underflows may not be in an uninterrupted sequence) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Number of terms in the sequence, N>=1 1 too large) 1 is large) 1 is too large) 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `CY`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (N). 1,Z) for real nonnegative abs(Y))*Y(FNU+L-1,Z),  L=1,...,N, Y=Im(Z) which remove the exponential growth in both the upper and lower half planes as Z goes to infinity.  Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1). 1,Z), L=1,...,N =2  returns 1,Z)*exp(-abs(Y)), L=1,...,N where Y=Im(Z) Result vector of type COMPLEX 0 for NZ values of L, usually on not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Number of underflows set to zero 0    Normal return 0 for NZ values of L, usually on not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `CWRK`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (N). A work vector of type COMPLEX and dimension N A work vector of type COMPLEX and dimension N not applicable or not stated by selected source not a workspace argument

## 8. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Error flag COMPUTATION COMPLETED NO COMPUTATION NO COMPUTATION COMPUTATION COMPLETED (Result has half precision or less NO COMPUTATION (Result has no precision because NO COMPUTATION (Termination condition not met) Long Description: The computation is carried out by the formula 3 is triggered where UR=R1MACH(4)=UNIT ROUNDOFF.  Also, if either is larger than U2=0.5/UR, then all significance is 4.  In order to use the INT function, arguments must be further restricted not to exceed the largest machine Error flag COMPUTATION COMPLETED NO COMPUTATION NO COMPUTATION COMPUTATION COMPLETED (Result has half precision or less NO COMPUTATION (Result has no precision because NO COMPUTATION (Termination condition not met) Long Description: The computation is carried out by the formula 3 is triggered where UR=R1MACH(4)=UNIT ROUNDOFF.  Also, if either is larger than U2=0.5/UR, then all significance is 4.  In order to use the INT function, arguments must be further restricted not to exceed the largest machine not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `Z`: not a workspace argument
- `FNU`: not a workspace argument
- `KODE`: not a workspace argument
- `N`: not a workspace argument
- `CY`: not a workspace argument
- `NZ`: not a workspace argument
- `CWRK`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::cbesy`
- Original SLATEC routine: `CBESY`
- Native symbol: `cbesy_`
- ABI fingerprint: `subroutine:void(mut_complex32,mut_f32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CBESY](https://www.netlib.org/slatec/src/cbesy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
