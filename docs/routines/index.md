# Pilot routine index

The pilot contains 20 routines spanning runtime support, machine constants, dense linear algebra, quadrature, interpolation, nonlinear equations, optimization, differential equations, special functions, transforms and sparse methods.

| Routine | Domain | Package/family | Source access | Page |
|---|---|---|---|---|
| `XERMSG` | `runtime-support` | `slatec-error` | `verified` | [xermsg](pilot/xermsg.md) |
| `D1MACH` | `runtime-support` | `slatec-machine-constants` | `verified` | [d1mach](pilot/d1mach.md) |
| `I1MACH` | `runtime-support` | `slatec-machine-constants` | `verified` | [i1mach](pilot/i1mach.md) |
| `DAXPY` | `linear-algebra-kernels` | `blas` | `linked-unrendered` | [daxpy](pilot/daxpy.md) |
| `DGEFA` | `dense-linear-algebra` | `linpack` | `linked-unrendered` | [dgefa](pilot/dgefa.md) |
| `DQAG` | `quadrature` | `quadpack` | `verified` | [dqag](pilot/dqag.md) |
| `DPCHIM` | `interpolation` | `pchip` | `verified` | [dpchim](pilot/dpchim.md) |
| `DPCHFE` | `interpolation` | `pchip` | `linked-unrendered` | [dpchfe](pilot/dpchfe.md) |
| `DFZERO` | `nonlinear-equations` | `individual` | `verified` | [dfzero](pilot/dfzero.md) |
| `DNSQ` | `nonlinear-equations` | `minpack-derived` | `verified` | [dnsq](pilot/dnsq.md) |
| `DNLS1` | `optimization` | `minpack-derived` | `verified` | [dnls1](pilot/dnls1.md) |
| `DDASSL` | `ode-dae` | `dassl` | `verified` | [ddassl](pilot/ddassl.md) |
| `DGAMMA` | `special-functions` | `fnlib` | `verified` | [dgamma](pilot/dgamma.md) |
| `ZBESJ` | `special-functions` | `amos-special-functions` | `verified` | [zbesj](pilot/zbesj.md) |
| `CFFTF` | `transforms` | `fftpack` | `linked-unrendered` | [cfftf](pilot/cfftf.md) |
| `RFFTF` | `transforms` | `fftpack` | `verified` | [rfftf](pilot/rfftf.md) |
| `DGMRES` | `sparse-linear-algebra` | `slap` | `verified` | [dgmres](pilot/dgmres.md) |
| `DSMV` | `sparse-linear-algebra` | `slap` | `linked-unrendered` | [dsmv](pilot/dsmv.md) |
| `DS2Y` | `sparse-linear-algebra` | `slap` | `linked-unrendered` | [ds2y](pilot/ds2y.md) |
| `XERBLA` | `runtime-support` | `blas` | `linked-unrendered` | [xerbla](pilot/xerbla.md) |

## Coverage notes

`verified` means the official source content was opened during this pass. `linked-unrendered` means the official Netlib path was located but the browsing service did not return its body; those records deliberately mark argument intent as inferred and retain ABI questions for later verification.

## Sources

Each page links directly to its official Netlib source. Cross-cutting classification and provenance use the repository source register, SLATEC table of contents, package metadata and citation policy.
