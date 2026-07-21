# Purpose

The methods in subroutine DSTEPS approximate the solution near X by a polynomial. Subroutine DINTP approximates the solution at

# Description

This canonical unsafe binding exposes original SLATEC routine `DINTP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DINTP](https://www.netlib.org/slatec/src/dintp.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Input current integration abscissa from `DSTEPS`. It and the history arguments must be from the same unmodified `DSTEPS` state. Input current integration abscissa from `DSTEPS`. It and the history arguments must be from the same unmodified `DSTEPS` state. not applicable or not stated by selected source not a workspace argument

## 2. `Y`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Readable current solution vector from `DSTEPS`, with at least `NEQN` elements. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `XOUT`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. by evaluating the polynomial there.  Information defining this polynomial is passed from  DSTEPS  so  DINTP  cannot be used alone. Subroutine DSTEPS is completely explained and documented in the text "Computer Solution of Ordinary Differential Equations, the Initial Value Problem"  by L. F. Shampine and M. K. Gordon. Input to DINTP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines point at which solution is desired. The remaining parameters are defined in  DSTEPS  and passed to DINTP  from that subroutine Output from  DINTP -- by evaluating the polynomial there.  Information defining this polynomial is passed from  DSTEPS  so  DINTP  cannot be used alone. Subroutine DSTEPS is completely explained and documented in the text "Computer Solution of Ordinary Differential Equations, the Initial Value Problem"  by L. F. Shampine and M. K. Gordon. Input to DINTP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines point at which solution is desired. The remaining parameters are defined in  DSTEPS  and passed to DINTP  from that subroutine Output from  DINTP -- not applicable or not stated by selected source not a workspace argument

## 4. `YOUT`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). solution at  XOUT not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `YPOUT`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). derivative of solution at  XOUT The remaining parameters are returned unaltered from their input values.  Integration with  DSTEPS  may be continued. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `NEQN`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Input number of differential equations. It is the required length of `Y`, `YOUT`, and `YPOUT` and the first dimension of `PHI`. Input number of differential equations. It is the required length of `Y`, `YOUT`, and `YPOUT` and the first dimension of `PHI`. not applicable or not stated by selected source not a workspace argument

## 7. `KOLD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Input interpolation order saved by `DSTEPS`. It controls how many columns of the `PHI` history are used and must be passed unchanged from that integrator state. Input interpolation order saved by `DSTEPS`. It controls how many columns of the `PHI` history are used and must be passed unchanged from that integrator state. not applicable or not stated by selected source not a workspace argument

## 8. `PHI`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (NEQN, 16). Readable `DSTEPS` history matrix with Fortran shape `(NEQN, 16)`. It defines the local interpolation polynomial and must not be synthesized independently. Readable `DSTEPS` history matrix with Fortran shape `(NEQN, 16)`. It defines the local interpolation polynomial and must not be synthesized independently. not applicable or not stated by selected source not a workspace argument

## 9. `IVC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Readable interpolation-cache control index supplied by `DSTEPS`; it selects cached data in `IV` and `OW` when the order changes. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `IV`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (10). Readable integer interpolation cache of length 10 supplied by `DSTEPS`. It is part of the persistent integrator state and must be passed unchanged. Readable integer interpolation cache of length 10 supplied by `DSTEPS`. It is part of the persistent integrator state and must be passed unchanged. not applicable or not stated by selected source not a workspace argument

## 11. `KGI`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Readable `DSTEPS` interpolation-history order marker used to decide whether cached `GI` values apply. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `GI`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (11). Readable interpolation cache of length 11 supplied by `DSTEPS`. It stores precomputed integral factors and must remain consistent with `KGI` and `KOLD`. Readable interpolation cache of length 11 supplied by `DSTEPS`. It stores precomputed integral factors and must remain consistent with `KGI` and `KOLD`. not applicable or not stated by selected source not a workspace argument

## 13. `ALPHA`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (12). Readable `DSTEPS` coefficient array of length 12 used to reconstruct the interpolation factors. Readable `DSTEPS` coefficient array of length 12 used to reconstruct the interpolation factors. not applicable or not stated by selected source not a workspace argument

## 14. `OG`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (13). Readable `DSTEPS` interpolation-history array of length 13 used when evaluating the local polynomial. Readable `DSTEPS` interpolation-history array of length 13 used when evaluating the local polynomial. not applicable or not stated by selected source not a workspace argument

## 15. `OW`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (12). Readable `DSTEPS` interpolation-cache array of length 12. It is indexed through `IVC` and `IV` and is not independent workspace. Readable `DSTEPS` interpolation-cache array of length 12. It is indexed through `IVC` and `IV` and is not independent workspace. not applicable or not stated by selected source

## 16. `OX`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Readable previous integration abscissa from `DSTEPS`; together with `X` it defines the interpolation interval. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `OY`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Readable previous solution vector from `DSTEPS`, with at least `NEQN` elements. It supplies the endpoint data for the smooth interpolant. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `XOUT`: not a workspace argument
- `YOUT`: not a workspace argument
- `YPOUT`: not a workspace argument
- `NEQN`: not a workspace argument
- `KOLD`: not a workspace argument
- `PHI`: not a workspace argument
- `IVC`: not a workspace argument
- `IV`: not a workspace argument
- `KGI`: not a workspace argument
- `GI`: not a workspace argument
- `ALPHA`: not a workspace argument
- `OG`: not a workspace argument
- `OW`: Readable `DSTEPS` interpolation-cache array of length 12. It is indexed through `IVC` and `IV` and is not independent workspace.
- `OX`: not a workspace argument
- `OY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::ode::dintp`
- Original SLATEC routine: `DINTP`
- Native symbol: `dintp_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DINTP](https://www.netlib.org/slatec/src/dintp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
