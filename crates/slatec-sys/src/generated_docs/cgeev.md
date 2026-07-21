# Purpose

CGEEV computes the eigenvalues and, optionally, the eigenvectors of a general complex matrix. Call Sequence Parameters- (The values of parameters marked with * (star) will be changed by CGEEV.)

# Description

This canonical unsafe binding exposes original SLATEC routine `CGEEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGEEV](https://www.netlib.org/slatec/src/cgeev.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). COMPLEX(LDA,N) complex nonsymmetric input matrix. are stored in the first N columns of V.  See also INFO below. (If the input matrix A is nearly degenerate, V will be badly conditioned, i.e. have nearly dependent columns.) must be distinct arrays. Also,  if LDA > LDV,  CGEEV changes all the elements of A thru column N.  If LDA < LDV, CGEEV changes all the elements of V through COMPLEX(LDA,N) complex nonsymmetric input matrix. are stored in the first N columns of V.  See also INFO below. (If the input matrix A is nearly degenerate, V will be badly conditioned, i.e. have nearly dependent columns.) must be distinct arrays. Also,  if LDA > LDV,  CGEEV changes all the elements of A thru column N.  If LDA < LDV, CGEEV changes all the elements of V through not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER set by the user to the leading dimension of the complex array A. LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CGEEV. INTEGER set by the user to the leading dimension of the complex array A. LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CGEEV. INTEGER set by the user to the leading dimension of the complex array A. LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CGEEV. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. are stored in the first N columns of V.  See also INFO below. (If the input matrix A is nearly degenerate, V will be badly conditioned, i.e. have nearly dependent columns.) LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CGEEV. by N input elements have been changed No. 5  warning      LDA < LDV,  elements of V other than the by N output elements have been changed not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `E`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). COMPLEX(N) on return from CGEEV E contains the eigenvalues of A. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `V`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). COMPLEX(LDV,N) on return from CGEEV if the user has set JOB = 0        V is not referenced. is also set nonzero.  In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. are referenced. = nonzero  eigenvalues and vectors to be calculated. must be distinct arrays. Also,  if LDA > LDV,  CGEEV changes all the elements of A thru column N.  If LDA < LDV, CGEEV changes all the elements of V through COMPLEX(LDV,N) on return from CGEEV if the user has set JOB = 0        V is not referenced. is also set nonzero.  In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. are referenced. = nonzero  eigenvalues and vectors to be calculated. must be distinct arrays. Also,  if LDA > LDV,  CGEEV changes all the elements of A thru column N.  If LDA < LDV, CGEEV changes all the elements of V through not applicable or not stated by selected source not a workspace argument

## 6. `LDV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER set by the user to are referenced. = nonzero  eigenvalues and vectors to be calculated. not stated by selected source INTEGER set by the user to are referenced. = nonzero  eigenvalues and vectors to be calculated. not a workspace argument

## 7. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(3N) temporary storage vector.  Contents changed by CGEEV. not stated by selected source not applicable or not stated by selected source

## 8. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is also set nonzero.  In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. INTEGER set by the user to = 0        eigenvalues only to be calculated by CGEEV. is also set nonzero.  In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. INTEGER set by the user to = 0        eigenvalues only to be calculated by CGEEV. not applicable or not stated by selected source not a workspace argument

## 9. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER on return from CGEEV the value of INFO is = 0  normal return, calculation successful. = K  if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero). not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

No. 1  recoverable  N is greater than LDA No. 2  recoverable  N is less than one. No. 3  recoverable  JOB is nonzero and N is greater than LDV No. 4  warning      LDA > LDV,  elements of A other than the

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `E`: not a workspace argument
- `V`: not a workspace argument
- `LDV`: not a workspace argument
- `WORK`: REAL(3N) temporary storage vector.  Contents changed by CGEEV.
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cgeev`
- Original SLATEC routine: `CGEEV`
- Native symbol: `cgeev_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CGEEV](https://www.netlib.org/slatec/src/cgeev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
