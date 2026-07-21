# Purpose

DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. The problem data consists of an M by N matrix

# Description

This canonical unsafe binding exposes original SLATEC routine `HFTI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HFTI](https://www.netlib.org/slatec/src/hfti.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (MDA, *). an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below.  The NB column vectors of B represent right-side vectors for NB distinct linear least squares problems. This set of problems can also be written as the matrix least squares problem AX = B, where X is the N by NB solution matrix. Note that if B is the M by M identity matrix, then X will be the pseudo-inverse of A. This subroutine first transforms the augmented matrix (A B) to a matrix (R C) using premultiplying Householder transformations with column interchanges.  All subdiagonal elements in the matrix R are zero and its diagonal elements satisfy ABS(R(I,I)).GE.ABS(R(I+1,I+1)), I = 1,...,L-1, where L = MIN(M,N). The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. Then a solution of minimum Euclidean length is computed using the first or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are contains the M by N contains the M by N B. The first dimensioning parameter of the array must satisfy MDA.GE.M The contents of the array A(*,*) will be modified by the subroutine. These contents are not generally required by the user. an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below.  The NB column vectors of B represent right-side vectors for NB distinct linear least squares problems. This set of problems can also be written as the matrix least squares problem AX = B, where X is the N by NB solution matrix. Note that if B is the M by M identity matrix, then X will be the pseudo-inverse of A. This subroutine first transforms the augmented matrix (A B) to a matrix (R C) using premultiplying Householder transformations with column interchanges.  All subdiagonal elements in the matrix R are zero and its diagonal elements satisfy ABS(R(I,I)).GE.ABS(R(I+1,I+1)), I = 1,...,L-1, where L = MIN(M,N). The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. Then a solution of minimum Euclidean length is computed using the first or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are contains the M by N contains the M by N B. The first dimensioning parameter of the array must satisfy MDA.GE.M The contents of the array A(*,*) will be modified by the subroutine. These contents are not generally required by the user. not applicable or not stated by selected source not a workspace argument

## 2. `MDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are contains the M by N must satisfy MDA.GE.M is considered an error. or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are contains the M by N must satisfy MDA.GE.M is considered an error. not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. contains the M by N is permitted.  There is permitted.  There is no restriction on the rank of A.  The is no restriction on the rank of A.  The is considered an error. 1 the array B(*) may be either doubly or singly subscripted.  In the latter case the value of MDB is arbitrary but it should be set to some valid integer not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are contains the M by N is permitted.  There is permitted.  There is no restriction on the rank of A.  The is no restriction on the rank of A.  The 1 the array B(*) may be either doubly or singly subscripted.  In the latter case the value of MDB is arbitrary but it should be set to some valid integer not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (MDB, *). or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are 0 the subroutine will perform the orthogonal decomposition but will make no must be doubly must be doubly subscripted with first dimensioning parameter subscripted with first dimensioning parameter contain the N by contain the N by 1,...,NB. or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are 0 the subroutine will perform the orthogonal decomposition but will make no must be doubly must be doubly subscripted with first dimensioning parameter subscripted with first dimensioning parameter contain the N by contain the N by 1,...,NB. not applicable or not stated by selected source not a workspace argument

## 6. `MDB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are 0 the subroutine will perform the orthogonal decomposition but will make no 1 the array B(*) may be either doubly or singly subscripted.  In the latter case the value of MDB is arbitrary but it should be set to some valid integer M. The condition of NB.GT.1.AND.MDB.LT. MAX(M,N) is considered an error. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `NB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are 0 the subroutine will perform the 0 the subroutine will perform the orthogonal decomposition but will make no orthogonal decomposition but will make no must be doubly subscripted with first dimensioning parameter 1 the array B(*) may be either doubly or singly subscripted.  In the latter case the value of MDB is arbitrary but it should be set to some valid integer solution matrix X. or B(M)),RNORM(NB),H(N),G(N),IP(N).  This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are 0 the subroutine will perform the 0 the subroutine will perform the orthogonal decomposition but will make no orthogonal decomposition but will make no must be doubly subscripted with first dimensioning parameter 1 the array B(*) may be either doubly or singly subscripted.  In the latter case the value of MDB is arbitrary but it should be set to some valid integer solution matrix X. not applicable or not stated by selected source not a workspace argument

## 8. `TAU`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Absolute tolerance parameter provided by user for pseudorank determination. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `KRANK`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. rows of (R C). To be specific we suggest that the user consider an easily computable matrix norm, such as, the maximum of all column sums of magnitudes. Now if the relative uncertainty of B is EPS, (norm of uncertainty/ norm of B), it is suggested that TAU be set approximately equal to EPS*(norm of A). The user must dimension all arrays appearing in the call list.. Set by the subroutine to indicate the pseudorank of A. rows of (R C). To be specific we suggest that the user consider an easily computable matrix norm, such as, the maximum of all column sums of magnitudes. Now if the relative uncertainty of B is EPS, (norm of uncertainty/ norm of B), it is suggested that TAU be set approximately equal to EPS*(norm of A). The user must dimension all arrays appearing in the call list.. Set by the subroutine to indicate the pseudorank of A. not applicable or not stated by selected source not a workspace argument

## 10. `RNORM`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the Euclidean contain the Euclidean norm of the residual vector for the problem norm of the residual vector for the problem defined by the J-th column vector of the array defined by the J-th column vector of the array not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `H`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Arrays of working space used by HFTI. On return these arrays respectively contain elements of the pre- and post-multiplying Householder transformations used to compute the minimum Euclidean length solution. Arrays of working space used by HFTI. On return these arrays respectively contain elements of the pre- and post-multiplying Householder transformations used to compute the minimum Euclidean length solution. not applicable or not stated by selected source not a workspace argument

## 12. `G`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Arrays of working space used by HFTI. On return these arrays respectively contain elements of the pre- and post-multiplying Householder transformations used to compute the minimum Euclidean length solution. Arrays of working space used by HFTI. On return these arrays respectively contain elements of the pre- and post-multiplying Householder transformations used to compute the minimum Euclidean length solution. not applicable or not stated by selected source not a workspace argument

## 13. `IP`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Arrays of working space used by HFTI. Array in which the subroutine records indices describing the permutation of column vectors. The contents of arrays H(*),G(*) and IP(*) are not generally required by the user. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `TAU`: not a workspace argument
- `KRANK`: not a workspace argument
- `RNORM`: not a workspace argument
- `H`: not a workspace argument
- `G`: not a workspace argument
- `IP`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::hfti`
- Original SLATEC routine: `HFTI`
- Native symbol: `hfti_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [HFTI](https://www.netlib.org/slatec/src/hfti.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
