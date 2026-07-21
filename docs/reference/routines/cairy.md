# CAIRY

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the Airy function Ai(z) or its derivative dAi/dz for complex argument z. A scaling option is available to help avoid underflow and overflow.

## Description

On KODE=1, CAIRY computes the complex Airy function Ai(z) or its derivative dAi/dz on ID=0 or ID=1 respectively. On KODE=2, a scaling option exp(zeta)*Ai(z) or exp(zeta)*dAi/dz is provided to remove the exponential decay in -pi/3<arg(z) <pi/3 and the exponential growth in pi/3<abs(arg(z))<pi where zeta=(2/3)*z**(3/2). While the Airy functions Ai(z) and dAi/dz are analytic in the whole z-plane, the corresponding scaled functions defined for KODE=2 have a cut along the negative real axis.

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
- GAMS classifications: `C10D`
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

- Canonical provider: `main-src/src/cairy.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cairy.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cairy.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cairy.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [CAIRY](https://www.netlib.org/slatec/src/cairy.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `Z` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | Argument of type COMPLEX Z =2  returns Z where zeta=(2/3)*z**(3/2) |
| 2 | `ID` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Order of derivative, ID=0 or ID=1 |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | A parameter to indicate the scaling option 1  returns |
| 4 | `AI` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | Ai(z)  on ID=0 dAi/dz on ID=1 exp(zeta)*Ai(z)  on ID=0 exp(zeta)*dAi/dz on ID=1 Result of type COMPLEX |
| 5 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Underflow indicator 0    Normal return 1    AI=0 due to underflow in -pi/3<arg(Z)<pi/3 on KODE=1 |
| 6 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

IERR=0  Normal return     - COMPUTATION COMPLETED IERR=2  Overflow          - NO COMPUTATION (Re(Z) too large with KODE=1) IERR=3  Precision warning - COMPUTATION COMPLETED (Result has less than half precision) (Result has no precision) (Termination condition not met) Long Description: Ai(z) and dAi/dz are computed from K Bessel functions by Ai(z) =  c*sqrt(z)*K(1/3,zeta) dAi/dz = -c*   z   *K(2/3,zeta) c =  1/(pi*sqrt(3)) zeta =  (2/3)*z**(3/2) when abs(z)>1 and from power series when abs(z)<=1. In most complex variable computation, one must evaluate ele- mentary functions.  When the magnitude of Z is large, losses of significance by argument reduction occur.  Consequently, if the magnitude of ZETA=(2/3)*Z**(3/2) exceeds U1=SQRT(0.5/UR), flag IERR=3 is triggered where UR=R1MACH(4)=UNIT ROUNDOFF. Also, if the magnitude of ZETA is larger than U2=0.5/UR, then all significance is lost and IERR=4.  In order to use the INT function, ZETA must be further restricted not to exceed U3=I1MACH(9)=LARGEST INTEGER.  Thus, the magnitude of ZETA must be restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 are approximately 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15, 2.1E+9 in double precision. This makes U2 limiting is single precision and U3 limiting in double precision.  This means that the magnitude of Z cannot exceed approximately 3.4E+4 in single precision and 2.1E+6 in double precision.  This also means that one can expect to retain, in the worst cases on 32-bit machines, no digits in single precision and only 6 digits in double precision. Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), ABS(LOG10(FNU))) approximately (i.e., S=MAX(1,ABS(EXPONENT OF ABS(Z),ABS(EXPONENT OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component. In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::complex::cairy`. Native symbol: `cairy_`. Declaration feature: `special-complex`. Provider feature: `special-complex`. ABI fingerprint: `subroutine:void(mut_complex32,mut_i32,mut_i32,mut_complex32,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::complex::cairy`
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
