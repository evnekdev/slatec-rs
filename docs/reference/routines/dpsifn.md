# DPSIFN

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute derivatives of the Psi function.

## Description

The following definitions are used in DPSIFN: Definition 1 PSI(X) = d/dx (ln(GAMMA(X)), the first derivative of the log GAMMA function. Definition 2 K K PSI(K,X) = d /dx (PSI(X)), the K-th derivative of PSI(X). ___________________________________________________________________ DPSIFN computes a sequence of SCALED derivatives of the PSI function; i.e. for fixed X and M it computes the M-member sequence ((-1)**(K+1)/GAMMA(K+1))*PSI(K,X) for K = N,...,N+M-1 where PSI(K,X) is as defined above. For KODE=1, DPSIFN returns the scaled derivatives as described. KODE=2 is operative only when K=0 and in that case DPSIFN returns -PSI(X) + LN(X). That is, the logarithmic behavior for large X is removed when KODE=2 and K=0. When sums or differences of PSI functions are computed the logarithmic terms can be combined analytically and computed separately to help retain significant digits. Note that CALL DPSIFN(X,0,1,1,ANS) results in ANS = -PSI(X) Input X is DOUBLE PRECISION X - Argument, X .gt. 0.0D0 N - First member of the sequence, 0 .le. N .le. 100 N=0 gives ANS(1) = -PSI(X) for KODE=1 -PSI(X)+LN(X) for KODE=2 KODE - Selection parameter KODE=1 returns scaled derivatives of the PSI function. KODE=2 returns scaled derivatives of the PSI function EXCEPT when N=0. In this case, ANS(1) = -PSI(X) + LN(X) is returned. M - Number of members of the sequence, M.ge.1 Output ANS is DOUBLE PRECISION ANS - A vector of length at least M whose first M components contain the sequence of derivatives scaled according to KODE. NZ - Underflow flag NZ.eq.0, A normal return NZ.ne.0, Underflow, last NZ components of ANS are set to zero, ANS(M-K+1)=0.0, K=1,...,NZ

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

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DPSIFN](https://www.netlib.org/slatec/src/dpsifn.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `ANS` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 7 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

IERR=0, A normal return, computation completed IERR=2, Overflow,        X too small or N+M-1 too large or both array TRMR(NMAX) is not large enough for N The nominal computational accuracy is the maximum of unit roundoff (=D1MACH(4)) and 1.0D-18 since critical constants are given to only 18 digits. PSIFN is the single precision version of DPSIFN. Long Description: The basic method of evaluation is the asymptotic expansion for large X.ge.XMIN followed by backward recursion on a two term recursion relation W(X+1) + X**(-N-1) = W(X). This is supplemented by a series SUM( (X+K)**(-N-1) , K=0,1,2,... ) which converges rapidly for large N. Both XMIN and the number of terms of the series are calculated from the unit roundoff of the machine environment.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dpsifn`. Native symbol: `dpsifn_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dpsifn`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
