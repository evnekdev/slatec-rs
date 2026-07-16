# Native component proof requirements

## Assignment of a unit to a component

Minimum proof:

- checksum-pinned selected source and provider;
- complete parsed direct references for the unit;
- classified unresolved references;
- common, saved-state, block-data, callback, entry, and equivalence records;
- one selected source owner and one selected symbol provider;
- component link test for supported build profiles;
- no open ownership/provider conflict.

Domain or GAMS classification alone is insufficient.

## Separating a native archive

Before splitting an archive, prove:

- all cross-boundary link-required edges are declared component dependencies;
- SCCs under the selected native edge profile are not split, unless the linker recipe is explicitly validated;
- no source file or object is compiled into multiple components;
- duplicate symbols are resolved;
- BLOCK DATA retention and COMMON ownership remain valid;
- representative minimal roots link without undeclared providers.

## Independent `*-sys` crate

An independent raw crate additionally requires:

- a stable native component manifest and build contract;
- deterministic source/provider selection;
- supported compiler/target ABI probes;
- complete link requirements exposed to Cargo consumers;
- coordinated symbol ownership with sibling crates;
- rights status permitting the proposed distribution scope;
- no need for another raw crate to compile the same upstream source.

Rust API separation may precede native archive separation; one native archive may support several Rust crates.

## Thread-safety claim

A thread-safety claim requires reviewed evidence for all reachable state:

- `COMMON`, `SAVE`, initialized data, error subsystem state, I/O units, callbacks, machine-provider caches, and external libraries;
- recursive/reentrant behavior where applicable;
- target/compiler-specific storage behavior;
- concurrency stress or controlled experiments;
- absence or synchronization of mutable shared state.

“No COMMON found” or successful parallel calls alone is insufficient. Unknown reachable state blocks a positive thread-safety claim.
