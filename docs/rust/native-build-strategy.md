# Native build and link strategy

## Status

This is a proposed implementation strategy for future work. Prompt 08 does not implement the build system.

## Goals

- reproducible acquisition and source selection;
- one definition for every native symbol;
- deterministic feature-to-source manifests;
- support for multiple platforms and Fortran compilers;
- build-time graph and symbol validation;
- clear bundled-versus-external provider policy;
- preservation of original source checksums and notices.

## Source acquisition

The build must not depend on mutable network downloads during ordinary Cargo builds. Pin upstream artifacts outside the build process, record checksums, and vendor or stage reviewed sources according to verified redistribution rights.

Maintain a generated source manifest containing:

- original URL and source ID;
- original checksum;
- repository path;
- selected implementation group;
- applied patch identifiers;
- resulting checksum;
- native component owner.

Patches should be minimal, reviewable and mechanically reproducible.

## Compilation granularity

Compile one source file per object where practical. This improves archive extraction, duplicate detection and source-to-symbol traceability. Preserve files containing multiple program units as one object unless a reviewed transformation deliberately separates them.

For every object, capture:

- defined global symbols;
- undefined global symbols;
- common symbols;
- local/static symbols when useful;
- compiler/runtime references;
- source checksum and compiler flags.

## Compiler matrix

Initial validation should include at least the primary supported GNU Fortran toolchain on Linux and Windows where feasible. Additional compilers require explicit ABI tests for:

- symbol mangling;
- default integer/logical representation;
- real and complex argument/return conventions;
- hidden character lengths;
- callback calling convention;
- common-block naming/layout;
- `BLOCK DATA` retention;
- runtime library linkage.

Do not claim cross-compiler binary compatibility merely because source compilation succeeds.

## Native component strategy

### Phase 1

Build one native archive/shared object from a conflict-resolved source set. Generate logical Rust modules around it. This minimizes duplicate support routines and cycle problems.

### Phase 2

Use the validated dependency graph to produce smaller archives. Each component manifest contains complete source closure plus declared component dependencies.

### Phase 3

Optionally permit external providers for BLAS or other packages. Provider selection must remove bundled conflicting implementations and verify ABI compatibility.

## Static archives

Archive order can matter when components are cyclic. Preferred approaches, in order:

1. collapse source-level SCCs into one archive/component;
2. order a component DAG deterministically;
3. use linker group/rescan options only as a platform-specific fallback;
4. avoid repeating archives on command lines as an undocumented fix.

`BLOCK DATA` may not be extracted by an ordinary unresolved procedure reference. The build must prove retention through object inspection/runtime tests or force inclusion explicitly.

## Shared libraries

A single shared library can simplify duplicate-symbol control and runtime state sharing, but introduces deployment and symbol-export concerns. Export only intended routine symbols where toolchains permit, while retaining internal dependencies.

The first prototype should compare:

- static bundled archive;
- shared native library;
- one monolithic versus several component archives.

## External BLAS policy

Provide explicit mutually exclusive policies:

- historical bundled BLAS for reproducibility;
- system/tuned BLAS for performance;
- no BLAS-dependent components.

When using system BLAS:

- omit bundled definitions;
- validate required routine set and integer ABI;
- record provider identity at build/run time where possible;
- test numerical and symbol compatibility;
- do not imply byte-for-byte historical behavior.

The same pattern may later apply to other packages, but should not be generalized before evidence exists.

## Name mangling and wrapper layer

Directly binding compiler-mangled Fortran symbols is fragile. Candidate strategies are:

1. compiler-specific generated declarations;
2. generated Fortran `BIND(C)` shims where a modern compiler can compile them;
3. generated C wrappers around legacy routines;
4. a mixed approach for callbacks, characters and complex functions.

Shims must preserve original routine semantics and be separately owned symbols. They do not change historical provenance of the underlying routine.

## Callback support

Callback-heavy routines need:

- ABI-correct trampoline functions;
- scoped context storage;
- prevention of Rust panic unwinding through native frames;
- cancellation/error transport using documented solver controls where possible;
- reentrancy and concurrency tests;
- cleanup after abnormal return.

Global callback registries should be avoided or serialized until nested-call behaviour is proven.

## Error and machine support

Select exactly one error subsystem and one implementation for machine constants/hooks. Test whether error routines use shared common state and whether fatal paths can terminate the host.

Potential policy:

- retain raw historical status and messages;
- configure nonfatal handling where supported;
- intercept site hooks with reviewed shims;
- serialize native calls if mutable process-global error state cannot be isolated.

## Build validation pipeline

For each supported feature/provider configuration:

1. resolve source and implementation choices;
2. reject duplicate selected program units/symbols;
3. generate component manifests;
4. compile objects;
5. inspect symbols;
6. compare observed undefined symbols with predicted dependencies;
7. archive/link components;
8. link minimal root executables;
9. run ABI probes;
10. run quick checks and representative smoke tests;
11. emit machine-readable validation results.

A build may compile successfully yet fail validation if it resolves symbols accidentally from undeclared host libraries.

## Cargo integration

One crate should own the native `links` identity for a selected native installation. Other raw/safe crates should depend on that owner rather than invoking independent builds.

Generated build outputs should include:

- link search paths;
- link libraries in deterministic order;
- provider configuration;
- compiler/runtime libraries;
- enabled routine/component manifest;
- build fingerprint.

Build scripts should not perform broad source discovery at runtime. They should consume generated manifests committed or produced by a controlled regeneration step.

## Platform testing

Test at minimum:

- debug and release optimization;
- static and shared linkage where supported;
- 32- and 64-bit default address spaces;
- Windows and ELF symbol conventions;
- callbacks;
- character arguments;
- complex function/subroutine interfaces;
- common blocks and block data;
- external BLAS configurations;
- parallel calls to routines classified as potentially global-state free and stateful.

## Reproducibility outputs

Every produced native library should have an adjacent manifest with:

- source and patch checksums;
- compiler identity/version;
- flags and target;
- selected components/providers;
- graph schema version;
- defined-symbol inventory checksum;
- quick-check results;
- build timestamp or reproducible-build metadata.

## ChatGPT versus Codex

ChatGPT is suitable for reviewing policies, interpreting specific failures and documenting platform findings. Codex High should implement the manifest generator, parser integration, build scripts, symbol inspection, matrix CI and regression tests.

## Open questions

- Is a Fortran compiler acceptable as a build-time dependency for all target users?
- Should prebuilt native artifacts be published for common platforms?
- Can modern `BIND(C)` shims be generated without changing numerical behaviour?
- Which routines require object-level whole-archive retention?
- How should system BLAS integer-width variants be selected?
- Can global error state be isolated in one shared library instance?
