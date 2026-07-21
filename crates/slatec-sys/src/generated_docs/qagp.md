# Purpose

Computation of a definite integral Standard fortran subroutine Real version

# Description

This canonical unsafe binding exposes original SLATEC routine `QAGP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QAGP](https://www.netlib.org/slatec/src/qagp.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand Function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Upper limit of integration.

## `NPTS2`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number equal to two more than the number of user-supplied break points within the integration range, NPTS. GE. 2. If NPTS2. LT. 2, The routine will end with IER = 6.

## `POINTS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of dimension NPTS2, the first (NPTS2-2) elements of which are the user provided break points. If these points do not constitute an ascending sequence there will be an automatic sorting.

## `EPSABS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Absolute accuracy requested.

## `EPSREL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Relative accuracy requested If EPSABS. LE. 0 And EPSREL. LT. MAX(50*REL. MACH.

## `RESULT`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral.

## `ABSERR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT).

## `NEVAL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of integrand evaluations.

## `IER`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine. The estimates for integral and error are less reliable.

## `LENIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer.

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be at least LENIW*2-NPTS2. If LENW. LT. LENIW*2-NPTS2, the routine will end with IER = 6.

## `LAST`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),. , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST if LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise IWORK(LIMIT+1),. ,IWORK(LIMIT+LAST) Contain the subdivision levels of the subintervals, i. Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise IWORK(LIMIT+1), ...,IWORK(LIMIT+LAST) Contain the subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration LIMIT, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L), IWORK(LIMIT*2+1), ..., IWORK(LIMIT*2+NPTS2) have no significance for the user, note that LIMIT = (LENIW-NPTS2)/2.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LENW. Real Vector of dimension at least LENW

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine. The estimates for integral and error are less reliable. it is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. one can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. |
| `IER` | `2` | 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the integration interval. |
| `IER` | `4` | 4 The algorithm does not converge. roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned RESULT is the best which can be obtained. |
| `IER` | `5` | 5 The integral is probably divergent, or slowly convergent. it must be noted that divergence can occur with any other value |
| `IER` | `>0` | . |
| `IER` | `6` | 6 The input is invalid because NPTS2.LT.2 or break points are specified outside the integration range or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) WORK(1) is set to A and WORK(LIMIT+1) to B (where LIMIT = (LENIW-NPTS2)/2). |

# Workspace and array requirements

- `POINTS`: not a workspace argument
- `IWORK`: Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise IWORK(LIMIT+1), ...,IWORK(LIMIT+LAST) Contain the subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration LIMIT, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L), IWORK(LIMIT*2+1), ..., IWORK(LIMIT*2+NPTS2) have no significance for the user, note that LIMIT = (LENIW-NPTS2)/2.
- `WORK`: Real Vector of dimension at least LENW

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qagp`
- Original SLATEC routine: `QAGP`
- Native symbol: `qagp_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [QAGP](https://www.netlib.org/slatec/src/qagp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
