# DBSKIN

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute repeated integrals of the K-zero Bessel function.

## Description

The following definitions are used in DBSKIN: Definition 1

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

- Canonical provider: `main-src/src/dbskin.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbskin.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbskin.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbskin.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DBSKIN](https://www.netlib.org/slatec/src/dbskin.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | zero Bessel function. Definition 2 Bickley Function 1,t)dt 1,2,... _____________________________________________________________________ DBSKIN computes a sequence of Bickley functions (repeated integrals 1,..., DBSKIN computes the sequence 1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). INPUT      X is DOUBLE PRECISION Argument, X .ge. 0.0D0 1,X), K=1,M = X(KI(L-3,X) - KI(L-1,X)) + (L-2)*KI(L-2,X) is stable where recurrence is carried forward or backward away from INT(X+0.5).  The power series for indices 0,1 and 2 on 0.le.X.le.2 starts a stable recurrence for indices greater than 2.  If N is sufficiently large (N.gt.NLIM), the uniform asymptotic expansion for N to INFINITY is more economical.  On X.gt.2 the recursion is started by evaluating the uniform expansion for the three members whose indices are 1.  Forward recurrence, backward recurrence or both complete the sequence depending on the relation of INT(X+0.5) to the ACM Transactions on Mathematical Software, 1983. D. E. Amos, A portable Fortran subroutine for the Bickley functions KI(N,X), Algorithm 609, ACM Transactions on Mathematical Software, 1983. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Bickley Function 1,t)dt 1,2,... _____________________________________________________________________ DBSKIN computes a sequence of Bickley functions (repeated integrals 1,..., DBSKIN computes the sequence 1,X) for KODE=1 or 1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). INPUT      X is DOUBLE PRECISION Order of first member of the sequence N .ge. 0 1,X), K=1,M 1,X), K=1,M 1.  Forward 1.  Forward recurrence, backward recurrence or both complete the recurrence, backward recurrence or both complete the sequence depending on the relation of INT(X+0.5) to the sequence depending on the relation of INT(X+0.5) to the 1. 1. ACM Transactions on Mathematical Software, 1983. D. E. Amos, A portable Fortran subroutine for the Bickley functions KI(N,X), Algorithm 609, ACM Transactions on Mathematical Software, 1983. |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Selection parameter 1,X), K=1,M 1.  Y(K)=0.0D0, K=1,...,M is returned 1 AND Y(K)=0.0E0, K=1,...,M IS RETURNED |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of members in the sequence, M.ge.1 OUTPUT     Y is a DOUBLE PRECISION VECTOR 1.  Forward recurrence, backward recurrence or both complete the sequence depending on the relation of INT(X+0.5) to the 1. |
| 5 | `Y` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 1,X) for KODE=1 or 1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). INPUT      X is DOUBLE PRECISION 1,X), K=1,M 1,X), K=1,M A vector of dimension at least M containing the sequence selected by KODE. |
| 6 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Underflow flag 0 means computation completed = 1 means an exponential underflow occurred on |
| 7 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Error flag 0, Normal return, computation completed 1, Input error,   no computation 2, Error,         no computation Algorithm termination condition not met The nominal computational accuracy is the maximum of unit roundoff (=D1MACH(4)) and 1.0D-18 since critical constants are given to only 18 digits. BSKIN is the single precision version of DBSKIN. Long Description: Numerical recurrence on |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dbskin`. Native symbol: `dbskin_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dbskin`
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
