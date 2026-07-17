# Family linkage validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Families: 19
- Reviewed physical sources in the union: 229
- Native example binaries validated: 5/5.
- Single-gamma unrelated-domain retention check: passed.
- Object policy: one object per selected physical source; no whole-archive linking.
- Provider policy: checksum-pinned automatic source acquisition for `bundled`; verified local cache for `source-build`; explicit `system` and inert `external-backend` escape hatches.
- Rights boundary: source and native bytes remain outside Git and crate packages.
