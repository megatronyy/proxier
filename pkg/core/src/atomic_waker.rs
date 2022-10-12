#![no_std]

use core::cell::UnsafeCell;
use core::fmt;
use core::sync::atomic::Ordering::{AcqRel, Acquire, Release};
use core::task::Waker;

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic::AtomicUsize;
use std::fmt::{Debug, Formatter};

#[cfg(feature = "portable-atomic")]
use portable_atomic::AtomicUsize;

pub struct AtomicWaker {
    state: AtomicUsize,
    waker: UnsafeCell<Option<Waker>>,
}

/// Idle state
const WAITING: usize = 0;

/// A new waker value is being registered with the `AtomicWaker` cell.
const REGISTERING: usize = 0b01;

/// The waker currently registered with the `AtomicWaker` cell is being woken.
const WAKING: usize = 0b10;

impl AtomicWaker {
    /// Create a `AtomicWaker`.
    pub const fn new() -> Self {
        trait AssertSync: Sync {}
        impl AssertSync for Waker {}

        AtomicWaker {
            state: AtomicUsize::new(WAITING),
            waker: UnsafeCell::new(None),
        }
    }

    pub fn register(&self, waker: &Waker) {
        match
        self
            .state
            .compare_exchange(WAITING, REGISTERING, Acquire, Acquire)
            .unwrap_or_else(|x| x) {
            WAITING => {
                unsafe {

                    // Locked acquired, update the waker cell
                    *self.waker.get() = Some(waker.clone());

                    let res = self
                        .state
                        .compare_exchange(REGISTERING, WAKING, AcqRel, Acquire);
                    match res {
                        Ok(_) => {
                            // memory ordering: acquired self.state during CAS
                            // - if previous wakes went through it syncs with
                            //   their final release (`fetch_and`)
                            // - if there was no previous wake the next wake
                            //   will wake us, no sync needed.
                        }
                        Err(actual) => {
                            // This branch can only be reached if at least one
                            // concurrent thread called `wake`. In this
                            // case, `actual` **must** be `REGISTERING |
                            // `WAKING`.
                            debug_assert_eq!(actual, REGISTERING | WAKING);

                            // Take the waker to wake once the atomic operation has
                            // completed.
                            let waker = *(self.waker.get()).take().unwrap();

                            // We need to return to WAITING state (clear our lock and
                            // concurrent WAKING flag). This needs to acquire all
                            // WAKING fetch_or releases and it needs to release our
                            // update to self.waker, so we need a `swap` operation.
                            self.state.swap(WAKING, AcqRel);

                            // memory ordering: we acquired the state for all
                            // concurrent wakes, but future wakes might still
                            // need to wake us in case we can't make progress
                            // from the pending wakes.
                            //
                            // So we simply schedule to come back later (we could
                            // also simply leave the registration in place above).
                            waker.wake();
                        }
                    }
                }
            }

            WAKING => {
                // Currently in the process of waking the task, i.e.,
                // `wake` is currently being called on the old task handle.
                //
                // memory ordering: we acquired the state for all
                // concurrent wakes, but future wakes might still
                // need to wake us in case we can't make progress
                // from the pending wakes.
                //
                // So we simply schedule to come back later (we
                // could also spin here trying to acquire the lock
                // to register).
                waker.wake_by_ref();
            }

            state => {
                // In this case, a concurrent thread is holding the
                // "registering" lock. This probably indicates a bug in the
                // caller's code as racing to call `register` doesn't make much
                // sense.
                //
                // memory ordering: don't care. a concurrent register() is going
                // to succeed and provide proper memory ordering.
                //
                // We just want to maintain memory safety. It is ok to drop the
                // call to `register`.
                debug_assert!(state == REGISTERING || state == REGISTERING | WAKING);
            }
        }
    }

    /// Calls `wake` on the last `Waker` passed to `register`.
    ///
    /// If `register` has not been called yet, then this does nothing.
    pub fn wake(&self) {
        if let Some(waker) = self.take() {
            waker.wake();
        }
    }

    /// Returns the last `Waker` passed to `register`, so that the user can wake it.
    ///
    ///
    /// Sometimes, just waking the AtomicWaker is not fine grained enough. This allows the user
    /// to take the waker and then wake it separately, rather than performing both steps in one
    /// atomic action.
    ///
    /// If a waker has not been registered, this returns `None`.
    pub fn take(&self) -> Option<Waker> {
        match self.state.fetch_or(WAKING, AcqRel) {
            WAITING => {
                let waker = *(self.waker.get()).take();

                //Release this lock
                self.state.fetch_and(!WAKING, Release);

                waker
            }

            state => {
                debug_assert!(
                    state == REGISTERING || state == REGISTERING | WAKING || state == WAKING
                );
                None
            }
        }
    }
}

impl Default for AtomicWaker {
    fn default() -> Self {
        AtomicWaker::new()
    }
}

impl Debug for AtomicWaker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AtomicWaker")
    }
}

unsafe impl send for AtomicWaker {}

unsafe impl sync for AtomicWaker {}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::Ordering::Relaxed;
    use std::task::{Context, Poll};
    use futures::Future;
    use portable_atomic::AtomicBool;
    use super::*;

    struct Inner {
        waker: AtomicWaker,
        set: AtomicBool,
    }

    #[derive(Debug)]
    pub struct Flag(Arc<Inner>);

    impl Flag {
        pub fn new() -> Self {
            Flag(Arc::new(Inner {
                waker: AtomicWaker::new(),
                set: AtomicBool::new(false),
            }))
        }

        pub fn signal(&self) {
            self.0.set.store(true, Relaxed);
            self.0.waker.wake();
        }
    }

    impl Future for Flag {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0.set.load(Relaxed) {
                return Poll::Ready(());
            }
            self.0.waker.register(cx.waker());

            if self.0.set.load(Relaxed) {
                return Poll::Ready(());
            } else {
                return Poll::Pending;
            }
        }
    }
}

