# Purpose

FZERO searches for a zero of a REAL function F(X) between the given REAL values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion,

# Description

This canonical unsafe binding exposes original SLATEC routine `FZERO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [FZERO](https://www.netlib.org/slatec/src/fzero.f).

# Arguments

## 1. `F`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. EXT   - Name of the REAL external function.  This name must be in an EXTERNAL statement in the calling program. must be a function of one REAL argument. decreased in magnitude as (B,C) collapsed. 0.  However, the interval (B,C) may not have collapsed to the requested tolerance. 3  B may be near a singular point of F(X). increased in magnitude as (B,C) collapsed, i.e. out)) .GT. MAX(ABS(F(B in)),ABS(F(C in))) 4  No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether EXT   - Name of the REAL external function.  This name must be in an EXTERNAL statement in the calling program. must be a function of one REAL argument. decreased in magnitude as (B,C) collapsed. 0.  However, the interval (B,C) may not have collapsed to the requested tolerance. 3  B may be near a singular point of F(X). increased in magnitude as (B,C) collapsed, i.e. out)) .GT. MAX(ABS(F(B in)),ABS(F(C in))) 4  No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether not applicable or not stated by selected source not a workspace argument

## 2. `B`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is designed primarily for problems where F(B) and F(C) have opposite signs. C) .LE. 2.*(RW*ABS(B)+AE). The method used is an efficient combination of bisection and the secant rule and is due to T. J. Dekker. INOUT - One end of the REAL interval (B,C).  The value returned for B usually is the better approximation to a zero of F. if not, but F(R) and F(C) have opposite signs, a root will be found in the interval (R,C); otherwise, the interval (B,C) will be searched for a possible root.  When no better guess is known, it is recommended that r be set to B or C, since if R is not interior to the interval (B,C), it will be ignored. contains the origin, then a nonzero value should be chosen for AE. 0.  However, the interval (B,C) may not have collapsed to the requested tolerance. 3  B may be near a singular point of F(X). erance and the function changes sign in (B,C), but out)) .GT. MAX(ABS(F(B in)),ABS(F(C in))) 4  No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether is near a local minimum of F(X), or B is near a zero of even multiplicity, or neither of these. 5  Too many (.GT. 500) function evaluations used. is designed primarily for problems where F(B) and F(C) have opposite signs. C) .LE. 2.*(RW*ABS(B)+AE). The method used is an efficient combination of bisection and the secant rule and is due to T. J. Dekker. INOUT - One end of the REAL interval (B,C).  The value returned for B usually is the better approximation to a zero of F. if not, but F(R) and F(C) have opposite signs, a root will be found in the interval (R,C); otherwise, the interval (B,C) will be searched for a possible root.  When no better guess is known, it is recommended that r be set to B or C, since if R is not interior to the interval (B,C), it will be ignored. contains the origin, then a nonzero value should be chosen for AE. 0.  However, the interval (B,C) may not have collapsed to the requested tolerance. 3  B may be near a singular point of F(X). erance and the function changes sign in (B,C), but out)) .GT. MAX(ABS(F(B in)),ABS(F(C in))) 4  No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether is near a local minimum of F(X), or B is near a zero of even multiplicity, or neither of these. 5  Too many (.GT. 500) function evaluations used. not applicable or not stated by selected source not a workspace argument

## 3. `C`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is designed primarily for problems where F(B) and F(C) have opposite signs. INOUT - The other end of the REAL interval (B,C) contains the origin, then a nonzero value should be chosen for AE. erance and the function changes sign in (B,C), but not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `R`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. OUT   - A (better) REAL guess of a zero of F which could help in speeding up convergence.  If F(B) and F(R) have opposite signs, a root will be found in the interval if not, but F(R) and F(C) have opposite signs, a root will be found in the interval (R,C); otherwise, the interval (B,C) will be searched for a possible root.  When no better guess is known, it is recommended that r be set to B or C, since if R is not interior to the interval (B,C), it will be ignored. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `RE`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN    - Relative error used for RW in the stopping criterion. If the requested RE is less than machine precision, then RW is set to approximately machine precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `AE`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN    - Absolute error used in the stopping criterion.  If not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `IFLAG`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT   - A status code.  User must check IFLAG after each call.  Control returns to the user from FZERO in all cases. 1  B is within the requested tolerance of a zero. The interval (B,C) collapsed to the requested tolerance, the function changes sign in (B,C), and OUT   - A status code.  User must check IFLAG after each call.  Control returns to the user from FZERO in all cases. 1  B is within the requested tolerance of a zero. The interval (B,C) collapsed to the requested tolerance, the function changes sign in (B,C), and not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `F`: not a workspace argument
- `B`: not a workspace argument
- `C`: not a workspace argument
- `R`: not a workspace argument
- `RE`: not a workspace argument
- `AE`: not a workspace argument
- `IFLAG`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::roots::scalar::fzero`
- Original SLATEC routine: `FZERO`
- Native symbol: `fzero_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [FZERO](https://www.netlib.org/slatec/src/fzero.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
