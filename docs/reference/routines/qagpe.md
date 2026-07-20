# QAGPE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Approximate a given definite integral I = Integral of F over (A,B), hopefully satisfying the accuracy claim: ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)). Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user.

## Description

Computation of a definite integral Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Real Lower limit of integration B - Real Upper limit of integration NPTS2 - Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. If NPTS2.LT.2, the routine will end with IER = 6. POINTS - Real Vector of dimension NPTS2, the first (NPTS2-2) elements of which are the user provided break POINTS. If these POINTS do not constitute an ascending sequence there will be an automatic sorting. EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.NPTS2 If LIMIT.LT.NPTS2, the routine will end with IER = 6. ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs At some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because NPTS2.LT.2 or Break points are specified outside the integration range or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or LIMIT.LT.NPTS2. RESULT, ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. ALIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) BLIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) RLIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the integral approximations on the subintervals ELIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the moduli of the absolute error estimates on the subintervals PTS - Real Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. LEVEL - Integer Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration limit, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L). NDIN - Integer Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), I = 0,1, ..., NPTS2-2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. If this happens for the subinterval numbered K, NDIN(K) is put to 1, otherwise NDIN(K) = 0. IORD - Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, such that ELIST(IORD(1)), ..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST If LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise LAST - Integer Number of subintervals actually produced in the subdivisions process

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qagpe.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qagpe.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qagpe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qagpe.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F` | callback | `REAL` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Computation of a definite integral Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | Computation of a definite integral Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration NPTS2 - Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NPTS2` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration NPTS2 - Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `POINTS` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | A - Real Lower limit of integration B - Real Upper limit of integration NPTS2 - Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSABS` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSREL` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIMIT` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration NPTS2 - Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | output | `REAL` (`explicit`) | `*mut f32` | scalar | ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | output | `REAL` (`explicit`) | `*mut f32` | scalar | ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEVAL` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If NPTS2.LT.2, the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ALIST(1) and BLIST(1) are set to A and B respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BLIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ALIST(1) and BLIST(1) are set to A and B respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RLIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | RESULT, ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | RESULT, ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PTS` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ALIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) BLIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) RLIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the integral approximations on the subintervals ELIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the moduli of the absolute error estimates on the subintervals PTS - Real Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. | ALIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) BLIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) RLIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the integral approximations on the subintervals ELIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the moduli of the absolute error estimates on the subintervals PTS - Real Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IORD` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IORD - Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, such that ELIST(IORD(1)), ..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST If LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise LAST - Integer Number of subintervals actually produced in the subdivisions process | IORD - Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, such that ELIST(IORD(1)), ..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST If LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise LAST - Integer Number of subintervals actually produced in the subdivisions process Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LEVEL` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | LEVEL - Integer Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i.e. | LEVEL - Integer Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i.e. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NDIN` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | NDIN - Integer Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), I = 0,1, ..., NPTS2-2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. | NDIN - Integer Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), I = 0,1, ..., NPTS2-2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LAST` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | RESULT, ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::callbacks::qagpe`. Native symbol: `qagpe_`. Feature: `quadrature-callbacks`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qagpe`
- Compatibility aliases: `none`
- Public declaration feature: `quadrature-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
