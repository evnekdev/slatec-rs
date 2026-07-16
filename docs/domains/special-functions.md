# Elementary and special functions

## Scope

SLATEC’s GAMS `C` category is one of its largest and most diverse domains. It includes elementary transcendental functions, gamma/beta families, error functions, Bessel and Airy functions, elliptic integrals, orthogonal functions, exponential integrals, angular-momentum coefficients and support for accurate Chebyshev-series evaluation ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

This domain is not one package. FNLIB is a major collection, but complex Bessel/Airy routines, elliptic integrals and other families have distinct provenance.

## Principal families

| Family | Representative routines | Typical provenance | Variants |
|---|---|---|---|
| Elementary accuracy helpers | `ALNREL/DLNREL/CLNREL`, `EXPREL/DEXPRL/CEXPRL` | FNLIB | S/D/C where available |
| Gamma and beta | `GAMMA/DGAMMA/CGAMMA`, `GAMR`, `ALNGAM`, `BETA`, incomplete gamma/beta | FNLIB and related families | S/D, selected complex |
| Error functions | `ERF/DERF`, `ERFC/DERFC` | FNLIB | S/D |
| Bessel sequences | `BESJ/DBESJ`, `BESY`, `BESI`, `BESK`, scaled variants | multiple real-function families | S/D |
| Complex Bessel/Airy | `CBESJ/ZBESJ`, `CBESK/ZBESK`, `CAIRY/ZAIRY`, `CBIRY/ZBIRY` | Amos-associated family | complex single/double naming |
| Airy real functions | `AI/DAI`, `AIE/DAIE`, `BI/DBI`, `BIE/DBIE` | FNLIB/related | S/D |
| Carlson elliptic integrals | `RC/DRC`, `RD/DRD`, `RF/DRF`, `RJ/DRJ` | Carlson algorithms | S/D |
| Chebyshev support | `CSEVL/DCSEVL`, `INITS/INITDS` | FNLIB infrastructure | S/D |
| Legendre functions | `XLEGF/DXLEGF`, `XNRMP/DXNRMP` | specialized family | S/D |
| Angular momentum | `RC3JJ/DRC3JJ`, `RC3JM`, `RC6J` | specialized physical functions | S/D |
| Exponential/logarithmic integrals | `E1/DE1`, `EI/DEI`, `EXINT/DEXINT`, `SPENC/DSPENC` | FNLIB/individual | S/D |

Sources: [`slatec-toc`](https://www.netlib.org/slatec/toc), [`slatec-fnlib`](https://www.netlib.org/slatec/fnlib/).

## FNLIB architecture

FNLIB uses Chebyshev expansions, carefully selected argument regions, scaling and machine constants. Initialization routines determine how many series coefficients are needed for the target precision. Many public functions call subsidiary routines for argument reduction, series evaluation and overflow/underflow checks.

Netlib explicitly warns that the relationship between `slatec/fnlib` and another `/fn` collection is unresolved. Therefore FNLIB source lineage must be established by file comparison rather than assumed ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

## Scaling and sequence interfaces

Bessel/Airy routines commonly provide:

- unscaled and exponentially scaled variants;
- sequences of consecutive orders;
- underflow counts;
- region or error flags;
- real order with real or complex argument.

Safe APIs should preserve scaled variants as explicit operations. Automatically rescaling can overflow and erase the reason the scaled routine exists.

## Precision variants

The table of contents uses vertical routine groups and type markers, but families are irregular:

- many functions have S/D pairs;
- selected elementary functions have complex variants;
- Amos routines use `C` and `Z` prefixes for complex single/double;
- some physical-function routines are real only;
- integer-valued functions still return floating values for range reasons.

Generic Rust traits should not imply unsupported scalar combinations.

## Dependencies

Common dependencies include:

- machine constants;
- error handling;
- Chebyshev evaluation;
- gamma/log-gamma support;
- complex arithmetic helpers;
- sequence recurrence and scaling routines.

The dependency graph within special functions is highly interconnected and may create large strongly connected groups.

## Numerical limitations

- every function has argument-domain and overflow/underflow limits;
- near singularities or zeros, relative error can be ill-conditioned;
- sequence recurrences may be stable only in one direction;
- branch cuts and principal values matter for complex functions;
- unscaled outputs may overflow when scaled outputs remain useful;
- historical machine-constant assumptions must be modernized carefully;
- status flags and partial sequences can remain meaningful after underflow.

## Modern equivalents

AMOS-derived complex Bessel routines, Cephes-like libraries, Boost.Math, SciPy Special and Rust special-function crates provide overlapping capabilities. Differences in branch conventions, accuracy, supported orders and status behavior require numerical comparison rather than name matching.

## FFI considerations

| Risk | Mitigation proposal |
|---|---|
| complex ABI | compiler-specific ABI tests or real/imag pair wrappers |
| domain errors via global error handler | capture and translate without discarding raw status |
| output sequences and underflow counts | return structured sequence results |
| scaled/unscaled ambiguity | distinct methods/types |
| workspace and initialization state | internal owned initialization or one-time constants |
| branch conventions | document directly from source/manual |
| irregular precision families | generated per-routine bindings, not broad unsafe generics |

## Project proposals

Safe modules could include:

```text
elementary
gamma_beta
error_functions
bessel
airy
elliptic
orthogonal
angular_momentum
exponential_integrals
```

Result types should distinguish value-only functions from status-bearing sequence functions. A facade may use traits for `f32`/`f64` only where both implementations are verified.

## Tentative crate ownership

- Raw: `slatec-fnlib-sys`, `slatec-amos-sys`, and additional family crates only if provenance and dependencies justify them.
- Safe: one `slatec-special` crate with family modules, unless compile/link size motivates subcrates.
- Runtime machine constants and errors remain shared dependencies.

## Open questions

- What exact routine inventory belongs to FNLIB versus other families?
- Which files differ from Netlib `/fn`?
- What are the complex branch conventions for every Amos routine?
- Which functions are thread-safe despite initialization or saved state?
- How should partial-underflow sequence results be represented?
- What accuracy tests and reference datasets are authoritative?
- Do any routines depend on nonportable complex intrinsics or machine hooks?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-fnlib`](https://www.netlib.org/slatec/fnlib/)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
