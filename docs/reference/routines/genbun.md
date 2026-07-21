# GENBUN

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve by a cyclic reduction algorithm the linear system of equations that results from a finite difference approximation to certain 2-d elliptic PDE's on a centered grid .

## Description

Subroutine GENBUN solves the linear system of equations

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

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [GENBUN](https://www.netlib.org/slatec/fishfft/genbun.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the values that X(I,0) and X(I,N+1) are assumed to have. = 0  If X(I,0) = X(I,N) and X(I,N+1) = X(I,1). = 1  If X(I,0) = X(I,N+1) = 0  . |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1), or 1), or 1), or X(I,1) depending on an input parameter. X(I,1) depending on an input parameter. X(I,1) depending on an input parameter. * * * * * * *    Parameter Description     * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * 1). 1). 1). 1). = 4  If X(I,0) = X(I,2) and X(I,N+1) = 0. = 4  If X(I,0) = X(I,2) and X(I,N+1) = 0. The number of unknowns in the J-direction.  N must be greater than 2. |
| 3 | `MPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0 the array elements must not depend upon the index I, but must be constant.  Specifically, the subroutine checks the following condition 1 |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are not zero. = 1 if A(1) = C(M) = 0. The number of unknowns in the I-direction.  M must be greater than 2. C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine GENBUN was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of and N. N)    MPEROD    NPEROD    T(MSECS)    E |
| 5 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I = 1,2,...,M  and  J = 1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e., X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to are not zero. = 1 if A(1) = C(M) = 0. One-dimensional arrays of length M that specify the C(1) dimensional array that specifies the values of the right side of the linear system of equations given above.  Y must be dimensioned at least M*N. dimensional array that must be provided by the user for work space.  W may require up to 4*N + (10 + INT(log2(N)))*M locations.  The actual number of locations used is computed by GENBUN and is returned in location W(1). solution array X for the system given in the 'PURPOSE' with 0.5*B(I) = 1,       I=1,2,...,M C(M) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine GENBUN was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of |
| 6 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length M that specify the B(1) for I=1,2,...,M. |
| 7 | `C` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are not zero. = 1 if A(1) = C(M) = 0. One-dimensional arrays of length M that specify the C(1) 0.5*B(I) = 1,       I=1,2,...,M |
| 8 | `IDIMY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the two-dimensional array Y as it appears in the program calling GENBUN.  This parameter is must be at least M. |
| 9 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMY, *) | must be at least M. Contains the solution X. |
| 10 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters.  Except for number zero, a solution is not attempted. = 0  No error. = 1  M .LE. 2 = 2  N .LE. 2 = 3  IDIMY .LT. M = 4  NPEROD .LT. 0 or NPEROD .GT. 4 = 5  MPEROD .LT. 0 or MPEROD .GT. 1 = 6  A(I) .NE. C(1) or C(I) .NE. C(1) or B(I) .NE. B(1) for some I=1,2,...,M. = 7  A(1) .NE. 0 or C(M) .NE. 0 and MPEROD = 1 |
| 11 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(M),B(M),C(M),Y(IDIMY,N),W(see parameter list) |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(M),B(M),C(M),Y(IDIMY,N),W(see parameter list)

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::genbun`. Native symbol: `genbun_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::genbun`
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
