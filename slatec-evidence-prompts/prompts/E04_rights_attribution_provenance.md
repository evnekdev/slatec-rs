# E04 — Build the Rights, Attribution, and Provenance Register

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`. Read E01–E03 and existing licensing documentation.

## Objective

Create artifact-specific and family-specific rights records for the candidate and selected corpus. Do not assume all Netlib material has one licence. This is risk documentation, not legal advice.

## Investigation

For each artifact or family, identify:

- explicit licence text;
- copyright notice;
- public-domain or government-work statement;
- redistribution and modification permission;
- attribution or citation requirements;
- absence of explicit licence;
- conflicts;
- third-party incorporated material.

Distinguish source-code rights, manual/documentation rights, paper copyright, project metadata, and project patches.

## Required outputs

```text
docs/source-corpus/
    rights-register.md
    attribution-policy.md
    redistribution-risk.md
    provenance-recording-policy.md
metadata/
    rights.toml
```

Use cautious statuses:

- `explicit-permissive`;
- `public-domain-stated`;
- `government-work-stated`;
- `redistribution-permitted`;
- `no-explicit-license-found`;
- `conflicting`;
- `requires-review`.

## Completion criteria

Every selected family has a rights status, ambiguity remains visible, attribution requirements are captured, and packaging decisions can reference the register.
