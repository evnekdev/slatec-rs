# Purpose

NON-ADAPTIVE INTEGRATION STANDARD FORTRAN SUBROUTINE REAL VERSION

# Description

This canonical unsafe binding exposes original SLATEC routine `QNG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QNG](https://www.netlib.org/slatec/src/qng.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

version Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

version Lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

version Upper limit of integration.

## `EPSABS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Absolute accuracy requested.

## `EPSREL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Relative accuracy requested If EPSABS. LE. 0 And EPSREL. LT. MAX(50*REL. MACH.

## `RESULT`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral I is obtained by applying the 21-POINT GAUSS-KRONROD RULE (RES21) obtained by optimal addition of abscissae to the 10-POINT GAUSS RULE (RES10), or by applying the 43-POINT RULE (RES43) 21-POINT GAUSS-KRONROD RULE, or by applying the 87-POINT RULE (RES87) obtained by optimal addition of abscissae to the 43-POINT RULE.

## `ABSERR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Estimate of the modulus of the absolute error, which should EQUAL or EXCEED ABS(I-RESULT).

## `NEVAL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of integrand evaluations.

## `IER`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IER = 0 normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine. It is not been achieved.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `>0` | Abnormal termination of the routine. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 The maximum number of steps has been executed. The integral is probably too difficult to be calculated by DQNG. |
| `IER` | `6` | 6 The input is invalid, because EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28). RESULT, ABSERR and NEVAL are set to zero. |

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qng`
- Original SLATEC routine: `QNG`
- Native symbol: `qng_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [QNG](https://www.netlib.org/slatec/src/qng.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
