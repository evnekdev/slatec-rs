# ZUOIK

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to ZBESH, ZBESI and ZBESK

## Description

ZUOIK COMPUTES THE LEADING TERMS OF THE UNIFORM ASYMPTOTIC EXPANSIONS FOR THE I AND K FUNCTIONS AND COMPARES THEM (IN LOGARITHMIC FORM) TO ALIM AND ELIM FOR OVER AND UNDERFLOW WHERE ALIM.LT.ELIM. IF THE MAGNITUDE, BASED ON THE LEADING EXPONENTIAL, IS LESS THAN ALIM OR GREATER THAN -ALIM, THEN THE RESULT IS ON SCALE. IF NOT, THEN A REFINED TEST USING OTHER MULTIPLIERS (IN LOGARITHMIC FORM) IS MADE BASED ON ELIM. HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. Y MUST BE SET BY ANOTHER ROUTINE

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f64`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `ZBESH, ZBESI, ZBESK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/zuoik.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/zuoik.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/zuoik.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/zuoik.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ZR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ZI` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNU` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IKFLG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (N) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YI` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (N) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NUF` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. | HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELIM` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ZUOIK COMPUTES THE LEADING TERMS OF THE UNIFORM ASYMPTOTIC EXPANSIONS FOR THE I AND K FUNCTIONS AND COMPARES THEM (IN LOGARITHMIC FORM) TO ALIM AND ELIM FOR OVER AND UNDERFLOW WHERE ALIM.LT.ELIM. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALIM` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ZUOIK COMPUTES THE LEADING TERMS OF THE UNIFORM ASYMPTOTIC EXPANSIONS FOR THE I AND K FUNCTIONS AND COMPARES THEM (IN LOGARITHMIC FORM) TO ALIM AND ELIM FOR OVER AND UNDERFLOW WHERE ALIM.LT.ELIM. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
