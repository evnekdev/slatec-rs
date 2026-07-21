# CBESH

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute a sequence of the Hankel functions H(m,a,z) for superscript m=1 or 2, real nonnegative orders a=b, b+1,... where b>0, and nonzero complex argument z. A scaling option is available to help avoid overflow.

## Description

On KODE=1, CBESH computes an N member sequence of complex Hankel (Bessel) functions CY(L)=H(M,FNU+L-1,Z) for superscript M=1 or 2, real nonnegative orders FNU+L-1, L=1,..., N, and complex nonzero Z in the cut plane -pi<arg(Z)<=pi. On KODE=2, CBESH returns the scaled functions CY(L) = H(M,FNU+L-1,Z)*exp(-(3-2*M)*Z*i), i**2=-1 which removes the exponential behavior in both the upper and lower half planes. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

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
- GAMS classifications: `C10A4`
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

- Canonical provider: `main-src/src/cbesh.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cbesh.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cbesh.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cbesh.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [CBESH](https://www.netlib.org/slatec/src/cbesh.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `Z` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | pi<arg(Z)<=pi. Nonzero argument of type COMPLEX 2 and Im(Z)<0, then 1 too large) 1 is large) 1 is too large) a*t)*K(a,z*exp(-t)) t = (3-2*m)*i*pi/2 where the K Bessel function is computed as described in the prologue to CBESK. Exponential decay of H(m,a,z) occurs in the upper half z 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), OF FNU)) ).  However, the phase angle may have only absolute accuracy.  This is most likely to occur when one component (in magnitude) is larger than the other by several orders of magnitude.  If one component is 10**K larger than the other, then one can expect only MAX(ABS(LOG10(P))-K, 0) significant digits; or, stated another way, when K exceeds the exponent of P, no significant digits remain in the smaller component.  However, the phase angle retains absolute accuracy because, in complex arithmetic with precision P, the smaller component will not (as a rule) decrease below P times the magnitude of the larger component.  In these extreme cases, the principal phase angle is on the order of +P, -P, PI/2-P, or -PI/2+P. |
| 2 | `FNU` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | 1,Z) for super- 1, L=1,..., 1,Z)*exp(-(3-2*M)*Z*i),  i**2=-1 which removes the exponential behavior in both the upper and lower half planes.  Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1). Initial order of type REAL, FNU>=0 1,Z), L=1,...,N =2  returns 1,Z)*exp(-(3-2M)*Z*i), 1 too large) 1 is large) 1 is too large) 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), MAX(1,ABS(EXPONENT OF |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1, CBESH computes an N member sequence of complex 2, CBESH returns the scaled functions A parameter to indicate the scaling option 1  returns |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 or 2, real nonnegative orders a=b, b+1,... where b>0, and nonzero complex argument z.  A scaling option is available to help avoid overflow. 1,Z) for super- 1, L=1,..., 1,Z)*exp(-(3-2*M)*Z*i),  i**2=-1 which removes the exponential behavior in both the upper and lower half planes.  Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1). 1,Z), L=1,...,N =2  returns 1,Z)*exp(-(3-2M)*Z*i), Superscript of Hankel function, M=1 or 2 2 and Im(Z)<0, then a*t)*K(a,z*exp(-t)) t = (3-2*m)*i*pi/2 where the K Bessel function is computed as described in the prologue to CBESK. Exponential decay of H(m,a,z) occurs in the upper half z 1 and the lower half z plane for m=2.  Exponential growth occurs in the complementary half planes.  Scaling by exp(-(3-2*m)*z*i) removes the exponential behavior in the whole z plane as z goes to infinity. For negative orders, the formula a,z) = H(m,a,z)*exp((3-2*m)*a*pi*i) can be used. In most complex variable computation, one must evaluate ele- |
| 5 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | pi<arg(Z)<=pi. Number of terms in the sequence, N>=1 1 too large) 1 is large) 1 is too large) 1 is large, losses of significance by argument reduction occur. Consequently, if either one exceeds U1=SQRT(0.5/UR), then losses exceeding half precision are likely and an error flag 1 is restricted by MIN(U2,U3).  In IEEE arithmetic, U1,U2, and U3 approximate 2.0E+3, 4.2E+6, 2.1E+9 in single precision and 4.7E+7, 2.3E+15 and 2.1E+9 in double precision.  This makes U2 limiting in single precision and U3 limiting in double precision.  This means that one can expect to retain, in the worst cases on IEEE machines, no digits in single pre- cision and only 6 digits in double precision.  Similar con- siderations hold for other machines. The approximate relative error in the magnitude of a complex Bessel function can be expressed as P*10**S where P=MAX(UNIT ROUNDOFF,1.0E-18) is the nominal precision and 10**S repre- sents the increase in error due to argument reduction in the elementary functions.  Here, S=MAX(1,ABS(LOG10(ABS(Z))), |
| 6 | `CY` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (N) | 1,Z) for super- 1,Z)*exp(-(3-2*M)*Z*i),  i**2=-1 which removes the exponential behavior in both the upper and lower half planes.  Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1). 1,Z), L=1,...,N =2  returns 1,Z)*exp(-(3-2M)*Z*i), Result vector of type COMPLEX 0 for NZ values of L (if M=1 and plementary half planes, the underflows may not be in an uninterrupted sequence) |
| 7 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of underflows set to zero 0    Normal return 0 for NZ values of L (if M=1 and plementary half planes, the underflows may not be in an uninterrupted sequence) |
| 8 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Error flag COMPUTATION COMPLETED NO COMPUTATION NO COMPUTATION COMPUTATION COMPLETED (Result has half precision or less NO COMPUTATION (Result has no precision because NO COMPUTATION (Termination condition not met) Long Description: The computation is carried out by the formula 3 is triggered where UR=R1MACH(4)=UNIT ROUNDOFF.  Also, if either is larger than U2=0.5/UR, then all significance is 4.  In order to use the INT function, arguments must be further restricted not to exceed the largest machine |

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

Canonical Rust path: `slatec_sys::special::complex::cbesh`. Native symbol: `cbesh_`. Declaration feature: `special-complex`. Provider feature: `special-complex`. ABI fingerprint: `subroutine:void(mut_complex32,mut_f32,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::complex::cbesh`
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
