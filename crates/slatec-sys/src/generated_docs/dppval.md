# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DPPVAL is the PPVALU function of the reference.

# Description

This canonical unsafe binding exposes original SLATEC routine `DPPVAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPPVAL](https://www.netlib.org/slatec/src/dppval.f).

# Arguments

## 1. `LDC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. leading dimension of C matrix, LDC .GE. K leading dimension of C matrix, LDC .GE. K leading dimension of C matrix, LDC .GE. K not a workspace argument

## 2. `C`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDC, *). The Taylor expansion about XI(J) for X in are double precision matrix of dimension at least (K,LXI) containing right derivatives at break points XI(*). The Taylor expansion about XI(J) for X in are double precision matrix of dimension at least (K,LXI) containing right derivatives at break points XI(*). not applicable or not stated by selected source not a workspace argument

## 3. `XI`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). The Taylor expansion about XI(J) for X in 1,LXI. 1,LXI. are double precision break point vector of length LXI+1 The Taylor expansion about XI(J) for X in 1,LXI. 1,LXI. are double precision break point vector of length LXI+1 not applicable or not stated by selected source not a workspace argument

## 4. `LXI`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The Taylor expansion about XI(J) for X in 1 and set X=XI(J),J=2,LXI+1. number of polynomial pieces not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The Taylor expansion about XI(J) for X in order of B-spline, K .GE. 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IDERIV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. th derivative of the B-spline from the PP-representation. th derivative of the B-spline from the PP-representation order of the derivative, 0 .LE. IDERIV .LE. K-1 spline value not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. th derivative of the B-spline from the PP-representation 1,LXI. XI(J) are obtained.  DPPVAL will extrapolate beyond XI(1) and XI(LXI+1). To obtain left limiting values (left derivatives) at XI(J) are double precision argument, XI(1) .LE. X .LE. XI(LXI+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `INPPV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. an initialization parameter which must be set to 1 the first time DPPVAL is called. Output     DPPVAL is double precision INPPV contains information for efficient process- ing after the initial call and INPPV must not be changed by the user.  Distinct splines require distinct INPPV parameters. DPPVAL  - value of the IDERIV-th derivative at X an initialization parameter which must be set to 1 the first time DPPVAL is called. Output     DPPVAL is double precision INPPV contains information for efficient process- ing after the initial call and INPPV must not be changed by the user.  Distinct splines require distinct INPPV parameters. DPPVAL  - value of the IDERIV-th derivative at X not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input is a fatal error

# Workspace and array requirements

- `LDC`: not a workspace argument
- `C`: not a workspace argument
- `XI`: not a workspace argument
- `LXI`: not a workspace argument
- `K`: not a workspace argument
- `IDERIV`: not a workspace argument
- `X`: not a workspace argument
- `INPPV`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dppval`
- Original SLATEC routine: `DPPVAL`
- Native symbol: `dppval_`
- ABI fingerprint: `function:f64(mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`
- Exact Netlib source file: [DPPVAL](https://www.netlib.org/slatec/src/dppval.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
