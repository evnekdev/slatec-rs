# Purpose

Computation of a definite integral Standard fortran subroutine Double precision version

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAGS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAGS](https://www.netlib.org/slatec/src/dqags.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand Function F(X). The actual name for F needs to be Declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Upper limit of integration.

## `EPSABS`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Absolute accuracy requested.

## `EPSREL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Relative accuracy requested If EPSABS. LE. 0 And EPSREL. LT. MAX(50*REL. MACH.

## `RESULT`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Approximation to the integral.

## `ABSERR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT).

## `NEVAL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of integrand evaluations.

## `IER`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved.

## `LIMIT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer.

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be at least LIMIT*4. If LENW. LT. LIMIT*4, the routine will end with IER = 6.

## `LAST`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

On return, LAST equals the number of subintervals produced in the subdivision process, determines the number of significant elements actually in the WORK Arrays.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals such that WORK(LIMIT*3+IWORK(1)),. , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST IF LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise. Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals such that WORK(LIMIT*3+IWORK(1)),... , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST IF LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LENW. Double precision Vector of dimension at least LENW

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of LIMIT (and taking the according dimension adjustments into account. However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (E.G. SINGULARITY, DISCONTINUITY WITHIN THE INTERVAL) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. |
| `IER` | `2` | 2 The occurrence of roundoff error is detec- ted, which prevents the requested tolerance from being achieved. The error may be under-estimated. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the integration interval. |
| `IER` | `4` | 4 The algorithm does not converge. Roundoff error is detected in the Extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. |
| `IER` | `5` | 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. |
| `IER` | `6` | 6 The input is invalid, because (EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LIMIT or LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) TO B. |

# Workspace and array requirements

- `IWORK`: Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals such that WORK(LIMIT*3+IWORK(1)),... , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST IF LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise
- `WORK`: Double precision Vector of dimension at least LENW

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqags`
- Original SLATEC routine: `DQAGS`
- Native symbol: `dqags_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQAGS](https://www.netlib.org/slatec/src/dqags.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
