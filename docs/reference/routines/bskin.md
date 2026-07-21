# BSKIN

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute repeated integrals of the K-zero Bessel function.

## Description

The following definitions are used in BSKIN: Definition 1 KI(0,X) = K-zero Bessel function. Definition 2 KI(N,X) = Bickley Function = integral from X to infinity of KI(N-1,t)dt for X .ge. 0 and N = 1,2,... ____________________________________________________________________ BSKIN computes sequences of Bickley functions (repeated integrals of the K0 Bessel function); i.e. for fixed X and N and K=1,..., BSKIN computes the M-member sequence Y(K) = KI(N+K-1,X) for KODE=1 or Y(K) = EXP(X)*KI(N+K-1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10F`
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

- Canonical provider: `main-src/src/bskin.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bskin.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bskin.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bskin.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [BSKIN](https://www.netlib.org/slatec/src/bskin.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Argument, X .ge. 0.0E0 1,X), K=1,M |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Order of first member of the sequence N .ge. 0 1,X), K=1,M 1,X), K=1,M |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Selection parameter 1,X), K=1,M 1.  Y(K)=0.0E0, K=1,...,M is returned |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of members in the sequence, M.ge.1 |
| 5 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1,X), K=1,M 1,X), K=1,M A vector of dimension at least M containing the sequence selected by KODE. |
| 6 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Underflow flag 0 means computation completed = M means an exponential underflow occurred on |
| 7 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

IERR = 0, Normal return, computation completed. termination condition was not met. The nominal computational accuracy is the maximum of unit roundoff (=R1MACH(4)) and 1.0e-18 since critical constants are given to only 18 digits. DBSKIN is the double precision version of BSKIN. Long Description: Numerical recurrence on (L-1)*KI(L,X) = X(KI(L-3,X) - KI(L-1,X)) + (L-2)*KI(L-2,X) is stable where recurrence is carried forward or backward away from INT(X+0.5).  The power series for indices 0,1 and 2 on 0.le.X.le. 2 starts a stable recurrence for indices greater than 2.  If N is sufficiently large (N.gt.NLIM), the uniform asymptotic expansion for N to INFINITY is more economical.  On X.gt.2 the recursion is started by evaluating the uniform expansion for the three members whose indices are closest to INT(X+0.5) within the set N,...,N+M-1.  Forward recurrence, backward recurrence or both, complete the sequence depending on the relation of INT(X+0.5) to the indices N,...,N+M-1.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::bskin`. Native symbol: `bskin_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::bskin`
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
