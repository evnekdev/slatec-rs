#![no_std]
#![deny(missing_docs)]

//! Native implementation-provider selection for `slatec`.
//!
//! This crate intentionally exposes no numerical API. Its Cargo features
//! select one native provider and keep that provider in the dependency graph.
//! The runtime library is `no_std`; acquisition, compilation, and linker
//! configuration are confined to its hosted Cargo build script.

/// Keeps the selected implementation provider in the final dependency graph.
///
/// Safe wrapper crates call this zero-cost marker internally. It performs no
/// runtime initialization and is not a numerical interface.
#[doc(hidden)]
#[inline(never)]
pub fn ensure_linked() {}

#[cfg(feature = "optimization-linear-programming-in-memory")]
mod lp_forbidden_io {
    use core::sync::atomic::{AtomicUsize, Ordering};

    static PAGE_ENTRIES: AtomicUsize = AtomicUsize::new(0);
    static OPEN_ENTRIES: AtomicUsize = AtomicUsize::new(0);
    static CLOSE_ENTRIES: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn reset() {
        PAGE_ENTRIES.store(0, Ordering::SeqCst);
        OPEN_ENTRIES.store(0, Ordering::SeqCst);
        CLOSE_ENTRIES.store(0, Ordering::SeqCst);
    }

    pub(crate) fn counts() -> (usize, usize, usize) {
        (
            PAGE_ENTRIES.load(Ordering::SeqCst),
            OPEN_ENTRIES.load(Ordering::SeqCst),
            CLOSE_ENTRIES.load(Ordering::SeqCst),
        )
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn dprwpg_(
        _key: *mut i32,
        _page: *mut i32,
        _page_len: *mut i32,
        _real: *mut f64,
        _integer: *mut i32,
    ) {
        PAGE_ENTRIES.fetch_add(1, Ordering::SeqCst);
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn prwpge_(
        _key: *mut i32,
        _page: *mut i32,
        _page_len: *mut i32,
        _real: *mut f32,
        _integer: *mut i32,
    ) {
        PAGE_ENTRIES.fetch_add(1, Ordering::SeqCst);
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn dprwvr_(
        _key: *mut i32,
        _page: *mut i32,
        _page_len: *mut i32,
        _real: *mut f64,
        _integer: *mut i32,
    ) {
        PAGE_ENTRIES.fetch_add(1, Ordering::SeqCst);
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn prwvir_(
        _key: *mut i32,
        _page: *mut i32,
        _page_len: *mut i32,
        _real: *mut f32,
        _integer: *mut i32,
    ) {
        PAGE_ENTRIES.fetch_add(1, Ordering::SeqCst);
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn sopenm_(_unit: *mut i32, _record_len: *mut i32) {
        OPEN_ENTRIES.fetch_add(1, Ordering::SeqCst);
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn sclosm_(_unit: *mut i32) {
        CLOSE_ENTRIES.fetch_add(1, Ordering::SeqCst);
    }
}

/// Clears the forbidden paging/I/O entry counters for one serialized LP call.
///
/// This is an internal provider contract hook, not a paging interface.
#[cfg(feature = "optimization-linear-programming-in-memory")]
#[doc(hidden)]
pub fn reset_lp_forbidden_io_entries() {
    lp_forbidden_io::reset();
}

/// Returns `(paging, open, close)` forbidden-entry counts.
///
/// A supported in-memory LP call requires all three values to remain zero.
#[cfg(feature = "optimization-linear-programming-in-memory")]
#[doc(hidden)]
#[must_use]
pub fn lp_forbidden_io_entries() -> (usize, usize, usize) {
    lp_forbidden_io::counts()
}
