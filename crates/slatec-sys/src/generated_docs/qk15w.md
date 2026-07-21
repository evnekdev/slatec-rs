# Purpose

Integration rules Standard fortran subroutine Real version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `QK15W`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QK15W](https://www.netlib.org/slatec/src/qk15w.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. I/(B-A)) The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `W`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand WEIGHT function W(X). The actual name for W needs to be declared E X T E R N A L in the calling program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `P1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Parameters in the WEIGHT function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `P2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Parameters in the WEIGHT function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `P3`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Parameters in the WEIGHT function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `P4`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Parameters in the WEIGHT function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `KP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Key for indicating the type of WEIGHT function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Lower limit of integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Upper limit of integration ON RETURN not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `RESULT`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral I point Kronrod rule (RESK) obtained by optimal addition of abscissae to the 7-point Gauss rule (RESG). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `RESABS`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral of ABS(F) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `RESASC`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `F`: not a workspace argument
- `W`: not a workspace argument
- `P1`: not a workspace argument
- `P2`: not a workspace argument
- `P3`: not a workspace argument
- `P4`: not a workspace argument
- `KP`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `RESABS`: not a workspace argument
- `RESASC`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qk15w`
- Original SLATEC routine: `QK15W`
- Native symbol: `qk15w_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),fn:f32(ref_f32,ref_f32,ref_f32,ref_f32,ref_f32,ref_i32),mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [QK15W](https://www.netlib.org/slatec/src/qk15w.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
