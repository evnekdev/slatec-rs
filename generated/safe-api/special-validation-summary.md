# Safe real scalar special-function expansion

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Reviewed scalar-special candidates: 127
- Safe Rust wrappers: 88
- New scope: logarithmic integral, Spence's integral, and Carlson RC/RF/RD/RJ in `f32` and `f64`
- Deferred: complex ABI, sequence/workspace APIs, CHU and Pochhammer reliability gaps, and cotangent's fatal near-pole path
- Native state: `SAVE`/`DATA` initialization and XERROR require process-global serialization
- Focused native evidence: GNU MinGW source-build object inspection and the narrow link probe completed; the broad audit worker did not reach a final report in this host run, so it supplies no concurrency-relaxation evidence
- Provider policy: the reviewed GNU MinGW source backend is serialized; external and system providers remain backend-dependent

No translated approximation, native binary, or source artifact is included.
