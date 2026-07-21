# BNDACC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the LU factorization of a banded matrices using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted.

## Description

These subroutines solve the least squares problem Ax = b for banded matrices A using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted. These subroutines are intended for the type of least squares systems that arise in applications such as curve or surface fitting of data. The least squares equations are accumulated and processed using only part of the data. This requires a certain user interaction during the solution of Ax = b. Specifically, suppose the data matrix (A B) is row partitioned into Q submatrices. Let (E F) be the T-th one of these submatrices where E = (0 C 0). Here the dimension of E is MT by N and the dimension of C is MT by NB. The value of NB is the bandwidth of A. The dimensions of the leading block of zeros in E

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

- Canonical provider: `main-src/src/bndacc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bndacc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bndacc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bndacc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [BNDACC](https://www.netlib.org/slatec/src/bndacc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `G` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDG, *) | Description of calling sequence for BNDACC.. The entire set of parameters for BNDACC are The working array into which the user will place the MT by NB+1 block (C F) in rows IR should be .GE. MU. The value of MU is defined in the abstract of these subprograms. The working array which will contain the processed rows of that part of the data matrix which has been passed to BNDACC. X(N) The entire set of parameters for BNDSOL are These arguments all have the same meaning and 0.E0 |
| 2 | `MDG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Description of calling sequence for BNDACC.. The entire set of parameters for BNDACC are The number of rows in the working array should be .GE. MU. The value of MU is defined in the abstract of these subprograms. is considered an error. X(N) The entire set of parameters for BNDSOL are These arguments all have the same meaning and |
| 3 | `NB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An acceptable value for MU is MU = MAX(MT + N + 1), where N is the number of unknowns. Description of calling sequence for BNDACC.. The entire set of parameters for BNDACC are The bandwidth of the data matrix A. X(N) The entire set of parameters for BNDSOL are contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. DO 10 J=1, NBP1 |
| 4 | `IP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Set by the user to the value 1 before the first call to BNDACC.  Its subsequent value is controlled by BNDACC to set up for the next call to BNDACC. are advanced by BNDACC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for BNDSOL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. |
| 5 | `IR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1, columns 1 through NB+1. See descriptions of IR and MT below. Index of the row of G(*,*) where the user is to place the new block of data (C F).  Set by the user to the value 1 before the first call to BNDACC.  Its subsequent value is controlled is considered an error. are advanced by BNDACC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for BNDSOL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. 0.E0 |
| 6 | `MT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1. The user of the subroutine BNDACC provides MT,JT,C and F for T=1,...,Q.  Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB.  The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL BNDSOL(1,...). The sequence of values for JT must be nondecreasing.  This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by 1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2.  The subprogram BNDACC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL BNDACC(...)  Introduce new blocks of data. CALL BNDSOL(1,...)Compute solution vector and length of residual vector. CALL BNDSOL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL BNDSOL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list.. 1, columns 1 through NB+1. See descriptions of IR and MT below. Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. 1 |
| 7 | `JT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1. The user of the subroutine BNDACC provides MT,JT,C and F for T=1,...,Q.  Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB.  The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL BNDSOL(1,...). The sequence of values for JT must be nondecreasing.  This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. N+1 CALL BNDACC(G,MDG,NB,IP,IR,MT,JT) |

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

Canonical Rust path: `slatec_sys::linear_algebra::banded::bndacc`. Native symbol: `bndacc_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::bndacc`
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
