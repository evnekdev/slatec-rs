# Purpose

Usage: PCHCM: Piecewise Cubic Hermite -- Check Monotonicity. Checks the piecewise cubic Hermite function defined by N,X,F,D for monotonicity. To provide compatibility with PCHIM and PCHIC, includes an

# Description

This canonical unsafe binding exposes original SLATEC routine `PCHCM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PCHCM](https://www.netlib.org/slatec/pchip/pchcm.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN  is the number of data points.  (Error return if N.LT.2 .) is monotonic. For data interval \[X(I),X(I+1)\], 1.  ISMON(N) indicates whether the entire function is monotonic on \[X(1),X(N)\]. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). IN  is a real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. is monotonic. For data interval \[X(I),X(I+1)\], not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (INCFD, N). IN  is a real array of function values.  F(1+(I-1)*INCFD) is the value corresponding to X(I). is monotonic. For data interval \[X(I),X(I+1)\], and D-arrays. Cautions: This provides the same capability as old PCHMC, except that a new output value, -3, was added February 1989.  (Formerly, -3 and +3 were lumped together in the single value 3.)  Codes that flag nonmonotonicity by "IF (ISMON.EQ.2)" need not be changed. Codes that check via "IF (ISMON.GE.3)" should change the test to "IF (IABS(ISMON).GE.3)".  Codes that declare monotonicity via "IF (ISMON.LE.1)" should change to "IF (IABS(ISMON).LE.1)". not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (INCFD, N). IN  is a real array of derivative values.  D(1+(I-1)*INCFD) is the value corresponding to X(I). is monotonic. For data interval \[X(I),X(I+1)\], values are near the boundary of the monotonicity region.  A small increase produces non-monotonicity; decrease, strict monotonicity. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCFD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ...) INTEGER  N, ISMON(N), IERR REAL  X(N), F(INCFD,N), D(INCFD,N) LOGICAL  SKIP CALL  PCHCM (N, X, F, D, INCFD, SKIP, ISMON, IERR) IN  is the increment between successive values in F and D. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `SKIP`

input-output `scalar` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and scalar. INOUT  is a logical variable which should be set to .TRUE. if the user wishes to skip checks for validity of preceding parameters, or to .FALSE. otherwise. This will save time in case these checks have already been performed. will be set to .TRUE. on normal return. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `ISMON`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (N). OUT  is an integer array indicating on which intervals the 3  if function is probably decreasing; 1  if function is strictly decreasing; 0  if function is constant; 1  if function is strictly increasing; monotonic; 3  if function is probably increasing. values are near the boundary of the monotonicity region.  A small increase produces non-monotonicity; decrease, strict monotonicity. array has not been changed in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. 3 and modified code so that 1,3,-1 produces ISMON(N)=2, rather than 3. 890306  Added caution about changed output. 890407  Changed name from PCHMC to PCHCM, as requested at the March 1989 SLATEC CML meeting, and made a few other minor modifications necessitated by this change. 890407  Converted to new SLATEC format. 890407  Modified DESCRIPTION to LDOC format. 900315  CALLs to XERROR changed to CALLs to XERMSG.  (THJ) 920429  Revised format and order of references.  (WRB,FNF) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT  is an error flag. Normal return: 0  (no errors). "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `INCFD`: not a workspace argument
- `SKIP`: not a workspace argument
- `ISMON`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::pchcm`
- Original SLATEC routine: `PCHCM`
- Native symbol: `pchcm_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [PCHCM](https://www.netlib.org/slatec/pchip/pchcm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
