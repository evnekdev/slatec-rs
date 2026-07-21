# Purpose

DBINOM(N,M) calculates the double precision binomial coefficient for integer arguments N and M. The result is (N!)/((M!)(N-M)!).

# Description

This canonical unsafe binding exposes original SLATEC routine `DBINOM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBINOM](https://www.netlib.org/slatec/fnlib/dbinom.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the binomial coefficients

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the binomial coefficients

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dbinom`
- Original SLATEC routine: `DBINOM`
- Native symbol: `dbinom_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DBINOM](https://www.netlib.org/slatec/fnlib/dbinom.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
