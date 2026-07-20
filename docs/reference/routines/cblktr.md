# CBLKTR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a block tridiagonal system of linear equations (usually resulting from the discretization of separable two-dimensional elliptic equations).

## Description

Subroutine CBLKTR is a complex version of subroutine BLKTRI. Both subroutines solve a system of linear equations of the form AN(J)*X(I,J-1) + AM(I)*X(I-1,J) + (BN(J)+BM(I))*X(I,J) + CN(J)*X(I,J+1) + CM(I)*X(I+1,J) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N), X(I,N+1) = X(I,1), X(0,J) = X(M,J), X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations. Boundary conditions may be Dirichlet, Neumann, or periodic. * * * * * * * * * * On INPUT * * * * * * * * * * IFLG = 0 Initialization only. Certain quantities that depend on NP, N, AN, BN, and CN are computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. NP = 0 If AN(1) and CN(N) are not zero, which corresponds to periodic boundary conditions. = 1 If AN(1) and CN(N) are zero. N The number of unknowns in the J-direction. N must be greater than 4. The operation count is proportional to MNlog2(N), hence N should be selected less than or equal to M. AN,BN,CN Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. MP = 0 If AM(1) and CM(M) are not zero, which corresponds to periodic boundary conditions. = 1 If AM(1) = CM(M) = 0 . M The number of unknowns in the I-direction. M must be greater than 4. AM,BM,CM Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. IDIMY The row (or first) dimension of the two-dimensional array Y as it appears in the program calling BLKTRI. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M. Y A complex two-dimensional array that specifies the values of the right side of the linear system of equations given above. Y must be dimensioned Y(IDIMY,N) with IDIMY .GE. M. W A one-dimensional array that must be provided by the user for work space. If NP=1 define K=INT(log2(N))+1 and set L=2**(K+1) then W must have dimension (K-2)*L+K+5+MAX(2N,12M) If NP=0 define K=INT(log2(N-1))+1 and set L=2**(K+1) then W must have dimension (K-2)*L+K+5+2N+MAX(2N,12M) **IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. * * * * * * * * * * On Output * * * * * * * * * * Y Contains the solution X. IERROR An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error. = 1 M is less than 5. = 2 N is less than 5. = 3 IDIMY is less than M. = 4 BLKTRI failed while computing results that depend on the coefficient arrays AN, BN, CN. Check these arrays. = 5 AN(J)*CN(J-1) is less than 0 for some J. Possible reasons for this condition are 1. The arrays AN and CN are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. W Contains intermediate values that must not be destroyed if CBLKTR will be called again with IFLG=1. W(1) contains the number of locations required by W in floating point format. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension of AN(N),BN(N),CN(N),AM(M),BM(M),CM(M),Y(IDIMY,N) Arguments W(see argument list) Latest June 1979 Revision Required CBLKTR,CBLKT1,PROC,PROCP,CPROC,CPROCP,CCMPB,INXCA, Subprograms INXCB,INXCC,CPADD,PGSF,PPGSF,PPPSF,BCRH,TEVLC, R1MACH Special The algorithm may fail if ABS(BM(I)+BN(J)) is less Conditions than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if AN(J)*CN(J-1) is less than zero for some J. See the description of the output parameter IERROR. Common CCBLK Blocks I/O NONE Precision Single Specialist Paul Swarztrauber Language FORTRAN History CBLKTR is a complex version of BLKTRI (version 3) Algorithm Generalized Cyclic Reduction (see reference below) Space Required CONTROL DATA 7600 Portability American National Standards Institute FORTRAN. The machine accuracy is set using function R1MACH. Required NONE Resident Routines

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

- Canonical provider: `fishfft/cblktr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cblktr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cblktr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_c_automated_public`
- Canonical Rust path: `slatec_sys::pde::fishpack::complex::cblktr`
- Current legacy Rust paths: `none`
- Public declaration feature: `batch-c-fishpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `representative_batch_smoke_only`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
