# HSTPLR

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in polar coordinates.

## Description

HSTPLR solves the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in polar coordinates (1/R)(d/DR)(R(dU/DR)) + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA) * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of R, i.e. A .LE. R .LE. B. A must be less than B and A must be non-negative. M The number of grid points in the interval (A,B). The grid points in the R-direction are given by R(I) = A + (I-0.5)DR for I=1,2,...,M where DR =(B-A)/M. M must be greater than 2. MBDCND Indicates the type of boundary conditions at R = A and R = B. = 1 If the solution is specified at R = A and R = B. = 2 If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. (see note 1 below) = 3 If the derivative of the solution with respect to R is specified at R = A (see note 2 below) and R = B. = 4 If the derivative of the solution with respect to R is specified at R = A (see note 2 below) and the solution is specified at R = B. = 5 If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6 If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE 1: If A = 0, MBDCND = 2, and NBDCND = 0 or 3, the system of equations to be solved is singular. The unique solution is determined by extrapolation to the specification of U(0,THETA(1)). But in this case the right side of the system will be perturbed by the constant PERTRB. NOTE 2: If A = 0, do not use MBDCND = 3 or 4, but instead use MBDCND = 1,2,5, or 6. BDA A one-dimensional array of length N that specifies the boundary values (if any) of the solution at R = A. When MBDCND = 1 or 2, BDA(J) = U(A,THETA(J)) , J=1,2,...,N. When MBDCND = 3 or 4, BDA(J) = (d/dR)U(A,THETA(J)) , J=1,2,...,N. When MBDCND = 5 or 6, BDA is a dummy variable. BDB A one-dimensional array of length N that specifies the boundary values of the solution at R = B. When MBDCND = 1,4, or 5, BDB(J) = U(B,THETA(J)) , J=1,2,...,N. When MBDCND = 2,3, or 6, BDB(J) = (d/dR)U(B,THETA(J)) , J=1,2,...,N. C,D The range of THETA, i.e. C .LE. THETA .LE. D. C must be less than D. N The number of unknowns in the interval (C,D). The unknowns in the THETA-direction are given by THETA(J) = C + (J-0.5)DT, J=1,2,...,N, where DT = (D-C)/N. N must be greater than 2. NBDCND Indicates the type of boundary conditions at THETA = C and THETA = D. = 0 If the solution is periodic in THETA, i.e. U(I,J) = U(I,N+J). = 1 If the solution is specified at THETA = C and THETA = D (see note below). = 2 If the solution is specified at THETA = C and the derivative of the solution with respect to THETA is specified at THETA = D (see note below). = 3 If the derivative of the solution with respect to THETA is specified at THETA = C and THETA = D. = 4 If the derivative of the solution with respect to THETA is specified at THETA = C and the solution is specified at THETA = d (see note below). NOTE: When NBDCND = 1, 2, or 4, do not use MBDCND = 5 or 6 (the former indicates that the solution is specified at R = 0; the latter indicates the solution is unspecified at R = 0). Use instead MBDCND = 1 or 2. BDC A one dimensional array of length M that specifies the boundary values of the solution at THETA = C. When NBDCND = 1 or 2, BDC(I) = U(R(I),C) , I=1,2,...,M. When NBDCND = 3 or 4, BDC(I) = (d/dTHETA)U(R(I),C), I=1,2,...,M. When NBDCND = 0, BDC is a dummy variable. BDD A one-dimensional array of length M that specifies the boundary values of the solution at THETA = D. When NBDCND = 1 or 4, BDD(I) = U(R(I),D) , I=1,2,...,M. When NBDCND = 2 or 3, BDD(I) = (d/dTHETA)U(R(I),D) , I=1,2,...,M. When NBDCND = 0, BDD is a dummy variable. ELMBDA The constant LAMBDA in the Helmholtz equation. If LAMBDA is greater than 0, a solution may not exist. However, HSTPLR will attempt to find a solution. F A two-dimensional array that specifies the values of the right side of the Helmholtz equation. For I=1,2,...,M and J=1,2,...,N F(I,J) = F(R(I),THETA(J)) . F must be dimensioned at least M X N. IDIMF The row (or first) dimension of the array F as it appears in the program calling HSTPLR. This parameter is used to specify the variable dimension of F. IDIMF must be at least M. W A one-dimensional array that must be provided by the user for work space. W may require up to 13M + 4N + M*INT(log2(N)) locations. The actual number of locations used is computed by HSTPLR and is returned in the location W(1). * * * * * On Output * * * * * * F Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)) for I=1,2,...,M, J=1,2,...,N. PERTRB If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a con- stant, calculated and subtracted from F, which ensures that a solution exists. HSTPLR then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution; hence, the solution is not unique. The value of PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained.

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

- Canonical provider: `fishfft/hstplr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hstplr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hstplr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [HSTPLR](https://www.netlib.org/slatec/fishfft/hstplr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `BDA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `BDB` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `D` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `BDC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 12 | `BDD` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 13 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 14 | `F` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | Array argument classified by fixed-form executable read/write analysis. |
| 15 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 16 | `PERTRB` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 17 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 18 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

=  1  A .LT. 0 =  2  A .GE. B =  3  MBDCND .LT. 1 or MBDCND .GT. 6 =  4  C .GE. D =  5  N .LE. 2 =  6  NBDCND .LT. 0 or NBDCND .GT. 4 =  7  A = 0 and MBDCND = 3 or 4 =  8  A .GT. 0 and MBDCND .GE. 5 =  9  MBDCND .GE. 5 and NBDCND .NE. 0 or 3 = 10  IDIMF .LT. M = 11  LAMBDA .GT. 0 = 12  M .LE. 2 Since this is the only means of indicating a possibly the call. W W(1) contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see ARGUMENT LIST) Latest         June 1, 1977 Revision Subprograms    HSTPLR,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hstplr`. Native symbol: `hstplr_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hstplr`
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
