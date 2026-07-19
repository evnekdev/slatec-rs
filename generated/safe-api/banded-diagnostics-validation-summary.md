# Safe LINPACK general-band diagnostics

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- `SGBCO`/`DGBCO` are included because they factor the private expanded copy and estimate reciprocal 1-norm conditioning. Exact singular factors are rejected; an otherwise valid zero estimate is retained.
- `SGBDI`/`DGBDI` read existing private factors and return `mantissa * 10^exponent10`; no unscaled determinant is made primary.
- The closure contains only general-band LINPACK and its genuine BLAS dependencies. Focused source audit found no mutable numerical global, XERROR, callback, or file path, but the public API remains `SerializedGlobal` under the repository runtime policy.
