# Purpose

Integration of functions having algebraico-logarithmic end point singularities Standard fortran subroutine Double precision version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAWSE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAWSE](https://www.netlib.org/slatec/src/dqawse.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Lower limit of integration 6. 1) or BETA.LE.(-1), or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Upper limit of integration, B.GT.A 6. 1) or BETA.LE.(-1), or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ALFA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision 1) 1), the routine will end with 1) or BETA.LE.(-1), or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BETA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision 1) 1), the routine will end with not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INTEGR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Indicates which WEIGHT function is to be used = 1  (X-A)**ALFA*(B-X)**BETA = 2  (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3  (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4  (X-A)**ALFA*(B-X)**BETA*LOG(X-A)*LOG(B-X) If INTEGR.LT.1 or INTEGR.GT.4, the routine or INTEGR.GT.4, or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `EPSABS`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `EPSREL`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Relative accuracy requested If  EPSABS.LE.0 28), 28), or LIMIT.LT.2. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `LIMIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.2 6. ON RETURN However, if this yields no improvement, it is advised to analyze the integrand in order to determine the integration difficulties which prevent the requested tolerance from being achieved. In case of a jump DISCONTINUITY or a local SINGULARITY of algebraico-logarithmic type at one or more interior points of the integration range, one should proceed by splitting up the interval at these points and calling the integrator on the subranges. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because LAST LAST otherwise form a decreasing sequence otherwise form a decreasing sequence not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `RESULT`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Approximation to the integral ABSERR, NEVAL, RLIST(1), ELIST(1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `ABSERR`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. 6. 6. 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine the estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `ALIST`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Double precision Vector of dimension at least LIMIT, the first Double precision Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 15. `BLIST`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Double precision Vector of dimension at least LIMIT, the first Double precision Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 16. `RLIST`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Double precision Vector of dimension at least LIMIT, the first Double precision Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 17. `ELIST`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Double precision Vector of dimension at least LIMIT, the first LAST LAST Double precision Vector of dimension at least LIMIT, the first LAST LAST not applicable or not stated by selected source not a workspace argument

## 18. `IORD`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. Integer Vector of dimension at least LIMIT, the first K of which are pointers to the error estimates over the subintervals, so that LAST LAST are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. Integer Vector of dimension at least LIMIT, the first K of which are pointers to the error estimates over the subintervals, so that LAST LAST not applicable or not stated by selected source not a workspace argument

## 19. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals LAST otherwise form a decreasing sequence Integer Number of subintervals actually produced in the subdivision process not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

= 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of

# Workspace and array requirements

- `F`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `ALFA`: not a workspace argument
- `BETA`: not a workspace argument
- `INTEGR`: not a workspace argument
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `LIMIT`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `ALIST`: not a workspace argument
- `BLIST`: not a workspace argument
- `RLIST`: not a workspace argument
- `ELIST`: not a workspace argument
- `IORD`: not a workspace argument
- `LAST`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqawse`
- Original SLATEC routine: `DQAWSE`
- Native symbol: `dqawse_`
- ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_f64,mut_f64,mut_i32,mut_f64,mut_f64,mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DQAWSE](https://www.netlib.org/slatec/src/dqawse.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
