# DPSIFN

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute derivatives of the Psi function.

## Description

The following definitions are used in DPSIFN: Definition 1 PSI(X) = d/dx (ln(GAMMA(X)), the first derivative of the log GAMMA function. Definition 2 K K PSI(K,X) = d /dx (PSI(X)), the K-th derivative of PSI(X). ___________________________________________________________________ DPSIFN computes a sequence of SCALED derivatives of the PSI function; i.e. for fixed X and M it computes the M-member sequence ((-1)**(K+1)/GAMMA(K+1))*PSI(K,X) for K = N,...,N+M-1 where PSI(K,X) is as defined above. For KODE=1, DPSIFN returns the scaled derivatives as described. KODE=2 is operative only when K=0 and in that case DPSIFN returns -PSI(X) + LN(X). That is, the logarithmic behavior for large X is removed when KODE=2 and K=0. When sums or differences of PSI functions are computed the logarithmic terms can be combined analytically and computed separately to help retain significant digits. Note that CALL DPSIFN(X,0,1,1,ANS) results in

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DPSIFN](https://www.netlib.org/slatec/src/dpsifn.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Argument, X. gt. 0. 0D0. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | First member of the sequence, 0. le. N. 100 0 gives ANS(1) = -PSI(X) for KODE=1 -PSI(X)+LN(X) for KODE=2. |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Selection parameter 1 returns scaled derivatives of the PSI function. 2 returns scaled derivatives of the PSI function EXCEPT when N=0. In this case,. |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of members of the sequence, M. ge. 1 Output ANS is DOUBLE PRECISION. |
| 5 | `ANS` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | -PSI(X) Input X is DOUBLE PRECISION -PSI(X) + LN(X) is returned. A vector of length at least M whose first M components contain the sequence of derivatives scaled according to KODE. |
| 6 | `NZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Underflow flag NZ. eq. 0, A normal return NZ. ne. 0, Underflow, last NZ components of ANS are set to zero, ANS(M-K+1)=0. 0, K=1,. |
| 7 | `IERR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Error flag 0, A normal return, computation completed 1, Input error, no computation 2, Overflow, X too small or N+M-1 too large or both 3, Error, N too large. Dimensioned array TRMR(NMAX) is not large enough for N The nominal computational accuracy is the maximum of unit roundoff (=D1MACH(4)) and 1. 0D-18 since critical constants are given to only 18 digits. PSIFN is the single precision version of DPSIFN. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | .0, K=1,...,NZ |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

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
