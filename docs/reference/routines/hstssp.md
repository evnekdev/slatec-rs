# HSTSSP

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in spherical coordinates and on the surface of the unit sphere (radius of 1).

## Description

HSTSSP solves the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in spherical coordinates and on the surface of the unit sphere (radius of 1)

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

- Canonical provider: `fishfft/hstssp.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hstssp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hstssp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [HSTSSP](https://www.netlib.org/slatec/fishfft/hstssp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i.e. A .LE. THETA .LE. B.  A negative.  A and B are in 0 corresponds to the north pole and B = PI corresponds to the south pole. * *  IMPORTANT  * * * dimensional array of length N that specifies the boundary values (if any) of the solution at THETA = A.  When dimensional array of length N that specifies the boundary values of the solution at THETA = B.  When MBDCND = 1,4, or 5, pole; the latter indicates the solution is unspecified).  Use specifies the boundary values of the solution at PHI = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at PHI = D.  When NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the Helmholtz equation.  For I=1,2,...,M and J=1,2,...,N dimensional array that must be provided by the user for work space.  W may require up to 13M + 4N + M*INT(log2(N)) locations.  The actual number of locations used is computed by HSTSSP and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTSSP then computes this that a solution exists.  HSTSSP then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. |
| 2 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i.e. A .LE. THETA .LE. B.  A negative.  A and B are in must be computed using the statement must be computed using the statement PIMACH(DUM) This insures that B in the user's program is equal to PI in this program which permits several tests of the input parameters that otherwise would not be possible. * * * * * * * * * * * A)/M.  M must be greater than 2. PI, do not use MBDCND = 2, 3, or 6, |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of grid points in the interval (A,B).  The grid points in the THETA-direction are given by THETA(I) = A + (I-0.5)DTHETA A)/M.  M must be greater than 2. specifies the boundary values of the solution at PHI = C.   When NBDCND = 1 or 2, J=1,2,...,N. are listed in the table below. The solution process employed results in a loss of no more than four significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine POISTG which is the routine that actually solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) |
| 4 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at THETA = A and THETA = B. = 1  If the solution is specified at THETA = A and THETA = B. (see note 3 below) = 2  If the solution is specified at THETA = A and the derivative of the solution with respect to THETA is specified at THETA = B (see notes 2 and 3 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = A (see notes 1, 2 below) and THETA = B. = 4  If the derivative of the solution with respect to THETA is specified at THETA = A (see notes 1 and 2 below) and the solution is specified at THETA = B. = 5  If the solution is unspecified at THETA = A = 0 and the solution is specified at THETA = B.  (see note 3 below) = 6  If the solution is unspecified at THETA = A = 0 and the derivative of the solution with respect to THETA is specified at THETA = B (see note 2 below). = 7  If the solution is specified at THETA = A and the solution is unspecified at THETA = B = PI. (see note 3 below) = 8  If the derivative of the solution with respect to THETA is specified at THETA = A (see note 1 below) and the solution is unspecified at THETA = B = PI. = 9  If the solution is unspecified at THETA = A = 0 and THETA = B = PI. NOTES:  1.  If A = 0, do not use MBDCND = 3, 4, or 8, 5, 6, or 9. 7, 8, or 9. 3.  When the solution is specified at THETA = 0 and/or THETA = PI and the other boundary conditions are combinations of unspecified, normal derivative, or periodicity a singular system results.  The unique solution is determined by extrapolation to the specification of the solution at either THETA = 0 or THETA = PI.  But in these cases the right side of the system will be perturbed by the constant PERTRB. 1, 2, or 7, 3, 4, or 8, is a dummy variable. 2,3, or 6, is a dummy variable. 1 or 2. |
| 5 | `BDA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(A,PHI(J)) ,              J=1,2,...,N. (d/dTHETA)U(A,PHI(J)) ,    J=1,2,...,N. is a dummy variable. |
| 6 | `BDB` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(B,PHI(J)) ,              J=1,2,...,N. (d/dTHETA)U(B,PHI(J)) ,    J=1,2,...,N. is a dummy variable. |
| 7 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of PHI (longitude), i.e. C .LE. PHI .LE. D. C = 2*PI, periodic boundary conditions are usually prescribed. |
| 8 | `D` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | + F(THETA,PHI) where THETA is colatitude and PHI is longitude. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * The range of PHI (longitude), i.e. C .LE. PHI .LE. D. C = 2*PI, periodic boundary C = 2*PI, periodic boundary conditions are usually prescribed. conditions are usually prescribed. C)/N.  N must be greater than 2. |
| 9 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the interval (C,D).  The unknowns in the PHI-direction are given by PHI(J) = C + (J-0.5)DPHI, C)/N.  N must be greater than 2. are listed in are listed in the table below. the table below. The solution process employed results in a loss The solution process employed results in a loss of no more than four significant digits for N and M of no more than four significant digits for N and M as large as 64.  More detailed information about as large as 64.  More detailed information about accuracy can be found in the documentation for accuracy can be found in the documentation for subroutine POISTG which is the routine that subroutine POISTG which is the routine that actually solves the finite difference equations. actually solves the finite difference equations. |
| 10 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at PHI = C and PHI = D. = 0  If the solution is periodic in PHI, i.e. U(I,J) = U(I,N+J). = 1  If the solution is specified at PHI = C and PHI = D (see note below). = 2  If the solution is specified at PHI = C and the derivative of the solution with respect to PHI is specified at PHI = D (see note below). = 3  If the derivative of the solution with respect to PHI is specified at PHI = C and PHI = D. = 4  If the derivative of the solution with respect to PHI is specified at PHI = C and the solution is specified at PHI = D (see note below). NOTE:  When NBDCND = 1, 2, or 4, do not use MBDCND = 5, 6, 7, 8, or 9 (the former indicates that the solution is specified at 3 or 4, 0, BDC is a dummy variable. 2 or 3, 0, BDD is a dummy variable. |
| 11 | `BDC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(THETA(I),C) ,              I=1,2,...,M. (d/dPHI)U(THETA(I),C),       I=1,2,...,M. |
| 12 | `BDD` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(THETA(I),D) ,              I=1,2,...,M. (d/dPHI)U(THETA(I),D) ,      I=1,2,...,M. |
| 13 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the Helmholtz equation.  If LAMBDA is greater than 0, a solution may not exist.  However, HSTSSP will attempt to find a solution. |
| 14 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | F(THETA(I),PHI(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),PHI(J)) for |
| 15 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the array F as it appears in the program calling HSTSSP.  This parameter is used to specify the must be at least M. |
| 16 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation stant, calculated and subtracted from F, which ensures that a solution exists.  HSTSSP then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that |
| 17 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for numbers 0 and 14, a solution is not attempted. =  0  No error =  1  A .LT. 0 or B .GT. PI =  2  A .GE. B =  3  MBDCND .LT. 1 or MBDCND .GT. 9 =  4  C .GE. D =  5  N .LE. 2 =  6  NBDCND .LT. 0 or NBDCND .GT. 4 =  7  A .GT. 0 and MBDCND = 5, 6, or 9 =  8  A = 0 and MBDCND = 3, 4, or 8 =  9  B .LT. PI and MBDCND .GE. 7 = 10  B = PI and MBDCND = 2,3, or 6 = 11  MBDCND .GE. 5 and NDBCND = 1, 2, or 4 = 12  IDIMF .LT. M = 13  M .LE. 2 = 14  LAMBDA .GT. 0 Since this is the only means of indicating a possibly incorrect call to HSTSSP, the user should test IERROR after the call. |
| 18 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    HSTSSP,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    HSTSSP,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hstssp`. Native symbol: `hstssp_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hstssp`
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
