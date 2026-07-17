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
}
