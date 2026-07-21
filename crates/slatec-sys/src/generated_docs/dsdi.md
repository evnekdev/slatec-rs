# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) DOUBLE PRECISION B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL DSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) This routine is supplied with the SLAP package to perform the MSOLVE operation for iterative drivers that require diagonal Scaling (e.g., DSDCG, DSDBCG). It conforms to the SLAP MSOLVE CALLING CONVENTION and hence does not require an interface routine as do some of the other pre- conditioners supplied with SLAP. SEE ALSO DSDS, DSD2S

# Description

This canonical unsafe binding exposes original SLATEC routine `DSDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSDI](https://www.netlib.org/slatec/lin/dsdi.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of the Matrix.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Double Precision B(N). Vector to multiply the diagonal by.

## `X`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Double Precision X(N). Result of DIAG*B.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

DUMMY Integer.

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

DUMMY Integer IA(NELT).

## `JA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

DUMMY Integer JA(NELT).

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NELT).

DUMMY Double Precision A(NELT).

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

DUMMY Integer. These are for compatibility with SLAP MSOLVE calling sequence.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Double Precision RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine DSDS or DSD2S. The length of RWORK must be >= IWORK(4)+N.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (10).

IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines DSDS or DSD2S. Integer IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines DSDS or DSD2S.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `B`: not a workspace argument
- `X`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `RWORK`: Double Precision RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine DSDS or DSD2S. The length of RWORK must be >= IWORK(4)+N.
- `IWORK`: Integer IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines DSDS or DSD2S.

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dsdi`
- Original SLATEC routine: `DSDI`
- Native symbol: `dsdi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSDI](https://www.netlib.org/slatec/lin/dsdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
