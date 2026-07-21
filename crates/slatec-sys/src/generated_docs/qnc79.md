# Purpose

QNC79 is a general purpose program for evaluation of one dimensional integrals of user defined functions. QNC79 will pick its own points for evaluation of the integrand and these will vary from problem to problem. Thus, QNC79 is not designed to integrate over data sets. Moderately smooth integrands will be integrated efficiently and reliably. For problems with strong singularities, oscillations etc., the user may wish to use more sophis- ticated routines such as those in QUADPACK. One measure of the reliability of QNC79 is the output parameter K, giving the number of integrand evaluations that were needed.

# Description

This canonical unsafe binding exposes original SLATEC routine `QNC79`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QNC79](https://www.netlib.org/slatec/src/qnc79.f).

# Arguments

## 1. `FUN`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. name of external function to be integrated.  This name must be in an EXTERNAL statement in your calling program.  You must write a Fortran function to evaluate This should be of the form REAL FUNCTION FUN (X) C C     X can vary from A to B C     FUN(X) should be finite for all X on interval. C ... RETURN END The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. name of external function to be integrated.  This name must be in an EXTERNAL statement in your calling program.  You must write a Fortran function to evaluate This should be of the form REAL FUNCTION FUN (X) C C     X can vary from A to B C     FUN(X) should be finite for all X on interval. C ... RETURN END not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. point adaptive Newton-Cotes point adaptive Newton-Cotes quadrature rule. quadrature rule. lower limit of integration are too nearly equal to are too nearly equal to allow normal integration.  ANS is set to zero. allow normal integration.  ANS is set to zero. - Abnormal code - Abnormal code 2  ANS probably does not meet requested error tolerance. 2  ANS probably does not meet requested error tolerance. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. upper limit of integration (may be less than A) are too nearly equal to are too nearly equal to allow normal integration.  ANS is set to zero. allow normal integration.  ANS is set to zero. - Abnormal code - Abnormal code 2  ANS probably does not meet requested error tolerance. 2  ANS probably does not meet requested error tolerance. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ERR`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a requested error tolerance.  Normally, pick a value 3. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `ANS`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computed value of the integral.  Hopefully, ANS is accurate to within ERR * integral of ABS(FUN(X)). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. a status code - Normal codes 1  ANS most likely meets requested error tolerance. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `K`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the number of function evaluations actually used to do the integration.  A value of K .GT. 1000 indicates a difficult problem; other programs may be more efficient. QNC79 will gracefully give up if K exceeds 2000. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `FUN`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `ERR`: not a workspace argument
- `ANS`: not a workspace argument
- `IERR`: not a workspace argument
- `K`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qnc79`
- Original SLATEC routine: `QNC79`
- Native symbol: `qnc79_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [QNC79](https://www.netlib.org/slatec/src/qnc79.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
