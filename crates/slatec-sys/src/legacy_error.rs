//! Minimal reviewed XERROR control declarations for internal safe-wrapper use.
//!
//! The safe crate does not re-export this module. It uses `XGETF` and `XSETF`
//! only while holding the native runtime lock to temporarily permit a known
//! level-one `DNLS1`/`SNLS1` completion message to return as an `INFO` value.

use crate::FortranInteger;

unsafe extern "C" {
    /// Reads the process-global legacy XERROR control flag.
    #[link_name = "xgetf_"]
    pub fn xgetf(control: *mut FortranInteger);

    /// Sets the process-global legacy XERROR control flag.
    #[link_name = "xsetf_"]
    pub fn xsetf(control: *mut FortranInteger);

    /// Reads the process-global legacy XERROR output-unit list.
    #[cfg(feature = "raw-family-optimization-linear-programming-in-memory")]
    #[link_name = "xgetua_"]
    pub fn xgetua(units: *mut FortranInteger, count: *mut FortranInteger);

    /// Restores the process-global legacy XERROR output-unit list.
    #[cfg(feature = "raw-family-optimization-linear-programming-in-memory")]
    #[link_name = "xsetua_"]
    pub fn xsetua(units: *mut FortranInteger, count: *mut FortranInteger);
}
