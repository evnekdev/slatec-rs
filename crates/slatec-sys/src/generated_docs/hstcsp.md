# Purpose

HSTCSP solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation spherical coordinates assuming axisymmetry (no dependence on longitude).

# Description

This canonical unsafe binding exposes original SLATEC routine `HSTCSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HSTCSP](https://www.netlib.org/slatec/fishfft/hstcsp.f).

# Arguments

## 1. `INTL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. = 0  On initial entry to HSTCSP or if any of the arguments 1.  Once a call with INTL = 0 has been made then subsequent solutions corresponding to different F, BDA, BDB, BDC, and BDD can be obtained 1 since initialization is not repeated. 1. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(See argument list) Latest         June 1979 Revision Subprograms    HSTCSP,HSTCS1,BLKTRI,BLKTR1,INDXA,INDXB,INDXC, Required       PROD,PRODP,CPROD,CPRODP,PPADD,PSGF,BSRH,PPSGF, PPSPF,COMPB,TEVLS,R1MACH Special        NONE Conditions Common         CBLKT Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN Some values are listed in the table below. The solution process employed results in a loss of no more than FOUR significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine BLKTRI which is the routine that actually solves the finite difference equations. = 0  On initial entry to HSTCSP or if any of the arguments 1.  Once a call with INTL = 0 has been made then subsequent solutions corresponding to different F, BDA, BDB, BDC, and BDD can be obtained 1 since initialization is not repeated. 1. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(See argument list) Latest         June 1979 Revision Subprograms    HSTCSP,HSTCS1,BLKTRI,BLKTR1,INDXA,INDXB,INDXC, Required       PROD,PRODP,CPROD,CPRODP,PPADD,PSGF,BSRH,PPSGF, PPSPF,COMPB,TEVLS,R1MACH Special        NONE Conditions Common         CBLKT Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN Some values are listed in the table below. The solution process employed results in a loss of no more than FOUR significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine BLKTRI which is the routine that actually solves the finite difference equations. not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1.  Once a call with INTL = 0 has been made then subsequent solutions corresponding to different F, BDA, BDB, BDC, and BDD can be obtained The range of THETA (colatitude), i.e. A .LE. THETA .LE. B.  A negative.  A and B are in 0 corresponds to the north pole and B = PI corresponds to the south pole. * *  IMPORTANT  * * * 0  and/or B = PI the only meaningful boundary condition is dU/dTHETA = 0.  (See D. Greenspan, 'Numerical Analysis of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) dimensional array of length N that specifies the boundary values (if any) of the solution at THETA = A.  When dimensional array of length N that specifies the boundary values of the solution at THETA = B.  When MBDCND = 1, 4, or 5, specifies the boundary values of the solution at R = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at R = D.  When NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the modified Helmholtz equation.  For I=1,2,...,M and dimensional array that must be provided by the user for work space.  With K = INT(log2(N))+1 and L = 2**(K+1), W may require up to (K-2)*L+K+MAX(2N,6M)+4(N+M)+5 locations.  The actual number of locations used is computed by HSTCSP and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCSP then computes this that a solution exists.  HSTCSP then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. 1.  Once a call with INTL = 0 has been made then subsequent solutions corresponding to different F, BDA, BDB, BDC, and BDD can be obtained The range of THETA (colatitude), i.e. A .LE. THETA .LE. B.  A negative.  A and B are in 0 corresponds to the north pole and B = PI corresponds to the south pole. * *  IMPORTANT  * * * 0  and/or B = PI the only meaningful boundary condition is dU/dTHETA = 0.  (See D. Greenspan, 'Numerical Analysis of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) dimensional array of length N that specifies the boundary values (if any) of the solution at THETA = A.  When dimensional array of length N that specifies the boundary values of the solution at THETA = B.  When MBDCND = 1, 4, or 5, specifies the boundary values of the solution at R = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at R = D.  When NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the modified Helmholtz equation.  For I=1,2,...,M and dimensional array that must be provided by the user for work space.  With K = INT(log2(N))+1 and L = 2**(K+1), W may require up to (K-2)*L+K+MAX(2N,6M)+4(N+M)+5 locations.  The actual number of locations used is computed by HSTCSP and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCSP then computes this that a solution exists.  HSTCSP then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of THETA (colatitude), i.e. A .LE. THETA .LE. B.  A negative.  A and B are in must be computed using the statement must be computed using the statement PIMACH(DUM) This insures that B in the user's program is equal to PI in this program which permits several tests of the input parameters that otherwise would not be possible. * * * * * * * * * * * A)/M.  M must be greater than 4. PI, do not use MBDCND = 1,2,3,4,5 or 6, The range of THETA (colatitude), i.e. A .LE. THETA .LE. B.  A negative.  A and B are in must be computed using the statement must be computed using the statement PIMACH(DUM) This insures that B in the user's program is equal to PI in this program which permits several tests of the input parameters that otherwise would not be possible. * * * * * * * * * * * A)/M.  M must be greater than 4. PI, do not use MBDCND = 1,2,3,4,5 or 6, not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of grid points in the interval (A,B).  The grid points in the THETA-direction are given by THETA(I) = A + (I-0.5)DTHETA A)/M.  M must be greater than 4. specifies the boundary values of the solution at R = C.   When NBDCND = 1 or 2, J=1,2,...,N. N)     INTL      MBDCND(=NBDCND)     T(MSECS) The number of grid points in the interval (A,B).  The grid points in the THETA-direction are given by THETA(I) = A + (I-0.5)DTHETA A)/M.  M must be greater than 4. specifies the boundary values of the solution at R = C.   When NBDCND = 1 or 2, J=1,2,...,N. N)     INTL      MBDCND(=NBDCND)     T(MSECS) not applicable or not stated by selected source not a workspace argument

