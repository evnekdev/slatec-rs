# Purpose

Integration Rule Standard Fortran subroutine Real version

# Description

This canonical unsafe binding exposes original SLATEC routine `QK15I`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QK15I](https://www.netlib.org/slatec/src/qk15i.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand FUNCTION F(X). The actual name for F needs to be Declared E X T E R N A L in the calling program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `BOUN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Finite bound of original integration Range (SET TO ZERO IF INF = +2).

## `INF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

If INF = -1, the original interval is (-INFINITY,BOUND), If INF = +1, the original interval is (BOUND,+INFINITY), If INF = +2, the original interval is (-INFINITY,+INFINITY) AND The integral is computed as the sum of two integrals, one over (-INFINITY,0) and one over (0,+INFINITY).

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Lower limit for integration over subrange of (0,1).

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Upper limit for integration over subrange of (0,1).

## `RESULT`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral I is computed by applying the 15-POINT KRONROD RULE(RESK) obtained by optimal addition of abscissae to the 7-POINT GAUSS RULE(RESG).

## `ABSERR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Estimate of the modulus of the absolute error, WHICH SHOULD EQUAL or EXCEED ABS(I-RESULT).

## `RESABS`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral J.

## `RESASC`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral of ABS((TRANSFORMED INTEGRAND)-I/(B-A)) over (A,B).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qk15i`
- Original SLATEC routine: `QK15I`
- Native symbol: `qk15i_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_i32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [QK15I](https://www.netlib.org/slatec/src/qk15i.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
