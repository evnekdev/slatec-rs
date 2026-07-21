# Purpose

DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. The problem data consists of an M by N matrix A, an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below. The NB column vectors of B represent right-side vectors for NB distinct linear least squares problems. This set of problems can also be written as the matrix least AX = B, where X is the N by NB solution matrix. Note that if B is the M by M identity matrix, then X will be the pseudo-inverse of A. This subroutine first transforms the augmented matrix (A B) to a matrix (R C) using premultiplying Householder transformations with column interchanges. All subdiagonal elements in the matrix R are zero and its diagonal elements satisfy ABS(R(I,I)).GE.ABS(R(I+1,I+1)), I = 1,...,L-1, where L = MIN(M,N). The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. Then a solution of minimum Euclidean length is computed using the first KRANK rows of (R C). To be specific we suggest that the user consider an easily computable matrix norm, such as, the maximum of all column sums of magnitudes. Now if the relative uncertainty of B is EPS, (norm of uncertainty/ norm of B), it is suggested that TAU be set approximately equal to EPS*(norm of A). The user must dimension all arrays appearing in the call list.. A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N). This permits the solution of a range of problems in the same array space. The entire set of parameters for DHFTI are

# Description

This canonical unsafe binding exposes original SLATEC routine `DHFTI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DHFTI](https://www.netlib.org/slatec/src/dhfti.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (MDA, *).

The array A(*,*) initially contains the M by N matrix A of the least squares problem AX = B. The first dimensioning parameter of the array is MDA, which must satisfy MDA. GE. M Either M. N or M. LT.

## `MDA`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array A(*,*) initially contains the M by N matrix A of the least squares problem AX = B. The first dimensioning parameter of the array.

## `M`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array A(*,*) initially contains the M by N matrix A of the least squares problem AX = B. The first dimensioning parameter of the array.

## `N`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array A(*,*) initially contains the M by N matrix A of the least squares problem AX = B. The first dimensioning parameter of the array.

## `B`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (MDB, *).

If NB = 0 the subroutine will perform the orthogonal decomposition but will make no references to the array B(*). If NB. GT. 0 the array B(*) must initially contain the M by NB matrix B of the least squares problem AX = If NB. GE. 2 the array B(*) must be doubly subscripted with first dimensioning parameter MDB.

## `MDB`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

If NB = 0 the subroutine will perform the orthogonal decomposition but will make no references to the array B(*). If NB. GT. 0 the array B(*) must initially contain the M by NB matrix B of the least squares problem AX =.

## `NB`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

If NB = 0 the subroutine will perform the orthogonal decomposition but will make no references to the array B(*). If NB. GT. 0 the array B(*) must initially contain the M by NB matrix B of the least squares problem AX =.

## `TAU`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Absolute tolerance parameter provided by user for pseudorank determination.

## `KRANK`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Set by the subroutine to indicate the pseudorank of A.

## `RNORM`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

On return, RNORM(J) will contain the Euclidean norm of the residual vector for the problem defined by the J-th column vector of the array B(*,*) for J = 1,. ,NB.

## `H`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Arrays of working space used by DHFTI. On return these arrays respectively contain elements of the pre- and post-multiplying Householder transformations used to compute the minimum Euclidean length solution.

## `G`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Arrays of working space used by DHFTI. On return these arrays respectively contain elements of the pre- and post-multiplying Householder transformations used to compute the minimum Euclidean length solution.

## `IP`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Arrays of working space used by DHFTI. Array in which the subroutine records indices describing the permutation of column vectors. The contents of arrays H(*),G(*) and IP(*) are not generally required by the user.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `RNORM`: not a workspace argument
- `H`: not a workspace argument
- `G`: not a workspace argument
- `IP`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dhfti`
- Original SLATEC routine: `DHFTI`
- Native symbol: `dhfti_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DHFTI](https://www.netlib.org/slatec/src/dhfti.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
