# FFI design proposal

## Status

This document proposes the future FFI boundary for `slatec-rs`. No binding implementation exists merely because a routine appears in the metadata catalogue.

## Design goals

- preserve the historical routine semantics and status values;
- isolate compiler-specific ABI details;
- generate declarations from verified metadata where possible;
- avoid handwritten duplication across hundreds of routines;
- make unsafe preconditions explicit;
- support callbacks, character arguments, complex values and persistent workspaces;
- allow safe crates to present modern ownership and result models.

## Layered FFI

### Layer 0 — original Fortran

Pinned, checksummed source selected by generated manifests. Patches are minimal and separately recorded.

### Layer 1 — ABI adaptation

Possible forms:

1. direct binding to compiler-mangled symbols;
2. generated modern Fortran `BIND(C)` wrappers;
3. generated C wrappers around legacy symbols;
4. a mixed approach.

Preferred direction:

- use direct declarations for simple scalar/array subroutines after ABI probes;
- use generated wrappers for character arguments, complex-valued functions, callbacks or compiler-dependent return conventions;
- use explicit wrapper symbols with project-owned names;
- never rename the underlying historical routine in provenance metadata.

### Layer 2 — raw Rust declarations

Raw declarations expose ABI-level arguments with minimal interpretation:

```text
unsafe extern "C" fn(
    n: *const FortranInt,
    x: *mut f64,
    ...
)
```

The raw layer does not:

- allocate workspaces;
- convert status codes into one generic error;
- infer array lengths;
- promise thread safety;
- accept arbitrary Rust closures;
- convert one-based indices automatically.

### Layer 3 — checked internal adapters

Internal Rust adapters validate:

- scalar ranges and integer conversion;
- array lengths;
- leading dimensions;
- packed/banded storage lengths;
- nullability;
- non-overlap where required;
- callback scope;
- workspace formulas;
- session-state transitions.

### Layer 4 — safe domain APIs

Safe wrappers own workspaces, model destructive outputs and return method-specific result structures.

## ABI type policy

### Fortran integers

Define one generated type alias per supported build configuration:

```text
FortranInt
```

Do not assume it equals Rust `usize`. All dimensions and indices are checked before conversion. The build fingerprint records the detected size and compiler mode.

### Logical values

Do not map Fortran `LOGICAL` directly to Rust `bool` without compiler-specific verification. Wrappers should use integer/logical adapter functions if the representation is not proven.

### Real values

Single and double precision map to `f32` and `f64` only after confirming compiler sizes. Historical precision families remain separate metadata records rather than one unsafe generic declaration.

### Complex values

Risks include:

- function-return ABI;
- alignment/layout;
- hidden result arguments;
- C interoperability differences.

Preferred policy:

- use `BIND(C)` or C wrapper shims for complex functions;
- use verified C-compatible complex structs or split real/imag arguments;
- keep raw and safe representations distinct;
- test both scalar returns and array arguments.

### Character arguments

Fortran 77 character routines may receive hidden length arguments whose position and type vary by compiler. Use wrappers that expose explicit pointer-plus-length C interfaces. Avoid direct cross-compiler declarations.

## Symbol strategy

A generated symbol manifest maps:

```text
canonical routine ID
-> selected implementation
-> source program unit
-> compiler symbol
-> optional stable wrapper symbol
-> native component
```

Direct mangled names remain an implementation detail. Stable wrappers should be prefixed to avoid collisions, for example:

```text
slatec_rs_dqag
slatec_rs_ddassl
```

The exact prefix is provisional.

## Array and matrix policy

### One-dimensional arrays

Raw pointers retain the original mutable/const ambiguity where the ABI does not encode intent. Checked adapters use prologue/source evidence to select immutable or mutable slices. An `inferred` intent is not sufficient for a safe public wrapper without review.

### Column-major matrices

Use checked column-major views with logical dimensions and leading dimension. Do not copy automatically unless the safe API explicitly chooses ownership over zero-copy behavior.

### Packed and banded storage

Define distinct internal types:

```text
PackedSymmetric
PackedHermitian
GeneralBand
SymmetricBand
TriangularBand
```

Each validates the routine-specific storage formula.

### Destructive transformations

Factorization and solver routines often overwrite inputs. Safe constructors should consume owned arrays or accept explicit mutable views and return factor/session objects.

## Workspaces

Three categories require different treatment:

1. disposable per-call workspace;
2. output workspace containing diagnostics;
3. persistent state required across continuation calls.

Safe wrappers allocate category 1 internally, return or summarize category 2, and own category 3 inside a session/plan object.

Workspace formulas come from verified routine metadata and are evaluated with checked integer arithmetic.

## Error handling

A call can report failure through multiple channels:

- explicit status argument;
- output flag;
- callback flag;
- `XERROR`/`XERMSG`;
- fatal site hook or `STOP`;
- compiler runtime failure.

The FFI layer must preserve explicit raw status and capture global diagnostics where possible. Safe result enums remain routine/family specific.

No wrapper should treat a successful native return as success if a captured fatal SLATEC error occurred.

## Global state classification

Every routine should eventually have a safety classification:

```text
pure_or_local
persistent_caller_state
read_only_global
mutable_global
unknown
```

Until verified, `unknown` routines are not documented as thread-safe. If the selected error subsystem uses mutable common state, even otherwise local routines may require serialization.

## Generated bindings

Binding generation consumes the routine catalogue and dependency metadata. Generation should fail closed when:

- a required argument type is unknown;
- character ABI is unresolved;
- a callback signature is incomplete;
- source ownership is ambiguous;
- duplicate provider selection is unresolved;
- workspace formulas cannot be represented safely.

Generated files include provenance comments and schema/build versions.

## Testing strategy

### ABI probes

- integer/logical sizes;
- symbol names;
- real and complex argument/return conventions;
- character hidden lengths;
- callback invocation;
- common-block layout;
- `BLOCK DATA` retention.

### Routine smoke tests

Use official quick checks where available, plus small deterministic calls for representative interface classes.

### Differential tests

Compare:

- raw Fortran driver output;
- Rust wrapper output;
- upstream package test values;
- modern reference implementations where appropriate.

Differences are investigated, not automatically normalized.

## Backend abstraction

Safe APIs may later support multiple implementations, but backend choice must be explicit:

```text
SlatecHistorical
SystemBlas
LapackCompatible
```

A backend trait should only cover operations with genuinely compatible semantics. It must not imply that LINPACK/EISPACK and LAPACK are interchangeable at routine level.

## Versioning impact

Changing raw C/Fortran ABI declarations is potentially breaking even if a safe API is unchanged. Therefore:

- raw crates use conservative semver;
- generated-binding schema versions are recorded;
- safe wrappers avoid exposing raw integer/status layout unnecessarily;
- provider/build metadata is available for bug reports.

## Sources

- [Routine prologues and calling conventions](../architecture/routine-prologues.md)
- [Native build strategy](native-build-strategy.md)
- [Machine constants](../architecture/machine-constants.md)
- [Error handling](../architecture/error-handling.md)
- [Routine metadata schema](../../metadata/routine-schema.md)
- [Domain surveys](../domains/overview.md)
