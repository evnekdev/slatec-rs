# Purpose

SSPEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form. Call Sequence Parameters- (The values of parameters marked with * (star) will be changed by SSPEV.)

# Description

This canonical unsafe binding exposes original SLATEC routine `SSPEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSPEV](https://www.netlib.org/slatec/lin/sspev.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N*(N+1)/2) real symmetric packed input matrix.  Contains upper triangle and diagonal of A, by column (elements 11, 12, 22, 13, 23, 33, ...). are stored in the first N columns of V.  See also INFO below. must be distinct arrays. Also, if LDA .GT. LDV, SSPEV changes all the elements of A thru column N.  If LDA < LDV, SSPEV changes all the elements of V through REAL(N*(N+1)/2) real symmetric packed input matrix.  Contains upper triangle and diagonal of A, by column (elements 11, 12, 22, 13, 23, 33, ...). are stored in the first N columns of V.  See also INFO below. must be distinct arrays. Also, if LDA .GT. LDV, SSPEV changes all the elements of A thru column N.  If LDA < LDV, SSPEV changes all the elements of V through not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER set by the user to the order of the matrix A. are stored in the first N columns of V.  See also INFO below. LDV, only A(I,J) and V(I, J) for I,J = 1,...,N are changed by SSPEV. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `E`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) on return from SSPEV, E contains the eigenvalues of A. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `V`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDV, *). REAL(LDV,N) on return from SSPEV, if the user has set JOB = 0        V is not referenced. is also set nonzero.  In that case, N must be .LE. LDV. If JOB is set to zero, LDV is not referenced. are referenced. = nonzero  eigenvalues and vectors to be calculated. must be distinct arrays. Also, if LDA .GT. LDV, SSPEV changes all the elements of A thru column N.  If LDA < LDV, SSPEV changes all the elements of V through REAL(LDV,N) on return from SSPEV, if the user has set JOB = 0        V is not referenced. is also set nonzero.  In that case, N must be .LE. LDV. If JOB is set to zero, LDV is not referenced. are referenced. = nonzero  eigenvalues and vectors to be calculated. must be distinct arrays. Also, if LDA .GT. LDV, SSPEV changes all the elements of A thru column N.  If LDA < LDV, SSPEV changes all the elements of V through not applicable or not stated by selected source not a workspace argument

## 5. `LDV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER set by the user to are referenced. = nonzero  eigenvalues and vectors to be calculated. not stated by selected source INTEGER set by the user to are referenced. = nonzero  eigenvalues and vectors to be calculated. not a workspace argument

## 6. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(2N) temporary storage vector.  Contents changed by SSPEV. not stated by selected source not applicable or not stated by selected source

## 7. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is also set nonzero.  In that case, N must be .LE. LDV. If JOB is set to zero, LDV is not referenced. INTEGER set by the user to = 0        eigenvalues only to be calculated by SSPEV. is also set nonzero.  In that case, N must be .LE. LDV. If JOB is set to zero, LDV is not referenced. INTEGER set by the user to = 0        eigenvalues only to be calculated by SSPEV. not applicable or not stated by selected source not a workspace argument

## 8. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER on return from SSPEV, the value of INFO is = 0 for normal return. = K if the eigenvalue iteration fails to converge. Eigenvalues and vectors 1 through K-1 are correct. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

No. 1   recoverable  N is greater than LDV and JOB is nonzero No. 2   recoverable  N is less than one

# Workspace and array requirements

- `A`: not a workspace argument
- `N`: not a workspace argument
- `E`: not a workspace argument
- `V`: not a workspace argument
- `LDV`: not a workspace argument
- `WORK`: REAL(2N) temporary storage vector.  Contents changed by SSPEV.
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::sspev`
- Original SLATEC routine: `SSPEV`
- Native symbol: `sspev_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SSPEV](https://www.netlib.org/slatec/lin/sspev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
