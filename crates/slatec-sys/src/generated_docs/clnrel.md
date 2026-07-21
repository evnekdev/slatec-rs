# Purpose

CLNREL(Z) = LOG(1+Z) with relative error accuracy near Z = 0. Let RHO = ABS(Z) and R**2 = ABS(1+Z)**2 = (1+X)**2 + Y**2 = 1 + 2*X + RHO**2 . Now if RHO is small we may evaluate CLNREL(Z) accurately by LOG(1+Z) = CMPLX (LOG(R), CARG(1+Z)) = CMPLX (0.5*LOG(R**2), CARG(1+Z)) = CMPLX (0.5*ALNREL(2*X+RHO**2), CARG(1+Z))

# Description

This canonical unsafe binding exposes original SLATEC routine `CLNREL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CLNREL](https://www.netlib.org/slatec/fnlib/clnrel.f).

# Arguments

## `Z`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate ln(1+X) accurate in the sense of relative error

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::clnrel`
- Original SLATEC routine: `CLNREL`
- Native symbol: `clnrel_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [CLNREL](https://www.netlib.org/slatec/fnlib/clnrel.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
