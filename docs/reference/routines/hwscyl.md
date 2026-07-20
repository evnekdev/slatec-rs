# HWSCYL

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a standard finite difference approximation to the Helmholtz equation in cylindrical coordinates.

## Description

Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of R, i.e., A .LE. R .LE. B. A must be less than B and A must be non-negative. M The number of panels into which the interval (A,B) is subdivided. Hence, there will be M+1 grid points in the R-direction given by R(I) = A+(I-1)DR, for I = 1,2,...,M+1, where DR = (B-A)/M is the panel width. M must be greater than 3. MBDCND Indicates the type of boundary conditions at R = A and R = B. = 1 If the solution is specified at R = A and R = B. = 2 If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. = 3 If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4 If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5 If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6 If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE: If A = 0, do not use MBDCND = 3 or 4, but instead use MBDCND = 1,2,5, or 6 . BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. When MBDCND = 3 or 4, BDA(J) = (d/dR)U(A,Z(J)), J = 1,2,...,N+1 . When MBDCND has any other value, BDA is a dummy variable. BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. When MBDCND = 2,3, or 6, BDB(J) = (d/dR)U(B,Z(J)), J = 1,2,...,N+1 . When MBDCND has any other value, BDB is a dummy variable. C,D The range of Z, i.e., C .LE. Z .LE. D. C must be less than D. N The number of panels into which the interval (C,D) is subdivided. Hence, there will be N+1 grid points in the Z-direction given by Z(J) = C+(J-1)DZ, for J = 1,2,...,N+1, where DZ = (D-C)/N is the panel width. N must be greater than 3. NBDCND Indicates the type of boundary conditions at Z = C and Z = D. = 0 If the solution is periodic in Z, i.e., U(I,1) = U(I,N+1). = 1 If the solution is specified at Z = C and Z = D. = 2 If the solution is specified at Z = C and the derivative of the solution with respect to Z is specified at Z = D. = 3 If the derivative of the solution with respect to Z is specified at Z = C and Z = D. = 4 If the derivative of the solution with respect to Z is specified at Z = C and the solution is specified at Z = D. BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = C. When NBDCND = 3 or 4, BDC(I) = (d/dZ)U(R(I),C), I = 1,2,...,M+1 . When NBDCND has any other value, BDC is a dummy variable. BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = D. When NBDCND = 2 or 3, BDD(I) = (d/dZ)U(R(I),D), I = 1,2,...,M+1 . When NBDCND has any other value, BDD is a dummy variable. ELMBDA The constant LAMBDA in the Helmholtz equation. If LAMBDA .GT. 0, a solution may not exist. However, HWSCYL will attempt to find a solution. LAMBDA must be zero when MBDCND = 5 or 6 . F A two-dimensional array that specifies the values of the right side of the Helmholtz equation and boundary data (if any). For I = 2,3,...,M and J = 2,3,...,N F(I,J) = F(R(I),Z(J)). On the boundaries F is defined by MBDCND F(1,J) F(M+1,J) ------ --------- --------- 1 U(A,Z(J)) U(B,Z(J)) 2 U(A,Z(J)) F(B,Z(J)) 3 F(A,Z(J)) F(B,Z(J)) J = 1,2,...,N+1 4 F(A,Z(J)) U(B,Z(J)) 5 F(0,Z(J)) U(B,Z(J)) 6 F(0,Z(J)) F(B,Z(J)) NBDCND F(I,1) F(I,N+1) ------ --------- --------- 0 F(R(I),C) F(R(I),C) 1 U(R(I),C) U(R(I),D) 2 U(R(I),C) F(R(I),D) I = 1,2,...,M+1 3 F(R(I),C) F(R(I),D) 4 F(R(I),C) U(R(I),D) F must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSCYL. This parameter is used to specify the variable dimension of F. IDIMF must be at least M+1 . W A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations. The actual number of locations used is computed by HWSCYL and is returned in location W(1). * * * * * * On Output * * * * * * F Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),Z(J)), I = 1,2,...,M+1, J = 1,2,...,N+1 . PERTRB If one specifies a combination of periodic, derivative, and unspecified boundary conditions for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HWSCYL then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution. Hence, the solution is not unique. The value of PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. IERROR An error flag which indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. = 0 No error. = 1 A .LT. 0 . = 2 A .GE. B. = 3 MBDCND .LT. 1 or MBDCND .GT. 6 . = 4 C .GE. D. = 5 N .LE. 3 = 6 NBDCND .LT. 0 or NBDCND .GT. 4 . = 7 A = 0, MBDCND = 3 or 4 . = 8 A .GT. 0, MBDCND .GE. 5 . = 9 A = 0, LAMBDA .NE. 0, MBDCND .GE. 5 . = 10 IDIMF .LT. M+1 . = 11 LAMBDA .GT. 0 . = 12 M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSCYL, the user should test IERROR after the call. W W(1) contains the required length of W. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension of BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments W(see argument list) Latest June 1, 1976 Revision Subprograms HWSCYL,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required TRIX,TRI3,PIMACH Special NONE Conditions Common NONE Blocks I/O NONE Precision Single Specialist Roland Sweet Language FORTRAN History Standardized September 1, 1973 Revised April 1, 1976 Algorithm The routine defines the finite difference equations, incorporates boundary data, and adjusts the right side of singular systems and then calls GENBUN to solve the system. Space 5818(decimal) = 13272(octal) locations on the NCAR Required Control Data 7600 Timing and The execution time T on the NCAR Control Data Accuracy 7600 for subroutine HWSCYL is roughly proportional to M*N*log2(N), but also depends on the input parameters NBDCND and MBDCND. Some typical values are listed in the table below. The solution process employed results in a loss of no more than three significant digits for N and M as large as 64. More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. M(=N) MBDCND NBDCND T(MSECS) ----- ------ ------ -------- 32 1 0 31 32 1 1 23 32 3 3 36 64 1 0 128 64 1 1 96 64 3 3 142 Portability American National Standards Institute FORTRAN. The machine dependent constant PI is defined in function PIMACH. Required COS Resident Routines Reference Swarztrauber, P. and R. Sweet, 'Efficient FORTRAN Subprograms for the Solution of Elliptic Equations' NCAR TN/IA-109, July, 1975, 138 pp. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

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
- GAMS classifications: `I2B1A1A`
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

- Canonical provider: `fishfft/hwscyl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hwscyl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hwscyl.f) — `verified_cached`
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
| `A` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. | Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of R, i.e., A .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | M The number of panels into which the interval (A,B) is subdivided. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MBDCND` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | MBDCND Indicates the type of boundary conditions at R = A and R = B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. | BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | C,D The range of Z, i.e., C .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. | Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBDCND` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | NBDCND Indicates the type of boundary conditions at Z = C and Z = D. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDC` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = C. | BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = C. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDD` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = D. | BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = D. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELMBDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | ELMBDA The constant LAMBDA in the Helmholtz equation. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (IDIMF, *) | Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. | Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDIMF` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSCYL. | IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSCYL. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PERTRB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | PERTRB If one specifies a combination of periodic, derivative, and unspecified boundary conditions for a Poisson equation (LAMBDA = 0), a solution may not exist. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IERROR An error flag which indicates invalid input parameters. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | W A one-dimensional array that must be provided by the user for work space. | W A one-dimensional array that must be provided by the user for work space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::pde::fishpack::hwscyl`. Native symbol: `hwscyl_`. Feature: `fishpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hwscyl`
- Compatibility aliases: `slatec_sys::pde::fishpack::numerical::hwscyl`
- Public declaration feature: `fishpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
