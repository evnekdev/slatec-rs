# Purpose

Integration of functions having algebraico-logarithmic end point singularities Standard fortran subroutine Real version

# Description

This canonical unsafe binding exposes original SLATEC routine `QAWSE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QAWSE](https://www.netlib.org/slatec/src/qawse.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Lower limit of integration.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Upper limit of integration, B. GT. A If B. LE. A, the routine will end with IER = 6.

## `ALFA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Parameter in the WEIGHT function, ALFA. GT. (-1) If ALFA. LE. (-1), the routine will end with IER = 6.

## `BETA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Parameter in the WEIGHT function, BETA. GT. (-1) If BETA. LE. (-1), the routine will end with IER = 6.

## `INTEGR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates which WEIGHT function is to be used = 1 (X-A)**ALFA*(B-X)**BETA = 2 (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3 (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4 (X-A)**ALFA*(B-X)**BETA*LOG(X-A)*LOG(B-X) If INTEGR. LT. 1 or INTEGR. GT. 4, the routine will end with IER = 6.

## `EPSABS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Absolute accuracy requested.

## `EPSREL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Relative accuracy requested If EPSABS. LE. 0 and EPSREL. LT. MAX(50*REL. MACH.

## `LIMIT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT. GE. 2 If LIMIT. LT. 2, the routine will end with IER = 6.

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

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine the estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved.

## `ALIST`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `BLIST`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `RLIST`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `ELIST`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first.

## `IORD`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Vector of dimension at least LIMIT, the first K of which are pointers to the error estimates over the subintervals, so that ELIST(IORD(1)),. , ELIST(IORD(K)) with K = LAST If LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise form a decreasing sequence.

## `LAST`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals Integer Number of subintervals actually produced in the subdivision process.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `6` | 6. |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine the estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT. However, if this yields no improvement, it is advised to analyze the integrand in order to determine the integration difficulties which prevent the requested tolerance from being achieved. In case of a jump DISCONTINUITY or a local SINGULARITY of algebraico-logarithmic type at one or more interior points of the integration range, one should proceed by splitting up the interval at these points and calling the integrator on the subranges. |
| `IER` | `2` | 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the integration interval. |
| `IER` | `6` | 6 The input is invalid, because B.LE.A or ALFA.LE.(-1) or BETA.LE.(-1), or |
| `IER` | `>0` | , or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), or LIMIT.LT.2. RESULT, ABSERR, NEVAL, RLIST(1), ELIST(1), IORD(1) and LAST are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. |

# Workspace and array requirements

- `ALIST`: not a workspace argument
- `BLIST`: not a workspace argument
- `RLIST`: not a workspace argument
- `ELIST`: not a workspace argument
- `IORD`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qawse`
- Original SLATEC routine: `QAWSE`
- Native symbol: `qawse_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [QAWSE](https://www.netlib.org/slatec/src/qawse.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