## 5. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at THETA = A and THETA = B. = 1  If the solution is specified at THETA = A and THETA = B. (See notes 1, 2 below) = 2  If the solution is specified at THETA = A and the derivative of the solution with respect to THETA is specified at THETA = B (See notes 1, 2 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = A (See notes 1, 2 below) and THETA = B. = 4  If the derivative of the solution with respect to THETA is specified at THETA = A (See notes 1, 2 below) and the solution is specified at THETA = B. = 5  If the solution is unspecified at THETA = A = 0 and the solution is specified at THETA = B. (See note 2 below) = 6  If the solution is unspecified at THETA = A = 0 and the derivative of the solution with respect to THETA is specified at THETA = B (See note 2 below). = 7  If the solution is specified at THETA = A and the solution is unspecified at THETA = B = PI. = 8  If the derivative of the solution with respect to THETA is specified at THETA = A (See note 1 below) and the solution is unspecified at THETA = B = PI. = 9  If the solution is unspecified at THETA = A = 0 and THETA = B = PI. NOTES:  1.  If A = 0, do not use MBDCND = 1,2,3,4,7 or 8, 5, 6, or 9. 7, 8, or 9. 1, 2, or 7, 3, 4, or 8, is a dummy variable. 2,3, or 6, is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `BDA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(A,R(J)) ,              J=1,2,...,N. (d/dTHETA)U(A,R(J)) ,    J=1,2,...,N. is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `BDB`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(B,R(J)) ,              J=1,2,...,N. (d/dTHETA)U(B,R(J)) ,    J=1,2,...,N. is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. are changed from a previous call. are all unchanged from previous call to HSTCSP. NOTE:  A call with INTL = 0 takes approximately 1.5 times as much The range of R , i.e. C .LE. R .LE. D. negative. negative. But in these cases the right side of the system will be perturbed by the constant PERTRB. NOTE 2:  NBDCND = 5 or 6 cannot be used with MBDCND = 1, 2, 4, 5, or 7 (the former indicates that the solution is unspecified at R = 0; the latter indicates that the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `D`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. + + (LAMBDA/(R*SIN(THETA))**2)U  =  F(THETA,R) where THETA is colatitude and R is the radial coordinate. This two-dimensional modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * are changed from a previous call. are all unchanged from previous call to HSTCSP. NOTE:  A call with INTL = 0 takes approximately 1.5 times as much The range of R , i.e. C .LE. R .LE. D. negative. C)/N.  N must be greater than 4. + + (LAMBDA/(R*SIN(THETA))**2)U  =  F(THETA,R) where THETA is colatitude and R is the radial coordinate. This two-dimensional modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * are changed from a previous call. are all unchanged from previous call to HSTCSP. NOTE:  A call with INTL = 0 takes approximately 1.5 times as much The range of R , i.e. C .LE. R .LE. D. negative. C)/N.  N must be greater than 4. not applicable or not stated by selected source not a workspace argument

