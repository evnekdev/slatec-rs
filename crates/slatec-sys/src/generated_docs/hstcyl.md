# Purpose

HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates

# Description

This canonical unsafe binding exposes original SLATEC routine `HSTCYL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HSTCYL](https://www.netlib.org/slatec/fishfft/hstcyl.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. dimensional Poisson equation. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * must be less than B and must be less than B and negative. dimensional array of length N that specifies the boundary values (if any) of the solution at R = A.  When MBDCND = 1 or 2, dimensional array of length N that specifies the boundary values of the solution at R = B.  When MBDCND = 1,4, or 5, specifies the boundary values of the solution at Z = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at Z = D.  when NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the modified Helmholtz equation.  For I=1,2,...,M and J=1,2,...,N dimensional array that must be provided by the user for work space.  W may require up to 13M + 4N + M*INT(log2(N)) locations.  The actual number of locations used is computed by HSTCYL and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCYL then computes this that a solution exists.  HSTCYL then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. dimensional Poisson equation. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * must be less than B and must be less than B and negative. dimensional array of length N that specifies the boundary values (if any) of the solution at R = A.  When MBDCND = 1 or 2, dimensional array of length N that specifies the boundary values of the solution at R = B.  When MBDCND = 1,4, or 5, specifies the boundary values of the solution at Z = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at Z = D.  when NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the modified Helmholtz equation.  For I=1,2,...,M and J=1,2,...,N dimensional array that must be provided by the user for work space.  W may require up to 13M + 4N + M*INT(log2(N)) locations.  The actual number of locations used is computed by HSTCYL and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCYL then computes this that a solution exists.  HSTCYL then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B and A)/M.  M must be greater than 2. must be less than B and A)/M.  M must be greater than 2. not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of grid points in the interval (A,B).  The grid points in the R-direction are given by R(I) = A + (I-0.5)DR for A)/M.  M must be greater than 2. specifies the boundary values of the solution at Z = C.   When NBDCND = 1 or 2, J=1,2,...,N. are listed in the table below. The solution process employed results in a loss of no more than four significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine POISTG which is the routine that actually solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) The number of grid points in the interval (A,B).  The grid points in the R-direction are given by R(I) = A + (I-0.5)DR for A)/M.  M must be greater than 2. specifies the boundary values of the solution at Z = C.   When NBDCND = 1 or 2, J=1,2,...,N. are listed in the table below. The solution process employed results in a loss of no more than four significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine POISTG which is the routine that actually solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) not applicable or not stated by selected source not a workspace argument

## 4. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at R = A and R = B. = 1  If the solution is specified at R = A (see note below) and R = B. = 2  If the solution is specified at R = A (see note below) and the derivative of the solution with respect to R is specified at R = B. = 3  If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4  If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5  If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6  If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE:  If A = 0, do not use MBDCND = 1,2,3, or 4, but instead 5 or 6.  The resulting approximation gives the only meaningful boundary condition, i.e. dU/dR = 0. (see D. Greenspan, 'Introductory Numerical Analysis Of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) 3 or 4, 5 or 6, BDA is a dummy variable. 2,3, or 6, 5 or 6. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BDA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(A,Z(J)) ,          J=1,2,...,N. (d/dR)U(A,Z(J)) ,    J=1,2,...,N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `BDB`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(B,Z(J)) ,          J=1,2,...,N. (d/dR)U(B,Z(J)) ,    J=1,2,...,N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less must be less than D. than D. must be less must be less than D. than D. not applicable or not stated by selected source not a workspace argument

## 8. `D`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results must be less than D. C)/N.  N must be greater than 2. + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results must be less than D. C)/N.  N must be greater than 2. not applicable or not stated by selected source not a workspace argument

## 9. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of unknowns in the interval (C,D).  The unknowns in the Z-direction are given by Z(J) = C + (J-0.5)DZ, C)/N.  N must be greater than 2. are listed in are listed in the table below. the table below. The solution process employed results in a loss The solution process employed results in a loss of no more than four significant digits for N and M of no more than four significant digits for N and M as large as 64.  More detailed information about as large as 64.  More detailed information about accuracy can be found in the documentation for accuracy can be found in the documentation for subroutine POISTG which is the routine that subroutine POISTG which is the routine that actually solves the finite difference equations. actually solves the finite difference equations. The number of unknowns in the interval (C,D).  The unknowns in the Z-direction are given by Z(J) = C + (J-0.5)DZ, C)/N.  N must be greater than 2. are listed in are listed in the table below. the table below. The solution process employed results in a loss The solution process employed results in a loss of no more than four significant digits for N and M of no more than four significant digits for N and M as large as 64.  More detailed information about as large as 64.  More detailed information about accuracy can be found in the documentation for accuracy can be found in the documentation for subroutine POISTG which is the routine that subroutine POISTG which is the routine that actually solves the finite difference equations. actually solves the finite difference equations. not applicable or not stated by selected source not a workspace argument

## 10. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at Z = C and Z = D. = 0  If the solution is periodic in Z, i.e. U(I,J) = U(I,N+J). = 1  If the solution is specified at Z = C and Z = D. = 2  If the solution is specified at Z = C and the derivative of the solution with respect to Z is specified at Z = D. = 3  If the derivative of the solution with respect to Z is specified at Z = C and Z = D. = 4  If the derivative of the solution with respect to Z is specified at Z = C and the solution is specified at Z = D. 3 or 4, 0, BDC is a dummy variable. 2 or 3, 0, BDD is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `BDC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(R(I),C) ,              I=1,2,...,M. (d/dZ)U(R(I),C),         I=1,2,...,M. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `BDD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(R(I),D) ,              I=1,2,...,M. (d/dZ)U(R(I),D) ,        I=1,2,...,M. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the modified Helmholtz equation.  If LAMBDA is greater than 0, a solution may not exist.  However, HSTCYL will attempt to find a solution.  LAMBDA must be zero The constant LAMBDA in the modified Helmholtz equation.  If LAMBDA is greater than 0, a solution may not exist.  However, HSTCYL will attempt to find a solution.  LAMBDA must be zero not applicable or not stated by selected source not a workspace argument

## 14. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). F(R(I),Z(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),Z(J)) for F(R(I),Z(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),Z(J)) for not applicable or not stated by selected source not a workspace argument

## 15. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HSTCYL.  This parameter is used to specify the must be at least M. The row (or first) dimension of the array F as it appears in the program calling HSTCYL.  This parameter is used to specify the must be at least M. The row (or first) dimension of the array F as it appears in the program calling HSTCYL.  This parameter is used to specify the must be at least M. not a workspace argument

## 16. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCYL then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. =  0  No error =  1  A .LT. 0 =  2  A .GE. B =  3  MBDCND .LT. 1 or MBDCND .GT. 6 =  4  C .GE. D =  5  N .LE. 2 =  6  NBDCND .LT. 0 or NBDCND .GT. 4 =  7  A = 0 and MBDCND = 1,2,3, or 4 =  8  A .GT. 0 and MBDCND .GE. 5 =  9  M .LE. 2 = 10  IDIMF .LT. M = 11  LAMBDA .GT. 0 = 12  A=0, MBDCND .GE. 5, ELMBDA .NE. 0 Since this is the only means of indicating a possibly incorrect call to HSTCYL, the user should test IERROR after the call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension OF   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    HSTCYL,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension OF   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    HSTCYL,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `M`: not a workspace argument
- `MBDCND`: not a workspace argument
- `BDA`: not a workspace argument
- `BDB`: not a workspace argument
- `C`: not a workspace argument
- `D`: not a workspace argument
- `N`: not a workspace argument
- `NBDCND`: not a workspace argument
- `BDC`: not a workspace argument
- `BDD`: not a workspace argument
- `ELMBDA`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `PERTRB`: not a workspace argument
- `IERROR`: not a workspace argument
- `W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension OF   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    HSTCYL,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hstcyl`
- Original SLATEC routine: `HSTCYL`
- Native symbol: `hstcyl_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HSTCYL](https://www.netlib.org/slatec/fishfft/hstcyl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
