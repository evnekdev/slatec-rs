# Purpose

Integration Rule Standard Fortran subroutine Real version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `QK15I`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QK15I](https://www.netlib.org/slatec/src/qk15i.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand FUNCTION F(X). The actual name for F needs to be Declared E X T E R N A L in the calling program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `BOUN`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Finite bound of original integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. +2) Integer 1, the original interval is (-INFINITY,BOUND), +1, the original interval is (BOUND,+INFINITY), +2, the original interval is (-INFINITY,+INFINITY) AND The integral is computed as the sum of two integrals, one over (-INFINITY,0) and one over (0,+INFINITY). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a part of (0,1). it is the purpose to compute I = Integral of transformed integrand over (A,B), J = Integral of ABS(Transformed Integrand) over (A,B). Real Lower limit for integration over subrange of (0,1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a part of (0,1). it is the purpose to compute I = Integral of transformed integrand over (A,B), J = Integral of ABS(Transformed Integrand) over (A,B). Real Upper limit for integration over subrange of (0,1) ON RETURN not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `RESULT`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral I POINT KRONROD RULE(RESK) obtained by optimal addition of abscissae to the 7-POINT GAUSS RULE(RESG). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate of the modulus of the absolute error, WHICH SHOULD EQUAL or EXCEED ABS(I-RESULT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `RESABS`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral J not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RESASC`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral of ABS((TRANSFORMED INTEGRAND)-I/(B-A)) over (A,B) not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `F`: not a workspace argument
- `BOUN`: not a workspace argument
- `INF`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `RESABS`: not a workspace argument
- `RESASC`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qk15i`
- Original SLATEC routine: `QK15I`
- Native symbol: `qk15i_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_i32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [QK15I](https://www.netlib.org/slatec/src/qk15i.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
