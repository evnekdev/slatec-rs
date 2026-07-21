# Purpose

This subroutine is a translation of the bisection technique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval, using bisection.

# Description

This canonical unsafe binding exposes original SLATEC routine `BISECT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BISECT](https://www.netlib.org/slatec/lin/bisect.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable.

## `EPS1`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is an absolute error tolerance for the computed eigenvalues. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. is a REAL variable. is unaltered unless it has been reset to its (last) default value.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the input matrix. is a one-dimensional REAL array, dimensioned D(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.

## `E`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the input matrix in its last N-1 positions. E(1) is arbitrary. is a one-dimensional REAL array, dimensioned E(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.

## `E2`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E. is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). is also set to zero.

## `LB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables.

## `UB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables.

## `MM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

should be set to an upper bound for the number of eigenvalues in the interval. WARNING - If more than MM eigenvalues are determined to lie in the interval, an error return is made with no eigenvalues found. is an INTEGER variable.

## `M`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvalues determined to lie in (LB,UB). is an INTEGER variable.

## `W`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the M eigenvalues in ascending order. is a one-dimensional REAL array, dimensioned W(MM).

## `IND`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. is an one-dimensional INTEGER array, dimensioned IND(MM).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, 3*N+1 if M exceeds MM. In this case, M contains the number of eigenvalues determined to lie in (LB,UB).

## `RV4`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY.

## `RV5`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

`IND` is a documented status output; its bounded argument contract states the available source semantics.

# Workspace and array requirements

- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `W`: not a workspace argument
- `IND`: not a workspace argument
- `RV4`: not a workspace argument
- `RV5`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bisect`
- Original SLATEC routine: `BISECT`
- Native symbol: `bisect_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BISECT](https://www.netlib.org/slatec/lin/bisect.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
