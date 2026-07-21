# DBNDAC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the LU factorization of a banded matrices using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted.

## Description

These subroutines solve the least squares problem Ax = b for banded matrices A using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted. These subroutines are intended for the type of least squares systems that arise in applications such as curve or surface fitting of data. The least squares equations are accumulated and processed using only part of the data. This requires a certain user interaction during the solution of Ax = b. Specifically, suppose the data matrix (A B) is row partitioned into Q submatrices. Let (E F) be the T-th one of these submatrices where E = (0 C 0). Here the dimension of E is MT by N and the dimension of C is MT by NB. The value of NB is the bandwidth of A. The dimensions of the leading block of zeros in E are MT by JT-1. The user of the subroutine DBNDAC provides MT,JT,C and F for T=1,...,Q. Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB. The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL DBNDSL(1,...). The sequence of values for JT must be nondecreasing. This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by NB+1. An acceptable value for MU is MU = MAX(MT + N + 1), where N is the number of unknowns. Here the maximum is taken over all values of MT for T=1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2. The subprogram DBNDAC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL DBNDAC(...) Introduce new blocks of data. CALL DBNDSL(1,...)Compute solution vector and length of residual vector. CALL DBNDSL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL DBNDSL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list.. G(MDG,NB+1) Description of calling sequence for DBNDAC.. The entire set of parameters for DBNDAC are Input.. All Type REAL variables are DOUBLE PRECISION G(*,*) The working array into which the user will place the MT by NB+1 block (C F) in rows IR through IR+MT-1, columns 1 through NB+1. See descriptions of IR and MT below. MDG The number of rows in the working array G(*,*). The value of MDG should be .GE. MU. The value of MU is defined in the abstract of these subprograms. NB The bandwidth of the data matrix A. IP Set by the user to the value 1 before the first call to DBNDAC. Its subsequent value is controlled by DBNDAC to set up for the next call to DBNDAC. IR Index of the row of G(*,*) where the user is to place the new block of data (C F). Set by the user to the value 1 before the first call to DBNDAC. Its subsequent value is controlled by DBNDAC. A value of IR .GT. MDG is considered

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

- Canonical provider: `main-src/src/dbndac.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbndac.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbndac.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbndac.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DBNDAC](https://www.netlib.org/slatec/src/dbndac.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `G` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDG, *) | Array argument classified by fixed-form executable read/write analysis. |
| 2 | `MDG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `NB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `IP` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `MT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `JT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

MT,JT             Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. Output.. All Type REAL variables are DOUBLE PRECISION G(*,*)            The working array which will contain the processed rows of that part of the data matrix which has been passed to DBNDAC. IP,IR             The values of these arguments are advanced by DBNDAC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for DBNDSL.. The user must dimension the arrays appearing in the call list.. G(MDG,NB+1), X(N) The entire set of parameters for DBNDSL are Input.. All Type REAL variables are DOUBLE PRECISION MODE              Set by the user to one of the values 1, 2, or 3.  These values respectively indicate that the solution of AX = B, YR = H or RZ = W is required. G(*,*),MDG,       These arguments all have the same meaning and NB,IP,IR         contents as following the last call to DBNDAC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine DBNDSL prints an appropriate message.  This condition is Output.. All Type REAL variables are DOUBLE PRECISION X(*)              This array contains the solution vectors X, Y or Z of the systems AX = B, YR = H or RZ = W depending on the value of MODE=1, 2 or 3. RNORM             If MODE=1 RNORM is the Euclidean length of the residual vector AX-B.  When MODE=2 or 3 RNORM is set to zero. Remarks.. To obtain the upper triangular matrix and transformed right-hand side vector D so that the super diagonals of R form the columns of G(*,*), execute the following Fortran statements. NBP1=NB+1 DO 10 J=1, NBP1 10 G(IR,J) = 0.E0 MT=1 JT=N+1 CALL DBNDAC(G,MDG,NB,IP,IR,MT,JT)

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::banded::dbndac`. Native symbol: `dbndac_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::dbndac`
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
