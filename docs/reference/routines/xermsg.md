# XERMSG

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Process error messages for SLATEC and other libraries.

## Description

XERMSG processes a diagnostic message in a manner determined by the value of LEVEL and the current value of the library error control flag, KONTRL. See subroutine XSETF for details. LIBRAR A character constant (or character variable) with the name of the library. This will be 'SLATEC' for the SLATEC Common Math Library. The error handling package is general enough to be used by many libraries simultaneously, so it is desirable for the routine that detects and reports an error to identify the library name as well as the routine name. SUBROU A character constant (or character variable) with the name of the routine that detected the error. Usually it is the name of the routine that is calling XERMSG. There are some instances where a user callable library routine calls lower level subsidiary routines where the error is detected. In such cases it may be more informative to supply the name of the routine the user called rather than the name of the subsidiary routine that detected the error. MESSG A character constant (or character variable) with the text of the error or warning message. In the example below, the message is a character constant that contains a generic message. CALL XERMSG ('SLATEC', 'MMPY', *'THE ORDER OF THE MATRIX EXCEEDS THE ROW DIMENSION', *3, 1) It is possible (and is sometimes desirable) to generate a specific message--e.g., one that contains actual numeric values. Specific numeric values can be converted into character strings using formatted WRITE statements into character variables. This is called standard Fortran internal file I/O and is exemplified in the first three lines of the following example. You can also catenate substrings of characters to construct the error message. Here is an example showing the use of both writing to an internal file and catenating character strings. CHARACTER*5 CHARN, CHARL WRITE (CHARN,10) N WRITE (CHARL,10) LDA 10 FORMAT(I5) CALL XERMSG ('SLATEC', 'MMPY', 'THE ORDER'//CHARN// * ' OF THE MATRIX EXCEEDS ITS ROW DIMENSION OF'// * CHARL, 3, 1) There are two subtleties worth mentioning. One is that the // for character catenation is used to construct the error message so that no single character constant is continued to the next line. This avoids confusion as to whether there are trailing blanks at the end of the line. The second is that by catenating the parts of the message as an actual argument rather than encoding the entire message into one large character variable, we avoid having to know how long the message will be in order to declare an adequate length for that large character variable. XERMSG calls XERPRN to print the message using multiple lines if necessary. If the message is very long, XERPRN will break it into pieces of 72 characters (as requested by XERMSG) for printing on multiple lines. Also, XERMSG asks XERPRN to prefix each line with ' * ' so that the total line length could be 76 characters. Note also that XERPRN scans the error message backwards to ignore trailing blanks. Another feature is that the substring '$$' is treated as a new line sentinel by XERPRN. If you want to construct a multiline message without having to count out multiples of 72 characters, just use '$$' as a separator. '$$' obviously must occur within 72 characters of the start of each line to have its intended effect since XERPRN is asked to wrap around at 72 characters in addition to looking for '$$'. NERR An integer value that is chosen by the library routine's author. It must be in the range -99 to 999 (three printable digits). Each distinct error should have its own error number. These error numbers should be described in the machine readable documentation for the routine. The error numbers need be unique only within each routine, so it is reasonable for each routine to start enumerating errors from 1 and proceeding to the next integer. LEVEL An integer value in the range 0 to 2 that indicates the level (severity) of the error. Their meanings are -1 A warning message. This is used if it is not clear that there really is an error, but the user's attention may be needed. An attempt is made to only print this message once. 0 A warning message. This is used if it is not clear that there really is an error, but the user's attention may be needed. 1 A recoverable error. This is used even if the error is so serious that the routine cannot return any useful answer. If the user has told the error package to return after recoverable errors, then XERMSG will return to the Library routine which can then return to the user's routine. The user may also permit the error package to terminate the program upon encountering a recoverable error. 2 A fatal error. XERMSG will not return to its caller after it receives a fatal error. This level should hardly ever be used; it is much better to allow the user a chance to recover. An example of one of the few cases in which it is permissible to declare a level 2 error is a reverse communication Library routine that is likely to be called repeatedly until it integrates across some interval. If there is a serious error in the input such that another step cannot be taken and the Library routine is called again without the input error having been corrected by the caller, the Library routine will probably be called forever with improper input. In this case, it is reasonable to declare the error to be fatal. Each of the arguments to XERMSG is input; none will be modified by XERMSG. A routine may make multiple calls to XERMSG with warning level messages; however, after a call to XERMSG with a recoverable error, the routine should return to the user. Do not try to call XERMSG with a second recoverable error after the first recoverable error because the error package saves the error number. The user can retrieve this error number by calling another entry point in the error handling package and then clear the error number when recovering from the error. Calling XERMSG in succession causes the old error number to be overwritten by the latest error number. This is considered harmless for error numbers associated with warning messages but must not be done for error numbers of serious errors. After a call to XERMSG with a recoverable error, the user must be given a chance to call NUMXER or XERCLR to retrieve or clear the error number.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Error handling`
- Mathematical domain: `runtime-support`
- Package provenance: `slatec-error`
- GAMS classifications: `R3C`
- Family evidence: `package_provenance` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/xermsg.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xermsg.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xermsg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xermsg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
