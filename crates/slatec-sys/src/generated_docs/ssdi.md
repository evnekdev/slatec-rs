# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) This routine is supplied with the SLAP package to perform the MSOLVE operation for iterative drivers that require diagonal Scaling (e.g., SSDCG, SSDBCG). It conforms to the SLAP MSOLVE CALLING CONVENTION and hence does not require an interface routine as do some of the other pre- conditioners supplied with SLAP. SEE ALSO SSDS, SSD2S

# Description

This canonical unsafe binding exposes original SLATEC routine `SSDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSDI](https://www.netlib.org/slatec/lin/ssdi.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of the Matrix.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

B(N). Vector to multiply the diagonal by.

## `X`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

X(N). Result of DIAG*B.

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

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (NELT).

DUMMY Real A(NELT).

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

DUMMY Integer. These are for compatibility with SLAP MSOLVE calling sequence.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine SSDS or SSD2S. The length of RWORK must be >= IWORK(4)+N.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (10).

IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines SSDS or SSD2S. Integer IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines SSDS or SSD2S.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `B`: not a workspace argument
- `X`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `RWORK`: RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine SSDS or SSD2S. The length of RWORK must be >= IWORK(4)+N.
- `IWORK`: Integer IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines SSDS or SSD2S.

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::ssdi`
- Original SLATEC routine: `SSDI`
- Native symbol: `ssdi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SSDI](https://www.netlib.org/slatec/lin/ssdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
