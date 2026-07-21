# Purpose

EXINT computes M member sequences of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0. The exponential integral is defined by E(N,X)=integral on (1,infinity) of EXP(-XT)/T**N where X=0.0 and N=1 cannot occur simultaneously. Formulas and notation are found in the NBS Handbook of Mathematical Functions (ref. 1). The power series is implemented for X .LE. XCUT and the confluent hypergeometric representation E(A,X) = EXP(-X)*(X**(A-1))*U(A,A,X) is computed for X .GT. XCUT. Since sequences are computed in a stable fashion by recurring away from X, A is selected as the integer closest to X within the constraint N .LE. A .LE. N+M-1. For the U computation, A is further modified to be the nearest even integer. Indices are carried forward or backward by the two term recursion relation K*E(K+1,X) + X*E(K,X) = EXP(-X) once E(A,X) is computed. The U function is computed by means of the backward recursive Miller algorithm applied to the three term contiguous relation for U(A+K,A,X), K=0,1,... This produces accurate ratios and determines U(A+K,A,X), and hence E(A,X), to within a multiplicative constant C. Another contiguous relation applied to C*U(A,A,X) and C*U(A+1,A,X) gets C*U(A+1,A+1,X), a quantity proportional to E(A+1,X). The normalizing constant C is obtained from the two term recursion relation above with K=A.

# Description

This canonical unsafe binding exposes original SLATEC routine `EXINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EXINT](https://www.netlib.org/slatec/src/exint.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

X. GT. 0. 0 for N=1 and X. GE. 0 for N.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the first member of the sequence, N. GE. 1 (X=0. 0 and N=1 is an error).

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

a selection parameter for scaled values 1 returns E(N+K,X), K=0,1,. ,M-1. =2 returns EXP(X)*E(N+K,X), K=0,1,.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of exponential integrals in the sequence,. GE. 1.

## `TOL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

relative accuracy wanted, ETOL. LE. TOL. 0. 1 ETOL = single precision unit roundoff = R1MACH(4).

## `EN`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a vector of dimension at least M containing values E(N+K-1,X) or EXP(X)*E(N+K-1,X), K=1,M depending on KODE 0. 0E0 , K=1,M returned on KODE=1.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

underflow indicator NZ=0 a normal return M X exceeds XLIM and an underflow occurs.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

error flag 0, normal return, computation completed 1, input error, no computation 2, error, no computation algorithm termination condition not met.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | a normal return |

# Workspace and array requirements

- `EN`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::exint`
- Original SLATEC routine: `EXINT`
- Native symbol: `exint_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [EXINT](https://www.netlib.org/slatec/src/exint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
