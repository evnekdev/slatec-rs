# Purpose

1. DRJ Standard FORTRAN function routine Double precision version The routine calculates an approximation result to DRJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt, where X, Y, and Z are nonnegative, at most one of them is zero, and P is positive. If X or Y or Z is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence DRJ( X, Y, Z, P, IER ) Parameters on Entry Values assigned by the calling routine

# Description

This canonical unsafe binding exposes original SLATEC routine `DRJ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DRJ](https://www.netlib.org/slatec/src/drj.f).

# Arguments

## `X`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Double precision, nonnegative variable unaltered. 3. Error Messages Value of IER assigned by the DRJ routine Value assigned Error Message printed IER = 1 MIN(X,Y,Z). LT. 0. 0D0 = 2 MIN(X+Y,X+Z,Y+Z,P).

## `Y`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Double precision, nonnegative variable unaltered. 3. Error Messages Value of IER assigned by the DRJ routine Value assigned Error Message printed IER = 1 MIN(X,Y,Z). LT. 0. 0D0 = 2 MIN(X+Y,X+Z,Y+Z,P).

## `Z`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Double precision, nonnegative variable unaltered. 3. Error Messages Value of IER assigned by the DRJ routine Value assigned Error Message printed IER = 1 MIN(X,Y,Z). LT. 0. 0D0 = 2 MIN(X+Y,X+Z,Y+Z,P).

## `P`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Double precision, positive variable On Return (values assigned by the DRJ routine) DRJ - Double precision approximation to the integral unaltered. 3. Error Messages Value of IER assigned by the DRJ routine Value assigned Error Message printed IER = 1 MIN(X,Y,Z). LT. 0. 0D0 = 2 MIN(X+Y,X+Z,Y+Z,P).

## `IER`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`. It has no separate Rust `Result` status channel.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine |
| `IER` | `1` | 1 MIN(X,Y,Z) .LT. 0.0D0 |
| `IER` | `2` | 2 MIN(X+Y,X+Z,Y+Z,P) .LT. LOLIM |
| `IER` | `3` | 3 MAX(X,Y,Z,P) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y, Z, and P LOLIM is not less than the cube root of the value of LOLIM used in the routine for DRC. UPLIM is not greater than 0.3 times the cube root of the value of UPLIM used in the routine for DRC. Acceptable values for: LOLIM UPLIM IBM 360/370 SERIES : 2.0D-26 3.0D+24 CDC 6000/7000 SERIES : 5.0D-98 3.0D+106 UNIVAC 1100 SERIES : 5.0D-103 6.0D+101 CRAY : 1.32D-822 1.4D+821 VAX 11 SERIES : 2.5D-13 9.0D+11 ERRTOL determines the accuracy of the answer the value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". Relative error due to truncation of the series for DRJ is less than 3 * ERRTOL ** 6 / (1 - ERRTOL) ** 3/2. The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the truncation error there will be round-off error, but in practice the total error from both sources is usually less than the amount given in the table. Sample choices: ERRTOL Relative truncation 1.0D-3 4.0D-18 3.0D-3 3.0D-15 1.0D-2 4.0D-12 3.0D-2 3.0D-9 1.0D-1 4.0D-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. |

# ABI notes

- Canonical Rust path: `slatec_sys::special::drj`
- Original SLATEC routine: `DRJ`
- Native symbol: `drj_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [DRJ](https://www.netlib.org/slatec/src/drj.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
