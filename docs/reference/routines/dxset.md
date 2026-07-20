# DXSET

[Family: Arithmetic and extended-range arithmetic](../families/arithmetic-and-extended-range-arithmetic.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

To provide double-precision floating-point arithmetic with an extended exponent range.

## Description

SUBROUTINE DXSET MUST BE CALLED PRIOR TO CALLING ANY OTHER EXTENDED-RANGE SUBROUTINE. IT CALCULATES AND STORES SEVERAL MACHINE-DEPENDENT CONSTANTS IN COMMON BLOCKS. THE USER MUST SUPPLY FOUR CONSTANTS THAT PERTAIN TO HIS PARTICULAR COMPUTER. THE CONSTANTS ARE IRAD = THE INTERNAL BASE OF DOUBLE-PRECISION ARITHMETIC IN THE COMPUTER. NRADPL = THE NUMBER OF RADIX PLACES CARRIED IN THE DOUBLE-PRECISION REPRESENTATION. DZERO = THE SMALLEST OF 1/DMIN, DMAX, DMAXLN WHERE DMIN = THE SMALLEST POSITIVE DOUBLE-PRECISION NUMBER OR AN UPPER BOUND TO THIS NUMBER, DMAX = THE LARGEST DOUBLE-PRECISION NUMBER OR A LOWER BOUND TO THIS NUMBER, DMAXLN = THE LARGEST DOUBLE-PRECISION NUMBER SUCH THAT LOG10(DMAXLN) CAN BE COMPUTED BY THE FORTRAN SYSTEM (ON MOST SYSTEMS DMAXLN = DMAX). NBITS = THE NUMBER OF BITS (EXCLUSIVE OF SIGN) IN AN INTEGER COMPUTER WORD. ALTERNATIVELY, ANY OR ALL OF THE CONSTANTS CAN BE GIVEN THE VALUE 0 (0.0D0 FOR DZERO). IF A CONSTANT IS ZERO, DXSET TRIES TO ASSIGN AN APPROPRIATE VALUE BY CALLING I1MACH (SEE P.A.FOX, A.D.HALL, N.L.SCHRYER, ALGORITHM 528 FRAMEWORK FOR A PORTABLE LIBRARY, ACM TRANSACTIONS ON MATH SOFTWARE, V.4, NO.2, JUNE 1978, 177-188). THIS IS THE SETTING-UP SUBROUTINE FOR A PACKAGE OF SUBROUTINES THAT FACILITATE THE USE OF EXTENDED-RANGE ARITHMETIC. EXTENDED-RANGE ARITHMETIC ON A PARTICULAR COMPUTER IS DEFINED ON THE SET OF NUMBERS OF THE FORM (X,IX) = X*RADIX**IX WHERE X IS A DOUBLE-PRECISION NUMBER CALLED THE PRINCIPAL PART, IX IS AN INTEGER CALLED THE AUXILIARY INDEX, AND RADIX IS THE INTERNAL BASE OF THE DOUBLE-PRECISION ARITHMETIC. OBVIOUSLY, EACH REAL NUMBER IS REPRESENTABLE WITHOUT ERROR BY MORE THAN ONE EXTENDED-RANGE FORM. CONVERSIONS BETWEEN DIFFERENT FORMS ARE ESSENTIAL IN CARRYING OUT ARITHMETIC OPERATIONS. WITH THE CHOICE OF RADIX WE HAVE MADE, AND THE SUBROUTINES WE HAVE WRITTEN, THESE CONVERSIONS ARE PERFORMED WITHOUT ERROR (AT LEAST ON MOST COMPUTERS). (SEE SMITH, J.M., OLVER, F.W.J., AND LOZIER, D.W., EXTENDED-RANGE ARITHMETIC AND NORMALIZED LEGENDRE POLYNOMIALS, ACM TRANSACTIONS ON MATHEMATICAL SOFTWARE, MARCH 1981). AN EXTENDED-RANGE NUMBER (X,IX) IS SAID TO BE IN ADJUSTED FORM IF X AND IX ARE ZERO OR RADIX**(-L) .LE. ABS(X) .LT. RADIX**L IS SATISFIED, WHERE L IS A COMPUTER-DEPENDENT INTEGER DEFINED IN THIS SUBROUTINE. TWO EXTENDED-RANGE NUMBERS IN ADJUSTED FORM CAN BE ADDED, SUBTRACTED, MULTIPLIED OR DIVIDED (IF THE DIVISOR IS NONZERO) WITHOUT CAUSING OVERFLOW OR UNDERFLOW IN THE PRINCIPAL PART OF THE RESULT. WITH PROPER USE OF THE EXTENDED-RANGE SUBROUTINES, THE ONLY OVERFLOW THAT CAN OCCUR IS INTEGER OVERFLOW IN THE AUXILIARY INDEX. IF THIS IS DETECTED, THE SOFTWARE CALLS XERROR (A GENERAL ERROR-HANDLING FORTRAN SUBROUTINE PACKAGE). MULTIPLICATION AND DIVISION IS PERFORMED BY SETTING (X,IX)*(Y,IY) = (X*Y,IX+IY) OR (X,IX)/(Y,IY) = (X/Y,IX-IY). PRE-ADJUSTMENT OF THE OPERANDS IS ESSENTIAL TO AVOID OVERFLOW OR UNDERFLOW OF THE PRINCIPAL PART. SUBROUTINE DXADJ (SEE BELOW) MAY BE CALLED TO TRANSFORM ANY EXTENDEDRANGE NUMBER INTO ADJUSTED FORM. ADDITION AND SUBTRACTION REQUIRE THE USE OF SUBROUTINE DXADD (SEE BELOW). THE INPUT OPERANDS NEED NOT BE IN ADJUSTED FORM. HOWEVER, THE RESULT OF ADDITION OR SUBTRACTION IS RETURNED IN ADJUSTED FORM. THUS, FOR EXAMPLE, IF (X,IX),(Y,IY), (U,IU), AND (V,IV) ARE IN ADJUSTED FORM, THEN (X,IX)*(Y,IY) + (U,IU)*(V,IV) CAN BE COMPUTED AND STORED IN ADJUSTED FORM WITH NO EXPLICIT CALLS TO DXADJ. WHEN AN EXTENDED-RANGE NUMBER IS TO BE PRINTED, IT MUST BE CONVERTED TO AN EXTENDED-RANGE FORM WITH DECIMAL RADIX. SUBROUTINE DXCON IS PROVIDED FOR THIS PURPOSE. THE SUBROUTINES CONTAINED IN THIS PACKAGE ARE SUBROUTINE DXADD USAGE CALL DXADD(X,IX,Y,IY,Z,IZ,IERROR) IF (IERROR.NE.0) RETURN FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). (Z,IZ) IS ADJUSTED BEFORE RETURNING. THE INPUT OPERANDS NEED NOT BE IN ADJUSTED FORM, BUT THEIR PRINCIPAL PARTS MUST SATISFY RADIX**(-2L).LE.ABS(X).LE.RADIX**(2L), RADIX**(-2L).LE.ABS(Y).LE.RADIX**(2L). SUBROUTINE DXADJ USAGE CALL DXADJ(X,IX,IERROR) IF (IERROR.NE.0) RETURN TRANSFORMS (X,IX) SO THAT RADIX**(-L) .LE. ABS(X) .LT. RADIX**L. ON MOST COMPUTERS THIS TRANSFORMATION DOES NOT CHANGE THE MANTISSA OF X PROVIDED RADIX IS THE NUMBER BASE OF DOUBLE-PRECISION ARITHMETIC. SUBROUTINE DXC210 USAGE CALL DXC210(K,Z,J,IERROR) IF (IERROR.NE.0) RETURN GIVEN K THIS SUBROUTINE COMPUTES J AND Z SUCH THAT RADIX**K = Z*10**J, WHERE Z IS IN THE RANGE 1/10 .LE. Z .LT. 1. THE VALUE OF Z WILL BE ACCURATE TO FULL DOUBLE-PRECISION PROVIDED THE NUMBER OF DECIMAL PLACES IN THE LARGEST INTEGER PLUS THE NUMBER OF DECIMAL PLACES CARRIED IN DOUBLE-PRECISION DOES NOT EXCEED 60. DXC210 IS CALLED BY SUBROUTINE DXCON WHEN NECESSARY. THE USER SHOULD NEVER NEED TO CALL DXC210 DIRECTLY. SUBROUTINE DXCON USAGE CALL DXCON(X,IX,IERROR) IF (IERROR.NE.0) RETURN CONVERTS (X,IX) = X*RADIX**IX TO DECIMAL FORM IN PREPARATION FOR PRINTING, SO THAT (X,IX) = X*10**IX WHERE 1/10 .LE. ABS(X) .LT. 1 IS RETURNED, EXCEPT THAT IF (ABS(X),IX) IS BETWEEN RADIX**(-2L) AND RADIX**(2L) THEN THE REDUCED FORM WITH IX = 0 IS RETURNED. SUBROUTINE DXRED USAGE CALL DXRED(X,IX,IERROR) IF (IERROR.NE.0) RETURN IF RADIX**(-2L) .LE. (ABS(X),IX) .LE. RADIX**(2L) THEN DXRED TRANSFORMS (X,IX) SO THAT IX=0. IF (X,IX) IS OUTSIDE THE ABOVE RANGE, THEN DXRED TAKES NO ACTION. THIS SUBROUTINE IS USEFUL IF THE RESULTS OF EXTENDED-RANGE CALCULATIONS ARE TO BE USED IN SUBSEQUENT ORDINARY DOUBLE-PRECISION CALCULATIONS.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Arithmetic and extended-range arithmetic`
- Mathematical domain: `numeric-support`
- Package provenance: `unknown`
- GAMS classifications: `A3D`
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

- Canonical provider: `main-src/src/dxset.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dxset.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dxset.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dxset.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Arithmetic and extended-range arithmetic](../families/arithmetic-and-extended-range-arithmetic.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `IRAD` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | THE CONSTANTS ARE IRAD = THE INTERNAL BASE OF DOUBLE-PRECISION ARITHMETIC IN THE COMPUTER. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NRADPL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NRADPL = THE NUMBER OF RADIX PLACES CARRIED IN THE DOUBLE-PRECISION REPRESENTATION. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DZERO` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DZERO = THE SMALLEST OF 1/DMIN, DMAX, DMAXLN WHERE DMIN = THE SMALLEST POSITIVE DOUBLE-PRECISION NUMBER OR AN UPPER BOUND TO THIS NUMBER, DMAX = THE LARGEST DOUBLE-PRECISION NUMBER OR A LOWER BOUND TO THIS NUMBER, DMAXLN = THE LARGEST DOUBLE-PRECISION NUMBER SUCH THAT LOG10(DMAXLN) CAN BE COMPUTED BY THE FORTRAN SYSTEM (ON MOST SYSTEMS DMAXLN = DMAX). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBITS` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NBITS = THE NUMBER OF BITS (EXCLUSIVE OF SIGN) IN AN INTEGER COMPUTER WORD. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | THE SUBROUTINES CONTAINED IN THIS PACKAGE ARE SUBROUTINE DXADD USAGE CALL DXADD(X,IX,Y,IY,Z,IZ,IERROR) IF (IERROR.NE.0) RETURN FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-numeric-scalar-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
