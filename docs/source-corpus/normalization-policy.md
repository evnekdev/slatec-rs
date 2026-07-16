# Source normalization policy

## Principle

The preserved artifact and canonical extracted source are immutable evidence. Normalization creates a separate deterministic view for parsing and comparison; it never edits the preserved or canonical bytes.

## Permitted normalization for the parser view

Policy version 1 permits only:

1. safe archive-path normalization in metadata: remove a leading `./`, use `/` as the recorded separator, and reject absolute or traversal paths;
2. text decoding as bytes compatible with ASCII for the historical source set, while preserving original bytes separately;
3. conversion of CRLF or lone CR line endings to LF;
4. creation of an ASCII-casefolded comparison key for paths and program-unit names without renaming canonical files;
5. generation of derived hashes and token streams with the exact normalizer/tool version recorded.

## Prohibited normalization

The normalized working files must not automatically:

- trim leading whitespace;
- expand or collapse tabs;
- reflow fixed-form statements;
- change columns 1–72 or sequence fields;
- remove comments or prologues from the file representation;
- change letter case;
- rewrite numeric literals;
- rename routines or files;
- resolve continuation lines into free-form source;
- insert `IMPLICIT NONE`;
- modernize declarations;
- select machine-constant blocks;
- replace error handlers;
- run a formatter.

Comment-stripped and executable-token representations may be generated for comparison, but they are derived analysis products, not normalized source files and not compilation inputs unless separately approved.

## Deterministic output layout

A normalizer must write to a new root, preserving archive-relative paths. Each output record must include:

- baseline artifact ID and SHA-256;
- original member path;
- original raw SHA-256;
- normalization policy version;
- normalizer name and version;
- normalized SHA-256;
- applied transformations;
- generation timestamp or reproducible-build timestamp policy.

The same inputs, policy version, and tool version must produce byte-identical normalized output and a stable sorted manifest.

## Fixed-form safety

Because fixed-form Fortran uses column position semantically, whitespace transformations beyond line-ending normalization require a future policy revision backed by parser and compiler tests. Trailing-whitespace removal is not part of the emitted normalized source policy, even if a separate comparison hash uses it.

## Build inputs

A build may compile either:

- canonical extracted bytes; or
- LF-normalized files generated under this policy.

The chosen mode must be recorded in the build manifest. Project patches apply to the declared source representation and must specify whether their preimage is canonical bytes or normalized bytes.
