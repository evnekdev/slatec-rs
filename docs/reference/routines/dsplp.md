# DSPLP

[Family: Optimization and least squares](../families/optimization-and-least-squares.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve linear programming problems involving at most a few thousand constraints and variables. Takes advantage of sparsity in the constraint matrix.

## Description

These are the short usage instructions; for details about other features, options and methods for defining the matrix A, see the extended usage instructions which are contained in the Long Description section below. |------------| |Introduction| |------------| The subprogram DSPLP( ) solves a linear optimization problem. The problem statement is as follows minimize (transpose of costs)*x subject to A*x=w. The entries of the unknowns x and w may have simple lower or upper bounds (or both), or be free to take on any value. By setting the bounds for x and w, the user is imposing the constraints of the problem. The matrix A has MRELAS rows and NVARS columns. The vectors costs, x, and w respectively have NVARS, NVARS, and MRELAS number of entries. The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). Only the nonzero entries of the matrix A are passed to DSPLP( ). The output from the problem (when output flag INFO=1) includes optimal values for x and w in PRIMAL(*), optimal values for dual variables of the equations A*x=w and the simple bounds on x in DUALS(*), and the indices of the basic columns, IBASIS(*). |------------------------------| |Fortran Declarations Required:| |------------------------------| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. The exact lengths will be determined by user-required options and data transferred to the subprogram DUSRMT( ). The values of LW and LIW, the lengths of the arrays WORK(*) and IWORK(*), must satisfy the inequalities LW .GE. 4*NVARS+ 8*MRELAS+LAMAT+ LBM LIW.GE. NVARS+11*MRELAS+LAMAT+2*LBM It is an error if they do not both satisfy these inequalities. (The subprogram will inform the user of the required lengths if either LW or LIW is wrong.) The values of LAMAT and LBM nominally are LAMAT=4*NVARS+7 and LBM =8*MRELAS LAMAT determines the length of the sparse matrix storage area. The value of LBM determines the amount of storage available to decompose and update the active basis matrix. |------|

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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
- Safe Rust paths: `slatec::linear_programming::LinearProgram::<f64>::solve`

## Providers

- Canonical provider: `main-src/src/dsplp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dsplp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dsplp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dsplp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Optimization and least squares](../families/optimization-and-least-squares.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `DUSRMT` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MRELAS` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The matrix A has MRELAS rows and NVARS columns. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NVARS` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The matrix A has MRELAS rows and NVARS columns. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `COSTS` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The problem statement is as follows minimize (transpose of costs)*x subject to A*x=w. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PRGOPT` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DATTRV` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). | The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BU` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). | The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IND` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). | The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The output from the problem (when output flag INFO=1) includes optimal values for x and w in PRIMAL(*), optimal values for dual variables of the equations A*x=w and the simple bounds on x in DUALS(*), and the indices of the basic columns, IBASIS(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PRIMAL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The output from the problem (when output flag INFO=1) includes optimal values for x and w in PRIMAL(*), optimal values for dual variables of the equations A*x=w and the simple bounds on x in DUALS(*), and the indices of the basic columns, IBASIS(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DUALS` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The output from the problem (when output flag INFO=1) includes optimal values for x and w in PRIMAL(*), optimal values for dual variables of the equations A*x=w and the simple bounds on x in DUALS(*), and the indices of the basic columns, IBASIS(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IBASIS` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The output from the problem (when output flag INFO=1) includes optimal values for x and w in PRIMAL(*), optimal values for dual variables of the equations A*x=w and the simple bounds on x in DUALS(*), and the indices of the basic columns, IBASIS(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. | \\|------------------------------\\| \\|Fortran Declarations Required:\\| \\|------------------------------\\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), *BL(NVARS+MRELAS),BU(NVARS+MRELAS),IND(NVARS+MRELAS), *PRIMAL(NVARS+MRELAS),DUALS(MRELAS+NVARS),IBASIS(NVARS+MRELAS), *WORK(LW),IWORK(LIW) EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::linear_programming::dsplp`. Native symbol: `dsplp_`. Feature: `optimization-linear-programming-in-memory`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::linear_programming::dsplp`
- Compatibility aliases: `none`
- Public declaration feature: `optimization-linear-programming-in-memory`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::linear_programming::LinearProgram::<f64>::solve`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
