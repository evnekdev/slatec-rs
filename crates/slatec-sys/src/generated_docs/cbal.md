# Purpose

This subroutine is a translation of the ALGOL procedure CBALANCE, which is a complex version of BALANCE, NUM. MATH. 13, 293-304(1969) by Parlett and Reinsch. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 315-326(1971). This subroutine balances a COMPLEX matrix and isolates eigenvalues whenever possible.

# Description

This canonical unsafe binding exposes original SLATEC routine `CBAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBAL](https://www.netlib.org/slatec/lin/cbal.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, AR and AI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM.

## `AR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex matrix to be balanced. two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). respectively, of the balanced matrix.

## `AI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex matrix to be balanced. two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). respectively, of the balanced matrix.

## `LOW`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables such that AR(I,J) and AI(I,J) are equal to zero if (1) I is greater than J and (2) J=1,. ,LOW-1 or I=IGH+1,. ,N.

## `IGH`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables such that AR(I,J) and AI(I,J) are equal to zero if (1) I is greater than J and (2) J=1,. ,LOW-1 or I=IGH+1,. ,N.

## `SCALE`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains information determining the permutations and scaling factors used. SCALE is a one-dimensional REAL array, dimensioned SCALE(N). Suppose that the principal submatrix in rows LOW through IGH has been balanced, that P(J) denotes the index interchanged with J during the permutation step, and that the elements of the diagonal matrix used are denoted by D(I,J). Then P(J), for J = 1,. ,LOW-1 = D(J,J) J = LOW,. ,IGH = P(J) J = IGH+1,.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `SCALE`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cbal`
- Original SLATEC routine: `CBAL`
- Native symbol: `cbal_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [CBAL](https://www.netlib.org/slatec/lin/cbal.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
