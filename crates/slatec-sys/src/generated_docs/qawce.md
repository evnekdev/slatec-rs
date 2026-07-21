# Purpose

Computation of a CAUCHY PRINCIPAL VALUE Standard fortran subroutine Real version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `QAWCE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QAWCE](https://www.netlib.org/slatec/src/qawce.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Lower limit of integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Upper limit of integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Parameter in the WEIGHT function, C.NE.A, C.NE.B A OR C = B, the routine will end with A or C = B or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `EPSABS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `EPSREL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `LIMIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN However, if this yields no improvement it is advised to analyze the the integrand, in order to determine the the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 2 The occurrence of roundoff error is detec- ted, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some interior points of the integration interval. = 6 The input is invalid, because LAST LAST otherwise, form a decreasing sequence otherwise, form a decreasing sequence not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `RESULT`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral ABSERR, NEVAL, RLIST(1), ELIST(1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. 6. Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine the estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `ALIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension at least LIMIT, the first Real Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 13. `BLIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension at least LIMIT, the first Real Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 14. `RLIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension at least LIMIT, the first Real Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 15. `ELIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension LIMIT, the first  LAST elements of which are the moduli of the absolute LAST LAST Real Vector of dimension LIMIT, the first  LAST elements of which are the moduli of the absolute LAST LAST not applicable or not stated by selected source not a workspace argument

## 16. `IORD`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, so that LAST LAST are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, so that LAST LAST not applicable or not stated by selected source not a workspace argument

## 17. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) elements of which are the integral approximations on the subintervals LAST otherwise, form a decreasing sequence Integer Number of subintervals actually produced in the subdivision process not stated by selected source not applicable or not stated by selected source not a workspace argument

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

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qawce`
- Original SLATEC routine: `QAWCE`
- Native symbol: `qawce_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [QAWCE](https://www.netlib.org/slatec/src/qawce.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
