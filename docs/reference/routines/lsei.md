# LSEI

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linearly constrained least squares problem with equality and inequality constraints, and optionally compute a covariance matrix.

## Description

This subprogram solves a linearly constrained least squares problem with both equality and inequality constraints, and, if the user requests, obtains a covariance matrix of the solution parameters. Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. This subroutine solves the linearly constrained least squares problem EX = F, (E ME by N) (equations to be exactly satisfied) AX = B, (A MA by N) (equations to be approximately satisfied, least squares sense) GX .GE. H,(G MG by N) (inequality constraints) The inequalities GX .GE. H mean that every component of the product GX must be .GE. the corresponding component of H. In case the equality constraints cannot be satisfied, a generalized inverse solution residual vector length is obtained for F-EX. This is the minimal length possible for F-EX.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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
- Safe Rust paths: `slatec::constrained_least_squares::solve_constrained_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/lsei.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/lsei.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/lsei.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/lsei.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [LSEI](https://www.netlib.org/slatec/src/lsei.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 2; dimensions (MDW, *) | where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are is doubly subscripted with is doubly subscripted with contains the N by N symmetric contains the N by N symmetric covariance matrix of the solution parameters, covariance matrix of the solution parameters, provided this was requested on input with provided this was requested on input with the option vector PRGOPT(*) and the output the option vector PRGOPT(*) and the output |
| 2 | `MDW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are is doubly subscripted with must satisfy MDW .GE. M.  The condition .LT. M is an error. The array W(*,*) contains the matrices and vectors (E  F) (A  B) (G  H) in rows and columns 1,...,M and 1,...,N+1 respectively. |
| 3 | `ME` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are permitted.  The rank of the matrix E is estimated during the computation.  We call this value KRANKE.  It is an output parameter in IP(1) defined below.  Using a generalized inverse solution of EX=F, a reduced least squares problem with inequality constraints is obtained. The tolerances used in these tests for determining the rank of E and the rank of the reduced least squares problem are given in Sandia Tech. Rept. SAND-78-1290.  They can be modified by the user if new values are provided in the option list of the array PRGOPT(*). The user must dimension all arrays appearing in the call list.. where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG.  Then are the respective matrix row dimensions of E, A and G.  Each matrix has N columns. |
| 4 | `MA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are permitted.  The rank of the matrix E is estimated during the computation.  We call this value KRANKE.  It is an output parameter in IP(1) defined below.  Using a generalized inverse solution of EX=F, a reduced least squares problem with inequality constraints is obtained. The tolerances used in these tests for determining the rank of E and the rank of the reduced least squares problem are given in Sandia Tech. Rept. SAND-78-1290.  They can be modified by the user if new values are provided in the option list of the array PRGOPT(*). The user must dimension all arrays appearing in the call list.. first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG.  Then are the respective matrix row dimensions of E, A and G.  Each matrix has N columns. |
| 5 | `MG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are permitted.  The rank of the matrix E is estimated during the computation.  We call this value KRANKE.  It is an output parameter in IP(1) defined below.  Using a generalized inverse solution of EX=F, a reduced least squares problem with inequality constraints is obtained. The tolerances used in these tests for determining the rank of E and the rank of the reduced least squares problem are given in Sandia Tech. Rept. SAND-78-1290.  They can be modified by the user if new values are provided in the option list of the array PRGOPT(*). The user must dimension all arrays appearing in the call list.. where K=MAX(MA+MG,N).  This allows for a solution of a range of where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual has been run, the output parameter IP(3) gives the actual dimension required for that problem. dimension required for that problem. The parameters for LSEI( ) are The parameters for LSEI( ) are first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG.  Then are the respective matrix row dimensions of E, A and G.  Each matrix has N columns. |
| 6 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | where K=MAX(MA+MG,N).  This allows for a solution of a range of where K=MAX(MA+MG,N).  This allows for a solution of a range of where K=MAX(MA+MG,N).  This allows for a solution of a range of where K=MAX(MA+MG,N).  This allows for a solution of a range of where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) problems in the given working space.  The dimension of WS(*) problems in the given working space.  The dimension of WS(*) problems in the given working space.  The dimension of WS(*) problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem given is a necessary overestimate.  Once a particular problem given is a necessary overestimate.  Once a particular problem given is a necessary overestimate.  Once a particular problem given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual has been run, the output parameter IP(3) gives the actual has been run, the output parameter IP(3) gives the actual has been run, the output parameter IP(3) gives the actual has been run, the output parameter IP(3) gives the actual dimension required for that problem. dimension required for that problem. dimension required for that problem. dimension required for that problem. dimension required for that problem. The parameters for LSEI( ) are The parameters for LSEI( ) are The parameters for LSEI( ) are The parameters for LSEI( ) are The parameters for LSEI( ) are first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG.  Then 1 (no more options to change) The contents of PRGOPT(*) are not modified by the subprogram. The options for WNNLS( ) can also be included in this array.  The values of KEY recognized by WNNLS( ) are 6, 7 and 8.  Their functions are documented in the usage instructions for subroutine WNNLS( ).  Normally these options do not need to be modified when using LSEI( ). |
| 7 | `PRGOPT` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are valued array is the option vector. If the user is satisfied with the nominal subprogram features set 1 (or PRGOPT(1)=1.0) Otherwise PRGOPT(*) is a linked list consisting of groups of data of the following form LINK KEY DATA SET The parameters LINK and KEY are each one word. The DATA SET can be comprised of several words. The number of items depends on the value of KEY. The value of LINK points to the first entry of the next group of data within The exception is when there are no more options to change.  In that case, LINK=1 and the values KEY and DATA SET are not referenced.  The general layout of is as follows. LINK1 (link to first entry of next group) KEY1 (key to the option change) data value (data value for this change) .       . .       . .       . LINK2 (link to the first entry of .                       next group) KEY2 (key to the option change) data value ...     . .       . .       . 1 (no more options to change) Values of LINK that are nonpositive are errors. A value of LINK .GT. NLINK=100000 is also an error. This helps prevent using invalid but positive values of LINK that will probably extend beyond the program limits of PRGOPT(*). Unrecognized values of KEY are ignored.  The order of the options is arbitrary and any number of options can be changed with the following restriction.  To prevent cycling in the processing of the option array, a count of the number of options changed is maintained. Whenever this count exceeds NOPT=1000, an error message is printed and the subprogram returns. Options.. KEY=1 Compute in W(*,*) the N by N covariance matrix of the solution variables as an output parameter.  Nominally the covariance matrix will not be computed. (This requires no user input.) The data set for this option is a single value. It must be nonzero when the covariance matrix is desired.  If it is zero, the covariance matrix is not computed.  When the covariance matrix is computed, the first dimensioning parameter of the array W(*,*) must satisfy MDW .GE. MAX(M,N). KEY=10 Suppress scaling of the inverse of the normal matrix by the scale factor RNORM**2/ MAX(1, no. of degrees of freedom).  This option only applies when the option for computing the covariance matrix (KEY=1) is used.  With KEY=1 and KEY=10 used as options the unscaled inverse of the normal matrix is returned in W(*,*). The data set for this option is a single value. When it is nonzero no scaling is done.  When it is zero scaling is done.  The nominal case is to do scaling so if option (KEY=1) is used alone, the matrix will be scaled on output. KEY=2 Scale the nonzero columns of the entire data matrix. (E) (A) (G) to have length one.  The data set for this option is a single value.  It must be nonzero if unit length column scaling is desired. KEY=3 Scale columns of the entire data matrix (E) (A) (G) with a user-provided diagonal matrix. The data set for this option consists of the N diagonal scaling factors, one for each matrix column. KEY=4 Change the rank determination tolerance for the equality constraint equations from the nominal value of SQRT(SRELPR).  This quantity can be no smaller than SRELPR, the arithmetic- storage precision.  The quantity SRELPR is the largest positive number such that T=1.+SRELPR satisfies T .EQ. 1.  The quantity used here is internally restricted to be at least SRELPR.  The data set for this option is the new tolerance. KEY=5 Change the rank determination tolerance for the reduced least squares equations from the nominal value of SQRT(SRELPR).  This quantity can be no smaller than SRELPR, the arithmetic- storage precision.  The quantity used here is internally restricted to be at least SRELPR.  The data set for this option is the new tolerance. For example, suppose we want to change the tolerance for the reduced least squares problem, compute the covariance matrix of the solution parameters, and provide column scaling for the data matrix.  For these options the dimension of PRGOPT(*) must be at least N+9.  The Fortran statements defining these options would be as follows: 4 (link to entry 4 in PRGOPT(*)) 1 (covariance matrix key) 1 (covariance matrix wanted) 7 (link to entry 7 in PRGOPT(*)) 5 (least squares equas.  tolerance key) ... (new value of the tolerance) N+9 (link to entry N+9 in PRGOPT(*)) provided column scaling key) CALL SCOPY (N, D, 1, PRGOPT(9), 1)  (Copy the N scaling factors from the user array D(*) PRGOPT(N+8)) 1 (no more options to change) The contents of PRGOPT(*) are not modified by the subprogram. The options for WNNLS( ) can also be included in this array.  The values of KEY recognized by WNNLS( ) are 6, 7 and 8.  Their functions are documented in the usage instructions for subroutine WNNLS( ).  Normally these options do not need to be modified when using LSEI( ). is not properly defined, or the lengths of the working arrays |
| 8 | `X` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are contains the solution parameters contains the solution parameters |
| 9 | `RNORME` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | contains the solution parameters |
| 10 | `RNORML` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | 0 or 1. The definition of MODE is given directly below. |
| 11 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0 or 1. The definition of MODE is given directly below. 0 or 1, RNORME and RNORML respectively contain the residual vector Euclidean lengths of F - EX and B - AX.  When 1 the equality constraint equations EX=F are contradictory, so RNORME .NE. 0.  The residual vector F-EX has minimal Euclidean length.  For .GE. 2, none of these parameters is defined. Integer flag that indicates the subprogram status after completion.  If MODE .GE. 2, no solution has been computed. 0  Both equality and inequality constraints are compatible and have been satisfied. 1  Equality constraints are contradictory. A generalized inverse solution of EX=F was used to minimize the residual vector length F-EX. In this sense, the solution is still meaningful. 2  Inequality constraints are contradictory. 3  Both equality and inequality constraints are contradictory. The following interpretation of 1,2 or 3 must be made.  The sets consisting of all solutions of the equality constraints EX=F and all vectors satisfying GX .GE. H have no points in common.  (In particular this does not say that each individual set has no points at all, although this could be the case.) 4  Usage error occurred.  The value of MDW is .LT. ME+MA+MG, MDW is .LT. N and a covariance matrix is requested, or the option vector 0 or 1. |
| 12 | `WS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are is an and IP(*), when specified in are respectively type real and type integer working arrays. Their required minimal lengths are given above. |
| 13 | `IP` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (3) | where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are The amounts of working storage actually allocated for the working arrays WS(*) and respectively.  These quantities are compared with the actual amounts of storage needed by LSEI( ).  Insufficient storage is an are not are not long enough. long enough. The integer working array has three entries that provide rank and working array length information after completion. rank of equality constraint matrix.  Define this quantity as KRANKE. rank of reduced least squares problem. the amount of storage in the working array WS(*) that was actually used by the subprogram. The formula given above for the length of WS(*) is a necessary overestimate. If exactly the same problem matrices are used in subsequent executions, the declared dimension of WS(*) can be reduced to this output value. User Designated Working Arrays.. are respectively type real and type integer working arrays. Their required minimal lengths are given above. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

because miscalculating the storage formulas for WS(*) and IP(*) might very well lead to subtle and hard-to-find execution errors. The length of WS(*) must be at least LW = 2*(ME+N)+K+(MG+2)*(N+7) where K = max(MA+MG,N) This test will not be made if IP(1).LE.0. The length of IP(*) must be at least LIP = MG+2*N+2 This test will not be made if IP(2).LE.0.

### Storage and workspace requirements

`W`: where K=MAX(MA+MG,N).  This allows for a solution of a range of problems in the given working space.  The dimension of WS(*) given is a necessary overestimate.  Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are is doubly subscripted with is doubly subscripted with contains the N by N symmetric contains the N by N symmetric covariance matrix of the solution parameters, covariance matrix of the solution parameters, provided this was requested on input with provided this was requested on input with the option vector PRGOPT(*) and the output the option vector PRGOPT(*) and the output

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::lsei`. Native symbol: `lsei_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::lsei`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::constrained_least_squares::solve_constrained_least_squares_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
