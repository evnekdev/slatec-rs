# Purpose

It is assumed that RWORK and IWORK have initialized with the information required for DLLTI2:

# Description

This canonical unsafe binding exposes original SLATEC routine `DSLLTI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSLLTI](https://www.netlib.org/slatec/lin/dsllti.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Input order of the sparse linear system and required vector length. Input order of the sparse linear system and required vector length. not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). X. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Writable output vector of length `N`. The interface computes the lower-triangular preconditioner solve `L^-1 * B` into this storage. Writable output vector of length `N`. The interface computes the lower-triangular preconditioner solve `L^-1 * B` into this storage. not applicable or not stated by selected source not a workspace argument

## 4. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Input number of stored nonzero entries in the sparse arrays `IA`, `JA`, and `A`. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). Readable integer sparse-index array with at least `NELT` entries. Together with `JA` it describes the SLAP column-format matrix or preconditioner data. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). Readable integer sparse-offset/index array with at least `NELT` entries as declared by this interface. Together with `IA` and `A` it identifies the SLAP column-format entries. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NELT). Readable sparse value array with at least `NELT` entries, stored in the SLAP column format described by `IA` and `JA`. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Input sparse-storage symmetry flag. `0` means all nonzero entries are present; `1` means symmetric storage contains only the lower triangle. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RWORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Readable real work/state array initialized for the underlying SLAP `*SLI2` routine. Its offsets are supplied through `IWORK`; native code uses it as preconditioner storage and does not retain it. not stated by selected source not applicable or not stated by selected source Caller-provided workspace; required extent is governed by the selected source and related size arguments.

## 10. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). NEL Starting location of IEL in IWORK. Starting location of JEL in IWORK. Starting location of EL in RWORK. Starting location of DINV in RWORK. See the DESCRIPTION of DLLTI2 for details. not stated by selected source not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `B`: not a workspace argument
- `X`: not a workspace argument
- `NELT`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `ISYM`: not a workspace argument
- `RWORK`: Caller-provided workspace; required extent is governed by the selected source and related size arguments.
- `IWORK`: NEL Starting location of IEL in IWORK. Starting location of JEL in IWORK. Starting location of EL in RWORK. Starting location of DINV in RWORK. See the DESCRIPTION of DLLTI2 for details.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsllti`
- Original SLATEC routine: `DSLLTI`
- Native symbol: `dsllti_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DSLLTI](https://www.netlib.org/slatec/lin/dsllti.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
