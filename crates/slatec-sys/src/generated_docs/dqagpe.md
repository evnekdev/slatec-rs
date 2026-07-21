# Purpose

Computation of a definite integral Standard fortran subroutine Double precision version

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAGPE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAGPE](https://www.netlib.org/slatec/src/dqagpe.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Upper limit of integration.

## `NPTS2`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number equal to two more than the number of user-supplied break points within the integration range, NPTS2. GE. 2. If NPTS2. LT. 2, the routine will end with IER = 6.

## `POINTS`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector of dimension NPTS2, the first (NPTS2-2) elements of which are the user provided break POINTS. If these POINTS do not constitute an ascending sequence there will be an automatic sorting.

## `EPSABS`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Absolute accuracy requested.

## `EPSREL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Relative accuracy requested If EPSABS. LE. 0 and EPSREL. LT. MAX(50*REL. MACH.

## `LIMIT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT. GE. NPTS2 If LIMIT. LT. NPTS2, the routine will end with IER = 6.

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

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine. The estimates for integral and error are less reliable.

## `ALIST`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `BLIST`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `RLIST`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `ELIST`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `PTS`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence.

## `IORD`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, such that ELIST(IORD(1)),. , ELIST(IORD(K)) form a decreasing sequence, with K = LAST If LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise.

## `LEVEL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i. e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration limit, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L).

## `NDIN`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), I = 0,1,. , NPTS2-2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. If this happens for the subinterval numbered K, NDIN(K) is put to 1, otherwise NDIN(K) = 0.

## `LAST`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals Integer Number of subintervals actually produced in the subdivisions process.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `6` | 6. |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. |
| `IER` | `2` | 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs At some points of the integration interval. |
| `IER` | `4` | 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. |
| `IER` | `5` | 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value |
| `IER` | `>0` | . |
| `IER` | `6` | 6 The input is invalid because NPTS2.LT.2 or Break points are specified outside the integration range or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or LIMIT.LT.NPTS2. RESULT, ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. |

# Workspace and array requirements

- `POINTS`: not a workspace argument
- `ALIST`: not a workspace argument
- `BLIST`: not a workspace argument
- `RLIST`: not a workspace argument
- `ELIST`: not a workspace argument
- `PTS`: not a workspace argument
- `IORD`: not a workspace argument
- `LEVEL`: not a workspace argument
- `NDIN`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqagpe`
- Original SLATEC routine: `DQAGPE`
- Native symbol: `dqagpe_`
- ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64,mut_f64,mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DQAGPE](https://www.netlib.org/slatec/src/dqagpe.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
