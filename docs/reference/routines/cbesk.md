# CBESK

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute a sequence of the Bessel functions K(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.

## Description

On KODE=1, CBESK computes an N member sequence of complex Bessel functions CY(L)=K(FNU+L-1,Z) for real nonnegative orders FNU+L-1, L=1,...,N and complex Z.NE.0 in the cut plane -pi<arg(Z)<=pi. On KODE=2, CBESJ returns the scaled functions CY(L) = exp(Z)*K(FNU+L-1,Z), L=1,...,N which remove the exponential growth in both the left and right half planes as Z goes to infinity. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B4`
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

- Canonical provider: `main-src/src/cbesk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cbesk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cbesk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cbesk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [CBESK](https://www.netlib.org/slatec/src/cbesk.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `Z` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | Nonzero argument of type COMPLEX |
| 2 | `FNU` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Initial order of type REAL, FNU>=0 1,Z), L=1,...,N =2  returns 1,Z)*EXP(Z), L=1,...,N |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | A parameter to indicate the scaling option 1  returns |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of terms in the sequence, N>=1 |
| 5 | `CY` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (N) | 1,Z), L=1,...,N =2  returns 1,Z)*EXP(Z), L=1,...,N Result vector of type COMPLEX 0 for NZ values of L (if Re(Z)>0 0 for L=1,...,NZ; in the complementary half plane the underflows may not be in an uninterrupted sequence) |
| 6 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of underflows set to zero 0    Normal return 0 for NZ values of L (if Re(Z)>0 |
| 7 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

IERR=0  Normal return     - COMPUTATION COMPLETED IERR=2  Overflow          - NO COMPUTATION (abs(Z) too small and/or FNU+N-1 too large) IERR=3  Precision warning - COMPUTATION COMPLETED (Result has half precision or less because abs(Z) or FNU+N-1 is large) (Result has no precision because abs(Z) or FNU+N-1 is too large) (Termination condition not met) Long Description: Equations of the reference are implemented to compute K(a,z) for small orders a and a+1 in the right half plane Re(z)>=0. Forward recurrence generates higher orders.  The formula K(a,z*exp((t)) = exp(-t)*K(a,z) - t*I(a,z),  Re(z)>0 t = i*pi or -i*pi continues K to the left half plane. For large orders, K(a,z) is computed by means of its uniform asymptotic expansion. For negative orders, the formula K(-a,z) = K(a,z) can be used. CBESK assumes that a significant digit sinh function is available. In most complex variable computation, one must evaluate ele- mentary functions.  When the magnitude of Z or FNU+N-1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then IERR=3 is triggered where UR=R1MACH(4)=UNIT ROUNDOFF.  Also, if either is larger than U2=0.5/UR, then all significance is lost and IERR=4.  In order to use the INT function, arguments must be further restricted not to exceed the largest machine integer, U3=I1MACH(9).  Thus, the magnitude of Z and FNU+N-1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), ABS(LOG10(FNU))) approximately (i.e., S=MAX(1,ABS(EXPONENT OF ABS(Z),ABS(EXPONENT OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component.  In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::complex::cbesk`. Native symbol: `cbesk_`. Declaration feature: `special-complex`. Provider feature: `special-complex`. ABI fingerprint: `subroutine:void(mut_complex32,mut_f32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::complex::cbesk`
- Public declaration feature: `special-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