## 10. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are changed from a previous call. are all unchanged from previous call to HSTCSP. NOTE:  A call with INTL = 0 takes approximately 1.5 times as much The number of unknowns in the interval (C,D).  The unknowns in the R-direction are given by R(J) = C + (J-0.5)DR, C)/N.  N must be greater than 4. are changed from a previous call. are all unchanged from previous call to HSTCSP. NOTE:  A call with INTL = 0 takes approximately 1.5 times as much The number of unknowns in the interval (C,D).  The unknowns in the R-direction are given by R(J) = C + (J-0.5)DR, C)/N.  N must be greater than 4. not applicable or not stated by selected source not a workspace argument

## 11. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are changed from a previous call. are all unchanged from previous call to HSTCSP. NOTE:  A call with INTL = 0 takes approximately 1.5 times as much Indicates the type of boundary conditions at R = C and R = D. = 1  If the solution is specified at R = C and R = D. = 2  If the solution is specified at R = C and the derivative of the solution with respect to R is specified at R = D. (See note 1 below) = 3  If the derivative of the solution with respect to R is specified at R = C and R = D. = 4  If the derivative of the solution with respect to R is specified at R = C and the solution is specified at R = D. = 5  If the solution is unspecified at R = C = 0 (See note 2 below) and the solution is specified at R = D. = 6  If the solution is unspecified at R = C = 0 (See note 2 below) and the derivative of the solution with respect to R is specified at R = D. NOTE 1:  If C = 0 and MBDCND = 3,6,8 or 9, the system of equations to be solved is singular.  The unique solution is determined by extrapolation to the specification of 1 or 2. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `BDC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(THETA(I),C) ,              I=1,2,...,M. (d/dR)U(THETA(I),C),         I=1,2,...,M. is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `BDD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). U(THETA(I),D) ,              I=1,2,...,M. (d/dR)U(THETA(I),D) ,        I=1,2,...,M. is a dummy variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the modified Helmholtz equation.  If LAMBDA is greater than 0, a solution may not exist.  However, HSTCSP will attempt to find a solution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 15. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). F(THETA(I),R(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),R(J)) for F(THETA(I),R(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),R(J)) for not applicable or not stated by selected source not a workspace argument

## 16. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HSTCSP.  This parameter is used to specify the must be at least M. The row (or first) dimension of the array F as it appears in the program calling HSTCSP.  This parameter is used to specify the must be at least M. The row (or first) dimension of the array F as it appears in the program calling HSTCSP.  This parameter is used to specify the must be at least M. not a workspace argument

## 17. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCSP then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters. Except for numbers 0 and 10, a solution is not attempted. =  0  No error =  1  A .LT. 0 or B .GT. PI =  2  A .GE. B =  3  MBDCND .LT. 1 or MBDCND .GT. 9 =  4  C .LT. 0 =  5  C .GE. D =  6  NBDCND .LT. 1 or NBDCND .GT. 6 =  7  N .LT. 5 =  8  NBDCND = 5 or 6 and MBDCND = 1, 2, 4, 5, or 7 =  9  C .GT. 0 and NBDCND .GE. 5 = 10  ELMBDA .GT. 0 = 11  IDIMF .LT. M = 12  M .LT. 5 = 13  A = 0 and MBDCND =1,2,3,4,7 or 8 = 14  B = PI and MBDCND .LE. 6 = 15  A .GT. 0 and MBDCND = 5, 6, or 9 = 16  B .LT. PI and MBDCND .GE. 7 = 17  LAMBDA .NE. 0 and NBDCND .GE. 5 Since this is the only means of indicating a possibly incorrect call to HSTCSP, the user should test IERROR after the call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 19. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the required length of W.  Also  W contains intermediate values that must not be destroyed if HSTCSP contains the required length of W.  Also  W contains intermediate values that must not be destroyed if HSTCSP not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `INTL`: not a workspace argument
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
- `W`: contains the required length of W.  Also  W contains intermediate values that must not be destroyed if HSTCSP

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hstcsp`
- Original SLATEC routine: `HSTCSP`
- Native symbol: `hstcsp_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HSTCSP](https://www.netlib.org/slatec/fishfft/hstcsp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
