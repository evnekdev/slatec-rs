# Purpose

Abstract *** a DOUBLE PRECISION routine *** DQNC79 is a general purpose program for evaluation of one dimensional integrals of user defined functions. DQNC79 will pick its own points for evaluation of the integrand and these will vary from problem to problem. Thus, DQNC79 is not designed to integrate over data sets. Moderately smooth integrands will be integrated efficiently and reliably. For problems with strong singularities, oscillations etc., the user may wish to use more sophis- ticated routines such as those in QUADPACK. One measure of the reliability of DQNC79 is the output parameter K, giving the number of integrand evaluations that were needed.

# Description

This canonical unsafe binding exposes original SLATEC routine `DQNC79`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQNC79](https://www.netlib.org/slatec/src/dqnc79.f).

# Arguments

## `FUN`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

name of external function to be integrated. This name must be in an EXTERNAL statement in your calling program. You must write a Fortran function to evaluate This should be of the form DOUBLE PRECISION FUNCTION FUN (X) C X can vary from A to B C FUN(X) should be finite for all X on interval. FUN =. RETURN END. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

upper limit of integration (may be less than A).

## `ERR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

is a requested error tolerance. Normally, pick a value 0. LT. ERR. 1. 0D-8.

## `ANS`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

computed value of the integral. Hopefully, ANS is accurate to within ERR * integral of ABS(FUN(X)).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

a status code - Normal codes 1 ANS most likely meets requested error tolerance. -1 A equals B, or A and B are too nearly equal to allow normal integration. ANS is set to zero. - Abnormal code 2 ANS probably does not meet requested error tolerance.

## `K`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the number of function evaluations actually used to do the integration. A value of K. GT. 1000 indicates a difficult problem; other programs may be more efficient. DQNC79 will gracefully give up if K exceeds 2000.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqnc79`
- Original SLATEC routine: `DQNC79`
- Native symbol: `dqnc79_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQNC79](https://www.netlib.org/slatec/src/dqnc79.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
