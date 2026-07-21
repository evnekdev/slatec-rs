# Purpose

Subroutine POIS3D solves the linear system of equations

# Description

This canonical unsafe binding exposes original SLATEC routine `POIS3D`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POIS3D](https://www.netlib.org/slatec/fishfft/pois3d.f).

# Arguments

## 1. `LPEROD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the values that X(0,J,K) and X(L+1,J,K) are assumed to have. = 0  If X(0,J,K) = X(L,J,K) and X(L+1,J,K) = X(1,J,K). = 1  If X(0,J,K) = X(L+1,J,K) = 0. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `L`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are assumed to take on certain prescribed values described below. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * 1,J,K). 1,J,K). 1,J,K). 1,J,K). = 4  If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. = 4  If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. direction. L must be at least 3. + INT((M+1)/2)). M=N)   LPEROD    MPEROD    T(MSECS)    E are assumed to take on certain prescribed values described below. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * 1,J,K). 1,J,K). 1,J,K). 1,J,K). = 4  If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. = 4  If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. direction. L must be at least 3. + INT((M+1)/2)). M=N)   LPEROD    MPEROD    T(MSECS)    E not applicable or not stated by selected source not a workspace argument

## 3. `C1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1,J,K)-2.*X(I,J,K)+X(I+1,J,K)) The real constant that appears in the above equation. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `MPEROD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the values that X(I,0,K) and X(I,M+1,K) are assumed to have. = 0  If X(I,0,K) = X(I,M,K) and X(I,M+1,K) = X(I,1,K). = 1  If X(I,0,K) = X(I,M+1,K) = 0. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are assumed to take on certain prescribed values described below. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * 1,K). 1,K). 1,K). 1,K). = 4  If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. = 4  If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. direction. M must be at least 3. and K=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of L,M and N. are assumed to take on certain prescribed values described below. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * 1,K). 1,K). 1,K). 1,K). = 4  If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. = 4  If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. direction. M must be at least 3. and K=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of L,M and N. not applicable or not stated by selected source not a workspace argument

## 6. `C2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1,K)-2.*X(I,J,K)+X(I,J+1,K)) The real constant which appears in the above equation. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `NPEROD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 0  If A(1) and C(N) are not zero. = 1  If A(1) = C(N) = 0. 0 the array elements must not depend upon the index K, but must be constant.  Specifically, the subroutine checks the following condition 0. To measure the accuracy of the algorithm a uniform random number generator was used to create 1 0  If A(1) and C(N) are not zero. = 1  If A(1) = C(N) = 0. 0 the array elements must not depend upon the index K, but must be constant.  Specifically, the subroutine checks the following condition 0. To measure the accuracy of the algorithm a uniform random number generator was used to create 1 not applicable or not stated by selected source not a workspace argument

## 8. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. direction. N must be at least 3. C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POIS3D was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J,K)-X(I,J,K)))/MAX(ABS(X(I,J,K))) where the two maxima are taken over I=1,2,...,L, direction. N must be at least 3. C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POIS3D was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J,K)-X(I,J,K)))/MAX(ABS(X(I,J,K))) where the two maxima are taken over I=1,2,...,L, not applicable or not stated by selected source not a workspace argument

## 9. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional block tridiagonal linear system which arises from a finite difference approximation to a three-dimensional Poisson equation using the Fourier transform package FFTPAK written by Paul Swarztrauber. 1)+B(K)*X(I,J,K)+C(K)*X(I,J,K+1) = F(I,J,K) for  I=1,2,...,L , J=1,2,...,M , and K=1,2,...,N . The indices K-1 and K+1 are evaluated modulo N, i.e. X(I,J,0) = X(I,J,N) and X(I,J,N+1) = X(I,J,1). The unknowns dimensional arrays of length N that specify the coefficients in the linear equations given above. C(1) dimensional array that specifies the values of the right side of the linear system of equations given above.  F must be dimensioned at least L x M x N. dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + solution array X for the system given in the 'PURPOSE' with 0.5*B(K) = 1,       K=1,2,...,N C(N) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POIS3D was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J,K)-X(I,J,K)))/MAX(ABS(X(I,J,K))) where the two maxima are taken over I=1,2,...,L, dimensional block tridiagonal linear system which arises from a finite difference approximation to a three-dimensional Poisson equation using the Fourier transform package FFTPAK written by Paul Swarztrauber. 1)+B(K)*X(I,J,K)+C(K)*X(I,J,K+1) = F(I,J,K) for  I=1,2,...,L , J=1,2,...,M , and K=1,2,...,N . The indices K-1 and K+1 are evaluated modulo N, i.e. X(I,J,0) = X(I,J,N) and X(I,J,N+1) = X(I,J,1). The unknowns dimensional arrays of length N that specify the coefficients in the linear equations given above. C(1) dimensional array that specifies the values of the right side of the linear system of equations given above.  F must be dimensioned at least L x M x N. dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + solution array X for the system given in the 'PURPOSE' with 0.5*B(K) = 1,       K=1,2,...,N C(N) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POIS3D was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J,K)-X(I,J,K)))/MAX(ABS(X(I,J,K))) where the two maxima are taken over I=1,2,...,L, not applicable or not stated by selected source not a workspace argument

## 10. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional arrays of length N that specify the coefficients in the linear equations given above. B(1) for K=1,2,...,N. dimensional arrays of length N that specify the coefficients in the linear equations given above. B(1) for K=1,2,...,N. not applicable or not stated by selected source not a workspace argument

## 11. `C`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional arrays of length N that specify the coefficients in the linear equations given above. C(1) 0.5*B(K) = 1,       K=1,2,...,N dimensional arrays of length N that specify the coefficients in the linear equations given above. C(1) 0.5*B(K) = 1,       K=1,2,...,N not applicable or not stated by selected source not a workspace argument

## 12. `LDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension must be at least L. dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension must be at least L. dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension must be at least L. not a workspace argument

## 13. `MDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension must be at least M. dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension must be at least M. not applicable or not stated by selected source not a workspace argument

## 14. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 3; dimensions (LDIMF, MDIMF, *). must be at least L. must be at least M. dimensional array that specifies the values of the right side of the linear system of equations given above.  F must be dimensioned at least L x M x N. Contains the solution X. must be at least L. must be at least M. dimensional array that specifies the values of the right side of the linear system of equations given above.  F must be dimensioned at least L x M x N. Contains the solution X. not applicable or not stated by selected source not a workspace argument

## 15. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0  No error = 1  If LPEROD .LT. 0 or .GT. 4 = 2  If L .LT. 3 = 3  If MPEROD .LT. 0 or .GT. 4 = 4  If M .LT. 3 = 5  If NPEROD .LT. 0 or .GT. 1 = 6  If N .LT. 3 = 7  If LDIMF .LT. L = 8  If MDIMF .LT. M = 9  If A(K) .NE. C(1) or C(K) .NE. C(1) or B(I) .NE.B(1) for some K=1,2,...,N. = 10 If NPEROD = 1 and A(1) .NE. 0 or C(N) .NE. 0 Since this is the only means of indicating a possibly incorrect call to POIS3D, the user should test IERROR after the call. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(N),B(N),C(N),F(LDIMF,MDIMF,N), Arguments      W(see argument list) Latest         December 1, 1978 Revision Subprograms    POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1,RFFTB, Required       RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF,COSQF1 COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI,CFFTI1, CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB,CFFTF, CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF,PIMACH, Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0  No error = 1  If LPEROD .LT. 0 or .GT. 4 = 2  If L .LT. 3 = 3  If MPEROD .LT. 0 or .GT. 4 = 4  If M .LT. 3 = 5  If NPEROD .LT. 0 or .GT. 1 = 6  If N .LT. 3 = 7  If LDIMF .LT. L = 8  If MDIMF .LT. M = 9  If A(K) .NE. C(1) or C(K) .NE. C(1) or B(I) .NE.B(1) for some K=1,2,...,N. = 10 If NPEROD = 1 and A(1) .NE. 0 or C(N) .NE. 0 Since this is the only means of indicating a possibly incorrect call to POIS3D, the user should test IERROR after the call. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(N),B(N),C(N),F(LDIMF,MDIMF,N), Arguments      W(see argument list) Latest         December 1, 1978 Revision Subprograms    POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1,RFFTB, Required       RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF,COSQF1 COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI,CFFTI1, CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB,CFFTF, CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF,PIMACH, Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source not a workspace argument

## 16. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `LPEROD`: not a workspace argument
- `L`: not a workspace argument
- `C1`: not a workspace argument
- `MPEROD`: not a workspace argument
- `M`: not a workspace argument
- `C2`: not a workspace argument
- `NPEROD`: not a workspace argument
- `N`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `C`: not a workspace argument
- `LDIMF`: not a workspace argument
- `MDIMF`: not a workspace argument
- `F`: not a workspace argument
- `IERROR`: not a workspace argument
- `W`: dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) +

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::pois3d`
- Original SLATEC routine: `POIS3D`
- Native symbol: `pois3d_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [POIS3D](https://www.netlib.org/slatec/fishfft/pois3d.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
