# Symbol record specification

## Record fields

A symbol observation contains:

- stable ID and raw spelling;
- ABI-profile normalized spelling;
- kind: `defined`, `undefined`, `weak`, `local`, `common`, `runtime`;
- object ID, archive-member ID, source-file ID, program-unit IDs;
- size/type/binding where available;
- tool and version, raw-output locator, extraction run;
- compiler/build fingerprint;
- fact and review status;
- provider and duplicate-group IDs.

## Provider records

A provider record links a callable/state symbol to an implementation variant, selected source, object, native component, feature/target predicate, and rights record. Selection is scoped to a named policy/profile.

## Duplicate groups

Groups list all observations, relationship classification, selected provider, resolution scope, blockers, and evidence. No selected build may contain two strong definitions of the same normalized external symbol unless the platform ABI explicitly distinguishes them.

## COMMON validation

COMMON symbol records reference structural layout claims. Matching linker symbols are not enough; member sequence and target layout must also validate.
