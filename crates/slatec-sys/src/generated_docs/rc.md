# Purpose

1. RC Standard FORTRAN function routine Single precision version The routine calculates an approximation to RC(X,Y) = Integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. Logarithmic, inverse circular, and inverse hyper- bolic functions can be expressed in terms of RC. 2. Calling Sequence RC( X, Y, IER ) Parameters on Entry Values assigned by the calling routine

# Description

This canonical unsafe binding exposes original SLATEC routine `RC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RC](https://www.netlib.org/slatec/src/rc.f).

# Arguments

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Single precision, nonnegative variable unaltered. 3. Error Messages Value of IER assigned by the RC routine Value Assigned Error Message Printed IER = 1 X. LT. 0. 0E0.

## `Y`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Single precision, positive variable On Return (values assigned by the RC routine) RC - Single precision approximation to the integral unaltered. 3. Error Messages Value of IER assigned by the RC routine Value Assigned Error Message Printed IER = 1 X. LT. 0. 0E0.

## `IER`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

to indicate normal or abnormal termination. IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_i32)`. It has no separate Rust `Result` status channel.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine |
| `IER` | `1` | 1 X.LT.0.0E0.OR.Y.LE.0.0E0 |
| `IER` | `2` | 2 X+Y.LT.LOLIM |
| `IER` | `3` | 3 MAX(X,Y) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X and Y LOLIM - Lower limit of valid arguments Not less than 5 * (machine minimum) . UPLIM - Upper limit of valid arguments Not greater than (machine maximum) / 5 . Acceptable values for: LOLIM UPLIM IBM 360/370 SERIES : 3.0E-78 1.0E+75 CDC 6000/7000 SERIES : 1.0E-292 1.0E+321 UNIVAC 1100 SERIES : 1.0E-37 1.0E+37 CRAY : 2.3E-2466 1.09E+2465 VAX 11 SERIES : 1.5E-38 3.0E+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - Relative error due to truncation is less than 16 * ERRTOL ** 6 / (1 - 2 * ERRTOL). The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the trunca- tion error there will be round-off error, but in prac- tice the total error from both sources is usually less than the amount given in the table. Sample Choices: ERRTOL Relative Truncation 1.0E-3 2.0E-17 3.0E-3 2.0E-14 1.0E-2 2.0E-11 3.0E-2 2.0E-8 1.0E-1 2.0E-5 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. |

# ABI notes

- Canonical Rust path: `slatec_sys::special::rc`
- Original SLATEC routine: `RC`
- Native symbol: `rc_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [RC](https://www.netlib.org/slatec/src/rc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
