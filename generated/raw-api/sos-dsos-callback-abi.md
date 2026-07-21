# SOS/DSOS callback ABI

`FNC` is a synchronous scalar function. `SOS` uses `unsafe extern "C" fn(*const f32, *const FortranInteger) -> f32`; `DSOS` uses the corresponding `f64` type. `X` is a readable `NEQ`-element vector during the callback and `K` is a one-based equation index. There is no hidden result argument. The focused native tests exercise both callbacks and verify that both equation indices are observed.
