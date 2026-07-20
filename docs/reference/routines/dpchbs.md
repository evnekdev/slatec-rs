# DPCHBS

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Piecewise Cubic Hermite to B-Spline converter.

## Description

*Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . (not checked) X:IN is the real array of independent variable values. The elements of X must be strictly increasing: X(I-1) .LT. X(I), I = 2(1)N. (not checked) nmax, the dimension of X, must be .ge.N. F:IN is the real array of dependent variable values. F(1+(I-1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of F, must be .ge.N. D:IN is the real array of derivative values at the data points. D(1+(I-1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of D, must be .ge.N. INCFD:IN is the increment between successive values in F and D. This argument is provided primarily for 2-D applications. It may have the value 1 for one-dimensional applications, in which case F and D may be singly-subscripted arrays. KNOTYP:IN is a flag to control the knot sequence. The knot sequence T is normally computed from X by putting a double knot at each X and setting the end knot pairs according to the value of KNOTYP: KNOTYP = 0: Quadruple knots at X(1) and X(N). (default) KNOTYP = 1: Replicate lengths of extreme subintervals: T( 1 ) = T( 2 ) = X(1) - (X(2)-X(1)) ; T(M+4) = T(M+3) = X(N) + (X(N)-X(N-1)). KNOTYP = 2: Periodic placement of boundary knots: T( 1 ) = T( 2 ) = X(1) - (X(N)-X(N-1)); T(M+4) = T(M+3) = X(N) + (X(2)-X(1)) . Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used in a parametric setting. NKNOTS:INOUT is the number of knots. If KNOTYP.GE.0, then NKNOTS will be set to NDIM+4. If KNOTYP.LT.0, then NKNOTS is an input variable, and an error return will be taken if it is not equal to NDIM+4. T:INOUT is the array of 2*N+4 knots for the B-representation. If KNOTYP.GE.0, T will be returned by DPCHBS with the interior double knots equal to the X-values and the boundary knots set as indicated above. If KNOTYP.LT.0, it is assumed that T was set by a previous call to DPCHBS. (This routine does **not** verify that T forms a legitimate knot sequence.) BCOEF:OUT is the array of 2*N B-spline coefficients. NDIM:OUT is the dimension of the B-spline space. (Set to 2*N.) KORD:OUT is the order of the B-spline. (Set to 4.) IERR:OUT is an error flag. Normal return: IERR = 0 (no errors). "Recoverable" errors: IERR = -4 if KNOTYP.GT.2 . IERR = -5 if KNOTYP.LT.0 and NKNOTS.NE.(2*N+4). *Description: DPCHBS computes the B-spline representation of the PCH function determined by N,X,F,D. To be compatible with the rest of PCHIP, DPCHBS includes INCFD, the increment between successive values of the F- and D-arrays. The output is the B-representation for the function: NKNOTS, T, BCOEF, NDIM, KORD. *Caution: Since it is assumed that the input PCH function has been computed by one of the other routines in the package PCHIP, input arguments N, X, INCFD are **not** checked for validity. *Restrictions/assumptions: 1. N.GE.2 . (not checked) 2. X(i).LT.X(i+1), i=1,...,N . (not checked) 3. INCFD.GT.0 . (not checked) 4. KNOTYP.LE.2 . (error return if not) *5. NKNOTS = NDIM+4 = 2*N+4 . (error return if not) *6. T(2*k+1) = T(2*k) = X(k), k=1,...,N . (not checked) * Indicates this applies only if KNOTYP.LT.0 . *Portability: Argument INCFD is used only to cause the compiler to generate efficient code for the subscript expressions (1+(I-1)*INCFD) . The normal usage, in which DPCHBS is called with one-dimensional arrays F and D, is probably non-Fortran 77, in the strict sense, but it works on all systems on which DPCHBS has been tested. *See Also: PCHIC, PCHIM, or PCHSP can be used to determine an interpolating PCH function from a set of data. The B-spline routine DBVALU can be used to evaluate the B-representation that is output by DPCHBS. (See BSPDOC for more information.)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E3`
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

- Canonical provider: `pchip/dpchbs.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dpchbs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dpchbs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [PCHIP](../families/pchip.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (INCFD, *) | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (INCFD, *) | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INCFD` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KNOTYP` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NKNOTS` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BCOEF` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NDIM` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KORD` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) DOUBLE PRECISION X(nmax), F(INCFD,nmax), D(INCFD,nmax), * T(2*nmax+4), BCOEF(2*nmax) CALL DPCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, * NDIM, KORD, IERR) *Arguments: N:IN is the number of data points, N.ge.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::interpolation::dpchbs`. Native symbol: `dpchbs_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dpchbs`
- Compatibility aliases: `slatec_sys::interpolation::numerical::dpchbs`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
