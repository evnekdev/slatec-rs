# Purpose

Computation of Fourier integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAWF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAWF](https://www.netlib.org/slatec/src/dqawf.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Lower limit of integration 1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), FOR K = 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. Double precision Lower limit of integration 1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), FOR K = 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. not applicable or not stated by selected source not a workspace argument

## 3. `OMEGA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Parameter in the integrand WEIGHT function 0 and INTEGR = 1, The integral is calculated by means of DQAGIE, 0, then LST is set to 1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INTEGR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Indicates which of the WEIGHT functions is used 1      W(X) = COS(OMEGA*X) 2      W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine AND INTEGR.NE.2) or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `EPSABS`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Absolute accuracy requested, EPSABS.GT.0. 6. ON RETURN or LIMLST.LT.1 or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `RESULT`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Approximation to the integral are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and on this interval is the best which can be obtained. = 5 The integral over the K th cycle is probably divergent or slowly convergent. It must be noted that divergence can occur with any other value of Double precision Approximation to the integral are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and on this interval is the best which can be obtained. = 5 The integral over the K th cycle is probably divergent or slowly convergent. It must be noted that divergence can occur with any other value of not applicable or not stated by selected source not a workspace argument

## 7. `ABSERR`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of cycles allowed has been achieved, i.e. of subintervals 1, it is advised to examine the array IWORK which contains the error flags on the cycles. = 6 The input is invalid because IWORK(1) (with meaning as described 6. 6. 6. 6. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `LIMLST`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. or MAXP1.LT.1 or Integer gives an upper bound on the number of cycles, LIMLST.GE.3. 6. ..., WORK(LIMLST+LST) contain the error estimates over the cycles. further elements of WORK have no specific meaning for the user. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `LST`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and is the number of cycles actually needed (see below). Integer On return, LST indicates the number of cycles actually needed for the integration. contain the integral approximations over the cycles, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `LENIW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. or MAXP1.LT.1 or LIMLST) /2) has been achieved on the K th cycle. = 2 Occurrence of roundoff error is detected and prevents the tolerance imposed on the K th cycle, from being achieved on this cycle. = 3 Extremely bad integrand behaviour occurs at some points of the K th cycle. = 4 The integration procedure over the K th cycle does not converge (to within the required accuracy) due to roundoff in the extrapolation procedure invoked on this cycle. It is assumed that the Integer Dimensioning parameter for IWORK. On entry, LIMLST)/2 equals the maximum number of subintervals allowed in the partition of each cycle, LENIW.GE.(LIMLST+2). If LENIW.LT.(LIMLST+2), the routine will end with or MAXP1.LT.1 or LIMLST) /2) has been achieved on the K th cycle. = 2 Occurrence of roundoff error is detected and prevents the tolerance imposed on the K th cycle, from being achieved on this cycle. = 3 Extremely bad integrand behaviour occurs at some points of the K th cycle. = 4 The integration procedure over the K th cycle does not converge (to within the required accuracy) due to roundoff in the extrapolation procedure invoked on this cycle. It is assumed that the Integer Dimensioning parameter for IWORK. On entry, LIMLST)/2 equals the maximum number of subintervals allowed in the partition of each cycle, LENIW.GE.(LIMLST+2). If LENIW.LT.(LIMLST+2), the routine will end with not applicable or not stated by selected source not a workspace argument

## 13. `MAXP1`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the intervals of lengths ABS(B-A)*2**(-L), 2, MAXP1.GE.1. 6. Integer gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the intervals of lengths ABS(B-A)*2**(-L), 2, MAXP1.GE.1. 6. not applicable or not stated by selected source not a workspace argument

## 14. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Dimensioning parameter for WORK must be at least LENIW*2+MAXP1*25. If LENW.LT.(LENIW*2+MAXP1*25), the routine will Integer Dimensioning parameter for WORK must be at least LENIW*2+MAXP1*25. If LENW.LT.(LENIW*2+MAXP1*25), the routine will not applicable or not stated by selected source not a workspace argument

## 15. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). is the number of cycles actually needed (see below). 1 The maximum number of 1). DIMENSIONING PARAMETERS Integer Vector of dimension at least LENIW 1, 2, ..., LST contain the error flags on the cycles. is the number of cycles actually needed (see below). 1 The maximum number of 1). DIMENSIONING PARAMETERS Integer Vector of dimension at least LENIW 1, 2, ..., LST contain the error flags on the cycles. not applicable or not stated by selected source

## 16. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). ARRAYS Double precision Vector of dimension at least On return, contain the integral contain the integral approximations over the cycles, approximations over the cycles, ..., WORK(LIMLST+LST) contain the error estimates over the cycles. further elements of WORK have no specific meaning for the user. ARRAYS Double precision Vector of dimension at least On return, contain the integral contain the integral approximations over the cycles, approximations over the cycles, ..., WORK(LIMLST+LST) contain the error estimates over the cycles. further elements of WORK have no specific meaning for the user. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

If OMEGA.NE.0

# Workspace and array requirements

- `F`: not a workspace argument
- `A`: not a workspace argument
- `OMEGA`: not a workspace argument
- `INTEGR`: not a workspace argument
- `EPSABS`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `LIMLST`: not a workspace argument
- `LST`: not a workspace argument
- `LENIW`: not a workspace argument
- `MAXP1`: not a workspace argument
- `LENW`: not a workspace argument
- `IWORK`: is the number of cycles actually needed (see below). 1 The maximum number of 1). DIMENSIONING PARAMETERS Integer Vector of dimension at least LENIW 1, 2, ..., LST contain the error flags on the cycles.
- `WORK`: ARRAYS Double precision Vector of dimension at least On return, contain the integral contain the integral approximations over the cycles, approximations over the cycles, ..., WORK(LIMLST+LST) contain the error estimates over the cycles. further elements of WORK have no specific meaning for the user.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqawf`
- Original SLATEC routine: `DQAWF`
- Native symbol: `dqawf_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQAWF](https://www.netlib.org/slatec/src/dqawf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
