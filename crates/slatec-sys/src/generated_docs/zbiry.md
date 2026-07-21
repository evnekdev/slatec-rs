# Purpose

A DOUBLE PRECISION ROUTINE***

# Description

This canonical unsafe binding exposes original SLATEC routine `ZBIRY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ZBIRY](https://www.netlib.org/slatec/src/zbiry.f).

# Arguments

## 1. `ZR`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. DOUBLE PRECISION real part of argument Z not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `ZI`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. DOUBLE PRECISION imag part of argument Z not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `ID`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 0 or ID=1 respectively. Order of derivative, ID=0 or ID=1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `KODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, ZBIRY computes the complex Airy function Bi(z) 2, a scaling option exp(abs(Re(zeta)))*Bi(z) or exp(abs(Re(zeta)))*dBi/dz is provided to remove the exponential behavior in both the left and right half planes where zeta=(2/3)*z**(3/2). The Airy functions Bi(z) and dBi/dz are analytic in the whole z-plane, and the scaling option does not destroy this property. A parameter to indicate the scaling option 1  returns BI=Bi(z)  on ID=0 BI=dBi/dz on ID=1 at z=Z =2  returns BI=exp(abs(Re(zeta)))*Bi(z)  on ID=0 BI=exp(abs(Re(zeta)))*dBi/dz on ID=1 at z=Z where zeta=(2/3)*z**(3/2) 1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BIR`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. DOUBLE PRECISION real part of result not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `BII`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. DOUBLE PRECISION imag part of result not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Error flag COMPUTATION COMPLETED NO COMPUTATION NO COMPUTATION COMPUTATION COMPLETED (Result has less than half precision) NO COMPUTATION (Result has no precision) NO COMPUTATION (Termination condition not met) Long Description: Bi(z) and dBi/dz are computed from I Bessel functions by Bi(z) =  c*sqrt(z)*( I(-1/3,zeta) + I(1/3,zeta) ) dBi/dz =  c*   z   *( I(-2/3,zeta) + I(2/3,zeta) ) c =  1/sqrt(3) zeta =  (2/3)*z**(3/2) when abs(z)>1 and from power series when abs(z)<=1. In most complex variable computation, one must evaluate ele- mentary functions.  When the magnitude of Z is large, losses of significance by argument reduction occur.  Consequently, if the magnitude of ZETA=(2/3)*Z**(3/2) exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error 18) is double precision unit roundoff limited to 18 digits precision. Also, if the magnitude of ZETA is larger than U2=0.5/UR, then 4.  In order to use the INT function, ZETA must be further restricted not to exceed U3=I1MACH(9)=LARGEST INTEGER.  Thus, the magnitude of ZETA must be restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 are approximately 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15, 2.1E+9 in double precision. This makes U2 limiting is single precision and U3 limiting in double precision.  This means that the magnitude of Z cannot exceed approximately 3.4E+4 in single precision and 2.1E+6 in double precision.  This also means that one can expect to retain, in the worst cases on 32-bit machines, no digits in single precision and only 6 digits in double precision. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), ABS(LOG10(FNU))) approximately (i.e., S=MAX(1,ABS(EXPONENT OF ABS(Z),ABS(EXPONENT OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component. In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P. Error flag COMPUTATION COMPLETED NO COMPUTATION NO COMPUTATION COMPUTATION COMPLETED (Result has less than half precision) NO COMPUTATION (Result has no precision) NO COMPUTATION (Termination condition not met) Long Description: Bi(z) and dBi/dz are computed from I Bessel functions by Bi(z) =  c*sqrt(z)*( I(-1/3,zeta) + I(1/3,zeta) ) dBi/dz =  c*   z   *( I(-2/3,zeta) + I(2/3,zeta) ) c =  1/sqrt(3) zeta =  (2/3)*z**(3/2) when abs(z)>1 and from power series when abs(z)<=1. In most complex variable computation, one must evaluate ele- mentary functions.  When the magnitude of Z is large, losses of significance by argument reduction occur.  Consequently, if the magnitude of ZETA=(2/3)*Z**(3/2) exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error 18) is double precision unit roundoff limited to 18 digits precision. Also, if the magnitude of ZETA is larger than U2=0.5/UR, then 4.  In order to use the INT function, ZETA must be further restricted not to exceed U3=I1MACH(9)=LARGEST INTEGER.  Thus, the magnitude of ZETA must be restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 are approximately 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15, 2.1E+9 in double precision. This makes U2 limiting is single precision and U3 limiting in double precision.  This means that the magnitude of Z cannot exceed approximately 3.4E+4 in single precision and 2.1E+6 in double precision.  This also means that one can expect to retain, in the worst cases on 32-bit machines, no digits in single precision and only 6 digits in double precision. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), ABS(LOG10(FNU))) approximately (i.e., S=MAX(1,ABS(EXPONENT OF ABS(Z),ABS(EXPONENT OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component. In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `ZR`: not a workspace argument
- `ZI`: not a workspace argument
- `ID`: not a workspace argument
- `KODE`: not a workspace argument
- `BIR`: not a workspace argument
- `BII`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::zbiry`
- Original SLATEC routine: `ZBIRY`
- Native symbol: `zbiry_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [ZBIRY](https://www.netlib.org/slatec/src/zbiry.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
