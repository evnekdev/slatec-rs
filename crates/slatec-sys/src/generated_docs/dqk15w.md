# Purpose

Integration rules Standard fortran subroutine Double precision version

# Description

This canonical unsafe binding exposes original SLATEC routine `DQK15W`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQK15W](https://www.netlib.org/slatec/src/dqk15w.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `W`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand WEIGHT function W(X). The actual name for W needs to be declared E X T E R N A L in the calling program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `P1`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Parameters in the WEIGHT function.

## `P2`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Parameters in the WEIGHT function.

## `P3`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Parameters in the WEIGHT function.

## `P4`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Parameters in the WEIGHT function.

## `KP`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Key for indicating the type of WEIGHT function.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Upper limit of integration.

## `RESULT`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral I is computed by applying the 15-point Kronrod rule (RESK) obtained by optimal addition of abscissae to the 7-point Gauss rule (RESG).

## `ABSERR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT).

## `RESABS`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral of ABS(F).

## `RESASC`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral of ABS(F-I/(B-A)).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqk15w`
- Original SLATEC routine: `DQK15W`
- Native symbol: `dqk15w_`
- ABI fingerprint: `subroutine:void(fn:f64(ref_f64),fn:f64(ref_f64,ref_f64,ref_f64,ref_f64,ref_f64,ref_i32),mut_f64,mut_f64,mut_f64,mut_f64,mut_i32,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64)`
- Exact Netlib source file: [DQK15W](https://www.netlib.org/slatec/src/dqk15w.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
