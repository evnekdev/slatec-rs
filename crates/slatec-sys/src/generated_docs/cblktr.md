# Purpose

Subroutine CBLKTR is a complex version of subroutine BLKTRI. Both subroutines solve a system of linear equations of the form

# Description

This canonical unsafe binding exposes original SLATEC routine `CBLKTR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBLKTR](https://www.netlib.org/slatec/fishfft/cblktr.f).

# Arguments

## 1. `IFLG`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. = 0  Initialization only.  Certain quantities that depend on NP, 0 takes approximately one half the time 1.  However, the initialization does not have to be repeated unless NP, N, 1.  W(1) contains the number of locations required by W in floating point format. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   AN(N),BN(N),CN(N),AM(M),BM(M),CM(M),Y(IDIMY,N) Arguments      W(see argument list) Latest         June 1979 Revision Required       CBLKTR,CBLKT1,PROC,PROCP,CPROC,CPROCP,CCMPB,INXCA, Subprograms    INXCB,INXCC,CPADD,PGSF,PPGSF,PPPSF,BCRH,TEVLC, R1MACH = 0  Initialization only.  Certain quantities that depend on NP, 0 takes approximately one half the time 1.  However, the initialization does not have to be repeated unless NP, N, 1.  W(1) contains the number of locations required by W in floating point format. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   AN(N),BN(N),CN(N),AM(M),BM(M),CM(M),Y(IDIMY,N) Arguments      W(see argument list) Latest         June 1979 Revision Required       CBLKTR,CBLKT1,PROC,PROCP,CPROC,CPROCP,CCMPB,INXCA, Subprograms    INXCB,INXCC,CPADD,PGSF,PPGSF,PPPSF,BCRH,TEVLC, R1MACH not applicable or not stated by selected source not a workspace argument

## 2. `NP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 define K=INT(log2(N))+1 and set L=2**(K+1) then 1))+1 and set L=2**(K+1) then not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. The number of unknowns in the J-direction. N must be greater than 4. The operation count is proportional to MNlog2(N), hence should be selected less than or equal to M. 1))+1 and set L=2**(K+1) then are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. The number of unknowns in the J-direction. N must be greater than 4. The operation count is proportional to MNlog2(N), hence should be selected less than or equal to M. 1))+1 and set L=2**(K+1) then not applicable or not stated by selected source

## 4. `AN`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1) + AM(I)*X(I-1,J) + (BN(J)+BM(I))*X(I,J) are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). BN, or CN change. are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. error flag that indicates invalid input parameters.  Except for number zero, a solution is not attempted. = 0  No error. = 1  M is less than 5. = 2  N is less than 5. = 3  IDIMY is less than M. = 4  BLKTRI failed while computing results that depend on the coefficient arrays AN, BN, CN.  Check these arrays. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN 1) + AM(I)*X(I-1,J) + (BN(J)+BM(I))*X(I,J) are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). BN, or CN change. are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. error flag that indicates invalid input parameters.  Except for number zero, a solution is not attempted. = 0  No error. = 1  M is less than 5. = 2  N is less than 5. = 3  IDIMY is less than M. = 4  BLKTRI failed while computing results that depend on the coefficient arrays AN, BN, CN.  Check these arrays. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN not applicable or not stated by selected source

## 5. `BN`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. is less Conditions     than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. is less Conditions     than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if not applicable or not stated by selected source

## 6. `CN`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Y(I,J) For I = 1,2,...,M  and  J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N),  X(I,N+1) = X(I,1), X(0,J) = X(M,J),  X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations.  Boundary conditions may be Dirichlet, Neumann, or periodic. are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN Y(I,J) For I = 1,2,...,M  and  J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N),  X(I,N+1) = X(I,1), X(0,J) = X(M,J),  X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations.  Boundary conditions may be Dirichlet, Neumann, or periodic. are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN not applicable or not stated by selected source

## 7. `MP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Input M-direction coupling selector. `MP=0` requests the periodic coefficient case and requires the endpoint off-diagonal coefficients to be nonzero; `MP=1` selects the noncyclic endpoint case where those endpoint coefficients are zero. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . The number of unknowns in the I-direction. M must be greater than 4. are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . The number of unknowns in the I-direction. M must be greater than 4. not applicable or not stated by selected source not a workspace argument

