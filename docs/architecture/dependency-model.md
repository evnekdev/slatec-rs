# Dependency model and static linking

## Scope

This page defines direct and transitive dependencies for the documentation project, explains the evidence available from Netlib and prologues, and records static-linking consequences.

## Direct dependencies

The final prologue `ROUTINES CALLED` field lists SLATEC routines that are directly referenced or declared `EXTERNAL` and passed as arguments. It excludes Fortran intrinsics and excludes formal parameters that stand for caller-supplied external procedures. Entries should be alphabetized ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

Netlib separately advertises `tree1` as the list of routines directly referenced. The source register currently marks its content as linked but not yet ingested, so its exact format and source snapshot remain to be verified ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-dependency-direct`](../sources/source-register.md#core-slatec-sources)).

For this project, a **direct routine dependency** should mean a resolvable call or external-symbol reference from one program unit to another selected program unit. It should record the evidence source:

- executable `CALL` or function reference;
- `EXTERNAL` declaration and forwarding;
- prologue `ROUTINES CALLED` entry;
- Netlib `tree1` entry;
- compiler/object-file undefined symbol.

These evidence classes can disagree and should not be silently merged.

## Transitive dependencies

The transitive dependency closure of routine `R` is the union of all routines reachable by repeatedly following direct dependencies from `R`. Netlib advertises `tree` as the list of directly or indirectly referenced routines ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

The closure is source-snapshot dependent. If a selected routine has multiple implementations, compatibility shims or optional machine-specific hooks, the closure can change with build policy.

## Callbacks and unresolved externals

A caller-supplied callback is not a SLATEC routine dependency, even though it is an external call edge. The final prologue rules intentionally exclude formal callback parameters from `ROUTINES CALLED`, while including named SLATEC routines declared external and passed onward ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

`DQAG` demonstrates both edge types: it accepts caller-supplied `F` and calls named SLATEC/QUADPACK and error-handler routines. A dependency graph should represent `F` as a required callback interface, not as a missing library symbol ([`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)).

Other non-routine edges may include:

- compiler runtime functions;
- Fortran I/O runtime;
- `BLOCK DATA` initialization;
- common blocks;
- site-supplied hooks such as `FDUMP` and `XERHLT`;
- BLAS or package routines supplied by a separately linked implementation.

## Netlib “plus dependencies”

Netlib package indexes can generate a retrieval containing a selected routine plus indexed dependencies. This is useful as an independent closure check and acquisition aid, but it is not proof of complete platform linkage. Generated results can reflect Netlib's dependency database, an unspecified source snapshot and assumptions about external packages or compiler services ([`netlib-plus-dependencies`](../sources/source-register.md#core-slatec-sources)).

**Project policy:** Preserve the requested root routine, retrieval URL, date, returned file inventory and checksum whenever this mechanism is used.

## Static-library behavior

A traditional static archive contains object files. The linker normally extracts an object only when it satisfies an unresolved symbol. Consequences include:

1. A routine is usable only if its entire required symbol closure is available in searched archives or objects.
2. Archive order can matter on some toolchains, especially with circular dependencies.
3. Multiple routines compiled into one object may be pulled in together.
4. Duplicate global symbols across archives can select the wrong historical variant or fail the link.
5. `BLOCK DATA` or initialization-only units may not be extracted because no ordinary unresolved procedure symbol references them.
6. Common blocks with inconsistent declarations can link yet have invalid layout.
7. Machine and error hooks may resolve to generic defaults when a site-specific implementation was intended.

These are linker implications, not claims that every SLATEC build exhibits every problem.

## Proposed machine-readable graph

**Project proposal:** Use typed nodes and edges rather than a single unqualified call list.

Node kinds:

- routine/program unit;
- callback signature;
- common block;
- block-data initializer;
- external package/runtime;
- build-selected implementation.

Edge kinds:

- `calls`;
- `passes_external`;
- `requires_callback`;
- `reads_common` / `writes_common`;
- `initialized_by`;
- `requires_runtime`;
- `alternative_implementation`;
- `declared_in_prologue`;
- `observed_in_object`.

Every edge should carry provenance, source snapshot, confidence and verification status.

## FFI and crate-boundary implications

- A domain-specific raw crate cannot be planned from GAMS categories alone; it must include or depend on all native support needed for its routine closure.
- Shared low-level support may need a single native archive to avoid duplicate global symbols even when Rust APIs are split into several crates.
- Cycles in the routine graph are evidence for native-link units, not necessarily for public Rust modules.
- Callbacks and global-state edges affect safety but may not appear in ordinary direct-call lists.
- A facade crate can hide native closure complexity, while lower-level crates should expose exact linkage requirements.

## Verification plan

1. Pin and unpack the complete archive.
2. Parse all program units and prologues.
3. Extract executable references with a Fortran-aware parser.
4. Ingest `tree1`, `tree` and selected plus-dependency outputs.
5. Compile with the target compiler and collect undefined/defined symbols.
6. Reconcile discrepancies without discarding any evidence source.
7. Link minimal representative roots and run quick checks.
8. Record strongly connected components, duplicate symbols and global-state edges.

## Open questions

- Which snapshot generated Netlib's `tree1` and `tree` files?
- Do those files include moved package subsets and compatibility error routines?
- Which dependencies are conditional on input flags or callback choices?
- Which `BLOCK DATA` units require forced retention?
- Can one native archive safely back multiple Rust domain crates without symbol duplication?
- Which routine closures depend on a system BLAS versus the historical reference BLAS?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`slatec-dependency-direct`](../sources/source-register.md#core-slatec-sources)
- [`slatec-dependency-transitive`](../sources/source-register.md#core-slatec-sources)
- [`netlib-plus-dependencies`](../sources/source-register.md#core-slatec-sources)
- [`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)
