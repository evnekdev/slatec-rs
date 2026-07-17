# Family linkage validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Families: 23
- Reviewed physical sources in the union: 261
- Native example binaries validated: 15/15.
- Single-gamma unrelated-domain retention check: passed.
- Least-squares narrow-link check: passed. `DNLS1E` intentionally retains `DNLS1`, its direct original implementation; `DCKDER` remains in that object because its optional native checking branch cannot be extracted separately.
- Object policy: one object per selected physical source; no whole-archive linking.
- Provider policy: offline cache-only `source-build`; blocked `prebuilt`; explicit `system` and inert `external-backend` escape hatches.
- Rights boundary: source and native bytes remain outside Git and crate packages.
