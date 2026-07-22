# Complete FISHPACK coverage

- Retained canonical drivers: 17.
- Implemented checked safe drivers: 10: HWSCRT, HSTCSP, HSTCYL, HSTPLR, HSTSSP, HWSCSP, HWSCYL, HWSPLR, HWSSSP, and POIS3D.
- Every remaining driver is explicitly raw-only or subsidiary-only with the reason recorded in `fishpack-complete-coverage.json`; no generated declaration is counted as safe coverage.
- The cylindrical/polar feature uses an audited 16-source closure for centered and staggered driver roots.  The spherical feature uses a separate 43-source closure for unit-sphere and axisymmetric roots, including the exact XERROR dependency closure needed by the reviewed GNU-MinGW machine-constant profile support.
