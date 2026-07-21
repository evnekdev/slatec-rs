# Public module stability policy

The safe API roadmap uses two independent labels.

- **Frozen**: a high-level mathematical domain whose name is intended to be a
  durable public navigation path.
- **Reserved**: an accepted leaf path with no promised callable signature.
- **Provisional**: a path retained for navigation while source-family
  selection or terminology remains unsettled.
- **Implemented**: a status label meaning a reviewed initial safe scope is
  public.

Reserved placeholders contain documentation modules only. They add no safe
functions, native entry points, feature flags, or semver-promised signatures.
A placeholder becomes implemented only after a focused native inventory,
reviewed ABI, typed Rust API, checked storage/workspace policy, error/status
mapping, state/concurrency audit, tests, and deterministic metadata.

An audit may move a status backward when evidence reveals an unsafe native
protocol. Before the first release, the canonical path is corrected directly.
