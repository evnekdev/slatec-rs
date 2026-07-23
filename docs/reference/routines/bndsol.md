# BNDSOL

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the least squares problem for a banded matrix using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted.

## Description

These subroutines solve the least squares problem Ax = b for banded matrices A using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted. These subroutines are intended for the type of least squares systems that arise in applications such as curve or surface fitting of data. The least squares equations are accumulated and processed using only part of the data. This requires a certain user interaction during the solution of Ax = b. Specifically, suppose the data matrix (A B) is row partitioned into Q submatrices. Let (E F) be the T-th one of these submatrices where E = (0 C 0). Here the dimension of E is MT by N and the dimension of C is MT by NB. The value of NB is the bandwidth of A. The dimensions of the leading block of zeros in E are MT by JT-1. The user of the subroutine BNDACC provides MT,JT,C and F for T=1,...,Q. Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB. The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL BNDSOL(1,...). The sequence of values for JT must be nondecreasing. This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by NB+1. An acceptable value for MU is MU = MAX(MT + N + 1), where N is the number of unknowns. Here the maximum is taken over all values of MT for T=1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2. The subprogram BNDACC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL BNDACC(...) Introduce new blocks of data. CALL BNDSOL(1,...)Compute solution vector and length of residual vector. CALL BNDSOL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL BNDSOL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list..

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/bndsol.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bndsol.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bndsol.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bndsol.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [BNDSOL](https://www.netlib.org/slatec/src/bndsol.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Set by the user to one of the values 1, 2, or 3. These values respectively indicate that the solution of AX = B, YR = H or RZ = W is required. |
| 2 | `G` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDG, *) | Description of calling sequence for BNDACC. The entire set of parameters for BNDACC are The working array into which the user will place the MT by NB+1 block (C F) in rows IR through IR+MT-1, columns 1 through NB+1. See descriptions of IR and MT below. The value of MDG should be. GE. MU. |
| 3 | `MDG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of rows in the working array These arguments all have the same meaning and. |
| 4 | `NB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The bandwidth of the data matrix A. contents as following the last call to BNDACC. |
| 5 | `IP` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Set by the user to the value 1 before the first call to BNDACC. Its subsequent value is controlled by BNDACC to set up for the next call to BNDACC. The values of these arguments are advanced by BNDACC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for BNDSOL. The user must dimension the arrays appearing in the call list. G(MDG,NB+1), X(N) The entire set of parameters for BNDSOL are contents as following the last call to BNDACC. |
| 6 | `IR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Index of the row of G(*,*) where the user is the user to the value 1 before the first call to BNDACC. Its subsequent value is controlled by BNDACC. A value of IR. GT. MDG is considered an error. MT,JT Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. |
| 7 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. This array contains the solution vectors X, Y or Z of the systems AX = B, YR = H or RZ = W depending on the value of MODE=1, 2 or 3. |
| 8 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of variables in the solution vector. If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message. This condition is considered an error. |
| 9 | `RNORM` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If MODE=1 RNORM is the Euclidean length of the residual vector AX-B. When MODE=2 or 3 RNORM is set to zero. Remarks. To obtain the upper triangular matrix and transformed right-hand side vector D so that the super diagonals of R form the columns of G(*,*), execute the following Fortran statements. NBP1=NB+1 DO 10 J=1, NBP1 10 G(IR,J) = 0. E0 MT=1 JT=N+1 CALL BNDACC(G,MDG,NB,IP,IR,MT,JT). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::banded::bndsol`. Native symbol: `bndsol_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::bndsol`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
