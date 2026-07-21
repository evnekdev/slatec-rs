# Purpose

This subroutine forms the eigenvectors of a NONSYMMETRIC TRIDIAGONAL matrix by back transforming those of the corresponding symmetric matrix determined by FIGI.

# Description

This canonical unsafe binding exposes original SLATEC routine `BAKVEC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BAKVEC](https://www.netlib.org/slatec/lin/bakvec.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, T and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, T and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix T.  N is an INTEGER variable. must be less than or equal to NM. 1 positions of the first column, its diagonal in the N positions of the second column, 1 positions of are arbitrary. 1 positions.  E(1) is arbitrary. 1,3) non-zero. In this case, the symmetric matrix is not similar to the original matrix, and the eigenvectors cannot be found by this program. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix T.  N is an INTEGER variable. must be less than or equal to NM. 1 positions of the first column, its diagonal in the N positions of the second column, 1 positions of are arbitrary. 1 positions.  E(1) is arbitrary. 1,3) non-zero. In this case, the symmetric matrix is not similar to the original matrix, and the eigenvectors cannot be found by this program. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 3. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, 3). contains the nonsymmetric matrix.  Its subdiagonal is are arbitrary. are arbitrary. dimensional REAL array, dimensioned T(NM,3). is unaltered. 1,3) non-zero. 1,3) non-zero. In this case, the symmetric matrix is not similar In this case, the symmetric matrix is not similar to the original matrix, and the eigenvectors to the original matrix, and the eigenvectors cannot be found by this program. cannot be found by this program. Questions and comments should be directed to B. S. Garbow, Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the nonsymmetric matrix.  Its subdiagonal is are arbitrary. are arbitrary. dimensional REAL array, dimensioned T(NM,3). is unaltered. 1,3) non-zero. 1,3) non-zero. In this case, the symmetric matrix is not similar In this case, the symmetric matrix is not similar to the original matrix, and the eigenvectors to the original matrix, and the eigenvectors cannot be found by this program. cannot be found by this program. Questions and comments should be directed to B. S. Garbow, Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 4. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned E(N). is destroyed. 1,3) non-zero. In this case, the symmetric matrix is not similar to the original matrix, and the eigenvectors cannot be found by this program. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned E(N). is destroyed. 1,3) non-zero. In this case, the symmetric matrix is not similar to the original matrix, and the eigenvectors cannot be found by this program. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 5. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvectors to be back transformed. is an INTEGER variable. dimensional REAL array, dimensioned Z(NM,M). is the number of eigenvectors to be back transformed. is an INTEGER variable. dimensional REAL array, dimensioned Z(NM,M). not applicable or not stated by selected source not a workspace argument

## 6. `Z`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the eigenvectors to be back transformed dimensional REAL array, dimensioned Z(NM,M). contains the transformed eigenvectors in its first M columns. contains the eigenvectors to be back transformed dimensional REAL array, dimensioned Z(NM,M). contains the transformed eigenvectors in its first M columns. not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `T`: not a workspace argument
- `E`: not a workspace argument
- `M`: not a workspace argument
- `Z`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bakvec`
- Original SLATEC routine: `BAKVEC`
- Native symbol: `bakvec_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [BAKVEC](https://www.netlib.org/slatec/lin/bakvec.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
