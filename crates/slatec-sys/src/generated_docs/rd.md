# Purpose

1. RD Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the second kind Standard FORTRAN function routine Single precision version The routine calculates an approximation result to RD(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -3/2 (3/2)(t+X) (t+Y) (t+Z) dt, where X and Y are nonnegative, X + Y is positive, and Z is positive. If X or Y is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence RD( X, Y, Z, IER ) Parameters on Entry Values assigned by the calling routine

# Description

This canonical unsafe binding exposes original SLATEC routine `RD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RD](https://www.netlib.org/slatec/src/rd.f).

# Arguments

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Single precision, nonnegative variable positive unaltered. 3. Error Messages Value of IER assigned by the RD routine Value Assigned Error Message Printed IER = 1 MIN(X,Y). LT. 0. 0E0 = 2 MIN(X + Y, Z ).

## `Y`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Single precision, nonnegative variable positive unaltered. 3. Error Messages Value of IER assigned by the RD routine Value Assigned Error Message Printed IER = 1 MIN(X,Y). LT. 0. 0E0 = 2 MIN(X + Y, Z ).

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Real, positive variable On Return (values assigned by the RD routine) RD - Real approximation to the integral unaltered. 3. Error Messages Value of IER assigned by the RD routine Value Assigned Error Message Printed IER = 1 MIN(X,Y). LT. 0. 0E0 = 2 MIN(X + Y, Z ).

## `IER`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`. It has no separate Rust `Result` status channel.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine |
| `IER` | `1` | 1 MIN(X,Y) .LT. 0.0E0 |
| `IER` | `2` | 2 MIN(X + Y, Z ) .LT. LOLIM |
| `IER` | `3` | 3 MAX(X,Y,Z) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y, and Z LOLIM - Lower limit of valid arguments Not less than 2 / (machine maximum) ** (2/3). UPLIM - Upper limit of valid arguments Not greater than (0.1E0 * ERRTOL / machine minimum) ** (2/3), where ERRTOL is described below. In the following table it is assumed that ERRTOL will never be chosen smaller than 1.0E-5. Acceptable Values For: LOLIM UPLIM IBM 360/370 SERIES : 6.0E-51 1.0E+48 CDC 6000/7000 SERIES : 5.0E-215 2.0E+191 UNIVAC 1100 SERIES : 1.0E-25 2.0E+21 CRAY : 3.0E-1644 1.69E+1640 VAX 11 SERIES : 1.0E-25 4.5E+21 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL Relative error due to truncation is less than 3 * ERRTOL ** 6 / (1-ERRTOL) ** 3/2. The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the trunca- tion error there will be round-off error, but in prac- tice the total error from both sources is usually less than the amount given in the table. Sample Choices: ERRTOL Relative Truncation 1.0E-3 4.0E-18 3.0E-3 3.0E-15 1.0E-2 4.0E-12 3.0E-2 3.0E-9 1.0E-1 4.0E-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. |

# ABI notes

- Canonical Rust path: `slatec_sys::special::rd`
- Original SLATEC routine: `RD`
- Native symbol: `rd_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [RD](https://www.netlib.org/slatec/src/rd.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
