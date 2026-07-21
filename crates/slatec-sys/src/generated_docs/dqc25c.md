# Purpose

Integration rules for the computation of CAUCHY PRINCIPAL VALUE integrals Standard fortran subroutine Double precision version

# Description

This canonical unsafe binding exposes original SLATEC routine `DQC25C`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQC25C](https://www.netlib.org/slatec/src/dqc25c.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Left end point of the integration interval.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Right end point of the integration interval, B. GT. A.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Parameter in the WEIGHT function.

## `RESULT`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. In the other case the 15-point Kronrod rule obtained by optimal addition of abscissae to the 7-point Gauss rule, is applied.

## `ABSERR`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT).

## `KRUL`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used.

## `NEVAL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of integrand evaluations.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqc25c`
- Original SLATEC routine: `DQC25C`
- Native symbol: `dqc25c_`
- ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_i32,mut_i32)`
- Exact Netlib source file: [DQC25C](https://www.netlib.org/slatec/src/dqc25c.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
