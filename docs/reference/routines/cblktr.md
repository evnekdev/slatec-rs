# CBLKTR

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a block tridiagonal system of linear equations (usually resulting from the discretization of separable two-dimensional elliptic equations).

## Description

Subroutine CBLKTR is a complex version of subroutine BLKTRI. Both subroutines solve a system of linear equations of the form

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

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [CBLKTR](https://www.netlib.org/slatec/fishfft/cblktr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IFLG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = 0  Initialization only.  Certain quantities that depend on NP, 0 takes approximately one half the time 1.  However, the initialization does not have to be repeated unless NP, N, 1.  W(1) contains the number of locations required by W in floating point format. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   AN(N),BN(N),CN(N),AM(M),BM(M),CM(M),Y(IDIMY,N) Arguments      W(see argument list) Latest         June 1979 Revision Required       CBLKTR,CBLKT1,PROC,PROCP,CPROC,CPROCP,CCMPB,INXCA, Subprograms    INXCB,INXCC,CPADD,PGSF,PPGSF,PPPSF,BCRH,TEVLC, R1MACH |
| 2 | `NP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 define K=INT(log2(N))+1 and set L=2**(K+1) then 1))+1 and set L=2**(K+1) then |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. The number of unknowns in the J-direction. N must be greater than 4. The operation count is proportional to MNlog2(N), hence should be selected less than or equal to M. 1))+1 and set L=2**(K+1) then |
| 4 | `AN` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1) + AM(I)*X(I-1,J) + (BN(J)+BM(I))*X(I,J) are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). BN, or CN change. are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. error flag that indicates invalid input parameters.  Except for number zero, a solution is not attempted. = 0  No error. = 1  M is less than 5. = 2  N is less than 5. = 3  IDIMY is less than M. = 4  BLKTRI failed while computing results that depend on the coefficient arrays AN, BN, CN.  Check these arrays. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN |
| 5 | `BN` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. is less Conditions     than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if |
| 6 | `CN` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Y(I,J) For I = 1,2,...,M  and  J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N),  X(I,N+1) = X(I,1), X(0,J) = X(M,J),  X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations.  Boundary conditions may be Dirichlet, Neumann, or periodic. are computed and stored in the work array  W. = 1  The quantities that were computed in the initialization are used to obtain the solution X(I,J). are not zero, which corresponds to periodic boundary conditions. are zero. Real one-dimensional arrays of length N that specify the coefficients in the linear equations given above. 1) is less than 0 for some J. Possible reasons for this condition are are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. 1) is less than zero for some J. See the description of the output parameter IERROR. Common         CCBLK Blocks I/O            NONE Precision      Single Specialist     Paul Swarztrauber Language       FORTRAN |
| 7 | `MP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input M-direction coupling selector. `MP=0` requests the periodic coefficient case and requires the endpoint off-diagonal coefficients to be nonzero; `MP=1` selects the noncyclic endpoint case where those endpoint coefficients are zero. |
| 8 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . The number of unknowns in the I-direction. M must be greater than 4. |
| 9 | `AM` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. |
| 10 | `BM` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. is less Conditions     than ABS(AM(I))+ABS(AN(J))+ABS(CM(I))+ABS(CN(J)) for some I and J. The algorithm will also fail if |
| 11 | `CM` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Y(I,J) For I = 1,2,...,M  and  J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N),  X(I,N+1) = X(I,1), X(0,J) = X(M,J),  X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations.  Boundary conditions may be Dirichlet, Neumann, or periodic. are not zero, which corresponds to periodic boundary conditions. = 1  If AM(1) = CM(M) = 0  . Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above. |
| 12 | `IDIMY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the two-dimensional array Y as it appears in the program calling BLKTRI.  This parameter is must be at least M. |
| 13 | `Y` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (IDIMY, *) | must be at least M. A complex two-dimensional array that specifies the values of the right side of the linear system of equations given above. must be dimensioned Y(IDIMY,N) with IDIMY .GE. M. Contains the solution X. |
| 14 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable status output. `0` means success; `1` means `M < 5`; `2` means `N < 5`; `3` means `IDIMY < M`; `4` reports a coefficient-array failure; and `5` reports an invalid negative product in the tridiagonal coefficient condition. Except for `0`, no solution is produced. |
| 15 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space. 2)*L+K+5+MAX(2N,12M) 2)*L+K+5+2N+MAX(2N,12M) IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: A one-dimensional array that must be provided by the user for work space. 2)*L+K+5+MAX(2N,12M) 2)*L+K+5+2N+MAX(2N,12M) IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::complex::cblktr`. Native symbol: `cblktr_`. Declaration feature: `fishpack-complex`. Provider feature: `fishpack-complex`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank2,mut_i32,mut_f32_array_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::complex::cblktr`
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
