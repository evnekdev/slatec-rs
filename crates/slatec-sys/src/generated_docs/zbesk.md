# Purpose

A DOUBLE PRECISION ROUTINE***

# Description

This canonical unsafe binding exposes original SLATEC routine `ZBESK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ZBESK](https://www.netlib.org/slatec/src/zbesk.f).

# Arguments

## 1. `ZR`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. DOUBLE PRECISION real part of nonzero argument Z not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `ZI`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. DOUBLE PRECISION imag part of nonzero argument Z not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `FNU`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1,Z) for real nonnegative 1, L=1,...,N and complex Z.NE.0 in the cut plane -pi<arg(Z)<=pi where Z=ZR+i*ZI.  On KODE=2, CBESJ returns the scaled functions 1,Z),  L=1,...,N which remove the exponential growth in both the left and right half planes as Z goes to infinity.  Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1). DOUBLE PRECISION initial order, FNU>=0 1,Z), L=1,...,N =2  returns 1,Z)*EXP(Z), L=1,...,N 1 too large) 1 is large) 1 is too large) 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), MAX(1,ABS(EXPONENT OF ABS(Z),ABS(EXPONENT OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component.  In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P. 1,Z) for real nonnegative 1, L=1,...,N and complex Z.NE.0 in the cut plane -pi<arg(Z)<=pi where Z=ZR+i*ZI.  On KODE=2, CBESJ returns the scaled functions 1,Z),  L=1,...,N which remove the exponential growth in both the left and right half planes as Z goes to infinity.  Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1). DOUBLE PRECISION initial order, FNU>=0 1,Z), L=1,...,N =2  returns 1,Z)*EXP(Z), L=1,...,N 1 too large) 1 is large) 1 is too large) 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), MAX(1,ABS(EXPONENT OF ABS(Z),ABS(EXPONENT OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component.  In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P. not applicable or not stated by selected source not a workspace argument

## 4. `KODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, ZBESK computes an N member sequence of complex A parameter to indicate the scaling option 1  returns not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Number of terms in the sequence, N>=1 1 too large) 1 is large) 1 is too large) 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `CYR`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). DOUBLE PRECISION real part of result vector not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `CYI`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). DOUBLE PRECISION imag part of result vector not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Number of underflows set to zero 0    Normal return 0 for NZ values of L (if Re(Z)>0 then CY(L)=0 for L=1,...,NZ; in the complementary half plane the underflows may not be in an uninterrupted sequence) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Error flag COMPUTATION COMPLETED NO COMPUTATION NO COMPUTATION COMPUTATION COMPLETED (Result has half precision or less NO COMPUTATION (Result has no precision because NO COMPUTATION (Termination condition not met) Long Description: Equations of the reference are implemented to compute K(a,z) for small orders a and a+1 in the right half plane Re(z)>=0. Forward recurrence generates higher orders.  The formula K(a,z*exp((t)) = exp(-t)*K(a,z) - t*I(a,z),  Re(z)>0 t = i*pi or -i*pi continues K to the left half plane. For large orders, K(a,z) is computed by means of its uniform asymptotic expansion. For negative orders, the formula K(-a,z) = K(a,z) can be used. CBESK assumes that a significant digit sinh function is available. In most complex variable computation, one must evaluate ele- 18) is double precision unit roundoff limited to 18 digits precision.  Also, if either is larger than U2=0.5/UR, then all significance is 4.  In order to use the INT function, arguments must be further restricted not to exceed the largest machine Error flag COMPUTATION COMPLETED NO COMPUTATION NO COMPUTATION COMPUTATION COMPLETED (Result has half precision or less NO COMPUTATION (Result has no precision because NO COMPUTATION (Termination condition not met) Long Description: Equations of the reference are implemented to compute K(a,z) for small orders a and a+1 in the right half plane Re(z)>=0. Forward recurrence generates higher orders.  The formula K(a,z*exp((t)) = exp(-t)*K(a,z) - t*I(a,z),  Re(z)>0 t = i*pi or -i*pi continues K to the left half plane. For large orders, K(a,z) is computed by means of its uniform asymptotic expansion. For negative orders, the formula K(-a,z) = K(a,z) can be used. CBESK assumes that a significant digit sinh function is available. In most complex variable computation, one must evaluate ele- 18) is double precision unit roundoff limited to 18 digits precision.  Also, if either is larger than U2=0.5/UR, then all significance is 4.  In order to use the INT function, arguments must be further restricted not to exceed the largest machine not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `ZR`: not a workspace argument
- `ZI`: not a workspace argument
- `FNU`: not a workspace argument
- `KODE`: not a workspace argument
- `N`: not a workspace argument
- `CYR`: not a workspace argument
- `CYI`: not a workspace argument
- `NZ`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::zbesk`
- Original SLATEC routine: `ZBESK`
- Native symbol: `zbesk_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [ZBESK](https://www.netlib.org/slatec/src/zbesk.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
