# Purpose

Subroutine HWSCSP solves a finite difference approximation to the modified Helmholtz equation in spherical coordinates assuming axisymmetry (no dependence on longitude) (1/R**2)(d/dR)((R**2)(d/dR)U) + (1/(R**2)SIN(THETA))(d/dTHETA)(SIN(THETA)(d/dTHETA)U) + (LAMBDA/(RSIN(THETA))**2)U = F(THETA,R). This two dimensional modified Helmholtz equation results from the Fourier transform of the three dimensional Poisson equation

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSCSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSCSP](https://www.netlib.org/slatec/fishfft/hwscsp.f).

# Arguments

## 1. `INTL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. = 0  On initial entry to HWSCSP or if any of the arguments 0 takes approximately 1.5 times as 1.  Once a call with 0 has been made then subsequent solutions corresponding to different F, BDTS, BDTF, BDRS, BDRF can 1 since initialization is not repeated. 1.  W(1) contains the number of locations which W must have. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDTS(N+1),BDTF(N+1),BDRS(M+1),BDRF(M+1), Arguments      F(IDIMF,N+1),W(see argument list) Latest         June 1979 Revision Subprograms    HWSCSP,HWSCS1,BLKTRI,BLKTR1,PROD,PRODP,CPROD,CPRODP Required       ,COMBP,PPADD,PSGF,BSRH,PPSGF,PPSPF,TEVLS,INDXA, ,INDXB,INDXC,R1MACH Special Conditions Common         CBLKT Blocks I/O            NONE Precision      Single Specialist     Paul N Swarztrauber Language       FORTRAN = 0  On initial entry to HWSCSP or if any of the arguments 0 takes approximately 1.5 times as 1.  Once a call with 0 has been made then subsequent solutions corresponding to different F, BDTS, BDTF, BDRS, BDRF can 1 since initialization is not repeated. 1.  W(1) contains the number of locations which W must have. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDTS(N+1),BDTF(N+1),BDRS(M+1),BDRF(M+1), Arguments      F(IDIMF,N+1),W(see argument list) Latest         June 1979 Revision Subprograms    HWSCSP,HWSCS1,BLKTRI,BLKTR1,PROD,PRODP,CPROD,CPRODP Required       ,COMBP,PPADD,PSGF,BSRH,PPSGF,PPSPF,TEVLS,INDXA, ,INDXB,INDXC,R1MACH Special Conditions Common         CBLKT Blocks I/O            NONE Precision      Single Specialist     Paul N Swarztrauber Language       FORTRAN not applicable or not stated by selected source not a workspace argument

## 2. `TS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds zero corresponds to the north pole and a TF of PI corresponds to the south pole. to the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds zero corresponds to the north pole and a TF of PI corresponds to the south pole. to the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement not applicable or not stated by selected source not a workspace argument

## 3. `TF`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds zero corresponds to the north pole and a TF of PI corresponds to the south pole. to the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement PIMACH(DUM). This insures that TF in the users program is equal to PI in this program which permits several tests of the TS)/M is the panel width. PI, do not use MBDCND = 2,3, or 6, but 1,2,...,N+1 6      F(0,R(J))         F(TF,R(J)) 7      U(TS,R(J))        F(PI,R(J)) 8      F(TS,R(J))        F(PI,R(J)) 9      F(0,R(J))         F(PI,R(J)) The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds zero corresponds to the north pole and a TF of PI corresponds to the south pole. to the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement PIMACH(DUM). This insures that TF in the users program is equal to PI in this program which permits several tests of the TS)/M is the panel width. PI, do not use MBDCND = 2,3, or 6, but 1,2,...,N+1 6      F(0,R(J))         F(TF,R(J)) 7      U(TS,R(J))        F(PI,R(J)) 8      F(TS,R(J))        F(PI,R(J)) 9      F(0,R(J))         F(PI,R(J)) not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (TS,TF) is subdivided.  Hence, there will be M+1 grid points in the THETA-direction given by THETA(K) = (I-1)DTHETA+TS for TS)/M is the panel width. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary condition at THETA = TS and THETA = TF. = 1  If the solution is specified at THETA = TS and THETA = TF. = 2  If the solution is specified at THETA = TS and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = TS and THETA = TF (see notes 1,2 below). = 4  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is specified at THETA = TF. = 5  If the solution is unspecified at THETA = TS = 0 and the solution is specified at THETA = TF. = 6  If the solution is unspecified at THETA = TS = 0 and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 7  If the solution is specified at THETA = TS and the solution is unspecified at THETA = TF = PI. = 8  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is unspecified at THETA = TF = PI. = 9  If the solution is unspecified at THETA = TS = 0 and THETA = TF = PI. NOTES:  1.  If TS = 0, do not use MBDCND = 3,4, or 8, but 5,6, or 9  . 7,8, or 9  . is a dummy variable. is a dummy variable. 1,2,4,5, or 7 (the former indicates that the solution is unspecified at R = 0, the latter indicates that the solution is specified). Use instead 5,6,7,8, or 9, ELMBDA must be zero. F(1,J)            F(M+1,J) Indicates the type of boundary condition at THETA = TS and THETA = TF. = 1  If the solution is specified at THETA = TS and THETA = TF. = 2  If the solution is specified at THETA = TS and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = TS and THETA = TF (see notes 1,2 below). = 4  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is specified at THETA = TF. = 5  If the solution is unspecified at THETA = TS = 0 and the solution is specified at THETA = TF. = 6  If the solution is unspecified at THETA = TS = 0 and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 7  If the solution is specified at THETA = TS and the solution is unspecified at THETA = TF = PI. = 8  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is unspecified at THETA = TF = PI. = 9  If the solution is unspecified at THETA = TS = 0 and THETA = TF = PI. NOTES:  1.  If TS = 0, do not use MBDCND = 3,4, or 8, but 5,6, or 9  . 7,8, or 9  . is a dummy variable. is a dummy variable. 1,2,4,5, or 7 (the former indicates that the solution is unspecified at R = 0, the latter indicates that the solution is specified). Use instead 5,6,7,8, or 9, ELMBDA must be zero. F(1,J)            F(M+1,J) not applicable or not stated by selected source not a workspace argument

## 6. `BDTS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TS.  When MBDCND = 3,4, or 8, (d/dTHETA)U(TS,R(J)), J = 1,2,...,N+1  . is a dummy variable. A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TS.  When MBDCND = 3,4, or 8, (d/dTHETA)U(TS,R(J)), J = 1,2,...,N+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 7. `BDTF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TF.  When MBDCND = 2,3, or 6, (d/dTHETA)U(TF,R(J)), J = 1,2,...,N+1  . is a dummy variable. A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TF.  When MBDCND = 2,3, or 6, (d/dTHETA)U(TF,R(J)), J = 1,2,...,N+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 8. `RS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. are changed from a previous call. are all unchanged from previous call to HWSCSP. must be less than must be less than negative. RF. = 5  If the solution is unspecified at R = RS = 0 (see note below) and the solution is specified at R = RF. = 6  If the solution is unspecified at R = RS = 0 (see note below) and the derivative of the solution with respect to R is specified at R = RF. NOTE:  NBDCND = 5 or 6 cannot be used with 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) are changed from a previous call. are all unchanged from previous call to HWSCSP. must be less than must be less than negative. RF. = 5  If the solution is unspecified at R = RS = 0 (see note below) and the solution is specified at R = RF. = 6  If the solution is unspecified at R = RS = 0 (see note below) and the derivative of the solution with respect to R is specified at R = RF. NOTE:  NBDCND = 5 or 6 cannot be used with 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) not applicable or not stated by selected source not a workspace argument

## 9. `RF`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. are changed from a previous call. are all unchanged from previous call to HWSCSP. must be less than negative. RS)/N is the panel width. 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) are changed from a previous call. are all unchanged from previous call to HWSCSP. must be less than negative. RS)/N is the panel width. 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) not applicable or not stated by selected source not a workspace argument

## 10. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are changed from a previous call. are all unchanged from previous call to HWSCSP. The number of panels into which the interval (RS,RF) is subdivided.  Hence, there will be N+1 grid points in the R-direction given by R(J) = (J-1)DR+RS for J = 1,2,...,N+1, must be greater than 2 1 are changed from a previous call. are all unchanged from previous call to HWSCSP. The number of panels into which the interval (RS,RF) is subdivided.  Hence, there will be N+1 grid points in the R-direction given by R(J) = (J-1)DR+RS for J = 1,2,...,N+1, must be greater than 2 1 not applicable or not stated by selected source not a workspace argument

## 11. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are changed from a previous call. are all unchanged from previous call to HWSCSP. Indicates the type of boundary condition at R = RS and R = RF. = 1  If the solution is specified at R = RS and R = RF. = 2  If the solution is specified at R = RS and the derivative of the solution with respect to R is specified at R = RF. = 3  If the derivative of the solution with respect to R is specified at R = RS and R = RF. = 4  If the derivative of the solution with respect to R is 1 or 2  . 3 or 4, is a dummy variable. 2,3, or 6, is a dummy variable. 5 or 6 or F(I,1)            F(I,N+1) 2,4 or 6 define NUNK=N 1 3        define NUNK=N+1 Now set K=INT(log2(NUNK))+1 and L=2**(K+1) then W must be dimensioned at least (K-2)*L+K+5*(M+N)+MAX(2*N,6*M)+23 IMPORTANT** For purposes of checking, the required length of W is computed by HWSCSP and stored in W(1) in floating point format. are changed from a previous call. are all unchanged from previous call to HWSCSP. Indicates the type of boundary condition at R = RS and R = RF. = 1  If the solution is specified at R = RS and R = RF. = 2  If the solution is specified at R = RS and the derivative of the solution with respect to R is specified at R = RF. = 3  If the derivative of the solution with respect to R is specified at R = RS and R = RF. = 4  If the derivative of the solution with respect to R is 1 or 2  . 3 or 4, is a dummy variable. 2,3, or 6, is a dummy variable. 5 or 6 or F(I,1)            F(I,N+1) 2,4 or 6 define NUNK=N 1 3        define NUNK=N+1 Now set K=INT(log2(NUNK))+1 and L=2**(K+1) then W must be dimensioned at least (K-2)*L+K+5*(M+N)+MAX(2*N,6*M)+23 IMPORTANT** For purposes of checking, the required length of W is computed by HWSCSP and stored in W(1) in floating point format. not applicable or not stated by selected source not a workspace argument

## 12. `BDRS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RS. (d/dR)U(THETA(I),RS), I = 1,2,...,M+1  . is a dummy variable. A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RS. (d/dR)U(THETA(I),RS), I = 1,2,...,M+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 13. `BDRF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RF. (d/dR)U(THETA(I),RF), I = 1,2,...,M+1  . is a dummy variable. A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RF. (d/dR)U(THETA(I),RF), I = 1,2,...,M+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 14. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the Helmholtz equation.  If LAMBDA .GT. 0, a solution may not exist.  However, HWSCSP will not stated by selected source not applicable or not stated by selected source not a workspace argument

## 15. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). A two-dimensional array that specifies the value of the right side of the Helmholtz equation and boundary values (if any). for I = 2,3,...,M and J = 2,3,...,N F(THETA(I),R(J)). On the boundaries F is defined by 1,2,...,N+1 6      F(0,R(J))         F(TF,R(J)) 7      U(TS,R(J))        F(PI,R(J)) 8      F(TS,R(J))        F(PI,R(J)) 9      F(0,R(J))         F(PI,R(J)) 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),R(J)), I = 1,2,...,M+1,   J = 1,2,...,N+1  . which ensures that a solution exists.  HWSCSP then computes this solution, which is a least squares solution to the original approximation. This solution is not unique and is unnormalized. The value of PERTRB should be small compared to the right side Otherwise , a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. A two-dimensional array that specifies the value of the right side of the Helmholtz equation and boundary values (if any). for I = 2,3,...,M and J = 2,3,...,N F(THETA(I),R(J)). On the boundaries F is defined by 1,2,...,N+1 6      F(0,R(J))         F(TF,R(J)) 7      U(TS,R(J))        F(PI,R(J)) 8      F(TS,R(J))        F(PI,R(J)) 9      F(0,R(J))         F(PI,R(J)) 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),R(J)), I = 1,2,...,M+1,   J = 1,2,...,N+1  . which ensures that a solution exists.  HWSCSP then computes this solution, which is a least squares solution to the original approximation. This solution is not unique and is unnormalized. The value of PERTRB should be small compared to the right side Otherwise , a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. not applicable or not stated by selected source not a workspace argument

## 16. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HWSCSP.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSCSP.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSCSP.  This parameter is used to specify the must be at least M+1  . not a workspace argument

## 17. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist.  PERTRB is a constant, calculated and subtracted from not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters.  Except for numbers 0 and 10, a solution is not attempted. = 1  TS.LT.0. or TF.GT.PI = 2  TS.GE.TF = 3  M.LT.5 = 4  MBDCND.LT.1 or MBDCND.GT.9 = 5  RS.LT.0 = 6  RS.GE.RF = 7  N.LT.5 = 8  NBDCND.LT.1 or NBDCND.GT.6 = 9  ELMBDA.GT.0 = 10 IDIMF.LT.M+1 = 11 ELMBDA.NE.0 and MBDCND.GE.5 = 12 ELMBDA.NE.0 and NBDCND equals 5 or 6 = 13 MBDCND equals 5,6 or 9 and TS.NE.0 = 14 MBDCND.GE.7 and TF.NE.PI = 15 TS.EQ.0 and MBDCND equals 3,4 or 8 = 16 TF.EQ.PI and MBDCND equals 2,3 or 6 = 17 NBDCND.GE.5 and RS.NE.0 = 18 NBDCND.GE.5 and MBDCND equals 1,2,4,5 or 7 Since this is the only means of indicating a possibly incorrect call to HWSCSP, the user should test IERROR after a call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 19. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array that must be provided by the user for work space. Its length can be computed from the formula below which depends on the value of NBDCND. Contains intermediate values that must not be destroyed if A one-dimensional array that must be provided by the user for work space. Its length can be computed from the formula below which depends on the value of NBDCND. Contains intermediate values that must not be destroyed if not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `INTL`: not a workspace argument
- `TS`: not a workspace argument
- `TF`: not a workspace argument
- `M`: not a workspace argument
- `MBDCND`: not a workspace argument
- `BDTS`: not a workspace argument
- `BDTF`: not a workspace argument
- `RS`: not a workspace argument
- `RF`: not a workspace argument
- `N`: not a workspace argument
- `NBDCND`: not a workspace argument
- `BDRS`: not a workspace argument
- `BDRF`: not a workspace argument
- `ELMBDA`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `PERTRB`: not a workspace argument
- `IERROR`: not a workspace argument
- `W`: A one-dimensional array that must be provided by the user for work space. Its length can be computed from the formula below which depends on the value of NBDCND. Contains intermediate values that must not be destroyed if

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwscsp`
- Original SLATEC routine: `HWSCSP`
- Native symbol: `hwscsp_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HWSCSP](https://www.netlib.org/slatec/fishfft/hwscsp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
