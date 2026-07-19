# SLPREP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Prepare one direct access file and three sequential files for the SLATEC documentation program.

## Description

This program reads a sequential documentation file where each module in the file consists of the complete subprogram statement and a SLATEC-style prologue. The program uses the information obtained to generate four files. These are 1) a direct access file of the subprogram statement and prologue for each routine in the library. 2) a sequential file of routine names, categories, etc., 3) a sequential file of keywords and pointers to the routines. 4) a sequential file of expanded categories and messages. These four files constitute the database for the SLADOC documentation program. There are a number of system and library dependent parameters which the user of this program may have to change before compiling and running the code. All parameters are defined in the records which immediately follow this prologue. In the discussion here, we refer to the default values which are distributed with this code; in order to assist others using this code, we give values for several different machine/operating system configurations. MXLFN - the maximum length of a file name to be used. The value used is highly user and system dependent. Set the value to the length of longest of the 8 file names FOUT, FERR, FINP, FDAF, FKWD, FTBL, FCLASS, and FCAT described below. FINP - the name of the input file which contains the prologues. Each prologue must be preceeded by a

## Classification

- Historical role: `unknown`
- Program-unit kind: `program`
- Identity kind: `documentation_or_tooling_program_unit`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Documentation and source-processing tools`
- Mathematical domain: `documentation-tools`
- Package provenance: `slatec-documentation-tools`
- Family evidence: `reviewed_override` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `slprep/slprep` (`documentation-support-source-candidate`)

## Official references

- Official references unavailable from current cached evidence.

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
