# Naming and precision conventions

## Scope

This page documents SLATEC routine-name limits, type prefixes, precision families and practical exceptions relevant to future symbol inventories.

## Official naming rules

The 1993 guide limits a subprogram name to six characters and recommends unusual, distinctive names to reduce collisions. It assigns leading letters as follows:

- `D` — double-precision routine;
- `C` — complex routine;
- `Z` — reserved for possible double-precision complex routines;
- other routines should not begin with `D`, `C` or `Z` unless they have those meanings.

The guide also required equivalent routines of different precision to remain as structurally similar as possible: same logic, statement numbers and variable names where feasible, use of generic intrinsics and avoidance of unnecessary explicit conversions ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

These are project conventions, not a complete naming algorithm. Many family names had to fit the six-character limit, and inherited packages brought established names.

## `TYPE` field and equivalence families

The final prologue `TYPE` field assigns one of:

- `SINGLE PRECISION`;
- `DOUBLE PRECISION`;
- `COMPLEX`;
- `INTEGER`;
- `CHARACTER`;
- `LOGICAL`;
- `ALL`, a pseudo-type for routines not meaningfully convertible to another type.

An optional equivalence list records routine-name/type pairs. The type letters are `S`, `D`, `C`, `I`, `H`, `L` and `A`, with `H` used for character and `A` for the pseudo-type. The list includes the current routine and is ordered by those type categories ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

The table of contents uses the same letters after routine names and groups equivalent routines vertically. It shows, for example, single/double pairs, single/double/complex families, and routines classified as type `A` ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Precision families are not always prefix-only

Examples in the table of contents include:

- `ACOSH` / `DACOSH` / `CACOSH`;
- `CSEVL` / `DCSEVL`;
- `GAMMA` / `DGAMMA` / `CGAMMA`;
- `R9PAK` / `D9PAK`;
- `XADD` / `DXADD`.

These demonstrate that family relationships can involve adding a prefix, replacing a precision-indicating character, or preserving a package-specific base name. Exact family membership should therefore come from the `TYPE` field and catalogue cross-checks, not from name heuristics alone ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Complex and double-complex considerations

The final guide reserved `Z` for potential double-precision complex routines. This does not establish that every `Z...` symbol in every incorporated package follows the same provenance or that a complete double-complex family exists for each complex routine. The full source inventory must determine actual declarations and symbols ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

For FFI, Fortran `COMPLEX` and double-complex representations must be verified against the selected compiler ABI. A naming convention does not prove memory layout or return convention.

## Integer, character, logical and typeless services

The table of contents and `TYPE` grammar recognize more than floating-point precision. Integer algorithms, character utilities, logical routines and type-independent support services must retain distinct metadata. The pseudo-type `ALL` does not mean a Fortran generic function; it means that conversion to another scalar type is not a reasonable concept ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Source-level type authority

The executable declaration is the authority for the actual source signature. Prologue `TYPE`, catalogue family and name prefix are metadata that should agree but may contain historical inconsistencies. Imported package sources may use older metadata formats or implicit typing.

**Project policy:** Routine ingestion should record separately:

- declared program-unit kind;
- explicit or implicit return type;
- argument declarations;
- prologue type;
- catalogue type letter;
- declared equivalent-family members;
- inferred name stem;
- confidence and conflicts.

## FFI implications

- Never generate ABI types solely from routine names.
- Preserve external linker names independently of canonical uppercase routine identifiers; compiler mangling is toolchain-specific.
- Treat default `INTEGER`, `LOGICAL`, `REAL`, `DOUBLE PRECISION`, `COMPLEX` and character arguments as ABI-sensitive.
- Verify function-return conventions, especially complex and character functions.
- Generate family-level safe APIs only after confirming semantic equivalence, not merely similar names.
- Keep integer-width and logical-representation choices tied to a compiler/build profile.

## Open questions

- Which routines violate the nominal `D`, `C` and reserved-`Z` rules?
- How many families have incomplete single/double/complex coverage?
- Which sources depend on implicit typing in a way that disagrees with prologue metadata?
- Do any six-character names collide after compiler-specific case folding or underscore decoration?
- Which character functions have compiler-specific hidden result buffers or length arguments?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/)
