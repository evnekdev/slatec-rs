//! Hand-reviewed declaration for SLATEC FISHPACK `POIS3D`.
//!
//! The selected driver is `fishfft/pois3d.f`. Its scalar mode and dimension
//! arguments are read-only. For cyclic third-axis problems it temporarily
//! overwrites and restores `a`, `b`, and `c`; `f`, `ierror`, and `w` are also
//! mutable. `f` has Fortran layout `F(LDIMF, MDIMF, N)`.

use crate::FortranInteger;

unsafe extern "C" {
    /// Solves the reviewed structured three-dimensional block-tridiagonal system.
    ///
    /// Original SLATEC routine: `POIS3D`; supported ABI profile:
    /// `ffi-profile-gnu-mingw-x86_64`; native symbol: `pois3d_`. Source:
    /// <https://www.netlib.org/slatec/fishfft/pois3d.f>.
    ///
    /// # Arguments
    ///
    /// - `LPEROD`, `MPEROD`, and `NPEROD` are non-null readable mode scalars.
    /// - `L`, `M`, and `N` are non-null readable dimension scalars.
    /// - `C1` and `C2` are non-null readable coefficient scalars.
    /// - `A`, `B`, and `C` are non-null, writable, non-aliased arrays of at
    ///   least `N` elements. Cyclic problems may overwrite and restore them.
    /// - `LDIMF` and `MDIMF` are non-null readable leading dimensions with
    ///   `LDIMF >= L` and `MDIMF >= M`.
    /// - `F` is non-null, writable, non-aliased Fortran column-major storage
    ///   for `F(LDIMF, MDIMF, N)`. It is input RHS data and is overwritten with
    ///   the solution.
    /// - `IERROR` is a non-null writable status output: `0` is success; `1`,
    ///   `3` report invalid `LPEROD`/`MPEROD`; `2`, `4`, `6` require `L`, `M`,
    ///   `N >= 3`; `5` reports an invalid `NPEROD`; `7`/`8` require
    ///   `LDIMF >= L`/`MDIMF >= M`; `9` reports nonconstant `A`, `B`, or `C`
    ///   when `NPEROD == 0`; and `10` requires `A[0] == 0` and `C[N - 1] == 0`
    ///   when `NPEROD == 1`. A non-zero code prevents a solve.
    /// - `W` is non-null writable, non-aliased workspace with the documented
    ///   at least `30 + L + M + 2*N + max(L, M, N) + 7*((L+1)/2 + (M+1)/2)`
    ///   elements, where `/` is integer division. Native code does not retain
    ///   it.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null, aligned, and valid for the stated input
    /// or output extent. Storage must use the documented Fortran layout and no
    /// writable arguments may alias. All buffers and scalar pointers must stay
    /// alive for the synchronous call; native code retains no pointer. The
    /// caller is responsible for serializing legacy SLATEC runtime use.
    #[link_name = "pois3d_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pois3d.md"))]
    pub fn pois3d(
        lperod: *const FortranInteger,
        l: *const FortranInteger,
        c1: *const f32,
        mperod: *const FortranInteger,
        m: *const FortranInteger,
        c2: *const f32,
        nperod: *const FortranInteger,
        n: *const FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        ldimf: *const FortranInteger,
        mdimf: *const FortranInteger,
        f: *mut f32,
        ierror: *mut FortranInteger,
        w: *mut f32,
    );
}
