# Small raw-API candidate batch review

This focused evidence resolves every requested candidate. `DFQAD` and `DVSUP` are exact-name misses, not withheld precision counterparts. `CDRIV3` and `DBVSUP` remain non-public because their independent callback contracts are not ABI-proven.

| Routine | Historical role | Final disposition | Canonical path | ABI evidence |
| --- | --- | --- | --- | --- |
| `CDRIV1` | ordinary complex single-precision initial-value ODE driver | `canonical-public` | slatec_sys::ode::cdriv1 | validated complex32 argument callback through focused native integration |
| `CDRIV2` | advanced complex single-precision ODE driver with root controls | `canonical-public` | slatec_sys::ode::cdriv2 | validated complex32 derivative and real root-function callbacks through focused native integration |
| `CDRIV3` | expert complex ODE driver | `unsupported-abi` | — | unsupported advanced complex callback ABI |
| `DBVSUP` | boundary-value support routine with implicit external dependencies | `unsupported-abi` | — | unsupported implicit external-procedure ABI |
| `DDRIV1` | ordinary double-precision initial-value ODE driver | `canonical-public` | slatec_sys::ode::ddriv1 | validated real f64 derivative subroutine callback |
| `DDRIV2` | advanced double-precision ODE driver with method and root controls | `canonical-public` | slatec_sys::ode::ddriv2 | validated real f64 derivative and root-function callbacks |
| `DDRIV3` | expert double-precision ODE driver | `canonical-public` | slatec_sys::ode::ddriv3 | previously reviewed expert callback ABI |
| `DFQAD` | no retained SLATEC program unit | `missing-symbol` | — | not-applicable |
| `DPFQAD` | independently callable double-precision piecewise-polynomial quadrature driver | `canonical-public` | slatec_sys::quadrature::dpfqad | validated scalar f64 callback; direct scalar return |
| `DVSUP` | no retained SLATEC program unit | `missing-symbol` | — | not-applicable |
| `SDRIV1` | ordinary single-precision initial-value ODE driver | `canonical-public` | slatec_sys::ode::sdriv1 | validated real f32 derivative subroutine callback |
| `SDRIV2` | advanced single-precision ODE driver with method and root controls | `canonical-public` | slatec_sys::ode::sdriv2 | validated real f32 derivative and root-function callbacks |
| `SDRIV3` | expert single-precision ODE driver | `canonical-public` | slatec_sys::ode::sdriv3 | previously reviewed expert callback ABI |

The JSON companion records source hashes, exact Netlib URLs, native symbols, provider features, direct callers/callees, callback counts, link/runtime statuses, and the precise reason for each non-public disposition.
