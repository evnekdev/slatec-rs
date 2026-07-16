# GNU MinGW SLATEC runtime-profile validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Compiler profile: `native-profile-7e29d91c176d0c60` (`GNU Fortran (MinGW-W64 x86_64-ucrt-posix-seh, built by Brecht Sanders, r1) 14.2.0`)
- Machine-constant discrepancies before profile support: 24
- Machine-constant discrepancies after profile support: 0
- Previously failing runtime probes recovered: 5
- Fatal error containment: child process with deterministic exit status 70
- Thread-safety: legacy error configuration and saved state are process-global and require serialization

The immutable selected source hashes are unchanged. Profile-specific machine providers and the `XERHLT` site hook are build-layer compatibility sources. ABI validity, machine-constant validity, legacy-error behavior, and FNLIB initialization are reported separately. No public safe special-function API or translated numerical algorithm is included.
