# Purpose

Integration rules for integrands having ALGEBRAICO-LOGARITHMIC end point singularities Standard fortran subroutine Real version PARAMETERS

# Description

This canonical unsafe binding exposes original SLATEC routine `QC25S`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QC25S](https://www.netlib.org/slatec/src/qc25s.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand The actual name for F needs to be declared E X T E R N A L  in the driver program. I/(B-A)) The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a part of (A,B). Real Left end point of the original interval not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a part of (A,B). Real Right end point of the original interval, B.GT.A X) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `BL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a part of (A,B). Real Lower limit of integration, BL.GE.A not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BR`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a part of (A,B). Real Upper limit of integration, BR.LE.B not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ALFA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real PARAMETER IN THE WEIGHT FUNCTION not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `BETA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Parameter in the weight function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `RI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RJ`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `RG`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `RH`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `RESULT`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral is computed by using a generalized CLENSHAW-CURTIS method if B1 = A or BR = B. in all other cases the 15-POINT KRONROD RULE is applied, obtained by optimal addition of Abscissae to the 7-POINT GAUSS RULE. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `RESASC`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real not stated by selected source not applicable or not stated by selected source not a workspace argument

## 15. `INTEGR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Which determines the weight function = 1   W(X) = (X-A)**ALFA*(B-X)**BETA = 2   W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3   W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4   W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(X-A)* not stated by selected source not applicable or not stated by selected source not a workspace argument

## 16. `NEV`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `F`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `BL`: not a workspace argument
- `BR`: not a workspace argument
- `ALFA`: not a workspace argument
- `BETA`: not a workspace argument
- `RI`: not a workspace argument
- `RJ`: not a workspace argument
- `RG`: not a workspace argument
- `RH`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `RESASC`: not a workspace argument
- `INTEGR`: not a workspace argument
- `NEV`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qc25s`
- Original SLATEC routine: `QC25S`
- Native symbol: `qc25s_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_f32,mut_i32,mut_i32)`
- Exact Netlib source file: [QC25S](https://www.netlib.org/slatec/src/qc25s.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
