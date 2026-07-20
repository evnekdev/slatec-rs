# SPLP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve linear programming problems involving at most a few thousand constraints and variables. Takes advantage of sparsity in the constraint matrix.

## Description

These are the short usage instructions; for details about other features, options and methods for defining the matrix A, see the extended usage instructions which are contained in the Long Description section below. |------------| |Introduction| |------------| The subprogram SPLP( ) solves a linear optimization problem. The problem statement is as follows minimize (transpose of costs)*x subject to A*x=w. The entries of the unknowns x and w may have simple lower or upper bounds (or both), or be free to take on any value. By setting the bounds for x and w, the user is imposing the constraints of the problem. The matrix A has MRELAS rows and NVARS columns. The vectors costs, x, and w respectively have NVARS, NVARS, and MRELAS number of entries. The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). Only the nonzero entries of the matrix A are passed to SPLP( ). The output from the problem (when output flag INFO=1) includes optimal values for x and w in PRIMAL(*), optimal values for dual variables of the equations A*x=w and the simple bounds on x in DUALS(*), and the indices of the basic columns, IBASIS(*). |------------------------------| |Fortran Declarations Required:| |------------------------------| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL USRMAT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. The exact lengths will be determined by user-required options and data transferred to the subprogram USRMAT( ). The values of LW and LIW, the lengths of the arrays WORK(*) and IWORK(*), must satisfy the inequalities LW .GE. 4*NVARS+ 8*MRELAS+LAMAT+ LBM LIW.GE. NVARS+11*MRELAS+LAMAT+2*LBM It is an error if they do not both satisfy these inequalities. (The subprogram will inform the user of the required lengths if either LW or LIW is wrong.) The values of LAMAT and LBM nominally are LAMAT=4*NVARS+7 and LBM =8*MRELAS LAMAT determines the length of the sparse matrix storage area. The value of LBM determines the amount of storage available to decompose and update the active basis matrix. |------|

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Optimization and least squares`
- Mathematical domain: `optimization`
- Package provenance: `unknown`
- GAMS classifications: `G2A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::linear_programming::LinearProgram::<f32>::solve`

## Providers

- Canonical provider: `main-src/src/splp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/splp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/splp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/splp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::linear_programming::splp`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `slatec::linear_programming::LinearProgram::<f32>::solve`
- Exclusion or deferment reason: `pre-existing declaration remains deferred until the R1 source-hash, argument-documentation, and ABI review gate passes`
<!-- raw-api-status:end -->
