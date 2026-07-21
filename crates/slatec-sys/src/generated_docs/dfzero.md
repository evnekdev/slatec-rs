# Purpose

DFZERO searches for a zero of a DOUBLE PRECISION function F(X) between the given DOUBLE PRECISION values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B-C) .LE. 2.*(RW*ABS(B)+AE). The method used is an efficient combination of bisection and the secant rule and is due to T. J. Dekker.

# Description

This canonical unsafe binding exposes original SLATEC routine `DFZERO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DFZERO](https://www.netlib.org/slatec/src/dfzero.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

- Name of the DOUBLE PRECISION external function. This name must be in an EXTERNAL statement in the calling program. F must be a function of one DOUBLE PRECISION argument.

## `B`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

- One end of the DOUBLE PRECISION interval (B,C). The value returned for B usually is the better approximation to a zero of F. is near a local minimum of F(X), or B is near a zero of even multiplicity, or neither of these. 5 Too many (. GT. 500) function evaluations used.

## `C`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

- The other end of the DOUBLE PRECISION interval (B,C).

## `R`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

- A (better) DOUBLE PRECISION guess of a zero of F which could help in speeding up convergence. If F(B) and F(R) have opposite signs, a root will be found in the interval (B,R); if not, but F(R) and F(C) have (R,C); otherwise, the interval (B,C) will be searched for a possible root. When no better guess is known, it is recommended that R be set to B or C, since if R is not interior to the interval (B,C), it will be ignored.

## `RE`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

- Relative error used for RW in the stopping criterion. If the requested RE is less than machine precision, then RW is set to approximately machine precision.

## `AE`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

- Absolute error used in the stopping criterion. If the given interval (B,C) contains the origin, then a nonzero value should be chosen for AE.

## `IFLAG`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

- A status code. User must check IFLAG after each call. Control returns to the user from DFZERO in all cases. 1 B is within the requested tolerance of a zero. The interval (B,C) collapsed to the requested tolerance, the function changes sign in (B,C), and F(X) decreased in magnitude as (B,C) collapsed. 2 F(B) = 0.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IFLAG` | `0` | 0. However, the interval (B,C) may not have collapsed to the requested tolerance. 3 B may be near a singular point of F(X). The interval (B,C) collapsed to the requested tol- erance and the function changes sign in (B,C), but F(X) increased in magnitude as (B,C) collapsed, i.e. |
| `IFLAG` | `>0` | MAX(ABS(F(B in)),ABS(F(C in))) 4 No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether |

# ABI notes

- Canonical Rust path: `slatec_sys::roots::scalar::dfzero`
- Original SLATEC routine: `DFZERO`
- Native symbol: `dfzero_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DFZERO](https://www.netlib.org/slatec/src/dfzero.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
