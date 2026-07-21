# Purpose

Usage: PARAMETER (INCFD = ...) INTEGER N, ISMON(N), IERR REAL X(N), F(INCFD,N), D(INCFD,N) LOGICAL SKIP CALL PCHCM (N, X, F, D, INCFD, SKIP, ISMON, IERR) PCHCM: Piecewise Cubic Hermite -- Check Monotonicity. Checks the piecewise cubic Hermite function defined by N,X,F,D for monotonicity. To provide compatibility with PCHIM and PCHIC, includes an increment between successive values of the F- and D-arrays. Cautions: This provides the same capability as old PCHMC, except that a new output value, -3, was added February 1989. (Formerly, -3 and +3 were lumped together in the single value 3.) Codes that flag nonmonotonicity by "IF (ISMON.EQ.2)" need not be changed. Codes that check via "IF (ISMON.GE.3)" should change the test to "IF (IABS(ISMON).GE.3)". Codes that declare monotonicity via "IF (ISMON.LE.1)" should change to "IF (IABS(ISMON).LE.1)".

# Description

This canonical unsafe binding exposes original SLATEC routine `PCHCM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PCHCM](https://www.netlib.org/slatec/pchip/pchcm.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of data points. (Error return if N. LT. 2. ).

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

is a real array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ).

## `F`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (INCFD, N).

is a real array of function values. F(1+(I-1)*INCFD) is the value corresponding to X(I).

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (INCFD, N).

is a real array of derivative values. D(1+(I-1)*INCFD) is the value corresponding to X(I).

## `INCFD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the increment between successive values in F and D. (Error return if INCFD. LT. 1. ).

## `SKIP`

**Direction:** `input-output`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

is a logical variable which should be set to. TRUE. if the user wishes to skip checks for validity of preceding parameters, or to. FALSE. otherwise. This will save time in case these checks have already been performed.

## `ISMON`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (N).

is an integer array indicating on which intervals the PCH function defined by N, X, F, D is monotonic. For data interval \[X(I),X(I+1)\], -3 if function is probably decreasing; -1 if function is strictly decreasing; 0 if function is constant; 1 if function is strictly increasing; 2 if function is non-monotonic; 3 if function is probably increasing. If ABS(ISMON)=3, this means that the D-values are near the boundary of the monotonicity region. A small increase produces non-monotonicity; decrease, strict monotonicity. The above applies to I=1(1)N-1. ISMON(N) indicates whether the entire function is monotonic on \[X(1),X(N)\].

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an error flag. Normal return: 0 (no errors). "Recoverable" errors: -1 if N. LT. 2. -2 if INCFD.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `ISMON`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::pchcm`
- Original SLATEC routine: `PCHCM`
- Native symbol: `pchcm_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [PCHCM](https://www.netlib.org/slatec/pchip/pchcm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
