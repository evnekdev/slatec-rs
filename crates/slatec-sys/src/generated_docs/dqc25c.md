# Purpose

Integration rules for the computation of CAUCHY PRINCIPAL VALUE integrals Standard fortran subroutine Double precision version PARAMETERS

# Description

This canonical unsafe binding exposes original SLATEC routine `DQC25C`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQC25C](https://www.netlib.org/slatec/src/dqc25c.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Double precision Function subprogram defining the integrand function The actual name for F needs to be declared E X T E R N A L  in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Left end point of the integration interval not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Right end point of the integration interval, B.GT.A not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `C`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Parameter in the WEIGHT function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `RESULT`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Approximation to the integral is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. In the other case the 15-point Kronrod rule obtained by optimal addition of abscissae to the 7-point Gauss rule, is applied. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ABSERR`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `KRUL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `NEVAL`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations ...................................................................... not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `C`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `KRUL`: not a workspace argument
- `NEVAL`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqc25c`
- Original SLATEC routine: `DQC25C`
- Native symbol: `dqc25c_`
- ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_i32,mut_i32)`
- Exact Netlib source file: [DQC25C](https://www.netlib.org/slatec/src/dqc25c.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
