# POIS3D

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a three-dimensional block tridiagonal linear system which arises from a finite difference approximation to a three-dimensional Poisson equation using the Fourier transform package FFTPAK written by Paul Swarztrauber.

## Description

Subroutine POIS3D solves the linear system of equations C1*(X(I-1,J,K)-2.*X(I,J,K)+X(I+1,J,K)) + C2*(X(I,J-1,K)-2.*X(I,J,K)+X(I,J+1,K)) + A(K)*X(I,J,K-1)+B(K)*X(I,J,K)+C(K)*X(I,J,K+1) = F(I,J,K) for I=1,2,...,L , J=1,2,...,M , and K=1,2,...,N . The indices K-1 and K+1 are evaluated modulo N, i.e. X(I,J,0) = X(I,J,N) and X(I,J,N+1) = X(I,J,1). The unknowns X(0,J,K), X(L+1,J,K), X(I,0,K), and X(I,M+1,K) are assumed to take on certain prescribed values described below. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * LPEROD Indicates the values that X(0,J,K) and X(L+1,J,K) are assumed to have. = 0 If X(0,J,K) = X(L,J,K) and X(L+1,J,K) = X(1,J,K). = 1 If X(0,J,K) = X(L+1,J,K) = 0. = 2 If X(0,J,K) = 0 and X(L+1,J,K) = X(L-1,J,K). = 3 If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = X(L-1,J,K). = 4 If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. L The number of unknowns in the I-direction. L must be at least 3. C1 The real constant that appears in the above equation. MPEROD Indicates the values that X(I,0,K) and X(I,M+1,K) are assumed to have. = 0 If X(I,0,K) = X(I,M,K) and X(I,M+1,K) = X(I,1,K). = 1 If X(I,0,K) = X(I,M+1,K) = 0. = 2 If X(I,0,K) = 0 and X(I,M+1,K) = X(I,M-1,K). = 3 If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = X(I,M-1,K). = 4 If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. M The number of unknowns in the J-direction. M must be at least 3. C2 The real constant which appears in the above equation. NPEROD = 0 If A(1) and C(N) are not zero. = 1 If A(1) = C(N) = 0. N The number of unknowns in the K-direction. N must be at least 3. A,B,C One-dimensional arrays of length N that specify the coefficients in the linear equations given above. If NPEROD = 0 the array elements must not depend upon the index K, but must be constant. Specifically, the subroutine checks the following condition A(K) = C(1) C(K) = C(1) B(K) = B(1) for K=1,2,...,N. LDIMF The row (or first) dimension of the three-dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension of F. LDIMF must be at least L. MDIMF The column (or second) dimension of the three-dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension of F. MDIMF must be at least M. F A three-dimensional array that specifies the values of the right side of the linear system of equations given above. F must be dimensioned at least L x M x N. W A one-dimensional array that must be provided by the user for work space. The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)). * * * * * * On Output * * * * * * F Contains the solution X. IERROR An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error = 1 If LPEROD .LT. 0 or .GT. 4 = 2 If L .LT. 3 = 3 If MPEROD .LT. 0 or .GT. 4 = 4 If M .LT. 3 = 5 If NPEROD .LT. 0 or .GT. 1 = 6 If N .LT. 3 = 7 If LDIMF .LT. L = 8 If MDIMF .LT. M = 9 If A(K) .NE. C(1) or C(K) .NE. C(1) or B(I) .NE.B(1) for some K=1,2,...,N. = 10 If NPEROD = 1 and A(1) .NE. 0 or C(N) .NE. 0 Since this is the only means of indicating a possibly incorrect call to POIS3D, the user should test IERROR after the call. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension of A(N),B(N),C(N),F(LDIMF,MDIMF,N), Arguments W(see argument list) Latest December 1, 1978 Revision Subprograms POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1,RFFTB, Required RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF,COSQF1 COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI,CFFTI1, CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB,CFFTF, CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF,PIMACH, Special NONE Conditions Common NONE Blocks I/O NONE Precision Single Specialist Roland Sweet Language FORTRAN History Written by Roland Sweet at NCAR in July 1977 Algorithm This subroutine solves three-dimensional block tridiagonal linear systems arising from finite difference approximations to three-dimensional Poisson equations using the Fourier transform package FFTPAK written by Paul Swarztrauber. Space 6561(decimal) = 14641(octal) locations on the Required NCAR Control Data 7600 Timing and The execution time T on the NCAR Control Data Accuracy 7600 for subroutine POIS3D is roughly proportional to L*M*N*(log2(L)+log2(M)+5), but also depends on input parameters LPEROD and MPEROD. Some typical values are listed in the table below when NPEROD=0. To measure the accuracy of the algorithm a uniform random number generator was used to create a solution array X for the system given in the 'PURPOSE' with A(K) = C(K) = -0.5*B(K) = 1, K=1,2,...,N and, when NPEROD = 1 A(1) = C(N) = 0 A(N) = C(1) = 2. The solution X was substituted into the given system and, using double precision, a right side Y was computed. Using this array Y subroutine POIS3D was called to produce an approximate solution Z. Then the relative error, defined as E = MAX(ABS(Z(I,J,K)-X(I,J,K)))/MAX(ABS(X(I,J,K))) where the two maxima are taken over I=1,2,...,L, J=1,2,...,M and K=1,2,...,N, was computed. The value of E is given in the table below for some typical values of L,M and N. L(=M=N) LPEROD MPEROD T(MSECS) E ------ ------ ------ -------- ------ 16 0 0 272 1.E-13 15 1 1 287 4.E-13 17 3 3 338 2.E-13 32 0 0 1755 2.E-13 31 1 1 1894 2.E-12 33 3 3 2042 7.E-13 Portability American National Standards Institute FORTRAN. The machine dependent constant PI is defined in function PIMACH. Required COS,SIN,ATAN Resident Routines Reference NONE * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::differential_equations::pde::Pois3dProblem::solve`

## Providers

- Canonical provider: `fishfft/pois3d.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/pois3d.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/pois3d.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::pde::fishpack::pois3d`
- Current legacy Rust paths: `slatec_sys::fishpack_pois3d::pois3d`
- Public declaration feature: `raw-family-fishpack-pois3d`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::differential_equations::pde::Pois3dProblem::solve`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
