# HWSPLR

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a finite difference approximation to the Helmholtz equation in polar coordinates.

## Description

Subroutine HWSPLR solves a finite difference approximation to the Helmholtz equation in polar coordinates:

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

- Canonical provider: `fishfft/hwsplr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hwsplr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hwsplr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [HWSPLR](https://www.netlib.org/slatec/fishfft/hwsplr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | must be less than B must be less than B negative. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = C.  When NBDCND = 3 or 4, dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = D.  When NBDCND = 2 or 3, dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M and J = 2,3,...,N 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) dimensional array that must be provided by the user for work space.  W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations.  The actual number of locations used is computed by HWSPLR and is returned in location is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSPLR then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  PERTRB should be small compared to the right side. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. |
| 2 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | must be less than B A)/M is the panel width. M must be greater than 3. 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (A,B) is subdivided.  Hence, there will be M+1 grid points in the R-direction given by R(I) = A+(I-1)DR, for I = 1,2,...,M+1, as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) |
| 4 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary condition at R = A and R = B. = 1  If the solution is specified at R = A and R = B. = 2  If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. = 3  If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4  If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5  If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6  If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE:  If A = 0, do not use MBDCND = 3 or 4, but instead use 1,2,5, or 6  . 3 or 4, is a dummy variable. 2,3, or 6, is a dummy variable. F(1,J)            F(M+1,J) |
| 5 | `BDA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (d/dR)U(A,THETA(J)), J = 1,2,...,N+1  . is a dummy variable. |
| 6 | `BDB` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (d/dR)U(B,THETA(J)), J = 1,2,...,N+1  . is a dummy variable. |
| 7 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | must be less must be less than D. than D. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) |
| 8 | `D` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA). * * * * * * *    Parameter Description     * * * * * * * * * * must be less than D. C)/N is the panel width.  N must be greater than 3. 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) |
| 9 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (C,D) is subdivided.  Hence, there will be N+1 grid points in the THETA-direction given by THETA(J) = C+(J-1)DTHETA for C)/N is the panel width.  N must be greater than 3. |
| 10 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at THETA = C and at THETA = D. = 0  If the solution is periodic in THETA, i.e., U(I,J) = U(I,N+J). = 1  If the solution is specified at THETA = C and THETA = D (see note below). = 2  If the solution is specified at THETA = C and the derivative of the solution with respect to THETA is specified at THETA = D (see note below). = 4  If the derivative of the solution with respect to THETA is specified at THETA = C and the solution is specified at THETA = D (see note below). NOTE:  When NBDCND = 1,2, or 4, do not use MBDCND = 5 or 6 (the former indicates that the solution is specified at R = 0, the latter indicates the solution is unspecified at R = 0).  Use instead MBDCND = 1 or 2  . is a dummy variable. is a dummy variable. F(I,1)            F(I,N+1) |
| 11 | `BDC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (d/dTHETA)U(R(I),C), I = 1,2,...,M+1  . is a dummy variable. |
| 12 | `BDD` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (d/dTHETA)U(R(I),D), I = 1,2,...,M+1  . is a dummy variable. |
| 13 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the Helmholtz equation.  If LAMBDA .LT. 0, a solution may not exist.  However, HWSPLR will attempt to find a solution. |
| 14 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | F(R(I),THETA(J)). On the boundaries F is defined by 1,2,...,N+1 5      F(0,0)            U(B,THETA(J)) 6      F(0,0)            F(B,THETA(J)) 1,2,...,M+1 3      F(R(I),C)         F(R(I),D) 4      F(R(I),C)         U(R(I),D) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)), I = 1,2,...,M+1, J = 1,2,...,N+1  . |
| 15 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the array F as it appears in the program calling HWSPLR.  This parameter is used to specify the must be at least M+1  . |
| 16 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation is a constant, calculated and subtracted from F, which ensures that a solution exists.  HWSPLR then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution.  Hence, the solution is not unique.  PERTRB should be small compared to the right side. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. |
| 17 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters.  Except for numbers 0 and 11, a solution is not attempted. =  0  No error. =  1  A .LT. 0  . =  2  A .GE. B. =  3  MBDCND .LT. 1 or MBDCND .GT. 6  . =  4  C .GE. D. =  5  N .LE. 3 =  6  NBDCND .LT. 0 or .GT. 4  . =  7  A = 0, MBDCND = 3 or 4  . =  8  A .GT. 0, MBDCND .GE. 5  . =  9  MBDCND .GE. 5, NBDCND .NE. 0 and NBDCND .NE. 3  . = 10  IDIMF .LT. M+1  . = 11  LAMBDA .GT. 0  . = 12  M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSPLR, the user should test IERROR after the call. |
| 18 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSPLR,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        None Conditions Common         NONE Blocks I/O Precision      Single Specialist     Roland Sweet Language       FORTRAN |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments      W(see argument list) Latest         June 1, 1976 Revision Subprograms    HWSPLR,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required       TRIX,TRI3,PIMACH Special        None Conditions Common         NONE Blocks I/O Precision      Single Specialist     Roland Sweet Language       FORTRAN

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hwsplr`. Native symbol: `hwsplr_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hwsplr`
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
