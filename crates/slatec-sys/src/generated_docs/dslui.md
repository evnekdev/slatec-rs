# Purpose

It is assumed that RWORK and IWORK have initialized with the information required for DSLUI2:

# Description

This canonical unsafe binding exposes original SLATEC routine `DSLUI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSLUI](https://www.netlib.org/slatec/lin/dslui.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input order of the sparse linear system and required vector length.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Readable right-hand-side vector of length `N` for the SLAP lower-triangular preconditioner solve. The computed vector is returned through `X`, not by retaining `B`.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Writable output vector of length `N`. The interface computes the lower-triangular preconditioner solve `L^-1 * B` into this storage.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input number of stored nonzero entries in the sparse arrays `IA`, `JA`, and `A`.

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

Readable integer sparse-index array with at least `NELT` entries. Together with `JA` it describes the SLAP column-format matrix or preconditioner data.

## `JA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

Readable integer sparse-offset/index array with at least `NELT` entries as declared by this interface. Together with `IA` and `A` it identifies the SLAP column-format entries.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NELT).

Readable sparse value array with at least `NELT` entries, stored in the SLAP column format described by `IA` and `JA`.

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input sparse-storage symmetry flag. `0` means all nonzero entries are present; `1` means symmetric storage contains only the lower triangle.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Readable real work/state array initialized for the underlying SLAP `*SLI2` routine. Its offsets are supplied through `IWORK`; native code uses it as preconditioner storage and does not retain it.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (10).

Starting location of IL in IWORK. Starting location of JL in IWORK. Starting location of IU in IWORK. Starting location of JU in IWORK. Starting location of L in RWORK. Starting location of DINV in RWORK. Starting location of IL in IWORK. Starting location of JL in IWORK. Starting location of IU in IWORK. Starting location of JU in IWORK. Starting location of L in RWORK. Starting location of DINV in RWORK. Starting location of U in RWORK. See the DESCRIPTION of DSLUI2 for details.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `B`: not a workspace argument
- `X`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `RWORK`: not applicable or not stated by selected source
- `IWORK`: Starting location of IL in IWORK. Starting location of JL in IWORK. Starting location of IU in IWORK. Starting location of JU in IWORK. Starting location of L in RWORK. Starting location of DINV in RWORK. Starting location of U in RWORK. See the DESCRIPTION of DSLUI2 for details.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dslui`
- Original SLATEC routine: `DSLUI`
- Native symbol: `dslui_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DSLUI](https://www.netlib.org/slatec/lin/dslui.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
