# DPSIFN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute derivatives of the Psi function.

## Description

The following definitions are used in DPSIFN: Definition 1 PSI(X) = d/dx (ln(GAMMA(X)), the first derivative of the log GAMMA function. Definition 2 K K PSI(K,X) = d /dx (PSI(X)), the K-th derivative of PSI(X). ___________________________________________________________________ DPSIFN computes a sequence of SCALED derivatives of the PSI function; i.e. for fixed X and M it computes the M-member sequence ((-1)**(K+1)/GAMMA(K+1))*PSI(K,X) for K = N,...,N+M-1 where PSI(K,X) is as defined above. For KODE=1, DPSIFN returns the scaled derivatives as described. KODE=2 is operative only when K=0 and in that case DPSIFN returns -PSI(X) + LN(X). That is, the logarithmic behavior for large X is removed when KODE=2 and K=0. When sums or differences of PSI functions are computed the logarithmic terms can be combined analytically and computed separately to help retain significant digits. Note that CALL DPSIFN(X,0,1,1,ANS) results in ANS = -PSI(X) Input X is DOUBLE PRECISION X - Argument, X .gt. 0.0D0 N - First member of the sequence, 0 .le. N .le. 100 N=0 gives ANS(1) = -PSI(X) for KODE=1 -PSI(X)+LN(X) for KODE=2 KODE - Selection parameter KODE=1 returns scaled derivatives of the PSI function. KODE=2 returns scaled derivatives of the PSI function EXCEPT when N=0. In this case, ANS(1) = -PSI(X) + LN(X) is returned. M - Number of members of the sequence, M.ge.1 Output ANS is DOUBLE PRECISION ANS - A vector of length at least M whose first M components contain the sequence of derivatives scaled according to KODE. NZ - Underflow flag NZ.eq.0, A normal return NZ.ne.0, Underflow, last NZ components of ANS are set to zero, ANS(M-K+1)=0.0, K=1,...,NZ IERR - Error flag IERR=0, A normal return, computation completed IERR=1, Input error, no computation IERR=2, Overflow, X too small or N+M-1 too large or both IERR=3, Error, N too large. Dimensioned array TRMR(NMAX) is not large enough for N The nominal computational accuracy is the maximum of unit roundoff (=D1MACH(4)) and 1.0D-18 since critical constants are given to only 18 digits. PSIFN is the single precision version of DPSIFN. *Long Description: The basic method of evaluation is the asymptotic expansion for large X.ge.XMIN followed by backward recursion on a two term recursion relation W(X+1) + X**(-N-1) = W(X). This is supplemented by a series SUM( (X+K)**(-N-1) , K=0,1,2,... ) which converges rapidly for large N. Both XMIN and the number of terms of the series are calculated from the unit roundoff of the machine environment.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C7C`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dpsifn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpsifn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpsifn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dpsifn.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
