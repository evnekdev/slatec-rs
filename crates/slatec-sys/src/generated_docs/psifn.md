# Purpose

The following definitions are used in PSIFN: Definition 1 PSI(X) = d/dx (ln(GAMMA(X)), the first derivative of the LOG GAMMA function. Definition 2 K K PSI(K,X) = d /dx (PSI(X)), the K-th derivative of PSI(X). ___________________________________________________________________ PSIFN computes a sequence of SCALED derivatives of the PSI function; i.e. for fixed X and M it computes the M-member sequence ((-1)**(K+1)/GAMMA(K+1))*PSI(K,X) for K = N,...,N+M-1 where PSI(K,X) is as defined above. For KODE=1, PSIFN returns the scaled derivatives as described. KODE=2 is operative only when K=0 and in that case PSIFN returns -PSI(X) + LN(X). That is, the logarithmic behavior for large X is removed when KODE=1 and K=0. When sums or differences of PSI functions are computed the logarithmic terms can be combined analytically and computed separately to help retain significant digits. Note that CALL PSIFN(X,0,1,1,ANS) results in

# Description

This canonical unsafe binding exposes original SLATEC routine `PSIFN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PSIFN](https://www.netlib.org/slatec/src/psifn.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Argument, X. gt. 0. 0E0.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

First member of the sequence, 0. le. N. 100 0 gives ANS(1) = -PSI(X) for KODE=1 -PSI(X)+LN(X) for KODE=2.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Selection parameter 1 returns scaled derivatives of the PSI function. 2 returns scaled derivatives of the PSI function EXCEPT when N=0. In this case,.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of members of the sequence, M. ge. 1.

## `ANS`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

-PSI(X) -PSI(X) + LN(X) is returned. A vector of length at least M whose first M components contain the sequence of derivatives scaled according to KODE.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Underflow flag NZ. eq. 0, A normal return NZ. ne. 0, Underflow, last NZ components of ANS are set to zero, ANS(M-K+1)=0. 0, K=1,.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0, A normal return, computation completed 1, Input error, no computation 2, Overflow, X too small or N+M-1 too large or both 3, Error, N too large. Dimensioned array TRMR(NMAX) is not large enough for N The nominal computational accuracy is the maximum of unit roundoff (=R1MACH(4)) and 1. 0E-18 since critical constants are given to only 18 digits. DPSIFN is the Double Precision version of PSIFN.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | .0, K=1,...,NZ |

# Workspace and array requirements

- `ANS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::psifn`
- Original SLATEC routine: `PSIFN`
- Native symbol: `psifn_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [PSIFN](https://www.netlib.org/slatec/src/psifn.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
