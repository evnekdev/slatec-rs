# Storage interoperability policy

- Audited native arguments: 1601; matrix arguments: 78.
- The core API accepts slices and its existing checked views. A matrix is passed only in the documented Fortran column-major layout with the documented leading-dimension contract.
- No implicit transpose, dense-to-packed conversion, packed-to-dense expansion, CSR/CSC conversion, arbitrary-stride materialization, or hidden repacking is permitted. An owned preservation copy is allowed only when a reviewed native routine mutates its storage or requires an opaque workspace, and it preserves the logical column-major order.
- `matrixpacked` 0.1.0 and `nalgebra` 0.35.0 are audited for possible future optional adapters only. Neither is accepted by the current core API. A future adapter must validate exact layout, contiguity/stride, mutability, scalar support, and native leading dimensions; packed triangular storage cannot be passed to a routine expecting a full rectangular matrix.
- `ndarray` and `faer` remain possible optional integrations, but no core dependency or compatibility promise is introduced.
