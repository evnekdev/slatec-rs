# DHFTI

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a least squares problem for banded matrices using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted.

## Description

DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. The problem data consists of an M by N matrix A, an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below. The NB column vectors of B represent right-side vectors for NB distinct linear least squares problems. This set of problems can also be written as the matrix least squares problem AX = B, where X is the N by NB solution matrix. Note that if B is the M by M identity matrix, then X will be the pseudo-inverse of A. This subroutine first transforms the augmented matrix (A B) to a matrix (R C) using premultiplying Householder transformations with column interchanges. All subdiagonal elements in the matrix R are zero and its diagonal elements satisfy ABS(R(I,I)).GE.ABS(R(I+1,I+1)), I = 1,...,L-1, where L = MIN(M,N). The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. Then a solution of minimum Euclidean length is computed using the first KRANK rows of (R C). To be specific we suggest that the user consider an easily computable matrix norm, such as, the maximum of all column sums of magnitudes. Now if the relative uncertainty of B is EPS, (norm of uncertainty/ norm of B), it is suggested that TAU be set approximately equal to EPS*(norm of A). The user must dimension all arrays appearing in the call list.. A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N). This permits the solution of a range of problems in the same array space. The entire set of parameters for DHFTI are INPUT.. All TYPE REAL variables are DOUBLE PRECISION A(*,*),MDA,M,N The array A(*,*) initially contains the M by N matrix A of the least squares problem AX = B. The first dimensioning parameter of the array A(*,*) is MDA, which must satisfy MDA.GE.M Either M.GE.N or M.LT.N is permitted. There is no restriction on the rank of A. The

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
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

- Canonical provider: `main-src/src/dhfti.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dhfti.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dhfti.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dhfti.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DHFTI](https://www.netlib.org/slatec/src/dhfti.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDA, *) | Array argument classified by fixed-form executable read/write analysis. |
| 2 | `MDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `B` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDB, *) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `MDB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `NB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `TAU` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `KRANK` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `RNORM` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 11 | `H` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 12 | `G` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 13 | `IP` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

B(*),MDB,NB       If NB = 0 the subroutine will perform the orthogonal decomposition but will make no TAU               Absolute tolerance parameter provided by user for pseudorank determination. H(*),G(*),IP(*)   Arrays of working space used by DHFTI. OUTPUT.. All TYPE REAL variables are DOUBLE PRECISION A(*,*)            The contents of the array A(*,*) will be modified by the subroutine. These contents are not generally required by the user. B(*)              On return the array B(*) will contain the N by NB solution matrix X. KRANK             Set by the subroutine to indicate the pseudorank of A. RNORM(*)          On return, RNORM(J) will contain the Euclidean norm of the residual vector for the problem defined by the J-th column vector of the array B(*,*) for J = 1,...,NB. H(*),G(*)         On return these arrays respectively contain elements of the pre- and post-multiplying Householder transformations used to compute the minimum Euclidean length solution. IP(*)             Array in which the subroutine records indices describing the permutation of column vectors. The contents of arrays H(*),G(*) and IP(*) are not generally required by the user.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::banded::dhfti`. Native symbol: `dhfti_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::dhfti`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
