# POISTG

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a block tridiagonal system of linear equations that results from a staggered grid finite difference approximation to 2-D elliptic PDE's.

## Description

Subroutine POISTG solves the linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I=1,2,...,M and J=1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e. X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to X(I,1) or -X(I,1) and X(I,N+1) may be equal to X(I,N) or -X(I,N) depending on an input parameter. * * * * * * * Parameter Description * * * * * * * * * * * * * * * On Input * * * * * * NPEROD Indicates the values which X(I,0) and X(I,N+1) are assumed to have. = 1 If X(I,0) = -X(I,1) and X(I,N+1) = -X(I,N) = 2 If X(I,0) = -X(I,1) and X(I,N+1) = X(I,N) = 3 If X(I,0) = X(I,1) and X(I,N+1) = X(I,N) = 4 If X(I,0) = X(I,1) and X(I,N+1) = -X(I,N) N The number of unknowns in the J-direction. N must be greater than 2. MPEROD = 0 If A(1) and C(M) are not zero = 1 If A(1) = C(M) = 0 M The number of unknowns in the I-direction. M must be greater than 2. A,B,C One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend on the index I, but must be constant. Specifically, the subroutine checks the following condition A(I) = C(1) B(I) = B(1) C(I) = C(1) for I = 1, 2, ..., M. IDIMY The row (or first) dimension of the two-dimensional array Y as it appears in the program calling POISTG. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M. Y A two-dimensional array that specifies the values of the right side of the linear system of equations given above. Y must be dimensioned at least M X N. W A one-dimensional work array that must be provided by the user for work space. W may require up to 9M + 4N + M(INT(log2(N))) locations. The actual number of locations used is computed by POISTG and returned in location W(1). * * * * * On Output * * * * * * Y Contains the solution X.

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

- Canonical provider: `fishfft/poistg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/poistg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/poistg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [POISTG](https://www.netlib.org/slatec/fishfft/poistg.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `MPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `C` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `IDIMY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `Y` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMY, *) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 11 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

= 1  If M .LE. 2 = 2  If N .LE. 2 = 3  IDIMY .LT. M = 4  If NPEROD .LT. 1 or NPEROD .GT. 4 = 5  If MPEROD .LT. 0 or MPEROD .GT. 1 = 6  If MPEROD = 0 and A(I) .NE. C(1) or B(I) .NE. B(1) or C(I) .NE. C(1) for some I = 1, 2, ..., M. = 7 If MPEROD .EQ. 1 .AND. (A(1).NE.0 .OR. C(M).NE.0) W W(1) contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(M),B(M),C(M),Y(IDIMY,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    POISTG,POSTG2,COSGEN,MERGE,TRIX,TRI3,PIMACH Required Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of M and N. M (=N)    MPEROD    NPEROD    T(MSECS)    E ------    ------    ------    --------  ------ 31        0-1       1-4        45     9.E-13 31        1         1          21     4.E-13 31        1         3          41     3.E-13 32        0-1       1-4        51     3.E-12 32        1         1          32     3.E-13 32        1         3          48     1.E-13 33        0-1       1-4        42     1.E-12 33        1         1          30     4.E-13 33        1         3          34     1.E-13 63        0-1       1-4       186     3.E-12 63        1         1          91     1.E-12 63        1         3         173     2.E-13 64        0-1       1-4       209     4.E-12 64        1         1         128     1.E-12 64        1         3         199     6.E-13 65        0-1       1-4       143     2.E-13 65        1         1         160     1.E-11 65        1         3         138     4.E-13 Portability    American National Standards Institute FORTRAN. The machine dependent constant PI is defined in function PIMACH. Required       COS Resident Routines Reference      Schumann, U. and R. Sweet,'A Direct Method for the Solution of Poisson's Equation With Neumann Boundary Conditions on a Staggered Grid of Arbitrary Size,' J. Comp. Phys. 20(1976), pp. 171-182. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::poistg`. Native symbol: `poistg_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::poistg`
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
