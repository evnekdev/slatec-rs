# Purpose

DGLSS solves both underdetermined and overdetermined

# Description

This canonical unsafe binding exposes original SLATEC routine `DGLSS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGLSS](https://www.netlib.org/slatec/src/dglss.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (MDA, *). is an M by N matrix and B is an M by NB matrix of right hand sides. If B, with MDA the Contains the triangular part of the reduced matrix and the transformation information. It together with the first 2*MIN(M,N) elements of WORK (see below) completely specify the factorization of A. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `MDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. actual first dimension of A in the calling program. actual first dimension of A in the calling program. not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factor- ization). If the matrix A is determined to be rank deficient, that is the rank of A is less than then the minimal length least squares solution is computed. DGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes DLLSIA and DULSIA are recommended. DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. * WARNING - All input arrays are changed on exit.        * * SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). 0, B is never accessed. contain value necessary to reproduce the factorization of A. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factor- ization). If the matrix A is determined to be rank deficient, that is the rank of A is less than then the minimal length least squares solution is computed. DGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes DLLSIA and DULSIA are recommended. DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. * WARNING - All input arrays are changed on exit.        * * SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). 0, B is never accessed. contain value necessary to reproduce the factorization of A. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factor- ization). If the matrix A is determined to be rank deficient, that is the rank of A is less than then the minimal length least squares solution is computed. DGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes DLLSIA and DULSIA are recommended. DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. * WARNING - All input arrays are changed on exit.        * * SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION actual first dimension of A in the calling program. 0, B is never accessed. contain value necessary to reproduce the factorization of A. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. Reduced rank  rank=MIN(M,N)-INFO is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factor- ization). If the matrix A is determined to be rank deficient, that is the rank of A is less than then the minimal length least squares solution is computed. DGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes DLLSIA and DULSIA are recommended. DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. * WARNING - All input arrays are changed on exit.        * * SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION actual first dimension of A in the calling program. 0, B is never accessed. contain value necessary to reproduce the factorization of A. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. Reduced rank  rank=MIN(M,N)-INFO not applicable or not stated by selected source not a workspace argument

## 5. `B`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (MDB, *). is an M by N matrix and B is an M by NB matrix of right hand sides. If Right hand side(s), with MDB the actual first is the number of M by 1 right hand sides. Must have Contains the N by NB solution matrix X. AX(I), I=1,NB. is an M by N matrix and B is an M by NB matrix of right hand sides. If Right hand side(s), with MDB the actual first is the number of M by 1 right hand sides. Must have Contains the N by NB solution matrix X. AX(I), I=1,NB. not applicable or not stated by selected source not a workspace argument

## 6. `MDB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of M by 1 right hand sides. Must have 0, B is never accessed. is the number of M by 1 right hand sides. Must have 0, B is never accessed. not applicable or not stated by selected source not a workspace argument

## 7. `NB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the is the number of M by 1 right hand sides. Must have number of M by 1 right hand sides. Must have 0, B is never accessed. is the is the number of M by 1 right hand sides. Must have number of M by 1 right hand sides. Must have 0, B is never accessed. not applicable or not stated by selected source not a workspace argument

## 8. `RNORM`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Vector of length at least NB.  On input the contents of RNORM are unused. Contains the Euclidean length of the NB residual Vector of length at least NB.  On input the contents of RNORM are unused. Contains the Euclidean length of the NB residual not applicable or not stated by selected source not a workspace argument

## 9. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). A real work array dimensioned 5*MIN(M,N). contain value contain value necessary to reproduce the factorization of A. necessary to reproduce the factorization of A. A real work array dimensioned 5*MIN(M,N). contain value contain value necessary to reproduce the factorization of A. necessary to reproduce the factorization of A. not applicable or not stated by selected source

## 10. `LW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Actual dimension of WORK. IWORK, LIW, and the first 2*MIN(M,N) locations of WORK as output by the original call to DGLSS. Output..All TYPE REAL variables are DOUBLE PRECISION Actual dimension of WORK. IWORK, LIW, and the first 2*MIN(M,N) locations of WORK as output by the original call to DGLSS. Output..All TYPE REAL variables are DOUBLE PRECISION not applicable or not stated by selected source not a workspace argument

## 11. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer work array dimensioned at least N+M. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. Integer work array dimensioned at least N+M. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. not applicable or not stated by selected source

## 12. `LIW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Actual dimension of IWORK. Actual dimension of IWORK. not applicable or not stated by selected source not a workspace argument

## 13. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. A flag which provides for the efficient solution of subsequent problems involving the same A but different B. 0 original call 1 subsequent calls On subsequent calls, the user must supply A, INFO, Flag to indicate status of computation on completion -1   Parameter error(s) 0 - Full rank A flag which provides for the efficient solution of subsequent problems involving the same A but different B. 0 original call 1 subsequent calls On subsequent calls, the user must supply A, INFO, Flag to indicate status of computation on completion -1   Parameter error(s) 0 - Full rank not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `MDA`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `B`: not a workspace argument
- `MDB`: not a workspace argument
- `NB`: not a workspace argument
- `RNORM`: not a workspace argument
- `WORK`: A real work array dimensioned 5*MIN(M,N). contain value contain value necessary to reproduce the factorization of A. necessary to reproduce the factorization of A.
- `LW`: not a workspace argument
- `IWORK`: Integer work array dimensioned at least N+M. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns.
- `LIW`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dglss`
- Original SLATEC routine: `DGLSS`
- Native symbol: `dglss_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [DGLSS](https://www.netlib.org/slatec/src/dglss.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
