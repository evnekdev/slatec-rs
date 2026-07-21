# Purpose

Computation of oscillatory integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAWO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAWO](https://www.netlib.org/slatec/src/dqawo.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Double precision Function subprogram defining the function The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Lower limit of integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Upper limit of integration DIMENSIONING PARAMETERS A)*2**(-L), A)*2**(1-L). Double precision Upper limit of integration DIMENSIONING PARAMETERS A)*2**(-L), A)*2**(1-L). not applicable or not stated by selected source not a workspace argument

## 4. `OMEGA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Parameter in the integrand weight function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INTEGR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Indicates which of the weight functions is used 1      W(X) = COS(OMEGA*X) 2      W(X) = SIN(OMEGA*X) If INTEGR.NE.1.AND.INTEGR.NE.2, the routine will not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `EPSABS`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `EPSREL`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Relative accuracy requested If EPSABS.LE.0 and 28), 28)) or (INTEGR.NE.1 AND INTEGR.NE.2), or LENIW.LT.2 OR MAXP1.LT.1 or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `RESULT`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Approximation to the integral are set to zero. Except when LENIW, MAXP1 or LENW are invalid, WORK(LIMIT*2+1), WORK(LIMIT*3+1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `ABSERR`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) are set to zero. Except when LENIW, MAXP1 or LENW are invalid, WORK(LIMIT*2+1), WORK(LIMIT*3+1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to zero. Except when LENIW, MAXP1 or LENW are invalid, WORK(LIMIT*2+1), WORK(LIMIT*3+1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved (= LENIW/2). One can allow more subdivisions by increasing the value of LENIW (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some interior points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved due to roundoff in the extrapolation table, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because 6. 6. 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved (= LENIW/2). One can allow more subdivisions by increasing the value of LENIW (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some interior points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved due to roundoff in the extrapolation table, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because 6. 6. 6. not applicable or not stated by selected source not a workspace argument

## 12. `LENIW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Dimensioning parameter for IWORK. equals the maximum number of subintervals allowed in the partition of the given integration interval (A,B), LENIW.GE.2. 6. Integer Dimensioning parameter for IWORK. equals the maximum number of subintervals allowed in the partition of the given integration interval (A,B), LENIW.GE.2. 6. not applicable or not stated by selected source not a workspace argument

## 13. `MAXP1`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the 2, MAXP1.GE.1 6. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Dimensioning parameter for WORK must be at least LENIW*2+MAXP1*25. If LENW.LT.(LENIW*2+MAXP1*25), the routine will Integer Dimensioning parameter for WORK must be at least LENIW*2+MAXP1*25. If LENW.LT.(LENIW*2+MAXP1*25), the routine will not applicable or not stated by selected source not a workspace argument

## 15. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero. Except when LENIW, MAXP1 or LENW are invalid, WORK(LIMIT*2+1), WORK(LIMIT*3+1), Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. LAST otherwise. Furthermore, IWORK(LIMIT+1), ..., IWORK(LIMIT+ indicate the subdivision levels of the contain the left end points of the subintervals in the partition of (A,B), not stated by selected source not applicable or not stated by selected source

## 16. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). are set to zero, are set to zero, Integer Vector of dimension at least LENIW on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), .. form a decreasing sequence, with LIMIT = LENW/2 , and K = LAST L means that the subinterval numbered I is of length are set to zero, are set to zero, Integer Vector of dimension at least LENIW on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), .. form a decreasing sequence, with LIMIT = LENW/2 , and K = LAST L means that the subinterval numbered I is of length not applicable or not stated by selected source

## 17. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). is set to A and WORK(LIMIT+1) to ARRAYS form a decreasing sequence, with LIMIT = LENW/2 , and K = LAST Double precision Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. ..., WORK(LIMIT*4+MAXP1*25) Provide space for storing the Chebyshev moments. Note that LIMIT = LENW/2. is set to A and WORK(LIMIT+1) to ARRAYS form a decreasing sequence, with LIMIT = LENW/2 , and K = LAST Double precision Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. ..., WORK(LIMIT*4+MAXP1*25) Provide space for storing the Chebyshev moments. Note that LIMIT = LENW/2. not applicable or not stated by selected source

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
- `OMEGA`: not a workspace argument
- `INTEGR`: not a workspace argument
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `LENIW`: not a workspace argument
- `MAXP1`: not a workspace argument
- `LENW`: not a workspace argument
- `LAST`: are set to zero. Except when LENIW, MAXP1 or LENW are invalid, WORK(LIMIT*2+1), WORK(LIMIT*3+1), Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. LAST otherwise. Furthermore, IWORK(LIMIT+1), ..., IWORK(LIMIT+ indicate the subdivision levels of the contain the left end points of the subintervals in the partition of (A,B),
- `IWORK`: are set to zero, are set to zero, Integer Vector of dimension at least LENIW on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), .. form a decreasing sequence, with LIMIT = LENW/2 , and K = LAST L means that the subinterval numbered I is of length
- `WORK`: is set to A and WORK(LIMIT+1) to ARRAYS form a decreasing sequence, with LIMIT = LENW/2 , and K = LAST Double precision Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. ..., WORK(LIMIT*4+MAXP1*25) Provide space for storing the Chebyshev moments. Note that LIMIT = LENW/2.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqawo`
- Original SLATEC routine: `DQAWO`
- Native symbol: `dqawo_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQAWO](https://www.netlib.org/slatec/src/dqawo.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
