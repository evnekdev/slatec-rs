# Purpose

This subroutine is a translation of the ALGOL procedure BALANCE, NUM. MATH. 13, 293-304(1969) by Parlett and Reinsch. HANDBOOK FOR AUTO. COMP., Vol.II-LINEAR ALGEBRA, 315-326(1971). This subroutine balances a REAL matrix and isolates eigenvalues whenever possible.

# Description

This canonical unsafe binding exposes original SLATEC routine `BALANC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BALANC](https://www.netlib.org/slatec/lin/balanc.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the input matrix to be balanced. A is a two-dimensional REAL array, dimensioned A(NM,N). contains the balanced matrix.

## `LOW`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables such that A(I,J) is equal to zero if (1) I is greater than J and (2) J=1,. ,LOW-1 or I=IGH+1,. ,N.

## `IGH`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables such that A(I,J) is equal to zero if (1) I is greater than J and (2) J=1,. ,LOW-1 or I=IGH+1,. ,N.

## `SCALE`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains information determining the permutations and scaling factors used. SCALE is a one-dimensional REAL array, dimensioned SCALE(N). Suppose that the principal submatrix in rows LOW through IGH has been balanced, that P(J) denotes the index interchanged with J during the permutation step, and that the elements of the diagonal matrix used are denoted by D(I,J). Then P(J), for J = 1,. ,LOW-1 = D(J,J), J = LOW,. ,IGH = P(J) J = IGH+1,.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `SCALE`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::balanc`
- Original SLATEC routine: `BALANC`
- Native symbol: `balanc_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BALANC](https://www.netlib.org/slatec/lin/balanc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
