# Purpose

ALNREL(X) evaluates ln(1+X) accurately in the sense of relative error when X is very small. This routine must be used to maintain relative error accuracy whenever X is small and accurately known. Series for ALNR on the interval -3.75000D-01 to 3.75000D-01 with weighted error 1.93E-17 log weighted error 16.72 significant figures required 16.44 decimal places required 17.40

# Description

This canonical unsafe binding exposes original SLATEC routine `ALNREL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ALNREL](https://www.netlib.org/slatec/fnlib/alnrel.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate ln(1+X) accurate in the sense of relative error

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::alnrel`
- Original SLATEC routine: `ALNREL`
- Native symbol: `alnrel_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [ALNREL](https://www.netlib.org/slatec/fnlib/alnrel.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
