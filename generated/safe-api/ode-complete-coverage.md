# Complete ODE and DAE coverage

- Candidate public drivers and required subsidiaries are recorded from the selected corpus, rather than inferred from Rust path names.
- Reviewed safe sessions: SDRIV1/DDRIV1, SDRIV2/DDRIV2, SDRIV3/DDRIV3 (functional, internal finite-difference, and checked analytic `JACOBN` modes), CDRIV1/CDRIV2, and residual-only SDASSL/DDASSL with internal dense/banded finite differences.
- Explicit deferrals: DERKF/DDERKF and DEABM/DDEABM lack a documented callback abort; DEBDF/DDEBDF and INTYD/DINTYD retain COMMON history; CDRIV3 and SDRIV3 mass/event callbacks require their own ABI and lifecycle reviews; DASSL analytic `JAC` lacks a native abort protocol.
- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
