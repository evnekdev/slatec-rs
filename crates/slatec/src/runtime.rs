//! Shared serialization for selected Fortran process-global runtime state.

use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Condvar, Mutex};
use std::thread::ThreadId;

#[cfg(feature = "ode-sdrive-expert-native-tests")]
use std::sync::atomic::{AtomicUsize, Ordering};

struct LockState {
    owner: Option<ThreadId>,
    depth: usize,
}

struct NativeRuntimeLock {
    state: Mutex<LockState>,
    available: Condvar,
}

impl NativeRuntimeLock {
    const fn new() -> Self {
        Self {
            state: Mutex::new(LockState {
                owner: None,
                depth: 0,
            }),
            available: Condvar::new(),
        }
    }

    fn lock(&'static self) -> NativeRuntimeGuard {
        let current = std::thread::current().id();
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        loop {
            match state.owner.as_ref() {
                None => {
                    state.owner = Some(current);
                    state.depth = 1;
                    break;
                }
                Some(owner) if owner == &current => {
                    state.depth += 1;
                    break;
                }
                Some(_) => {
                    state = self
                        .available
                        .wait(state)
                        .unwrap_or_else(|error| error.into_inner());
                }
            }
        }
        NativeRuntimeGuard {
            lock: self,
            _not_send: PhantomData,
        }
    }
}

static NATIVE_RUNTIME_LOCK: NativeRuntimeLock = NativeRuntimeLock::new();

/// A reentrant-on-the-current-thread guard for the selected native runtime.
///
/// Reentrancy permits a Rust integrand to call a non-callback SLATEC facade
/// without deadlocking. Callback-bearing integration itself is rejected by
/// the quadrature callback registry before this lock is reacquired.
pub(crate) struct NativeRuntimeGuard {
    lock: &'static NativeRuntimeLock,
    _not_send: PhantomData<Rc<()>>,
}

impl Drop for NativeRuntimeGuard {
    fn drop(&mut self) {
        let current = std::thread::current().id();
        let mut state = self
            .lock
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if state.owner.as_ref() != Some(&current) || state.depth == 0 {
            return;
        }
        state.depth -= 1;
        if state.depth == 0 {
            state.owner = None;
            self.lock.available.notify_one();
        }
    }
}

pub(crate) fn lock_native() -> NativeRuntimeGuard {
    NATIVE_RUNTIME_LOCK.lock()
}

#[cfg(feature = "ode-sdrive-expert-native-tests")]
static ACTIVE_ODE_NATIVE_CALLS: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "ode-sdrive-expert-native-tests")]
static MAX_ACTIVE_ODE_NATIVE_CALLS: AtomicUsize = AtomicUsize::new(0);

/// Test-only scope that measures actual SDRIVE foreign-call overlap.
///
/// The scope is created only after the process-wide native lock is held and
/// surrounds the `SDRIV3` or `DDRIV3` ABI call, rather than merely measuring
/// Rust thread activity around a session.
#[cfg(feature = "ode-sdrive-expert-native-tests")]
pub(crate) struct OdeNativeCallAudit;

#[cfg(feature = "ode-sdrive-expert-native-tests")]
impl OdeNativeCallAudit {
    pub(crate) fn enter() -> Self {
        let active = ACTIVE_ODE_NATIVE_CALLS.fetch_add(1, Ordering::SeqCst) + 1;
        let mut observed = MAX_ACTIVE_ODE_NATIVE_CALLS.load(Ordering::SeqCst);
        while active > observed {
            match MAX_ACTIVE_ODE_NATIVE_CALLS.compare_exchange(
                observed,
                active,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(next) => observed = next,
            }
        }
        Self
    }
}

#[cfg(feature = "ode-sdrive-expert-native-tests")]
impl Drop for OdeNativeCallAudit {
    fn drop(&mut self) {
        let result =
            ACTIVE_ODE_NATIVE_CALLS.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |active| {
                active.checked_sub(1)
            });
        debug_assert!(result.is_ok(), "ODE native-call audit scope underflowed");
    }
}

#[cfg(feature = "ode-sdrive-expert-native-tests")]
pub(crate) fn reset_ode_native_call_audit() {
    let _runtime = lock_native();
    debug_assert_eq!(ACTIVE_ODE_NATIVE_CALLS.load(Ordering::SeqCst), 0);
    ACTIVE_ODE_NATIVE_CALLS.store(0, Ordering::SeqCst);
    MAX_ACTIVE_ODE_NATIVE_CALLS.store(0, Ordering::SeqCst);
}

#[cfg(feature = "ode-sdrive-expert-native-tests")]
pub(crate) fn max_ode_native_calls() -> usize {
    MAX_ACTIVE_ODE_NATIVE_CALLS.load(Ordering::SeqCst)
}

/// Temporarily makes reviewed legacy level-one completion messages return.
///
/// Reviewed native drivers can report nonfatal completion states through
/// level-one `XERMSG` after writing a final state. The validated profile's
/// default error policy terminates at level one, so the wrappers listed below
/// apply this scope only while the process-global native runtime lock is held
/// and restore the prior `XSETF` control value before returning to Rust.
#[cfg(any(
    feature = "least-squares-nonlinear-easy",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance",
    feature = "least-squares-linear-bounded",
    feature = "least-squares-linear-constrained",
    feature = "least-squares-linear-bounded-constrained",
    feature = "ode-sdrive-expert"
))]
pub(crate) fn permit_recoverable_native_statuses() -> RecoverableErrorScope {
    let mut previous = 0;
    // SAFETY: these reviewed XERROR controls take one valid INTEGER pointer.
    // The caller holds the shared process-global native runtime lock.
    unsafe { slatec_sys::legacy_error::xgetf(&mut previous) };
    let mut nonfatal = 0;
    // SAFETY: zero is a documented XSETF control value that suppresses the
    // fatal branch for level-one recoverable messages; restoration is RAII.
    unsafe { slatec_sys::legacy_error::xsetf(&mut nonfatal) };
    RecoverableErrorScope { previous }
}

/// Restores the prior XERROR control flag after a scoped native call.
#[cfg(any(
    feature = "least-squares-nonlinear-easy",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance",
    feature = "least-squares-linear-bounded",
    feature = "least-squares-linear-constrained",
    feature = "least-squares-linear-bounded-constrained",
    feature = "ode-sdrive-expert"
))]
pub(crate) struct RecoverableErrorScope {
    previous: slatec_sys::FortranInteger,
}

#[cfg(any(
    feature = "least-squares-nonlinear-easy",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance",
    feature = "least-squares-linear-bounded",
    feature = "least-squares-linear-constrained",
    feature = "least-squares-linear-bounded-constrained",
    feature = "ode-sdrive-expert"
))]
impl Drop for RecoverableErrorScope {
    fn drop(&mut self) {
        // SAFETY: previous came directly from XGETF while the native lock was
        // held; XSETF accepts this documented control range by construction.
        unsafe { slatec_sys::legacy_error::xsetf(&mut self.previous) };
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn lock_is_reentrant_and_serializes_other_threads() {
        let first = super::lock_native();
        let nested = super::lock_native();
        drop(nested);

        let (sender, receiver) = mpsc::channel();
        let handle = std::thread::spawn(move || {
            let _guard = super::lock_native();
            sender.send(()).unwrap();
        });
        assert!(receiver.recv_timeout(Duration::from_millis(20)).is_err());
        drop(first);
        receiver.recv_timeout(Duration::from_secs(1)).unwrap();
        handle.join().unwrap();
    }
}
