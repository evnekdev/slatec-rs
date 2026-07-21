# Purpose

Subroutine HWSPLR solves a finite difference approximation to the Helmholtz equation in polar coordinates:

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSPLR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSPLR](https://www.netlib.org/slatec/fishfft/hwsplr.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B must be less than B negative. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = C.  When NBDCND = 3 or 4, dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = D.  When NBDCND = 2 or 3, dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M and J = 2,3,...,N 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) dimensional array that must be provided by the user for work space.  W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations.  The actual number of locations used is computed by HWSPLR and is returned in location is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSPLR then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  PERTRB should be small compared to the right side. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. must be less than B must be less than B negative. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = C.  When NBDCND = 3 or 4, dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = D.  When NBDCND = 2 or 3, dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M and J = 2,3,...,N 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) dimensional array that must be provided by the user for work space.  W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations.  The actual number of locations used is computed by HWSPLR and is returned in location is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSPLR then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  PERTRB should be small compared to the right side. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B A)/M is the panel width. M must be greater than 3. 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) must be less than B A)/M is the panel width. M must be greater than 3. 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (A,B) is subdivided.  Hence, there will be M+1 grid points in the R-direction given by R(I) = A+(I-1)DR, for I = 1,2,...,M+1, as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary condition at R = A and R = B. = 1  If the solution is specified at R = A and R = B. = 2  If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. = 3  If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4  If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5  If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6  If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE:  If A = 0, do not use MBDCND = 3 or 4, but instead use 1,2,5, or 6  . 3 or 4, is a dummy variable. 2,3, or 6, is a dummy variable. F(1,J)            F(M+1,J) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BDA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dR)U(A,THETA(J)), J = 1,2,...,N+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `BDB`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dR)U(B,THETA(J)), J = 1,2,...,N+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less must be less than D. than D. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) must be less must be less than D. than D. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) not applicable or not stated by selected source not a workspace argument

## 8. `D`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA). * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N is the panel width.  N must be greater than 3. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA). * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N is the panel width.  N must be greater than 3. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) not applicable or not stated by selected source not a workspace argument

## 9. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (C,D) is subdivided.  Hence, there will be N+1 grid points in the THETA-direction given by THETA(J) = C+(J-1)DTHETA for C)/N is the panel width.  N must be greater than 3. The number of panels into which the interval (C,D) is subdivided.  Hence, there will be N+1 grid points in the THETA-direction given by THETA(J) = C+(J-1)DTHETA for C)/N is the panel width.  N must be greater than 3. not applicable or not stated by selected source not a workspace argument

## 10. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at THETA = C and at THETA = D. = 0  If the solution is periodic in THETA, i.e., U(I,J) = U(I,N+J). = 1  If the solution is specified at THETA = C and THETA = D (see note below). = 2  If the solution is specified at THETA = C and the derivative of the solution with respect to THETA is specified at THETA = D (see note below). = 4  If the derivative of the solution with respect to THETA is specified at THETA = C and the solution is specified at THETA = D (see note below). NOTE:  When NBDCND = 1,2, or 4, do not use MBDCND = 5 or 6 (the former indicates that the solution is specified at R = 0, the latter indicates the solution is unspecified at R = 0).  Use instead MBDCND = 1 or 2  . is a dummy variable. is a dummy variable. F(I,1)            F(I,N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `BDC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dTHETA)U(R(I),C), I = 1,2,...,M+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `BDD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (d/dTHETA)U(R(I),D), I = 1,2,...,M+1  . is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the Helmholtz equation.  If LAMBDA .LT. 0, a solution may not exist.  However, HWSPLR will attempt to find a solution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). F(R(I),THETA(J)). On the boundaries F is defined by 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)), I = 1,2,...,M+1, J = 1,2,...,N+1  . F(R(I),THETA(J)). On the boundaries F is defined by 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)), I = 1,2,...,M+1, J = 1,2,...,N+1  . not applicable or not stated by selected source not a workspace argument

## 15. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HWSPLR.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSPLR.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSPLR.  This parameter is used to specify the must be at least M+1  . not a workspace argument

## 16. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSPLR then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  PERTRB should be small compared to the right side. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters.  Except for numbers 0 and 11, a solution is not attempted. =  0  No error. =  1  A .LT. 0  . =  2  A .GE. B. =  3  MBDCND .LT. 1 or MBDCND .GT. 6  . =  4  C .GE. D. =  5  N .LE. 3 =  6  NBDCND .LT. 0 or .GT. 4  . =  7  A = 0, MBDCND = 3 or 4  . =  8  A .GT. 0, MBDCND .GE. 5  . =  9  MBDCND .GE. 5, NBDCND .NE. 0 and NBDCND .NE. 3  . = 10  IDIMF .LT. M+1  . = 11  LAMBDA .GT. 0  . = 12  M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSPLR, the user should test IERROR after the call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSPLR,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        None Conditions Common         NONE Blocks I/O Precision      Single Specialist     Roland Sweet Language       FORTRAN contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSPLR,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        None Conditions Common         NONE Blocks I/O Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source

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
- `W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSPLR,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        None Conditions Common         NONE Blocks I/O Precision      Single Specialist     Roland Sweet Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwsplr`
- Original SLATEC routine: `HWSPLR`
- Native symbol: `hwsplr_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HWSPLR](https://www.netlib.org/slatec/fishfft/hwsplr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
