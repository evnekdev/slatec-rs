# CINVIT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvectors of a complex upper Hessenberg associated with specified eigenvalues using inverse iteration.

## Description

This subroutine is a translation of the ALGOL procedure CXINVIT by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP. VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of A COMPLEX UPPER Hessenberg matrix corresponding to specified eigenvalues, using inverse iteration. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM. AR and AI contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). WR and WI contain the real and imaginary parts, respectively, of the eigenvalues of the matrix. The eigenvalues must be stored in a manner identical to that of subroutine COMLR, which recognizes possible splitting of the matrix. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). SELECT specifies the eigenvectors to be found. The eigenvector corresponding to the J-th eigenvalue is specified by setting SELECT(J) to .TRUE. SELECT is a one-dimensional LOGICAL array, dimensioned SELECT(N). MM should be set to an upper bound for the number of eigenvectors to be found. MM is an INTEGER variable. On OUTPUT AR, AI, WI, and SELECT are unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. M is the number of eigenvectors actually found. M is an INTEGER variable. ZR and ZI contain the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1. Any vector which fails the acceptance test is set to zero. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and ZI(NM,MM). IERR is an INTEGER flag set to Zero for normal return, -(2*N+1) if more than MM eigenvectors have been requested (the MM eigenvectors calculated to this point are in ZR and ZI), -K if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of ZR and ZI are set to zero vectors, -(N+K) if both error situations occur. RV1 and RV2 are one-dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process. RM1 and RM2 are two-dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C2B`
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

- Canonical provider: `lin/cinvit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cinvit.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cinvit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::cinvit`
- Current legacy Rust paths: `none`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
