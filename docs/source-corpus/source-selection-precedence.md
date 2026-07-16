# Source-selection precedence

This precedence defines deterministic provider selection. It is not a general claim of historical authority.

## Default numerical provider precedence

For a program unit requested by the default corpus profile, apply the first matching rule:

1. **Approved project patch output** whose preimage hash matches the selected canonical provider and whose patch is enabled by the active policy/profile.
2. **Selected canonical archive provider** from checksum-pinned `slatec_src.tgz`.
3. **Explicit machine/provider profile** approved for the target, such as a validated machine-constant replacement.
4. **Explicit historical profile** for reproduction work.
5. Otherwise, fail with an unresolved-provider error.

Rules 3 and 4 are opt-in. They never activate from host detection alone unless a future policy explicitly defines and validates deterministic target mapping.

## Non-selecting evidence sources

The following sources can support provenance, comparison, validation, or policy changes but do not automatically provide compilation units:

- live `slatec/src`;
- relocated `slatec/lin`, `fishfft`, `fnlib`, and `pchip` directories;
- standalone upstream packages;
- `tree`, `tree1`, and plus-dependencies;
- table of contents, routine lists, GAMS mappings, manuals, and indexes;
- quick-check sources;
- browser and documentation tools.

## Duplicate resolution algorithm

For each selected file:

1. parse every declared program unit, `ENTRY`, and `BLOCK DATA` unit;
2. create a key from unit kind and ASCII-casefolded declared name;
3. gather all candidate providers;
4. apply active profile and approved override records;
5. require exactly one selected provider for each key;
6. emit a fatal diagnostic for zero or multiple selected providers;
7. preserve all rejected candidates and reasons in the generated provider report.

Filename identity is only a pairing aid. It is never the provider key.

## Corrections

A source advertised as newer or corrected does not override the baseline until one of these actions occurs:

- policy baseline update with new artifact checksum;
- approved patch record applied to the baseline provider;
- approved provider override naming the exact replacement hash.

The current baseline's 1999 `SGEIR` correction and 2023 `QK15W`/`DQK15W` corrections are selected because they are already present in the pinned baseline.

## Machine-provider precedence

The default profile selects the archive source records for `D1MACH`, `I1MACH`, and `R1MACH`. A target-specific profile may override them only when it records:

- target triple or explicit configuration identifier;
- replacement source ID and SHA-256;
- external dependencies introduced;
- compiler and linker requirements;
- numerical validation results;
- thread-safety and initialization findings.

The Linux overlay is not the default because its `D1MACH` and `R1MACH` call LAPACK inquiry routines and cache results, while its `I1MACH` activates one historical platform block.

## Exclusion precedence

An explicit exclusion always wins over a broad include rule. Under policy version 1:

- `src/sgeir.f.0` is excluded from default source selection;
- non-Fortran archive metadata are excluded from compilation;
- quick checks are excluded from production providers;
- Linux-overlay files are excluded unless a named profile enables them;
- browser/tool files are excluded from numerical compilation;
- unverified live and upstream duplicates are excluded.
