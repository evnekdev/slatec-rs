# QZVEC

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The optional fourth step of the QZ algorithm for generalized eigenproblems. Accepts a matrix in quasi-triangular form and another in upper triangular and computes the eigenvectors of the triangular problem and transforms them back to the original coordinates Usually preceded by QZHES, QZIT, and QZVAL.

## Description

This subroutine is the optional fourth step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL matrices, one of them in quasi-triangular form (in which each 2-by-2 block corresponds to

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C3`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/qzvec.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/qzvec.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [QZVEC](https://www.netlib.org/slatec/lin/qzvec.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. contains the tolerance quantity (EPSB) |
| 3 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | pair of complex eigenvalues) and the other in upper triangular form.  It computes the eigenvectors of the triangular problem and transforms the results back to the original coordinate system. It is usually preceded by  QZHES,  QZIT, and  QZVAL. triangular matrix.  A is a two- triangular matrix.  A is a two- dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned Z(NM,N). is unaltered.  Its subdiagonal elements provide information about the storage of the complex eigenvectors. th and (J+1)-th columns of Z contain its eigenvector. If ALFI(J) .LT. 0.0, the eigenvalue is the second of 1)-th and J-th columns of Z contain the conjugate of its eigenvector. Each eigenvector is normalized so that the modulus of its largest component is 1.0 . Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY |
| 4 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains a real upper triangular matrix.  In addition, contains the tolerance quantity (EPSB) dimensional REAL array, dimensioned B(NM,N). has been destroyed. system Routines - EISPACK Guide, Springer-Verlag, 1976. |
| 5 | `ALFR` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. |
| 6 | `ALFI` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. th eigenvalue is real and the J-th column of Z contains its eigenvector. th eigenvalue is complex. If ALFI(J) .GT. 0.0, the eigenvalue is the first of |
| 7 | `BETA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. |
| 8 | `Z` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the transformation matrix produced in the reductions by  QZHES,  QZIT, and  QZVAL,  if performed.  If the eigenvectors of the triangular problem are desired, Z must dimensional REAL array, dimensioned Z(NM,N). contains the real and imaginary parts of the eigenvectors. |

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

Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzvec`. Native symbol: `qzvec_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzvec`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
