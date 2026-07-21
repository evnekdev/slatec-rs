# Purpose

EXINT computes M member sequences of exponential integrals

# Description

This canonical unsafe binding exposes original SLATEC routine `EXINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EXINT](https://www.netlib.org/slatec/src/exint.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1 for N .GE. 1 and X .GE. 0. 1 for N .GE. 1 and X .GE. 0.  The exponential integral is defined by XT)/T**N 0.0 and N=1 cannot occur simultaneously.  Formulas and notation are found in the NBS Handbook of Mathematical Functions (ref. 1). The power series is implemented for X .LE. XCUT and the confluent hypergeometric representation X)*(X**(A-1))*U(A,A,X) is computed for X .GT. XCUT.  Since sequences are computed in a stable fashion by recurring away from X, A is selected as the integer closest to X within the constraint N .LE. A .LE. X) X) X) once E(A,X) is computed.  The U function is computed by means once E(A,X) is computed.  The U function is computed by means once E(A,X) is computed.  The U function is computed by means of the backward recursive Miller algorithm applied to the of the backward recursive Miller algorithm applied to the of the backward recursive Miller algorithm applied to the 0,1,... This produces accurate ratios and determines U(A+K,A,X), and hence E(A,X), to within a multiplicative constant C. Another contiguous relation applied to C*U(A,A,X) and gets C*U(A+1,A+1,X), a quantity proportional to The normalizing constant C is obtained from the two term recursion relation above with K=A. 1 and  X .GE. 0.0 for N .GE. 2 1 and  X .GE. 0.0 for N .GE. 2 0.0 and N=1 is an error) 1. 1. 1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 for N .GE. 1 and X .GE. 0. 1 for N .GE. 1 and X .GE. 0.  The exponential integral is defined by XT)/T**N 1.  For the U computation, A is further modified to be the nearest even integer.  Indices are carried forward or backward by the two term recursion relation 1 and  X .GE. 0.0 for N .GE. 2 order of the first member of the sequence, N .GE. 1 1. 1. 1,X) or EXP(X)*E(N+K-1,X), K=1,M depending on KODE not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. a selection parameter for scaled values 1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 for N .GE. 1 and X .GE. 0. 1 for N .GE. 1 and X .GE. 0.  The exponential integral is defined by 1.  For the U computation, A is further modified to be the nearest even integer.  Indices are carried forward or backward by the two term recursion relation 1. 1. number of exponential integrals in the sequence, .GE. 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `TOL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. relative accuracy wanted, ETOL .LE. TOL .LE. 0.1 ETOL = single precision unit roundoff = R1MACH(4) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `EN`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a vector of dimension at least M containing values 1,X) or EXP(X)*E(N+K-1,X), K=1,M depending on KODE 0.0E0 , K=1,M returned on KODE=1 a vector of dimension at least M containing values 1,X) or EXP(X)*E(N+K-1,X), K=1,M depending on KODE 0.0E0 , K=1,M returned on KODE=1 not applicable or not stated by selected source not a workspace argument

## 7. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. underflow indicator 0   a normal return M   X exceeds XLIM and an underflow occurs. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. error flag 0, normal return, computation completed 1, input error,   no computation 2, error,         no computation algorithm termination condition not met not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `N`: not a workspace argument
- `KODE`: not a workspace argument
- `M`: not a workspace argument
- `TOL`: not a workspace argument
- `EN`: not a workspace argument
- `NZ`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::exint`
- Original SLATEC routine: `EXINT`
- Native symbol: `exint_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [EXINT](https://www.netlib.org/slatec/src/exint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
