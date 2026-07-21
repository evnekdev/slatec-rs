# DXNRMP

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute normalized Legendre polynomials.

## Description

SUBROUTINE TO CALCULATE NORMALIZED LEGENDRE POLYNOMIALS (XNRMP is single-precision version) DXNRMP calculates normalized Legendre polynomials of varying order and fixed argument and degree. The order MU and degree

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C3A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dxnrmp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dxnrmp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dxnrmp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dxnrmp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DXNRMP](https://www.netlib.org/slatec/src/dxnrmp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J.M. Smith, F.W.J. Olver, and D.W. Lozier, Extended-Range Arithmetic and Normalized Legendre Polynomials, ACM Transactions on Mathematical Soft- ware, 93-105, March 1981, for a complete description of the algorithm and special arithmetic. Also see program comments in DXSET. The normalized Legendre polynomials are multiples of the associated Legendre polynomials of the first kind where the normalizing coefficients are chosen so as to make the integral from -1 to 1 of the square of each function equal to 1. See E. Jahnke, F. Emde and F. Losch, Tables of Higher Functions, McGraw-Hill, New York, 1960, p. 121. The input values to DXNRMP are NU, MU1, MU2, DARG, and MODE. These must satisfy 1. NU .GE. 0 specifies the degree of the normalized Legendre polynomial that is wanted. MU1, |
| 2 | `MU1` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order normalized Legendre polynomial that is wanted. order normalized Leg- endre polynomial that is wanted. + 1, ..., MU2. |
| 3 | `MU2` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order normalized Leg- endre polynomial that is wanted. MU1 + 1 and DX = DARG or COS(DARG) according |
| 4 | `DARG` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | MU1, |
| 5 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1.0D0 .LE. DARG .LE. 1.0D0 specifies that 3.14159... .LT. DARG .LT. 3.14159... spec- ifies that Normalized Legendre(NU, MU, COS(DARG)) is wanted for MU = MU1, MU1 + 1, ..., MU2. The output of DXNRMP consists of the two vectors DPN and IPN and the error estimate ISIG. The computed values are stored as extended-range numbers such that 1 or 2. Finally, ISIG is an estimate of the number of decimal digits lost through rounding errors in the computation. For example if DARG is accurate to 12 significant decimals, then the computed function values are accurate to 12 - ISIG significant decimals (except in neighborhoods of zeros). |
| 6 | `DPN` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | NORMALIZED LEGENDRE(NU,MU1,DX) NORMALIZED LEGENDRE(NU,MU1+1,DX) . . NORMALIZED LEGENDRE(NU,MU2,DX) is DPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I) .NE. 0 the corre- sponding value of the normalized Legendre polynomial cannot be represented in double-precision because of overflow or under- flow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case that IPN(I) is nonzero, the user could rewrite his/her program to use extended range arithmetic. The interpretation of (DPN(I),IPN(I)) can be changed to range subroutine DXCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL DXCON(DPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, DPN(I), IPN(I) 10 FORMAT(1X, D30.18 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J |
| 7 | `IPN` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | NORMALIZED LEGENDRE(NU,MU1,DX) NORMALIZED LEGENDRE(NU,MU1+1,DX) . . NORMALIZED LEGENDRE(NU,MU2,DX) is DPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When 0 the value of the normalized Legendre polynomial is range subroutine DXCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL DXCON(DPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, DPN(I), IPN(I) 10 FORMAT(1X, D30.18 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J IPN(2) = ... = IPN(J) = 0. Because of the change of representation caused by calling DXCON, (DPN(I), J+1, J+2, ... cannot be used in subsequent extended-range computations. |
| 8 | `ISIG` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable output estimate of decimal digits lost to rounding in the extended-range normalized-Legendre calculation. Subtract it from the significant digits in the input argument to estimate retained result precision away from zeros. |
| 9 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an error indicator. If no errors are detected, 0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. 212 or 213, invalid input was provided to DXNRMP. 201,202,203, or 204, invalid input was provided to DXSET. 205 or 206, an internal consistency error occurred in DXSET (probably due to a software malfunction in the range number was detected in DXADJ. range number was detected in DXC210. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dxnrmp`. Native symbol: `dxnrmp_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dxnrmp`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
