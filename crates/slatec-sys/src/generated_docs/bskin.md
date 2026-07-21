# Purpose

The following definitions are used in BSKIN: Definition 1

# Description

This canonical unsafe binding exposes original SLATEC routine `BSKIN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSKIN](https://www.netlib.org/slatec/src/bskin.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. zero Bessel function. Definition 2 Bickley Function 1,t)dt 1,2,... ____________________________________________________________________ BSKIN computes sequences of Bickley functions (repeated integrals 1,..., 1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). Argument, X .ge. 0.0E0 1,X), K=1,M = X(KI(L-3,X) - KI(L-1,X)) + (L-2)*KI(L-2,X) is stable where recurrence is carried forward or backward away from INT(X+0.5).  The power series for indices 0,1 and 2 on 0.le.X.le. 2 starts a stable recurrence for indices greater than 2.  If N is sufficiently large (N.gt.NLIM), the uniform asymptotic expansion for N to INFINITY is more economical.  On X.gt.2 the recursion is started by evaluating the uniform expansion for the three members whose indices are 1.  Forward recurrence, backward recurrence or both, complete the sequence depending on the relation of INT(X+0.5) to the ACM Transactions on Mathematical Software, 1983. D. E. Amos, A portable Fortran subroutine for the Bickley functions KI(N,X), Algorithm 609, ACM Transactions on Mathematical Software, 1983. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Bickley Function 1,t)dt 1,2,... ____________________________________________________________________ BSKIN computes sequences of Bickley functions (repeated integrals 1,..., 1,X) for KODE=1 or 1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). Order of first member of the sequence N .ge. 0 1,X), K=1,M 1,X), K=1,M 1.  Forward 1.  Forward recurrence, backward recurrence or both, complete the recurrence, backward recurrence or both, complete the sequence depending on the relation of INT(X+0.5) to the sequence depending on the relation of INT(X+0.5) to the 1. 1. ACM Transactions on Mathematical Software, 1983. D. E. Amos, A portable Fortran subroutine for the Bickley functions KI(N,X), Algorithm 609, ACM Transactions on Mathematical Software, 1983. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Selection parameter 1,X), K=1,M 1.  Y(K)=0.0E0, K=1,...,M is returned not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. member sequence Number of members in the sequence, M.ge.1 1.  Forward recurrence, backward recurrence or both, complete the sequence depending on the relation of INT(X+0.5) to the 1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `Y`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1,X) for KODE=1 or 1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). 1,X), K=1,M 1,X), K=1,M A vector of dimension at least M containing the sequence selected by KODE. 1,X) for KODE=1 or 1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). 1,X), K=1,M 1,X), K=1,M A vector of dimension at least M containing the sequence selected by KODE. not applicable or not stated by selected source not a workspace argument

## 6. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Underflow flag 0 means computation completed = M means an exponential underflow occurred on not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Error flag 0, Normal return, computation completed. = 1, Input error,   no computation. = 2, Error,         no computation.  The termination condition was not met. The nominal computational accuracy is the maximum of unit roundoff (=R1MACH(4)) and 1.0e-18 since critical constants are given to only 18 digits. DBSKIN is the double precision version of BSKIN. Long Description: Numerical recurrence on not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `Y`: not a workspace argument
- `NZ`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bskin`
- Original SLATEC routine: `BSKIN`
- Native symbol: `bskin_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [BSKIN](https://www.netlib.org/slatec/src/bskin.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
