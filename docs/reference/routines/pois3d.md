# POIS3D

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a three-dimensional block tridiagonal linear system which arises from a finite difference approximation to a three-dimensional Poisson equation using the Fourier transform package FFTPAK written by Paul Swarztrauber.

## Description

Subroutine POIS3D solves the linear system of equations

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

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [POIS3D](https://www.netlib.org/slatec/fishfft/pois3d.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `LPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the values that X(0,J,K) and X(L+1,J,K) are assumed to have. = 0  If X(0,J,K) = X(L,J,K) and X(L+1,J,K) = X(1,J,K). = 1  If X(0,J,K) = X(L+1,J,K) = 0. |
| 2 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are assumed to take on certain prescribed values described below. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * 1,J,K). 1,J,K). 1,J,K). 1,J,K). = 4  If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. = 4  If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. direction. L must be at least 3. + INT((M+1)/2)). M=N)   LPEROD    MPEROD    T(MSECS)    E |
| 3 | `C1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | 1,J,K)-2.*X(I,J,K)+X(I+1,J,K)) The real constant that appears in the above equation. |
| 4 | `MPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the values that X(I,0,K) and X(I,M+1,K) are assumed to have. = 0  If X(I,0,K) = X(I,M,K) and X(I,M+1,K) = X(I,1,K). = 1  If X(I,0,K) = X(I,M+1,K) = 0. |
| 5 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are assumed to take on certain prescribed values described below. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * 1,K). 1,K). 1,K). 1,K). = 4  If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. = 4  If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. direction. M must be at least 3. and K=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of L,M and N. |
| 6 | `C2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | 1,K)-2.*X(I,J,K)+X(I,J+1,K)) The real constant which appears in the above equation. |
| 7 | `NPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0  If A(1) and C(N) are not zero. = 1  If A(1) = C(N) = 0. 0 the array elements must not depend upon the index K, but must be constant.  Specifically, the subroutine checks the following condition 0. To measure the accuracy of the algorithm a uniform random number generator was used to create 1 |
| 8 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | direction. N must be at least 3. C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POIS3D was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J,K)-X(I,J,K)))/MAX(ABS(X(I,J,K))) where the two maxima are taken over I=1,2,...,L, |
| 9 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional block tridiagonal linear system which arises from a finite difference approximation to a three-dimensional Poisson equation using the Fourier transform package FFTPAK written by Paul Swarztrauber. 1)+B(K)*X(I,J,K)+C(K)*X(I,J,K+1) = F(I,J,K) for  I=1,2,...,L , J=1,2,...,M , and K=1,2,...,N . The indices K-1 and K+1 are evaluated modulo N, i.e. X(I,J,0) = X(I,J,N) and X(I,J,N+1) = X(I,J,1). The unknowns dimensional arrays of length N that specify the coefficients in the linear equations given above. C(1) dimensional array that specifies the values of the right side of the linear system of equations given above.  F must be dimensioned at least L x M x N. dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + solution array X for the system given in the 'PURPOSE' with 0.5*B(K) = 1,       K=1,2,...,N C(N) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POIS3D was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J,K)-X(I,J,K)))/MAX(ABS(X(I,J,K))) where the two maxima are taken over I=1,2,...,L, |
| 10 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional arrays of length N that specify the coefficients in the linear equations given above. B(1) for K=1,2,...,N. |
| 11 | `C` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional arrays of length N that specify the coefficients in the linear equations given above. C(1) 0.5*B(K) = 1,       K=1,2,...,N |
| 12 | `LDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension must be at least L. |
| 13 | `MDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension must be at least M. |
| 14 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 3; dimensions (LDIMF, MDIMF, *) | must be at least L. must be at least M. dimensional array that specifies the values of the right side of the linear system of equations given above.  F must be dimensioned at least L x M x N. Contains the solution X. |
| 15 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0  No error = 1  If LPEROD .LT. 0 or .GT. 4 = 2  If L .LT. 3 = 3  If MPEROD .LT. 0 or .GT. 4 = 4  If M .LT. 3 = 5  If NPEROD .LT. 0 or .GT. 1 = 6  If N .LT. 3 = 7  If LDIMF .LT. L = 8  If MDIMF .LT. M = 9  If A(K) .NE. C(1) or C(K) .NE. C(1) or B(I) .NE.B(1) for some K=1,2,...,N. = 10 If NPEROD = 1 and A(1) .NE. 0 or C(N) .NE. 0 Since this is the only means of indicating a possibly incorrect call to POIS3D, the user should test IERROR after the call. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(N),B(N),C(N),F(LDIMF,MDIMF,N), Arguments      W(see argument list) Latest         December 1, 1978 Revision Subprograms    POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1,RFFTB, Required       RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF,COSQF1 COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI,CFFTI1, CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB,CFFTF, CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF,PIMACH, Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |
| 16 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) +

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::pois3d`. Native symbol: `pois3d_`. Declaration feature: `raw-family-fishpack-pois3d`. Provider feature: `fishpack-pois3d`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::pde::fishpack::pois3d`
- Public declaration feature: `raw-family-fishpack-pois3d`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::differential_equations::pde::Pois3dProblem::solve`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
