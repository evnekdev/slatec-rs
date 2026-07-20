# GENBUN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve by a cyclic reduction algorithm the linear system of equations that results from a finite difference approximation to certain 2-d elliptic PDE's on a centered grid .

## Description

Subroutine GENBUN solves the linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I = 1,2,...,M and J = 1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e., X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to 0, X(I,2), or X(I,N) and X(I,N+1) may be equal to 0, X(I,N-1), or X(I,1) depending on an input parameter. * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * NPEROD Indicates the values that X(I,0) and X(I,N+1) are assumed to have. = 0 If X(I,0) = X(I,N) and X(I,N+1) = X(I,1). = 1 If X(I,0) = X(I,N+1) = 0 . = 2 If X(I,0) = 0 and X(I,N+1) = X(I,N-1). = 3 If X(I,0) = X(I,2) and X(I,N+1) = X(I,N-1). = 4 If X(I,0) = X(I,2) and X(I,N+1) = 0. N The number of unknowns in the J-direction. N must be greater than 2. MPEROD = 0 if A(1) and C(M) are not zero. = 1 if A(1) = C(M) = 0. M The number of unknowns in the I-direction. M must be greater than 2. A,B,C One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition A(I) = C(1) C(I) = C(1) B(I) = B(1) for I=1,2,...,M. IDIMY The row (or first) dimension of the two-dimensional array Y as it appears in the program calling GENBUN. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M. Y A two-dimensional array that specifies the values of the right side of the linear system of equations given above. Y must be dimensioned at least M*N. W A one-dimensional array that must be provided by the user for work space. W may require up to 4*N + (10 + INT(log2(N)))*M locations. The actual number of locations used is computed by GENBUN and is returned in location W(1). * * * * * * On Output * * * * * * Y Contains the solution X. IERROR An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error. = 1 M .LE. 2 = 2 N .LE. 2 = 3 IDIMY .LT. M = 4 NPEROD .LT. 0 or NPEROD .GT. 4 = 5 MPEROD .LT. 0 or MPEROD .GT. 1 = 6 A(I) .NE. C(1) or C(I) .NE. C(1) or B(I) .NE. B(1) for some I=1,2,...,M. = 7 A(1) .NE. 0 or C(M) .NE. 0 and MPEROD = 1 W W(1) contains the required length of W. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension of A(M),B(M),C(M),Y(IDIMY,N),W(see parameter list) Arguments Latest June 1, 1976 Revision Subprograms GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE,TRIX,TRI3, Required PIMACH Special NONE Conditions Common NONE Blocks I/O NONE Precision Single Specialist Roland Sweet Language FORTRAN History Standardized April 1, 1973 Revised August 20,1973 Revised January 1, 1976 Algorithm The linear system is solved by a cyclic reduction algorithm described in the reference. Space 4944(decimal) = 11520(octal) locations on the NCAR Required Control Data 7600. Timing and The execution time T on the NCAR Control Data Accuracy 7600 for subroutine GENBUN is roughly proportional to M*N*log2(N), but also depends on the input parameter NPEROD. Some typical values are listed in the table below. More comprehensive timing charts may be found in the reference. To measure the accuracy of the algorithm a uniform random number generator was used to create a solution array X for the system given in the 'PURPOSE' with A(I) = C(I) = -0.5*B(I) = 1, I=1,2,...,M and, when MPEROD = 1 A(1) = C(M) = 0 A(M) = C(1) = 2. The solution X was substituted into the given system and, using double precision, a right side Y was computed. Using this array Y subroutine GENBUN was called to produce an approximate solution Z. Then the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed. The value of E is given in the table below for some typical values of M and N. M (=N) MPEROD NPEROD T(MSECS) E ------ ------ ------ -------- ------ 31 0 0 36 6.E-14 31 1 1 21 4.E-13 31 1 3 41 3.E-13 32 0 0 29 9.E-14 32 1 1 32 3.E-13 32 1 3 48 1.E-13 33 0 0 36 9.E-14 33 1 1 30 4.E-13 33 1 3 34 1.E-13 63 0 0 150 1.E-13 63 1 1 91 1.E-12 63 1 3 173 2.E-13 64 0 0 122 1.E-13 64 1 1 128 1.E-12 64 1 3 199 6.E-13 65 0 0 143 2.E-13 65 1 1 120 1.E-12 65 1 3 138 4.E-13 Portability American National Standards Institute Fortran. The machine dependent constant PI is defined in function PIMACH. Required COS Resident Routines Reference Sweet, R., 'A Cyclic Reduction Algorithm For Solving Block Tridiagonal Systems Of Arbitrary Dimensions,' SIAM J. on Numer. Anal., 14(Sept., 1977), PP. 706-720. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `fishfft/genbun.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/genbun.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/genbun.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