## 9. `AM`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. not applicable or not stated by selected source not a workspace argument

## 10. `BM`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. is less Conditions     than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. is less Conditions     than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if not applicable or not stated by selected source not a workspace argument

## 11. `CM`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). Y(I,J) For I = 1,2,...,M  and  J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N),  X(I,N+1) = X(I,1), X(0,J) = X(M,J),  X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations.  Boundary conditions may be Dirichlet, Neumann, or periodic. are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. Y(I,J) For I = 1,2,...,M  and  J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N),  X(I,N+1) = X(I,1), X(0,J) = X(M,J),  X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations.  Boundary conditions may be Dirichlet, Neumann, or periodic. are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. not applicable or not stated by selected source not a workspace argument

## 12. `IDIMY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the two-dimensional array Y as it appears in the program calling BLKTRI.  This parameter is must be at least M. The row (or first) dimension of the two-dimensional array Y as it appears in the program calling BLKTRI.  This parameter is must be at least M. The row (or first) dimension of the two-dimensional array Y as it appears in the program calling BLKTRI.  This parameter is must be at least M. not a workspace argument

## 13. `Y`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (IDIMY, *). must be at least M. A complex two-dimensional array that specifies the values of the right side of the linear system of equations given above. must be dimensioned Y(IDIMY,N) with IDIMY .GE. M. Contains the solution X. must be at least M. A complex two-dimensional array that specifies the values of the right side of the linear system of equations given above. must be dimensioned Y(IDIMY,N) with IDIMY .GE. M. Contains the solution X. not applicable or not stated by selected source not a workspace argument

## 14. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Writable status output. `0` means success; `1` means `M < 5`; `2` means `N < 5`; `3` means `IDIMY < M`; `4` reports a coefficient-array failure; and `5` reports an invalid negative product in the tridiagonal coefficient condition. Except for `0`, no solution is produced. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 15. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). A one-dimensional array that must be provided by the user for work space. 2)*L+K+5+MAX(2N,12M) 2)*L+K+5+2N+MAX(2N,12M) IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if A one-dimensional array that must be provided by the user for work space. 2)*L+K+5+MAX(2N,12M) 2)*L+K+5+2N+MAX(2N,12M) IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `IFLG`: not a workspace argument
- `NP`: not a workspace argument
- `N`: are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. The number of unknowns in the J-direction. N must be greater than 4. The operation count is proportional to MNlog2(N), hence should be selected less than or equal to M. 1))+1 and set L=2**(K+1) then
- `AN`: 1) + AM(I)*X(I-1,J) + (BN(J)+BM(I))*X(I,J) are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). BN, or CN change. are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. error flag that indicates invalid input parameters.  Except for number zero, a solution is not attempted. = 0  No error. = 1  M is less than 5. = 2  N is less than 5. = 3  IDIMY is less than M. = 4  BLKTRI failed while computing results that depend on the coefficient arrays AN, BN, CN.  Check these arrays. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN
- `BN`: are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. is less Conditions     than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if
- `CN`: Y(I,J) For I = 1,2,...,M  and  J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N),  X(I,N+1) = X(I,1), X(0,J) = X(M,J),  X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations.  Boundary conditions may be Dirichlet, Neumann, or periodic. are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN
- `MP`: not a workspace argument
- `M`: not a workspace argument
- `AM`: not a workspace argument
- `BM`: not a workspace argument
- `CM`: not a workspace argument
- `IDIMY`: not a workspace argument
- `Y`: not a workspace argument
- `IERROR`: not a workspace argument
- `W`: A one-dimensional array that must be provided by the user for work space. 2)*L+K+5+MAX(2N,12M) 2)*L+K+5+2N+MAX(2N,12M) IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::complex::cblktr`
- Original SLATEC routine: `CBLKTR`
- Native symbol: `cblktr_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank2,mut_i32,mut_f32_array_rank1)`
- Exact Netlib source file: [CBLKTR](https://www.netlib.org/slatec/fishfft/cblktr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
