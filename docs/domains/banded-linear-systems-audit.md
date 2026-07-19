# General banded LINPACK audit

The reviewed `banded-linear-systems` closure supports real single- and double-precision general-band LU factorization, direct and transpose solves, reciprocal 1-norm condition estimates, and scaled determinant metadata. It excludes dense, packed, symmetric-band, sparse, and eigenvalue routines.

`SGBCO` and `DGBCO` require expanded `2*ML+MU+1` storage and overwrite it while calculating an estimate of the reciprocal 1-norm condition. The safe API therefore copies compact input into owned private storage, checks native integer conversions and allocations, and returns factors only after excluding exact zero U diagonals. Their `Z` workspace has exact length `N`.

`SGBDI` and `DGBDI` consume neither factors nor pivots. They read compatible `GBFA`/`GBCO` output and return `DET(1), DET(2)` such that `det(A) = DET(1) * 10**DET(2)`. Pivot comparisons encode the determinant sign. The safe API exposes the pair rather than an overflow-prone unscaled value.

The selected numerical closure has no reviewed XERROR, callback, file-I/O, COMMON, SAVE, DATA, or BLOCK DATA dependency. It nonetheless remains `SerializedGlobal`: separate backend/runtime reentrancy has not been qualified, and the project preserves a common hosted-native serialization policy. `DERKF`/`DDERKF` remain separately deferred because their derivative callbacks have no abort/status channel, so caught Rust callback failures cannot safely stop native continuation.
