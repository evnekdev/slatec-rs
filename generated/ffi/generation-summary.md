# Complete selected-corpus raw FFI generation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- GNU Fortran profile: `gnu-fortran` (`x86_64-w64-mingw32`)
- Selected program units: 1476 from 1442 physical sources
- Sources compiled: 1442; failures: 0
- Observed defined symbols: 1742
- Generated standard bindings: 775; ABI-sensitive bindings: 480
- Manual review: 200; unsupported: 0; non-callable infrastructure: 21
- ABI profile validation: `passed`

Raw declarations are generated only for an observed compiled symbol with a supported, explicitly validated ABI class. Compiler logs, objects, archives, and source bytes remain ignored evidence. This is not a safe API and does not establish ABI correctness for gated interfaces.
