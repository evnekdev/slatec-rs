//! Hand-reviewed declaration for SLATEC FISHPACK `HWSCRT`.
//!
//! The selected driver is the single-precision `fishfft/hwscrt.f` source.
//! It receives scalar Fortran arguments by reference, mutates `f`, `pertrb`,
//! `ierror`, and its work array, and uses an `idimf`-by-`n + 1` column-major
//! `f` array.  The safe facade owns the mutable storage and validates every
//! dimensional precondition before entering this declaration.

use crate::FortranInteger;

unsafe extern "C" {
    /// Solves the reviewed Cartesian five-point Helmholtz finite-difference system.
    ///
    /// Original SLATEC routine: `HWSCRT`; supported ABI profile:
    /// `ffi-profile-gnu-mingw-x86_64`; native symbol: `hwscrt_`. Source:
    /// <https://www.netlib.org/slatec/fishfft/hwscrt.f>.
    ///
    /// # Arguments
    ///
    /// - `A`, `B`, `C`, `D`, and `ELMBDA` are non-null readable scalar inputs.
    /// - `M` and `N` are non-null readable `FortranInteger` panel counts.
    /// - `MBDCND` and `NBDCND` are non-null readable boundary-code scalars.
    /// - `BDA` and `BDB` are non-null readable edge arrays of at least `N + 1`
    ///   elements when their boundary codes require them; a readable dummy
    ///   element is required otherwise. Native code does not retain either.
    /// - `BDC` and `BDD` follow the same rule as `BDA` and `BDB`.
    /// - `F` is non-null, writable, and non-aliased column-major storage for
    ///   `F(IDIMF, N + 1)` with at least `IDIMF * (N + 1)` elements. It is input
    ///   RHS data and is overwritten with the solution.
    /// - `IDIMF` is a non-null readable leading dimension with `IDIMF >= M + 1`.
    /// - `PERTRB` is a non-null writable output. For singular Poisson cases it
    ///   is the compatibility correction subtracted from `F`; compare its
    ///   magnitude with the RHS before accepting the solution.
    /// - `IERROR` is a non-null writable status output: `0` is success; `1`
    ///   means `A >= B`; `2` an invalid `MBDCND`; `3` means `C >= D`; `4`
    ///   means `N <= 3`; `5` an invalid `NBDCND`; `6` means `ELMBDA > 0`
    ///   (the routine still attempts the solve); `7` means `IDIMF < M + 1`;
    ///   and `8` means `M <= 3`. Except for `0` and `6`, no solve is attempted.
    /// - `W` is non-null writable, non-aliased workspace. It must contain at
    ///   least `4*(N+1) + (13 + floor(log2(N+1)))*(M+1)` `f32` elements; native
    ///   code does not retain it. On return, `W[0]` contains the actual number
    ///   of workspace elements used.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null, aligned, and valid for the stated input
    /// or output extent. Array storage must be Fortran column-major and may not
    /// alias writable `F`, `PERTRB`, `IERROR`, or `W`. Boundary arrays and
    /// workspace must remain alive for the call; native code retains no pointer.
    /// The caller is responsible for serializing legacy SLATEC runtime use.
    #[link_name = "hwscrt_"]
    pub fn hwscrt(
        a: *const f32,
        b: *const f32,
        m: *const FortranInteger,
        mbdcnd: *const FortranInteger,
        bda: *const f32,
        bdb: *const f32,
        c: *const f32,
        d: *const f32,
        n: *const FortranInteger,
        nbdcnd: *const FortranInteger,
        bdc: *const f32,
        bdd: *const f32,
        elmbda: *const f32,
        f: *mut f32,
        idimf: *const FortranInteger,
        pertrb: *mut f32,
        ierror: *mut FortranInteger,
        w: *mut f32,
    );
}
