# Project patch policy

## Scope

Project patches are deliberate changes applied after canonical source selection. They are not edits to the preserved archive and must never be folded silently into a copied source tree.

## Patch record requirements

Every patch must have a stable ID and record:

- title and purpose;
- affected source ID and archive member path;
- baseline artifact SHA-256;
- exact preimage SHA-256;
- patch file SHA-256;
- expected postimage SHA-256;
- patch format and application tool version;
- statement class: upstream correction, portability, build integration, ABI adaptation, error-policy adaptation, machine-provider change, or project bug fix;
- upstream URL, notice, commit, release, or correspondence when applicable;
- rights and attribution notes;
- author, reviewer, approval status, and date;
- tests and compiler matrix;
- whether numerical executable behavior changes;
- rollback or disable mechanism.

## Patch application

Patch application must:

1. verify the baseline and preimage hashes before modification;
2. apply patches in an explicitly ordered list;
3. reject fuzz, offset guessing, or partial application by default;
4. verify each expected postimage hash;
5. emit a patch ledger and final working-corpus manifest;
6. fail if two patches modify the same region without a declared dependency/order relationship.

## Upstream corrections

A later Netlib or original-package correction may be incorporated as either:

- a new baseline policy version; or
- an upstream-correction patch.

The project must retain the original baseline and the evidence for the corrected source. A URL or newer date alone is insufficient; exact bytes and a classified diff are required.

## Portability and machine patches

Machine constants, symbol naming, compiler compatibility, error interception, and build-system changes must remain separate patch classes. A portability patch must not be described as normalization.

The Linux overlay is treated as an alternate provider/patch source. Its files cannot be copied over the baseline without individual patch records because they make executable changes and introduce external LAPACK dependencies.

## Source rewriting and generated bindings

Automatic Fortran modernization, source splitting, symbol renaming, or wrapper insertion is not a normalization operation. Such work must be represented as project patches or generated-source transformations with complete preimage/postimage provenance.

Generated Rust declarations and safe wrappers are outputs, not patches to the canonical Fortran corpus.

## Patch storage

Patch descriptions and small textual patch files may be version-controlled when rights permit. Original third-party artifacts remain in the preserved-artifact store according to the rights policy. Patch IDs and hashes must remain stable after publication; corrections create a new patch revision rather than rewriting history.
