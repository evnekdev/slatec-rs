# CNBFA

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a band matrix by elimination.

## Description

CNBFA factors a complex band matrix by elimination. CNBFA is usually called by CNBCO, but it can be called directly with a saving in time if RCOND is not needed. On Entry ABE COMPLEX(LDA, NC) contains the matrix in band storage. The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE. NC must be .GE. 2*ML+MU+1 . See the comments below for details. LDA INTEGER the leading dimension of the array ABE. LDA must be .GE. N . N INTEGER the order of the original matrix. ML INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . MU INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if ML .LE. MU . On Return ABE an upper triangular matrix in band storage and the multipliers which were used to obtain it. the factorization can be written A = L*U where L is a product of permutation and unit lower triangular matrices and U is upper triangular. IPVT INTEGER(N) an integer vector of pivot indices. INFO INTEGER =0 normal value =K if U(K,K) .EQ. 0.0 . This is not an error condition for this subroutine, but it does indicate that CNBSL will divide by zero if called. Use RCOND in CNBCO for a reliable indication of singularity. Band Storage If A is a band matrix, the following program segment will set up the input. ML = (band width below the diagonal) MU = (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 ABE(I,K) = A(I,J) 10 CONTINUE 20 CONTINUE This uses columns 1 through ML+MU+1 of ABE . Furthermore, ML additional columns are needed in ABE starting with column ML+MU+2 for elements generated during the triangularization. The total number of columns needed in ABE is 2*ML+MU+1 . Example: If the original matrix is 11 12 13 0 0 0 21 22 23 24 0 0 0 32 33 34 35 0 0 0 43 44 45 46 0 0 0 54 55 56 0 0 0 0 65 66 then N = 6, ML = 1, MU = 2, LDA .GE. 5 and ABE should contain * 11 12 13 + , * = not used 21 22 23 24 + , + = used for pivoting 32 33 34 35 + 43 44 45 46 + 54 55 56 * + 65 66 * * +

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2C2`
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

- Canonical provider: `main-src/src/cnbfa.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cnbfa.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cnbfa.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cnbfa.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-complex-arguments`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
