# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) This routine is supplied with the SLAP package to perform the MSOLVE operation for iterative drivers that require diagonal Scaling (e.g., SSDCG, SSDBCG). It conforms to the SLAP MSOLVE CALLING CONVENTION and hence does not require an interface routine as do some of the other pre- conditioners supplied with SLAP.

# Description

This canonical unsafe binding exposes original SLATEC routine `SSDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSDI](https://www.netlib.org/slatec/lin/ssdi.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer Order of the Matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). IN       Real B(N). Vector to multiply the diagonal by. by.  This array must be set by the user or by a call to the SLAP routine SSDS or SSD2S.  The length of RWORK must be >= IWORK(4)+N. conditioner setup routines SSDS or SSD2S. IN       Real B(N). Vector to multiply the diagonal by. by.  This array must be set by the user or by a call to the SLAP routine SSDS or SSD2S.  The length of RWORK must be >= IWORK(4)+N. conditioner setup routines SSDS or SSD2S. not applicable or not stated by selected source not a workspace argument

## 3. `X`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). DIAG*B, where DIAG is a diagonal matrix. OUT      Real X(N). Result of DIAG*B. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. DUMMY    Integer. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). DUMMY    Integer IA(NELT). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). DUMMY    Integer JA(NELT). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NELT). DUMMY    Real A(NELT). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. DUMMY    Integer. These are for compatibility with SLAP MSOLVE calling sequence. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RWORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). IN       Real RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale not stated by selected source not applicable or not stated by selected source

## 10. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (10). IN       Integer IWORK(10). holds the offset into RWORK for the diagonal matrix not stated by selected source not applicable or not stated by selected source

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
- `RWORK`: IN       Real RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale
- `IWORK`: IN       Integer IWORK(10). holds the offset into RWORK for the diagonal matrix

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::ssdi`
- Original SLATEC routine: `SSDI`
- Native symbol: `ssdi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SSDI](https://www.netlib.org/slatec/lin/ssdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
