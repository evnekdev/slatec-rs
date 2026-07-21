# Purpose

Integration rules Standard fortran subroutine Double precision version

# Description

This canonical unsafe binding exposes original SLATEC routine `DQK51`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQK51](https://www.netlib.org/slatec/src/dqk51.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subroutine defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the calling program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Upper limit of integration.

## `RESULT`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral I is computed by applying the 51-point Kronrod rule (RESK) obtained by optimal addition of abscissae to the 25-point Gauss rule (RESG).

## `ABSERR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Estimate of the modulus of the absolute error, which should not exceed ABS(I-RESULT).

## `RESABS`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral J.

## `RESASC`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral of ABS(F-I/(B-A)) over (A,B).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqk51`
- Original SLATEC routine: `DQK51`
- Native symbol: `dqk51_`
- ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64)`
- Exact Netlib source file: [DQK51](https://www.netlib.org/slatec/src/dqk51.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
