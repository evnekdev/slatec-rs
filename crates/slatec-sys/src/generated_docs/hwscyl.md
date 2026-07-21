# Purpose

Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates:

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSCYL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSCYL](https://www.netlib.org/slatec/fishfft/hwscyl.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B must be less than B negative. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = C. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = D. dimensional array that specifies the values of the right side of the Helmholtz equation and boundary data (if any).  For I = 2,3,...,M and J = 2,3,...,N 1,2,...,N+1 4      F(A,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) dimensional array that must be provided by the user for work space.  W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations.  The actual number of locations used is computed by HWSCYL and is returned in location is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSCYL then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  The value of PERTRB should be small compared to the is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. must be less than B must be less than B negative. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = C. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = D. dimensional array that specifies the values of the right side of the Helmholtz equation and boundary data (if any).  For I = 2,3,...,M and J = 2,3,...,N 1,2,...,N+1 4      F(A,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) dimensional array that must be provided by the user for work space.  W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations.  The actual number of locations used is computed by HWSCYL and is returned in location is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSCYL then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  The value of PERTRB should be small compared to the is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B A)/M is the panel width. M must be greater than 3. 1,2,...,N+1 4      F(A,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) must be less than B A)/M is the panel width. M must be greater than 3. 1,2,...,N+1 4      F(A,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (A,B) is subdivided.  Hence, there will be M+1 grid points in the R-direction given by R(I) = A+(I-1)DR, for I = 1,2,...,M+1, as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at R = A and R = B. = 1  If the solution is specified at R = A and R = B. = 2  If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. = 3  If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4  If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5  If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6  If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE:  If A = 0, do not use MBDCND = 3 or 4, but instead use 1,2,5, or 6  . 3 or 4, is a dummy variable. 2,3, or 6, is a dummy variable. 5 or 6  . F(1,J)            F(M+1,J) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BDA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dR)U(A,Z(J)), J = 1,2,...,N+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `BDB`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dR)U(B,Z(J)), J = 1,2,...,N+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than D. must be less than D. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) must be less than D. must be less than D. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) not applicable or not stated by selected source not a workspace argument

## 8. `D`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N is the panel width. N must be greater than 3. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N is the panel width. N must be greater than 3. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) not applicable or not stated by selected source not a workspace argument

## 9. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (C,D) is subdivided.  Hence, there will be N+1 grid points in the Z-direction given by Z(J) = C+(J-1)DZ, for J = 1,2,...,N+1, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at Z = C and Z = D. = 0  If the solution is periodic in Z, i.e., U(I,1) = U(I,N+1). = 1  If the solution is specified at Z = C and Z = D. = 2  If the solution is specified at Z = C and the derivative of the solution with respect to Z is specified at Z = D. = 3  If the derivative of the solution with respect to Z is specified at Z = C and Z = D. = 4  If the derivative of the solution with respect to Z is specified at Z = C and the solution is specified at Z = D. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(I,1)            F(I,N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `BDC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dZ)U(R(I),C), I = 1,2,...,M+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `BDD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dZ)U(R(I),D), I = 1,2,...,M+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the Helmholtz equation.  If LAMBDA .GT. 0, a solution may not exist.  However, HWSCYL will attempt to find a solution.  LAMBDA must be zero when The constant LAMBDA in the Helmholtz equation.  If LAMBDA .GT. 0, a solution may not exist.  However, HWSCYL will attempt to find a solution.  LAMBDA must be zero when not applicable or not stated by selected source not a workspace argument

## 14. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). F(R(I),Z(J)). On the boundaries F is defined by 1,2,...,N+1 1,2,...,N+1 4      F(A,Z(J))         U(B,Z(J)) 4      F(A,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),Z(J)), I = 1,2,...,M+1, J = 1,2,...,N+1  . is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. F(R(I),Z(J)). On the boundaries F is defined by 1,2,...,N+1 1,2,...,N+1 4      F(A,Z(J))         U(B,Z(J)) 4      F(A,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 5      F(0,Z(J))         U(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) 6      F(0,Z(J))         F(B,Z(J)) 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),Z(J)), I = 1,2,...,M+1, J = 1,2,...,N+1  . is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 15. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HWSCYL.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSCYL.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSCYL.  This parameter is used to specify the must be at least M+1  . not a workspace argument

## 16. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If one specifies a combination of periodic, derivative, and unspecified boundary conditions for a Poisson equation is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSCYL then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  The value of PERTRB should be small compared to the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag which indicates invalid input parameters.  Except for numbers 0 and 11, a solution is not attempted. =  0  No error. =  1  A .LT. 0  . =  2  A .GE. B. =  3  MBDCND .LT. 1 or MBDCND .GT. 6  . =  4  C .GE. D. =  5  N .LE. 3 =  6  NBDCND .LT. 0 or NBDCND .GT. 4  . =  7  A = 0, MBDCND = 3 or 4  . =  8  A .GT. 0, MBDCND .GE. 5  . =  9  A = 0, LAMBDA .NE. 0, MBDCND .GE. 5  . = 10  IDIMF .LT. M+1  . = 11  LAMBDA .GT. 0  . = 12  M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSCYL, the user should test IERROR after the call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSCYL,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSCYL,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source

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
- `W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSCYL,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwscyl`
- Original SLATEC routine: `HWSCYL`
- Native symbol: `hwscyl_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HWSCYL](https://www.netlib.org/slatec/fishfft/hwscyl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
