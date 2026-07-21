# Purpose

Subroutine HWSSSP solves a finite difference approximation to the Helmholtz equation in spherical coordinates and on the surface of the unit sphere (radius of 1): (1/SIN(THETA))(d/dTHETA)(SIN(THETA)(dU/dTHETA)) + (1/SIN(THETA)**2)(d/dPHI)(dU/dPHI) + LAMBDA*U = F(THETA,PHI) Where THETA is colatitude and PHI is longitude. * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSSSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSSSP](https://www.netlib.org/slatec/fishfft/hwsssp.f).

# Arguments

## 1. `TS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds to zero corresponds to the north pole and a TF of PI corresponds to the south pole. the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds to zero corresponds to the north pole and a TF of PI corresponds to the south pole. the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement not applicable or not stated by selected source not a workspace argument

## 2. `TF`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds to zero corresponds to the north pole and a TF of PI corresponds to the south pole. the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement PIMACH(DUM). This insures that TF in the users program is equal to PI in this program which permits several tests of the TS)/M is the panel width. PI, do not use MBDCND = 2,3, or 6, but 1,2,...,N+1 6      F(0,PS)           F(TF,PHI(J)) 7      U(TS,PHI(J))      F(PI,PS) 8      F(TS,PHI(J))      F(PI,PS) 9      F(0,PS)           F(PI,PS) The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds to zero corresponds to the north pole and a TF of PI corresponds to the south pole. the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement PIMACH(DUM). This insures that TF in the users program is equal to PI in this program which permits several tests of the TS)/M is the panel width. PI, do not use MBDCND = 2,3, or 6, but 1,2,...,N+1 6      F(0,PS)           F(TF,PHI(J)) 7      U(TS,PHI(J))      F(PI,PS) 8      F(TS,PHI(J))      F(PI,PS) 9      F(0,PS)           F(PI,PS) not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (TS,TF) is subdivided.  Hence, there will be M+1 grid points in the THETA-direction given by THETA(I) = (I-1)DTHETA+TS for TS)/M is the panel width. must be greater than 5. as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) The number of panels into which the interval (TS,TF) is subdivided.  Hence, there will be M+1 grid points in the THETA-direction given by THETA(I) = (I-1)DTHETA+TS for TS)/M is the panel width. must be greater than 5. as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) not applicable or not stated by selected source not a workspace argument

## 4. `MBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary condition at THETA = TS and THETA = TF. = 1  If the solution is specified at THETA = TS and THETA = TF. = 2  If the solution is specified at THETA = TS and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = TS and THETA = TF (see notes 1,2 below). = 4  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is specified at THETA = TF. = 5  If the solution is unspecified at THETA = TS = 0 and the solution is specified at THETA = TF. = 6  If the solution is unspecified at THETA = TS = 0 and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 7  If the solution is specified at THETA = TS and the solution is unspecified at THETA = TF = PI. = 8  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is unspecified at THETA = TF = PI. = 9  If the solution is unspecified at THETA = TS = 0 and THETA = TF = PI. NOTES:  1.  If TS = 0, do not use MBDCND = 3,4, or 8, but 5,6, or 9  . 7,8, or 9  . is a dummy variable. is a dummy variable. 5,6,7,8, or 9 (the former indicates that the solution is specified at a pole, the latter indicates that the solution is unspecified). Use instead 1 or 2  . F(1,J)            F(M+1,J) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BDTS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TS.  When MBDCND = 3,4, or 8, (d/dTHETA)U(TS,PHI(J)), J = 1,2,...,N+1  . is a dummy variable. A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TS.  When MBDCND = 3,4, or 8, (d/dTHETA)U(TS,PHI(J)), J = 1,2,...,N+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 6. `BDTF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TF.  When MBDCND = 2,3, or 6, (d/dTHETA)U(TF,PHI(J)), J = 1,2,...,N+1  . is a dummy variable. A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TF.  When MBDCND = 2,3, or 6, (d/dTHETA)U(TF,PHI(J)), J = 1,2,...,N+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 7. `PS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of PHI (longitude), i.e., PS .LE. PHI .LE. PF.  PS 0 and 0 and PF (see note below). NOTE:  NBDCND = 1,2, or 4 cannot be used with 1,2,...,N+1 6      F(0,PS)           F(TF,PHI(J)) 7      U(TS,PHI(J))      F(PI,PS) 8      F(TS,PHI(J))      F(PI,PS) 9      F(0,PS)           F(PI,PS) 1,2,...,M+1 3      F(THETA(I),PS)    F(THETA(I),PF) 4      F(THETA(I),PS)    U(THETA(I),PF) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `PF`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The range of PHI (longitude), i.e., PS .LE. PHI .LE. PF.  PS 0 and 0 and 2*PI, periodic boundary conditions are usually prescribed. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If PF is equal to 2*PI then it must be computed using the 2.*PIMACH(DUM). This insures that PF in the users program is equal to 2*PI in this program which permits tests of the input parameters that otherwise would not be possible. PS)/N is the panel width. 1,2,...,M+1 3      F(THETA(I),PS)    F(THETA(I),PF) 4      F(THETA(I),PS)    U(THETA(I),PF) The range of PHI (longitude), i.e., PS .LE. PHI .LE. PF.  PS 0 and 0 and 2*PI, periodic boundary conditions are usually prescribed. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If PF is equal to 2*PI then it must be computed using the 2.*PIMACH(DUM). This insures that PF in the users program is equal to 2*PI in this program which permits tests of the input parameters that otherwise would not be possible. PS)/N is the panel width. 1,2,...,M+1 3      F(THETA(I),PS)    F(THETA(I),PF) 4      F(THETA(I),PS)    U(THETA(I),PF) not applicable or not stated by selected source not a workspace argument

## 9. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of panels into which the interval (PS,PF) is subdivided.  Hence, there will be N+1 grid points in the PHI-direction given by PHI(J) = (J-1)DPHI+PS  for PS)/N is the panel width. must be greater than 4. The number of panels into which the interval (PS,PF) is subdivided.  Hence, there will be N+1 grid points in the PHI-direction given by PHI(J) = (J-1)DPHI+PS  for PS)/N is the panel width. must be greater than 4. not applicable or not stated by selected source not a workspace argument

## 10. `NBDCND`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the type of boundary condition at PHI = PS and PHI = PF. = 0  If the solution is periodic in PHI, i.e., U(I,J) = U(I,N+J). = 1  If the solution is specified at PHI = PS and PHI = PF (see note below). = 2  If the solution is specified at PHI = PS (see note below) and the derivative of the solution with respect to PHI is specified at PHI = PF. = 3  If the derivative of the solution with respect to PHI is specified at PHI = PS and PHI = PF. = 4  If the derivative of the solution with respect to PHI is is a dummy variable. is a dummy variable. F(I,1)            F(I,N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `BDPS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to PHI at PHI = PS.  When NBDCND = 3 or 4, (d/dPHI)U(THETA(I),PS), I = 1,2,...,M+1  . is a dummy variable. A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to PHI at PHI = PS.  When NBDCND = 3 or 4, (d/dPHI)U(THETA(I),PS), I = 1,2,...,M+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 12. `BDPF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to PHI at PHI = PF.  When NBDCND = 2 or 3, (d/dPHI)U(THETA(I),PF), I = 1,2,...,M+1  . is a dummy variable. A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to PHI at PHI = PF.  When NBDCND = 2 or 3, (d/dPHI)U(THETA(I),PF), I = 1,2,...,M+1  . is a dummy variable. not applicable or not stated by selected source not a workspace argument

## 13. `ELMBDA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The constant LAMBDA in the Helmholtz equation.  If LAMBDA .GT. 0, a solution may not exist.  However, HWSSSP will attempt to find a solution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMF, *). A two-dimensional array that specifies the value of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M  and  J = 2,3,...,N F(THETA(I),PHI(J)). On the boundaries F is defined by 1,2,...,N+1 6      F(0,PS)           F(TF,PHI(J)) 7      U(TS,PHI(J))      F(PI,PS) 8      F(TS,PHI(J))      F(PI,PS) 9      F(0,PS)           F(PI,PS) 1,2,...,M+1 3      F(THETA(I),PS)    F(THETA(I),PF) 4      F(THETA(I),PS)    U(THETA(I),PF) must be dimensioned at least (M+1)*(N+1). NOTE* If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),PHI(J)), I = 1,2,...,M+1,   J = 1,2,...,N+1  . A two-dimensional array that specifies the value of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M  and  J = 2,3,...,N F(THETA(I),PHI(J)). On the boundaries F is defined by 1,2,...,N+1 6      F(0,PS)           F(TF,PHI(J)) 7      U(TS,PHI(J))      F(PI,PS) 8      F(TS,PHI(J))      F(PI,PS) 9      F(0,PS)           F(PI,PS) 1,2,...,M+1 3      F(THETA(I),PS)    F(THETA(I),PF) 4      F(THETA(I),PS)    U(THETA(I),PF) must be dimensioned at least (M+1)*(N+1). NOTE* If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),PHI(J)), I = 1,2,...,M+1,   J = 1,2,...,N+1  . not applicable or not stated by selected source not a workspace argument

## 15. `IDIMF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the array F as it appears in the program calling HWSSSP.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSSSP.  This parameter is used to specify the must be at least M+1  . The row (or first) dimension of the array F as it appears in the program calling HWSSSP.  This parameter is used to specify the must be at least M+1  . not a workspace argument

## 16. `PERTRB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. If one specifies a combination of periodic, derivative or unspecified boundary conditions for a Poisson equation (LAMBDA = 0), a solution may not exist.  PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSSSP then computes this solution, which is a least squares solution to the original approximation.  This solution is not unique and is unnormalized. The value of PERTRB should be small compared to the right side F. Otherwise , a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters.  Except for numbers 0 and 8, a solution is not attempted. = 0  No error = 1  TS.LT.0 or TF.GT.PI = 2  TS.GE.TF = 3  MBDCND.LT.1 or MBDCND.GT.9 = 4  PS.LT.0 or PS.GT.PI+PI = 5  PS.GE.PF = 6  N.LT.5 = 7  M.LT.5 = 8  NBDCND.LT.0 or NBDCND.GT.4 = 9  ELMBDA.GT.0 = 10 IDIMF.LT.M+1 = 11 NBDCND equals 1,2 or 4 and MBDCND.GE.5 = 12 TS.EQ.0 and MBDCND equals 3,4 or 8 = 13 TF.EQ.PI and MBDCND equals 2,3 or 6 = 14 MBDCND equals 5,6 or 9 and TS.NE.0 = 15 MBDCND.GE.7 and TF.NE.PI Since this is the only means of indicating a possibly incorrect call to HWSSSP, the user should test IERROR after a call. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1)+(16+INT(log2(N+1)))(M+1) locations. The actual number of locations used is computed by HWSSSP and is output in location W(1). INT( ) denotes the FORTRAN integer function. Contains intermediate values that must not be destroyed if HWSSSP will be called again with INTL = 1. W(1) contains the required length of W . Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDTS(N+1),BDTF(N+1),BDPS(M+1),BDPF(M+1), Arguments      F(IDIMF,N+1),W(see argument list) Latest         January 1978 Revision Subprograms    HWSSSP,HWSSS1,GENBUN,POISD2,POISN2,POISP2,COSGEN,ME Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1)+(16+INT(log2(N+1)))(M+1) locations. The actual number of locations used is computed by HWSSSP and is output in location W(1). INT( ) denotes the FORTRAN integer function. Contains intermediate values that must not be destroyed if HWSSSP will be called again with INTL = 1. W(1) contains the required length of W . Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDTS(N+1),BDTF(N+1),BDPS(M+1),BDPF(M+1), Arguments      F(IDIMF,N+1),W(see argument list) Latest         January 1978 Revision Subprograms    HWSSSP,HWSSS1,GENBUN,POISD2,POISN2,POISP2,COSGEN,ME Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `TS`: not a workspace argument
- `TF`: not a workspace argument
- `M`: not a workspace argument
- `MBDCND`: not a workspace argument
- `BDTS`: not a workspace argument
- `BDTF`: not a workspace argument
- `PS`: not a workspace argument
- `PF`: not a workspace argument
- `N`: not a workspace argument
- `NBDCND`: not a workspace argument
- `BDPS`: not a workspace argument
- `BDPF`: not a workspace argument
- `ELMBDA`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `PERTRB`: not a workspace argument
- `IERROR`: not a workspace argument
- `W`: A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1)+(16+INT(log2(N+1)))(M+1) locations. The actual number of locations used is computed by HWSSSP and is output in location W(1). INT( ) denotes the FORTRAN integer function. Contains intermediate values that must not be destroyed if HWSSSP will be called again with INTL = 1. W(1) contains the required length of W . Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDTS(N+1),BDTF(N+1),BDPS(M+1),BDPF(M+1), Arguments      F(IDIMF,N+1),W(see argument list) Latest         January 1978 Revision Subprograms    HWSSSP,HWSSS1,GENBUN,POISD2,POISN2,POISP2,COSGEN,ME Required       TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwsssp`
- Original SLATEC routine: `HWSSSP`
- Native symbol: `hwsssp_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HWSSSP](https://www.netlib.org/slatec/fishfft/hwsssp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
