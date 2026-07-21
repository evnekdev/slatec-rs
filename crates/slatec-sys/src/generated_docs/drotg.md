# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DROTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DROTG](https://www.netlib.org/slatec/lin/drotg.f).

# Arguments

## `DA`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision scalar double precision result R.

## `DB`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision scalar double precision result Z.

## `DC`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision result.

## `DS`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision result Construct the Givens transformation ( DC DS ) G = ( ) , DC**2 + DS**2 = 1 , (-DS DC ) which zeros the second entry of the 2-vector (DA,DB)**T. The quantity R = (+/-)SQRT(DA**2 + DB**2) overwrites DA in storage. The value of DB is overwritten by a value Z which allows DC and DS to be recovered by the following algorithm. If Z=1 set DC=0. 0 and DS=1. 0 If ABS(Z).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::drotg`
- Original SLATEC routine: `DROTG`
- Native symbol: `drotg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DROTG](https://www.netlib.org/slatec/lin/drotg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
