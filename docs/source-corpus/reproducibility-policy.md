# Source-corpus reproducibility policy

## Reproduction contract

A conforming tool must be able to reproduce the same selected and normalized corpus from declared inputs without consulting mutable directory order or undocumented local state.

## Required preserved inputs

For the default policy version 1 corpus, record and verify:

| Artifact | Role | SHA-256 |
|---|---|---|
| `slatec_src.tgz` | primary numerical baseline | `4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc` |
| `slatec_chk.tgz` | historical test corpus | `a095f74665e165fa1a4bd3f9ab6a4573135e21b1d002c05607eb9394e1c0f2ca` |
| `slatec4linux.tgz` | excluded optional overlay evidence | `eef9234f8fcb49e7f4905a11eda8f453ec2ca314029a9ce303fdbc99cff42bf3` |
| `slatecm.tgz` | excluded browser/tool evidence | `1bcaf724d92b50ea2a88604f714e61b1117e007fffc8b6476720aad3132d2b1c` |

The numerical corpus can be reproduced from the first artifact alone. The other three are comparison, test, or tooling evidence and must not affect default provider selection.

## Acquisition manifest

For every artifact, preserve:

- stable project source ID;
- requested URL and final URL;
- redirect chain;
- retrieval timestamp in UTC;
- HTTP status and content type;
- byte length;
- SHA-256;
- acquisition tool and version;
- local preservation location or content-addressed object ID.

If the original URL later serves different bytes, the new download is a new artifact instance and cannot replace the pinned object silently.

## Safe extraction

Extraction must reject absolute paths, traversal paths, escaping links, device nodes, and unsupported special members. It must preserve member names and bytes and produce a sorted member manifest containing type, size, mode where relevant, raw SHA-256, and selected/excluded status.

## Canonical selection algorithm

1. verify the baseline archive SHA-256;
2. safely extract to a fresh directory;
3. enumerate regular members in bytewise-sorted path order;
4. select `src/*.f` members;
5. exclude `src/sgeir.f.0` and all non-matching members from default compilation;
6. parse program-unit identities;
7. require unique selected providers;
8. generate raw and normalized manifests;
9. apply an explicitly ordered approved patch set, empty by default;
10. verify final hashes and emit generated outputs.

## Manifest requirements

At minimum, generated manifests must record:

- policy and schema versions;
- baseline source ID and SHA-256;
- selected and excluded member lists with reasons;
- raw and normalized hashes;
- provider-resolution results;
- active profile;
- patch IDs and order;
- tool names, versions, and configuration;
- unresolved selections and whether they block the selected profile.

Manifests and generated metadata must be deterministically sorted. Timestamps must either be omitted from hash-critical outputs or sourced from a declared reproducible-build value.

## Verification gates

A run fails when:

- an input checksum differs;
- archive extraction is unsafe;
- a selected member is missing;
- an unexpected duplicate selected program unit exists;
- a patch preimage or postimage hash differs;
- an unresolved provider is selected;
- the tool or policy version is absent;
- generated output differs on a clean rerun with identical inputs.

## Future baseline updates

A baseline update requires a new policy version and a comparison report against the previous baseline covering member additions/removals, raw and normalized changes, program-unit changes, corrections, rights evidence, and generated-output impact. Previous baseline objects and manifests remain preserved.
