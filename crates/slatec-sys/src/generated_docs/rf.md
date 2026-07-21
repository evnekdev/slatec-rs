# Purpose

1. RF Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the first kind Standard FORTRAN function routine Single precision version The routine calculates an approximation result to RF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt, where X, Y, and Z are nonnegative and at most one of them is zero. If one of them is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence RF( X, Y, Z, IER ) Parameters on Entry Values assigned by the calling routine

# Description

This canonical unsafe binding exposes original SLATEC routine `RF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RF](https://www.netlib.org/slatec/src/rf.f).

# Arguments

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Single precision, nonnegative variable unaltered. 3. Error Messages Value of IER assigned by the RF routine Value assigned Error Message Printed IER = 1 MIN(X,Y,Z). LT. 0. 0E0 = 2 MIN(X+Y,X+Z,Y+Z).

## `Y`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Single precision, nonnegative variable unaltered. 3. Error Messages Value of IER assigned by the RF routine Value assigned Error Message Printed IER = 1 MIN(X,Y,Z). LT. 0. 0E0 = 2 MIN(X+Y,X+Z,Y+Z).

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Single precision, nonnegative variable On Return (values assigned by the RF routine) RF - Single precision approximation to the integral unaltered. 3. Error Messages Value of IER assigned by the RF routine Value assigned Error Message Printed IER = 1 MIN(X,Y,Z). LT. 0. 0E0 = 2 MIN(X+Y,X+Z,Y+Z).

## `IER`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`. It has no separate Rust `Result` status channel.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine |
| `IER` | `1` | 1 MIN(X,Y,Z) .LT. 0.0E0 |
| `IER` | `2` | 2 MIN(X+Y,X+Z,Y+Z) .LT. LOLIM |
| `IER` | `3` | 3 MAX(X,Y,Z) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y and Z LOLIM - Lower limit of valid arguments Not less than 5 * (machine minimum). UPLIM - Upper limit of valid arguments Not greater than (machine maximum) / 5. Acceptable Values For: LOLIM UPLIM IBM 360/370 SERIES : 3.0E-78 1.0E+75 CDC 6000/7000 SERIES : 1.0E-292 1.0E+321 UNIVAC 1100 SERIES : 1.0E-37 1.0E+37 CRAY : 2.3E-2466 1.09E+2465 VAX 11 SERIES : 1.5E-38 3.0E+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - Relative error due to truncation is less than ERRTOL ** 6 / (4 * (1-ERRTOL) . The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the trunca- tion error there will be round-off error, but in prac- tice the total error from both sources is usually less than the amount given in the table. Sample Choices: ERRTOL Relative Truncation 1.0E-3 3.0E-19 3.0E-3 2.0E-16 1.0E-2 3.0E-13 3.0E-2 2.0E-10 1.0E-1 3.0E-7 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. |

# ABI notes

- Canonical Rust path: `slatec_sys::special::rf`
- Original SLATEC routine: `RF`
- Native symbol: `rf_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [RF](https://www.netlib.org/slatec/src/rf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
