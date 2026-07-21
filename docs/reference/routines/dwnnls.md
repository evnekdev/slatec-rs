# DWNNLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linearly constrained least squares problem with equality constraints and nonnegativity constraints on selected variables.

## Description

This subprogram solves a linearly constrained least squares problem. Suppose there are given matrices E and A of respective dimensions ME by N and MA by N, and vectors F and B of respective lengths ME and MA. This subroutine solves the problem EX = F, (equations to be exactly satisfied) AX = B, (equations to be approximately satisfied, in the least squares sense) subject to components L+1,...,N nonnegative Any values ME.GE.0, MA.GE.0 and 0.LE. L .LE.N are permitted. The problem is reposed as problem DWNNLS (WT*E)X = (WT*F) ( A) ( B), (least squares) subject to components L+1,...,N nonnegative. The subprogram chooses the heavy weight (or penalty parameter) WT. The parameters for DWNNLS are INPUT.. All TYPE REAL variables are DOUBLE PRECISION W(*,*),MDW, The array W(*,*) is double subscripted with first ME,MA,N,L dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW.GE.M. The condition MDW.LT.M

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1A2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::linear_least_squares::solve_nonnegative_least_squares`

## Providers

- Canonical provider: `main-src/src/dwnnls.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dwnnls.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dwnnls.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dwnnls.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DWNNLS](https://www.netlib.org/slatec/src/dwnnls.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (MDW, *) | Workspace argument classified by fixed-form executable read/write analysis. |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `ME` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `MA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `PRGOPT` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 9 | `RNORM` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `MODE` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 12 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The array W(*,*) contains the matrices and vectors (E  F) (A  B) in rows and columns 1,...,M and 1,...,N+1 respectively.  Columns 1,...,L correspond to unconstrained variables X(1),...,X(L).  The remaining variables are constrained to be nonnegative. The condition L.LT.0 or L.GT.N is PRGOPT(*)    This double precision array is the option vector. If the user is satisfied with the nominal subprogram features set PRGOPT(1)=1 (or PRGOPT(1)=1.0) Otherwise PRGOPT(*) is a linked list consisting of groups of data of the following form LINK KEY DATA SET The parameters LINK and KEY are each one word. The DATA SET can be comprised of several words. The number of items depends on the value of KEY. The value of LINK points to the first entry of the next group of data within PRGOPT(*).  The exception is when there are no more options to change.  In that case LINK=1 and the values KEY and DATA SET are not referenced. The general layout of PRGOPT(*) is as follows. ...PRGOPT(1)=LINK1 (link to first entry of next group) .  PRGOPT(2)=KEY1 (key to the option change) .  PRGOPT(3)=DATA VALUE (data value for this change) .       . .       . .       . ...PRGOPT(LINK1)=LINK2 (link to the first entry of .                       next group) .  PRGOPT(LINK1+1)=KEY2 (key to the option change) .  PRGOPT(LINK1+2)=DATA VALUE ...     . .       . .       . ...PRGOPT(LINK)=1 (no more options to change) This helps prevent using invalid but positive values of LINK that will probably extend beyond the program limits of PRGOPT(*). Unrecognized values of KEY are ignored.  The order of the options is arbitrary and any number of options can be changed with the following restriction.  To prevent cycling in the processing of the option array a count of the number of options changed is maintained. message is printed and the subprogram returns. OPTIONS.. KEY=6 Scale the nonzero columns of the entire data matrix (E) (A) to have length one. The DATA SET for this option is a single value.  It must be nonzero if unit length column scaling is desired. KEY=7 Scale columns of the entire data matrix (E) (A) with a user-provided diagonal matrix. The DATA SET for this option consists of the N diagonal scaling factors, one for each matrix column. KEY=8 Change the rank determination tolerance from the nominal value of SQRT(SRELPR).  This quantity can be no smaller than SRELPR, The arithmetic- storage precision.  The quantity used here is internally restricted to be at least SRELPR.  The DATA SET for this option is the new tolerance. KEY=9 Change the blow-up parameter from the nominal value of SQRT(SRELPR).  The reciprocal of this parameter is used in rejecting solution components as too large when a variable is first brought into the active set.  Too large means that the proposed component times the reciprocal of the parameter is not less than the ratio of the norms of the right-side vector and the data matrix. This parameter can be no smaller than SRELPR, the arithmetic-storage precision. For example, suppose we want to provide a diagonal matrix to scale the problem matrix and change the tolerance used for determining linear dependence of dropped col vectors.  For these options the dimensions of PRGOPT(*) must be at least N+6.  The FORTRAN statements defining these options would be as follows. PRGOPT(1)=N+3 (link to entry N+3 in PRGOPT(*)) PRGOPT(2)=7 (user-provided scaling key) CALL DCOPY(N,D,1,PRGOPT(3),1) (copy the N scaling factors from a user array called D(*) into PRGOPT(3)-PRGOPT(N+2)) PRGOPT(N+3)=N+6 (link to entry N+6 of PRGOPT(*)) PRGOPT(N+4)=8 (linear dependence tolerance key) PRGOPT(N+5)=... (new value of the tolerance) PRGOPT(N+6)=1 (no more options to change) IWORK(1),    The amounts of working storage actually allocated IWORK(2)     for the working arrays WORK(*) and IWORK(*), respectively.  These quantities are compared with the actual amounts of storage needed for DWNNLS( ). Insufficient storage allocated for either WORK(*) was included in DWNNLS( ) because miscalculating the storage formulas for WORK(*) and IWORK(*) might very well lead to subtle and hard-to-find The length of WORK(*) must be at least LW = ME+MA+5*N This test will not be made if IWORK(1).LE.0. The length of IWORK(*) must be at least LIW = ME+MA+N This test will not be made if IWORK(2).LE.0. OUTPUT.. All TYPE REAL variables are DOUBLE PRECISION X(*)         An array dimensioned at least N, which will contain the N components of the solution vector on output. RNORM        The residual norm of the solution.  The value of RNORM contains the residual vector length of the equality constraints and least squares equations. MODE         The value of MODE indicates the success or failure of the subprogram. MODE = 0  Subprogram completed successfully. = 1  Max. number of iterations (equal to 3*(N-L)) exceeded. Nearly all problems should complete in fewer than this number of iterations. An approximate solution and its corresponding residual vector length are in X(*) and RNORM. processing subprogram, XERMSG( ). User-designated Working arrays.. WORK(*)      A double precision working array of length at least M + 5*N. IWORK(*)     An integer-valued working array of length at least M+N.

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`WORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::dwnnls`. Native symbol: `dwnnls_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::dwnnls`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::linear_least_squares::solve_nonnegative_least_squares`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
