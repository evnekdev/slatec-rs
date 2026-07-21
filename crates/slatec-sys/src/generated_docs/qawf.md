# Purpose

Computation of Fourier integrals Standard fortran subroutine Real version

# Description

This canonical unsafe binding exposes original SLATEC routine `QAWF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QAWF](https://www.netlib.org/slatec/src/qawf.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Lower limit of integration.

## `OMEGA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Parameter in the integrand WEIGHT function.

## `INTEGR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates which of the WEIGHT functions is used 1 W(X) = COS(OMEGA*X) 2 W(X) = SIN(OMEGA*X) IF INTEGR. NE. 1. AND. INTEGR. 2, the routine will end with IER = 6.

## `EPSABS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Absolute accuracy requested, EPSABS. GT. 0. If EPSABS. LE. 0, the routine will end with IER = 6.

## `RESULT`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral.

## `ABSERR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT).

## `NEVAL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of integrand evaluations.

## `IER`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine. The estimates for integral and error are less reliable.

## `LIMLST`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

gives an upper bound on the number of cycles, LIMLST. GE. 3. If LIMLST. LT. 3, the routine will end with IER = 6.

## `LST`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

On return, LST indicates the number of cycles actually needed for the integration. If OMEGA = 0, then LST is set to 1.

## `LENIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer.

## `MAXP1`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

gives an upper bound on the number of Chebyshev moments which can be stored, i. e. for the intervals of lengths ABS(B-A)*2**(-L), L = 0,1,. , MAXP1-2, MAXP1. GE. 1.

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be at least LENIW*2+MAXP1*25. If LENW. LT. (LENIW*2+MAXP1*25), the routine will end with IER = 6.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LENIW On return, IWORK(K) FOR K = 1, 2,. , LST contain the error flags on the cycles. Integer Vector of dimension at least LENIW On return, IWORK(K) FOR K = 1, 2, ..., LST contain the error flags on the cycles.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least. Real Vector of dimension at least

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. If OMEGA.NE.0 |
| `IER` | `1` | 1 Maximum number of cycles allowed has been achieved, i.e. of subintervals (A+(K-1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. |
| `IER` | `4` | 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. |
| `IER` | `1` | 1, it is advised to examine the array IWORK which contains the error flags on the cycles. |
| `IER` | `6` | 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). RESULT, ABSERR, NEVAL, LST are set to zero. |
| `IER` | `7` | 7 Bad integrand behaviour occurs within one or more of the cycles. Location and type of the difficulty involved can be determined from the first LST elements of vector IWORK. Here LST is the number of cycles actually needed (see below). |
| `IER` | `1` | 1 The maximum number of subdivisions (=(LENIW-LIMLST) /2) has been achieved on the K th cycle. |
| `IER` | `2` | 2 Occurrence of roundoff error is detected and prevents the tolerance imposed on the K th cycle, from being achieved on this cycle. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the K th cycle. |
| `IER` | `4` | 4 The integration procedure over the K th cycle does not converge (to within the required accuracy) due to roundoff in the extrapolation procedure invoked on this cycle. It is assumed that the result on this interval is the best which can be obtained. |
| `IER` | `5` | 5 The integral over the K th cycle is probably divergent or slowly convergent. It must be noted that divergence can occur with any other value of IWORK(K). |
| `IER` | `0` | 0 and INTEGR = 1, The integral is calculated by means of DQAGIE, and IER = IWORK(1) (with meaning as described |
| `IER` | `1` | 1). |

# Workspace and array requirements

- `IWORK`: Integer Vector of dimension at least LENIW On return, IWORK(K) FOR K = 1, 2, ..., LST contain the error flags on the cycles.
- `WORK`: Real Vector of dimension at least

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qawf`
- Original SLATEC routine: `QAWF`
- Native symbol: `qawf_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [QAWF](https://www.netlib.org/slatec/src/qawf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
