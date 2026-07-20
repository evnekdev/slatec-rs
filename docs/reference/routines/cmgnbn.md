# CMGNBN

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a complex block tridiagonal linear system of equations by a cyclic reduction algorithm.

## Description

Subroutine CMGNBN solves the complex linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e., X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to 0, X(I,2), or X(I,N) and X(I,N+1) may be equal to 0, X(I,N-1), or X(I,1) depending on an input parameter. * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * NPEROD Indicates the values that X(I,0) and X(I,N+1) are assumed to have. = 0 If X(I,0) = X(I,N) and X(I,N+1) = X(I,1). = 1 If X(I,0) = X(I,N+1) = 0 . = 2 If X(I,0) = 0 and X(I,N+1) = X(I,N-1). = 3 If X(I,0) = X(I,2) and X(I,N+1) = X(I,N-1). = 4 If X(I,0) = X(I,2) and X(I,N+1) = 0. N The number of unknowns in the J-direction. N must be greater than 2. MPEROD = 0 If A(1) and C(M) are not zero = 1 If A(1) = C(M) = 0 M The number of unknowns in the I-direction. N must be greater than 2. A,B,C One-dimensional complex arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition A(I) = C(1) C(I) = C(1) B(I) = B(1) For I=1,2,...,M. IDIMY The row (or first) dimension of the two-dimensional array Y as it appears in the program calling CMGNBN. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M. Y A two-dimensional complex array that specifies the values of the right side of the linear system of equations given above. Y must be dimensioned at least M*N. W A one-dimensional complex array that must be provided by the user for work space. W may require up to 4*N + (10 + INT(log2(N)))*M LOCATIONS. The actual number of locations used is computed by CMGNBN and is returned in location W(1). * * * * * * On Output * * * * * * Y Contains the solution X. IERROR An error flag which indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error. = 1 M .LE. 2 = 2 N .LE. 2 = 3 IDIMY .LT. M = 4 NPEROD .LT. 0 or NPEROD .GT. 4 = 5 MPEROD .LT. 0 or MPEROD .GT. 1 = 6 A(I) .NE. C(1) or C(I) .NE. C(1) or B(I) .NE. B(1) for some I=1,2,...,M. = 7 A(1) .NE. 0 or C(M) .NE. 0 and MPEROD = 1 W W(1) contains the required length of W. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension of A(M),B(M),C(M),Y(IDIMY,N),W(see parameter list) Arguments Latest June 1979 Revision Subprograms CMGNBN,CMPOSD,CMPOSN,CMPOSP,CMPCSG,CMPMRG, Required CMPTRX,CMPTR3,PIMACH Special None Conditions Common None Blocks I/O None Precision Single Specialist Roland Sweet Language FORTRAN History Written by Roland Sweet at NCAR in June, 1977 Algorithm The linear system is solved by a cyclic reduction algorithm described in the reference. Space 4944(DECIMAL) = 11520(octal) locations on the NCAR Required Control Data 7600 Timing and The execution time T on the NCAR Control Data Accuracy 7600 for subroutine CMGNBN is roughly proportional to M*N*log2(N), but also depends on the input parameter NPEROD. Some typical values are listed in the table below. To measure the accuracy of the algorithm a uniform random number generator was used to create a solution array X for the system given in the 'PURPOSE' with A(I) = C(I) = -0.5*B(I) = 1, I=1,2,...,M and, when MPEROD = 1 A(1) = C(M) = 0 A(M) = C(1) = 2. The solution X was substituted into the given system and a right side Y was computed. Using this array Y subroutine CMGNBN was called to produce an approximate solution Z. Then the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed. The value of E is given in the table below for some typical values of M and N. M (=N) MPEROD NPEROD T(MSECS) E ------ ------ ------ -------- ------ 31 0 0 77 1.E-12 31 1 1 45 4.E-13 31 1 3 91 2.E-12 32 0 0 59 7.E-14 32 1 1 65 5.E-13 32 1 3 97 2.E-13 33 0 0 80 6.E-13 33 1 1 67 5.E-13 33 1 3 76 3.E-12 63 0 0 350 5.E-12 63 1 1 215 6.E-13 63 1 3 412 1.E-11 64 0 0 264 1.E-13 64 1 1 287 3.E-12 64 1 3 421 3.E-13 65 0 0 338 2.E-12 65 1 1 292 5.E-13 65 1 3 329 1.E-11 Portability American National Standards Institute Fortran. The machine dependent constant PI is defined in function PIMACH. Required COS Resident Routines Reference Sweet, R., 'A Cyclic Reduction Algorithm for Solving Block Tridiagonal Systems Of Arbitrary Dimensions,' SIAM J. on Numer. Anal., 14(SEPT., 1977), PP. 706-720. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `FISHPACK elliptic PDE solvers`
- Mathematical domain: `pde-integral-equations`
- Package provenance: `fishpack`
- GAMS classifications: `I2B4B`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fishfft/cmgnbn.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cmgnbn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cmgnbn.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `NPEROD` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * NPEROD Indicates the values that X(I,0) and X(I,N+1) are assumed to have. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Subroutine CMGNBN solves the complex linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MPEROD` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | MPEROD = 0 If A(1) and C(M) are not zero = 1 If A(1) = C(M) = 0 M The number of unknowns in the I-direction. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Subroutine CMGNBN solves the complex linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | Subroutine CMGNBN solves the complex linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | Subroutine CMGNBN solves the complex linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | Subroutine CMGNBN solves the complex linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDIMY` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IDIMY The row (or first) dimension of the two-dimensional array Y as it appears in the program calling CMGNBN. | IDIMY The row (or first) dimension of the two-dimensional array Y as it appears in the program calling CMGNBN. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 2; dimensions (IDIMY, *) | Subroutine CMGNBN solves the complex linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IERROR An error flag which indicates invalid input parameters. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | W A one-dimensional complex array that must be provided by the user for work space. | W A one-dimensional complex array that must be provided by the user for work space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::pde::fishpack::complex::cmgnbn`. Native symbol: `cmgnbn_`. Feature: `fishpack-complex`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank2,mut_i32,mut_complex32_array_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::complex::cmgnbn`
- Compatibility aliases: `none`
- Public declaration feature: `fishpack-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
