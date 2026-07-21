# Purpose

Subroutine HW3CRT solves the standard seven-point finite difference approximation to the Helmholtz equation in Cartesian coordinates: (d/dX)(dU/dX) + (d/dY)(dU/dY) + (d/dZ)(dU/dZ) + LAMBDA*U = F(X,Y,Z) . * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HW3CRT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HW3CRT](https://www.netlib.org/slatec/fishfft/hw3crt.f).

# Arguments

## 1. `XS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of X, i.e. XS .LE. X .LE. XF . must be less than XF. 1,2,...,M+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) The range of X, i.e. XS .LE. X .LE. XF . must be less than XF. 1,2,...,M+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) not applicable or not stated by selected source not a workspace argument

## 2. `XF`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of X, i.e. XS .LE. X .LE. XF . XS)/L is the panel width.  L must be at least 5 . 1,2,...,M+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) The range of X, i.e. XS .LE. X .LE. XF . XS)/L is the panel width.  L must be at least 5 . 1,2,...,M+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) not applicable or not stated by selected source not a workspace argument

## 3. `L`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (XS,XF) is subdivided.  Hence, there will be L+1 grid points in the X-direction given by X(I) = XS+(I-1)DX for I=1,2,...,L+1, U(I,J,K). = 1  If the solution is specified at X = XS and X = XF. = 2  If the solution is specified at X = XS and the derivative of the solution with respect to X is specified at X = XF. = 3  If the derivative of the solution with respect to X is specified at X = XS and X = XF. = 4  If the derivative of the solution with respect to X is specified at X = XS and the solution is specified at X=XF. J=1,2,...,M+1, and K=1,2,...,N+1. M=N)     LBDCND(=MBDCND=NBDCND)      T(MSECS) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `LBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at X = XS and X = XF. = 0  If the solution is periodic in X, i.e. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(1,J,K)         F(L+1,J,K) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BDXS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (MDIMF, *). A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XS. (d/dX)U(XS,Y(J),Z(K)), J=1,2,...,M+1, is a dummy variable. must be dimensioned at least (M+1)*(N+1). A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XS. (d/dX)U(XS,Y(J),Z(K)), J=1,2,...,M+1, is a dummy variable. must be dimensioned at least (M+1)*(N+1). not applicable or not stated by selected source not a workspace argument

## 6. `BDXF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (MDIMF, *). A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XF. (d/dX)U(XF,Y(J),Z(K)), J=1,2,...,M+1, is a dummy variable. must be dimensioned at least (M+1)*(N+1). A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XF. (d/dX)U(XF,Y(J),Z(K)), J=1,2,...,M+1, is a dummy variable. must be dimensioned at least (M+1)*(N+1). not applicable or not stated by selected source not a workspace argument

## 7. `YS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of Y, i.e. YS .LE. Y .LE. YF. must be less than YF. 1,2,...,L+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) The range of Y, i.e. YS .LE. Y .LE. YF. must be less than YF. 1,2,...,L+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) not applicable or not stated by selected source not a workspace argument

## 8. `YF`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of Y, i.e. YS .LE. Y .LE. YF. YS)/M is the panel width.  M must be at least 5 . 1,2,...,L+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) The range of Y, i.e. YS .LE. Y .LE. YF. YS)/M is the panel width.  M must be at least 5 . 1,2,...,L+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) not applicable or not stated by selected source not a workspace argument

## 9. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (YS,YF) is subdivided.  Hence, there will be M+1 grid points in the Y-direction given by Y(J) = YS+(J-1)DY for J=1,2,...,M+1, U(I,J,K). = 1  If the solution is specified at Y = YS and Y = YF. = 2  If the solution is specified at Y = YS and the derivative of the solution with respect to Y is specified at Y = YF. = 3  If the derivative of the solution with respect to Y is specified at Y = YS and Y = YF. = 4  If the derivative of the solution with respect to Y is specified at Y = YS and the solution is specified at Y=YF. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at Y = YS and Y = YF. = 0  If the solution is periodic in Y, i.e. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(I,1,K)         F(I,M+1,K) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `BDYS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDIMF, *). A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YS. (d/dY)U(X(I),YS,Z(K)), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(N+1). A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YS. (d/dY)U(X(I),YS,Z(K)), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(N+1). not applicable or not stated by selected source not a workspace argument

## 12. `BDYF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDIMF, *). A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YF. (d/dY)U(X(I),YF,Z(K)), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(N+1). A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YF. (d/dY)U(X(I),YF,Z(K)), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(N+1). not applicable or not stated by selected source not a workspace argument

## 13. `ZS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of Z, i.e. ZS .LE. Z .LE. ZF. must be less than ZF. 1,2,...,L+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) The range of Z, i.e. ZS .LE. Z .LE. ZF. must be less than ZF. 1,2,...,L+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) not applicable or not stated by selected source not a workspace argument

## 14. `ZF`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of Z, i.e. ZS .LE. Z .LE. ZF. ZS)/N is the panel width.  N must be at least 5. 1,2,...,L+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) The range of Z, i.e. ZS .LE. Z .LE. ZF. ZS)/N is the panel width.  N must be at least 5. 1,2,...,L+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) not applicable or not stated by selected source not a workspace argument

## 15. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (ZS,ZF) is subdivided.  Hence, there will be N+1 grid points in the Z-direction given by Z(K) = ZS+(K-1)DZ for K=1,2,...,N+1, U(I,J,K). = 1  If the solution is specified at Z = ZS and Z = ZF. = 2  If the solution is specified at Z = ZS and the derivative of the solution with respect to Z is specified at Z = ZF. = 3  If the derivative of the solution with respect to Z is specified at Z = ZS and Z = ZF. = 4  If the derivative of the solution with respect to Z is specified at Z = ZS and the solution is specified at Z=ZF. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not stated by selected source not applicable or not stated by selected source not a workspace argument

## 16. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary conditions at Z = ZS and Z = ZF. = 0  If the solution is periodic in Z, i.e. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(I,J,1)         F(I,J,N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `BDZS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDIMF, *). A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZS. (d/dZ)U(X(I),Y(J),ZS), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(M+1). A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZS. (d/dZ)U(X(I),Y(J),ZS), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(M+1). not applicable or not stated by selected source not a workspace argument

## 18. `BDZF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDIMF, *). A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZF. (d/dZ)U(X(I),Y(J),ZF), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(M+1). A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZF. (d/dZ)U(X(I),Y(J),ZF), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(M+1). not applicable or not stated by selected source not a workspace argument

## 19. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the Helmholtz equation. If LAMBDA .GT. 0, a solution may not exist.  However, HW3CRT will attempt to find a solution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 20. `LDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the arrays F,BDYS,BDYF,BDZS, and BDZF as it appears in the program calling HW3CRT. this parameter is used to specify the variable dimension of these arrays.  LDIMF must be at least L+1. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN The row (or first) dimension of the arrays F,BDYS,BDYF,BDZS, and BDZF as it appears in the program calling HW3CRT. this parameter is used to specify the variable dimension of these arrays.  LDIMF must be at least L+1. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN The row (or first) dimension of the arrays F,BDYS,BDYF,BDZS, and BDZF as it appears in the program calling HW3CRT. this parameter is used to specify the variable dimension of these arrays.  LDIMF must be at least L+1. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not a workspace argument

## 21. `MDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The column (or second) dimension of the array F and the row (or first) dimension of the arrays BDXS and BDXF as it appears in the program calling HW3CRT.  This parameter is used to specify the variable dimension of these arrays. must be at least M+1. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN The column (or second) dimension of the array F and the row (or first) dimension of the arrays BDXS and BDXF as it appears in the program calling HW3CRT.  This parameter is used to specify the variable dimension of these arrays. must be at least M+1. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source not a workspace argument

## 22. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 3; dimensions (LDIMF, MDIMF, *). A three-dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any).  For I=2,3,...,L, J=2,3,...,M, and K=2,3,...,N F(X(I),Y(J),Z(K)). On the boundaries F is defined by 1,2,...,M+1 1,2,...,N+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) 1,2,...,L+1 1,2,...,N+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) 1,2,...,L+1 1,2,...,M+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) must be dimensioned at least (L+1)*(M+1)*(N+1). NOTE: If the table calls for both the solution U and the right side F on a boundary, then the solution must be specified. Contains the solution U(I,J,K) of the finite difference approximation for the grid point (X(I),Y(J),Z(K)) for argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN A three-dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any).  For I=2,3,...,L, J=2,3,...,M, and K=2,3,...,N F(X(I),Y(J),Z(K)). On the boundaries F is defined by 1,2,...,M+1 1,2,...,N+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) 1,2,...,L+1 1,2,...,N+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) 1,2,...,L+1 1,2,...,M+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) must be dimensioned at least (L+1)*(M+1)*(N+1). NOTE: If the table calls for both the solution U and the right side F on a boundary, then the solution must be specified. Contains the solution U(I,J,K) of the finite difference approximation for the grid point (X(I),Y(J),Z(K)) for argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source not a workspace argument

## 23. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist.  PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists.  PWSCRT then computes this solution, which is a least squares solution to the original approximation.  This solution is not unique and is unnormalized.  The value of PERTRB should be small compared to the right side F.  Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 24. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters.  Except for numbers 0 and 12, a solution is not attempted. =  0  No error =  1  XS .GE. XF =  2  L .LT. 5 =  3  LBDCND .LT. 0 .OR. LBDCND .GT. 4 =  4  YS .GE. YF =  5  M .LT. 5 =  6  MBDCND .LT. 0 .OR. MBDCND .GT. 4 =  7  ZS .GE. ZF =  8  N .LT. 5 =  9  NBDCND .LT. 0 .OR. NBDCND .GT. 4 = 10  LDIMF .LT. L+1 = 11  MDIMF .LT. M+1 = 12  LAMBDA .GT. 0 Since this is the only means of indicating a possibly incorrect call to HW3CRT, the user should test IERROR after the call. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDXS(MDIMF,N+1),BDXF(MDIMF,N+1),BDYS(LDIMF,N+1), Arguments      BDYF(LDIMF,N+1),BDZS(LDIMF,M+1),BDZF(LDIMF,M+1), An error flag that indicates invalid input parameters.  Except for numbers 0 and 12, a solution is not attempted. =  0  No error =  1  XS .GE. XF =  2  L .LT. 5 =  3  LBDCND .LT. 0 .OR. LBDCND .GT. 4 =  4  YS .GE. YF =  5  M .LT. 5 =  6  MBDCND .LT. 0 .OR. MBDCND .GT. 4 =  7  ZS .GE. ZF =  8  N .LT. 5 =  9  NBDCND .LT. 0 .OR. NBDCND .GT. 4 = 10  LDIMF .LT. L+1 = 11  MDIMF .LT. M+1 = 12  LAMBDA .GT. 0 Since this is the only means of indicating a possibly incorrect call to HW3CRT, the user should test IERROR after the call. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDXS(MDIMF,N+1),BDXF(MDIMF,N+1),BDYS(LDIMF,N+1), Arguments      BDYF(LDIMF,N+1),BDZS(LDIMF,M+1),BDZF(LDIMF,M+1), not applicable or not stated by selected source not a workspace argument

## 25. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 5*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)) argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN A one-dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 5*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)) argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `XS`: not a workspace argument
- `XF`: not a workspace argument
- `L`: not a workspace argument
- `LBDCND`: not a workspace argument
- `BDXS`: not a workspace argument
- `BDXF`: not a workspace argument
- `YS`: not a workspace argument
- `YF`: not a workspace argument
- `M`: not a workspace argument
- `MBDCND`: not a workspace argument
- `BDYS`: not a workspace argument
- `BDYF`: not a workspace argument
- `ZS`: not a workspace argument
- `ZF`: not a workspace argument
- `N`: not a workspace argument
- `NBDCND`: not a workspace argument
- `BDZS`: not a workspace argument
- `BDZF`: not a workspace argument
- `ELMBDA`: not a workspace argument
- `LDIMF`: not a workspace argument
- `MDIMF`: not a workspace argument
- `F`: not a workspace argument
- `PERTRB`: not a workspace argument
- `IERROR`: not a workspace argument
- `W`: A one-dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 5*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)) argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hw3crt`
- Original SLATEC routine: `HW3CRT`
- Native symbol: `hw3crt_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank3,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HW3CRT](https://www.netlib.org/slatec/fishfft/hw3crt.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
