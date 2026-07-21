# Purpose

NON-ADAPTIVE INTEGRATION STANDARD FORTRAN SUBROUTINE REAL VERSION

# Description

This canonical unsafe binding exposes original SLATEC routine `QNG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QNG](https://www.netlib.org/slatec/src/qng.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real version Function subprogram defining the integrand function The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real version Lower limit of integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real version Upper limit of integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `EPSABS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Absolute accuracy requested AND not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `EPSREL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Relative accuracy requested If  EPSABS.LE.0 28), 28). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `RESULT`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral I POINT GAUSS-KRONROD RULE (RES21) obtained by optimal addition of abscissae to the 10-POINT GAUSS RULE (RES10), or by applying the 43-POINT RULE (RES43) obtained by optimal addition of abscissae to the 21-POINT GAUSS-KRONROD RULE, or by applying the 87-POINT RULE (RES87) obtained by optimal addition of abscissae to the 43-POINT RULE. are set to zero. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate of the modulus of the absolute error, which should EQUAL or EXCEED ABS(I-RESULT) are set to zero. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `NEVAL`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to zero. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. ON RETURN IER = 0 normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. It is assumed that the requested accuracy has not been achieved. 1 The maximum number of steps has been executed. The integral is probably too difficult to be calculated by DQNG. = 6 The input is invalid, because not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qng`
- Original SLATEC routine: `QNG`
- Native symbol: `qng_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [QNG](https://www.netlib.org/slatec/src/qng.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
