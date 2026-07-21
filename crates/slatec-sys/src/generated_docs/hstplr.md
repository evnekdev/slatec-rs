# Purpose

HSTPLR solves the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in polar coordinates

# Description

This canonical unsafe binding exposes original SLATEC routine `HSTPLR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HSTPLR](https://www.netlib.org/slatec/fishfft/hstplr.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B and must be less than B and negative. dimensional array of length N that specifies the boundary values (if any) of the solution at R = A.  When MBDCND = 1 or 2, dimensional array of length N that specifies the boundary values of the solution at R = B.  When MBDCND = 1,4, or 5, specifies the boundary values of the solution at THETA = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at THETA = D.  When NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the Helmholtz equation.  For I=1,2,...,M and J=1,2,...,N dimensional array that must be provided by the user for work space.  W may require up to 13M + 4N + M*INT(log2(N)) locations.  The actual number of locations used is computed by HSTPLR and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTPLR then computes this that a solution exists.  HSTPLR then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. must be less than B and must be less than B and negative. dimensional array of length N that specifies the boundary values (if any) of the solution at R = A.  When MBDCND = 1 or 2, dimensional array of length N that specifies the boundary values of the solution at R = B.  When MBDCND = 1,4, or 5, specifies the boundary values of the solution at THETA = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at THETA = D.  When NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the Helmholtz equation.  For I=1,2,...,M and J=1,2,...,N dimensional array that must be provided by the user for work space.  W may require up to 13M + 4N + M*INT(log2(N)) locations.  The actual number of locations used is computed by HSTPLR and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTPLR then computes this that a solution exists.  HSTPLR then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less than B and A)/M.  M must be greater than 2. must be less than B and A)/M.  M must be greater than 2. not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of grid points in the interval (A,B).  The grid points in the R-direction are given by R(I) = A + (I-0.5)DR for A)/M.  M must be greater than 2. specifies the boundary values of the solution at THETA = C.   When NBDCND = 1 or 2, J=1,2,...,N. are listed in the table below. The solution process employed results in a loss of no more than four significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine POISTG which is the routine that actually solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) The number of grid points in the interval (A,B).  The grid points in the R-direction are given by R(I) = A + (I-0.5)DR for A)/M.  M must be greater than 2. specifies the boundary values of the solution at THETA = C.   When NBDCND = 1 or 2, J=1,2,...,N. are listed in the table below. The solution process employed results in a loss of no more than four significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine POISTG which is the routine that actually solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) not applicable or not stated by selected source not a workspace argument

## 4. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at R = A and R = B. = 1  If the solution is specified at R = A and R = B. = 2  If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. (see note 1 below) = 3  If the derivative of the solution with respect to R is specified at R = A (see note 2 below) and R = B. = 4  If the derivative of the solution with respect to R is specified at R = A (see note 2 below) and the solution is specified at R = B. = 5  If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6  If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE 1:  If A = 0, MBDCND = 2, and NBDCND = 0 or 3, the system of equations to be solved is singular.  The unique solution is determined by extrapolation to the specification of U(0,THETA(1)).  But in this case the right side of the system will be perturbed by the constant PERTRB. NOTE 2:  If A = 0, do not use MBDCND = 3 or 4, but instead use 1,2,5, or 6. 3 or 4, 5 or 6, BDA is a dummy variable. 2,3, or 6, 1 or 2. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BDA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(A,THETA(J)) ,          J=1,2,...,N. (d/dR)U(A,THETA(J)) ,    J=1,2,...,N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `BDB`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(B,THETA(J)) ,          J=1,2,...,N. (d/dR)U(B,THETA(J)) ,    J=1,2,...,N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. must be less must be less than D. than D. must be less must be less than D. than D. not applicable or not stated by selected source not a workspace argument

## 8. `D`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA) * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N.  N must be greater than 2. + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA) * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N.  N must be greater than 2. not applicable or not stated by selected source not a workspace argument

## 9. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of unknowns in the interval (C,D).  The unknowns in the THETA-direction are given by THETA(J) = C + (J-0.5)DT, C)/N.  N must be greater than 2. are listed in are listed in the table below. the table below. The solution process employed results in a loss The solution process employed results in a loss of no more than four significant digits for N and M of no more than four significant digits for N and M as large as 64.  More detailed information about as large as 64.  More detailed information about accuracy can be found in the documentation for accuracy can be found in the documentation for subroutine POISTG which is the routine that subroutine POISTG which is the routine that actually solves the finite difference equations. actually solves the finite difference equations. The number of unknowns in the interval (C,D).  The unknowns in the THETA-direction are given by THETA(J) = C + (J-0.5)DT, C)/N.  N must be greater than 2. are listed in are listed in the table below. the table below. The solution process employed results in a loss The solution process employed results in a loss of no more than four significant digits for N and M of no more than four significant digits for N and M as large as 64.  More detailed information about as large as 64.  More detailed information about accuracy can be found in the documentation for accuracy can be found in the documentation for subroutine POISTG which is the routine that subroutine POISTG which is the routine that actually solves the finite difference equations. actually solves the finite difference equations. not applicable or not stated by selected source not a workspace argument

## 10. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at THETA = C and THETA = D. = 0  If the solution is periodic in THETA, i.e. U(I,J) = U(I,N+J). = 1  If the solution is specified at THETA = C and THETA = D (see note below). = 2  If the solution is specified at THETA = C and the derivative of the solution with respect to THETA is specified at THETA = D (see note below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = C and THETA = D. = 4  If the derivative of the solution with respect to THETA is specified at THETA = C and the solution is specified at THETA = d (see note below). NOTE:  When NBDCND = 1, 2, or 4, do not use MBDCND = 5 or 6 (the former indicates that the solution is specified at R =  0; the latter indicates the solution is unspecified at R = 0).  Use 3 or 4, 0, BDC is a dummy variable. 2 or 3, 0, BDD is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `BDC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(R(I),C) ,              I=1,2,...,M. (d/dTHETA)U(R(I),C),     I=1,2,...,M. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `BDD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(R(I),D) ,              I=1,2,...,M. (d/dTHETA)U(R(I),D) ,    I=1,2,...,M. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the Helmholtz equation.  If LAMBDA is greater than 0, a solution may not exist.  However, HSTPLR will attempt to find a solution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). F(R(I),THETA(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)) for F(R(I),THETA(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)) for not applicable or not stated by selected source not a workspace argument

## 15. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HSTPLR.  This parameter is used to specify the must be at least M. The row (or first) dimension of the array F as it appears in the program calling HSTPLR.  This parameter is used to specify the must be at least M. The row (or first) dimension of the array F as it appears in the program calling HSTPLR.  This parameter is used to specify the must be at least M. not a workspace argument

## 16. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation stant, calculated and subtracted from F, which ensures that a solution exists.  HSTPLR then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. =  0  No error =  1  A .LT. 0 =  2  A .GE. B =  3  MBDCND .LT. 1 or MBDCND .GT. 6 =  4  C .GE. D =  5  N .LE. 2 =  6  NBDCND .LT. 0 or NBDCND .GT. 4 =  7  A = 0 and MBDCND = 3 or 4 =  8  A .GT. 0 and MBDCND .GE. 5 =  9  MBDCND .GE. 5 and NBDCND .NE. 0 or 3 = 10  IDIMF .LT. M = 11  LAMBDA .GT. 0 = 12  M .LE. 2 Since this is the only means of indicating a possibly incorrect call to HSTPLR, the user should test IERROR after the call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see ARGUMENT LIST) Latest         June 1, 1977 Revision Subprograms    HSTPLR,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see ARGUMENT LIST) Latest         June 1, 1977 Revision Subprograms    HSTPLR,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source

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
- `W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see ARGUMENT LIST) Latest         June 1, 1977 Revision Subprograms    HSTPLR,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hstplr`
- Original SLATEC routine: `HSTPLR`
- Native symbol: `hstplr_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HSTPLR](https://www.netlib.org/slatec/fishfft/hstplr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
