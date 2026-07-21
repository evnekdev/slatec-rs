# Purpose

The following definitions are used in PSIFN: Definition 1

# Description

This canonical unsafe binding exposes original SLATEC routine `PSIFN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PSIFN](https://www.netlib.org/slatec/src/psifn.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. d/dx (ln(GAMMA(X)), the first derivative of the LOG GAMMA function. Definition 2 K   K th derivative of PSI(X). th derivative of PSI(X). ___________________________________________________________________ ___________________________________________________________________ PSIFN computes a sequence of SCALED derivatives of PSIFN computes a sequence of SCALED derivatives of the PSI function; i.e. for fixed X and M it computes the PSI function; i.e. for fixed X and M it computes 1, PSIFN returns 1 and K=0.  When sums or differences of PSI functions are computed the logarithmic terms can be combined analytically and computed separately to help retain significant digits. Note that CALL PSIFN(X,0,1,1,ANS) results in Argument, X .gt. 0.0E0 for KODE=2 for KODE=2 1 too large or both N-1) = W(X). N-1) = W(X). This is supplemented by a series This is supplemented by a series N-1) , K=0,1,2,... ) which converges rapidly for large N. Both XMIN and the number of terms of the series are calculated from the unit roundoff of the machine environment. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 1 First member of the sequence, 0 .le. N .le. 100 PSI(X)       for KODE=1 0. In this case, 1 too large or both not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, PSIFN returns 2 is operative only when K=0 and in that case PSIFN returns -PSI(X) + LN(X).  That 1 and K=0.  When sums or differences of PSI functions are computed the logarithmic terms can be combined analytically and computed separately to help retain significant digits. Note that CALL PSIFN(X,0,1,1,ANS) results in Selection parameter 1 returns scaled derivatives of the PSI function. 2 returns scaled derivatives of the PSI not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. member sequence 1 Number of members of the sequence, M .ge. 1 K+1)=0.0, K=1,...,NZ 1 too large or both not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `ANS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). PSI(X) PSI(X)       for KODE=1 PSI(X) + LN(X) is returned. A vector of length at least M whose first M components contain the sequence of derivatives scaled according to KODE. K+1)=0.0, K=1,...,NZ PSI(X) PSI(X)       for KODE=1 PSI(X) + LN(X) is returned. A vector of length at least M whose first M components contain the sequence of derivatives scaled according to KODE. K+1)=0.0, K=1,...,NZ not applicable or not stated by selected source not a workspace argument

## 6. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Underflow flag A normal return Underflow, last NZ components of ANS are Underflow flag A normal return Underflow, last NZ components of ANS are not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Error flag 0, A normal return, computation completed 1, Input error,     no computation 1 too large or both 3, Error,           N too large. Dimensioned array TRMR(NMAX) is not large enough for N The nominal computational accuracy is the maximum of unit roundoff (=R1MACH(4)) and 1.0E-18 since critical constants are given to only 18 digits. DPSIFN is the Double Precision version of PSIFN. Long Description: The basic method of evaluation is the asymptotic expansion for large X.ge.XMIN followed by backward recursion on a two term recursion relation Error flag 0, A normal return, computation completed 1, Input error,     no computation 1 too large or both 3, Error,           N too large. Dimensioned array TRMR(NMAX) is not large enough for N The nominal computational accuracy is the maximum of unit roundoff (=R1MACH(4)) and 1.0E-18 since critical constants are given to only 18 digits. DPSIFN is the Double Precision version of PSIFN. Long Description: The basic method of evaluation is the asymptotic expansion for large X.ge.XMIN followed by backward recursion on a two term recursion relation not applicable or not stated by selected source not a workspace argument

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
- `ANS`: not a workspace argument
- `NZ`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::psifn`
- Original SLATEC routine: `PSIFN`
- Native symbol: `psifn_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [PSIFN](https://www.netlib.org/slatec/src/psifn.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
