# Purpose

Subroutine HWSCRT solves the standard five-point finite difference approximation to the Helmholtz equation in Cartesian coordinates:

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSCRT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSCRT](https://www.netlib.org/slatec/fishfft/hwscrt.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B. must be less than B. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = B. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = C. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = D. dimensional array which specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M and J = 2,3,...,N 1,2,...,N+1 3        F(A,Y(J))     F(B,Y(J)) 4        F(A,Y(J))     U(B,Y(J)) dimensional array that must be provided by the user for work space.  W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations.  The actual number of locations used is computed by HWSCRT and is returned in location 0), a solution may not exist.  PERTRB is a constant, calculated and subtracted from solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. must be less than B. must be less than B. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = B. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = C. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = D. dimensional array which specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M and J = 2,3,...,N 1,2,...,N+1 3        F(A,Y(J))     F(B,Y(J)) 4        F(A,Y(J))     U(B,Y(J)) dimensional array that must be provided by the user for work space.  W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations.  The actual number of locations used is computed by HWSCRT and is returned in location 0), a solution may not exist.  PERTRB is a constant, calculated and subtracted from solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B. A)/M is the panel width. M must be greater than 3. 1,2,...,N+1 3        F(A,Y(J))     F(B,Y(J)) 4        F(A,Y(J))     U(B,Y(J)) must be less than B. A)/M is the panel width. M must be greater than 3. 1,2,...,N+1 3        F(A,Y(J))     F(B,Y(J)) 4        F(A,Y(J))     U(B,Y(J)) not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (A,B) is subdivided.  Hence, there will be M+1 grid points in the X-direction given by X(I) = A+(I-1)DX for I = 1,2,...,M+1, as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at X = A and X = B. = 0  If the solution is periodic in X, i.e., U(I,J) = U(M+I,J). = 1  If the solution is specified at X = A and X = B. = 2  If the solution is specified at X = A and the derivative of the solution with respect to X is specified at X = B. = 3  If the derivative of the solution with respect to X is specified at X = A and X = B. = 4  If the derivative of the solution with respect to X is specified at X = A and the solution is specified at X = B. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(1,J)        F(M+1,J) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BDA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dX)U(A,Y(J)), J = 1,2,...,N+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `BDB`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dX)U(B,Y(J)), J = 1,2,...,N+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than D. must be less than D. 1,2,...,M+1 3        F(X(I),C)     F(X(I),D) 4        F(X(I),C)     U(X(I),D) must be less than D. must be less than D. 1,2,...,M+1 3        F(X(I),C)     F(X(I),D) 4        F(X(I),C)     U(X(I),D) not applicable or not stated by selected source not a workspace argument

## 8. `D`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. F(X,Y). F(X,Y). * * * * * * *    Parameter Description     * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N is the panel width.  N must be greater than 3. 1,2,...,M+1 3        F(X(I),C)     F(X(I),D) 4        F(X(I),C)     U(X(I),D) F(X,Y). F(X,Y). * * * * * * *    Parameter Description     * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N is the panel width.  N must be greater than 3. 1,2,...,M+1 3        F(X(I),C)     F(X(I),D) 4        F(X(I),C)     U(X(I),D) not applicable or not stated by selected source not a workspace argument

## 9. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (C,D) is subdivided.  Hence, there will be N+1 grid points in the Y-direction given by Y(J) = C+(J-1)DY for J = 1,2,...,N+1, where not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at Y = C and Y = D. = 0  If the solution is periodic in Y, i.e., U(I,J) = U(I,N+J). = 1  If the solution is specified at Y = C and Y = D. = 2  If the solution is specified at Y = C and the derivative of the solution with respect to Y is specified at Y = D. = 3  If the derivative of the solution with respect to Y is specified at Y = C and Y = D. = 4  If the derivative of the solution with respect to Y is specified at Y = C and the solution is specified at Y = D. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(I,1)        F(I,N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `BDC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dY)U(X(I),C), I = 1,2,...,M+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `BDD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dY)U(X(I),D), I = 1,2,...,M+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the Helmholtz equation.  If LAMBDA .GT. 0, a solution may not exist.  However, HWSCRT will attempt to find a solution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). F(X(I),Y(J)). On the boundaries F is defined by 1,2,...,N+1 3        F(A,Y(J))     F(B,Y(J)) 4        F(A,Y(J))     U(B,Y(J)) 1,2,...,M+1 3        F(X(I),C)     F(X(I),D) 4        F(X(I),C)     U(X(I),D) must be dimensioned at least (M+1)*(N+1). NOTE: If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (X(I),Y(J)), I = 1,2,...,M+1, J = 1,2,...,N+1  . which ensures that a solution exists.  HWSCRT then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  The value of F(X(I),Y(J)). On the boundaries F is defined by 1,2,...,N+1 3        F(A,Y(J))     F(B,Y(J)) 4        F(A,Y(J))     U(B,Y(J)) 1,2,...,M+1 3        F(X(I),C)     F(X(I),D) 4        F(X(I),C)     U(X(I),D) must be dimensioned at least (M+1)*(N+1). NOTE: If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (X(I),Y(J)), I = 1,2,...,M+1, J = 1,2,...,N+1  . which ensures that a solution exists.  HWSCRT then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  The value of not applicable or not stated by selected source not a workspace argument

## 15. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HWSCRT.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSCRT.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSCRT.  This parameter is used to specify the must be at least M+1  . not a workspace argument

## 16. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If a combination of periodic or derivative boundary conditions should be small compared to the right side F.  Otherwise, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters.  Except for numbers 0 and 6, a solution is not attempted. = 0  No error. = 1  A .GE. B. = 2  MBDCND .LT. 0 or MBDCND .GT. 4  . = 3  C .GE. D. = 4  N .LE. 3 = 5  NBDCND .LT. 0 or NBDCND .GT. 4  . = 6  LAMBDA .GT. 0  . = 7  IDIMF .LT. M+1  . = 8  M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSCRT, the user should test IERROR after the call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSCRT,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSCRT,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source

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
- `W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSCRT,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwscrt`
- Original SLATEC routine: `HWSCRT`
- Native symbol: `hwscrt_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [HWSCRT](https://www.netlib.org/slatec/fishfft/hwscrt.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
