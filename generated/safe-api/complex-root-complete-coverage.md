# complex-root safe-coverage disposition

This generated inventory joins canonical raw status with raw-to-safe coverage. `expert-raw-only` and blocked records are explicit decisions, not missing data. Call-graph fields are recorded as unavailable where the committed authoritative inputs do not contain a complete Fortran call graph.

## Disposition counts

- `candidate-implemented-in-this-milestone`: 4
- `direct-safe-wrapper`: 2

## Routine records

| Routine | Role | Raw path | Safe path | Provider feature | Docs | Link | Runtime | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `CPQR79` | historically_user_callable_driver | `slatec_sys::roots::complex::cpqr79` | `slatec::roots::complex_polynomial_roots_with_method` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `passed_focused_polynomial_roots_native` | `candidate-implemented-in-this-milestone` |
| `CPZERO` | historically_user_callable_driver | `slatec_sys::roots::complex::cpzero` | `slatec::roots::complex_polynomial_roots` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `passed_focused_polynomial_roots_native` | `candidate-implemented-in-this-milestone` |
| `DFZERO` | historically_user_callable_driver | `slatec_sys::roots::scalar::dfzero` | `slatec::roots::find_root` | `roots-scalar` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `FZERO` | historically_user_callable_driver | `slatec_sys::roots::scalar::fzero` | `slatec::roots::find_root_f32` | `roots-scalar` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `RPQR79` | historically_user_callable_driver | `slatec_sys::roots::complex::rpqr79` | `slatec::roots::real_polynomial_roots_with_method` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `passed_focused_polynomial_roots_native` | `candidate-implemented-in-this-milestone` |
| `RPZERO` | historically_user_callable_driver | `slatec_sys::roots::complex::rpzero` | `slatec::roots::real_polynomial_roots` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `passed_focused_polynomial_roots_native` | `candidate-implemented-in-this-milestone` |
