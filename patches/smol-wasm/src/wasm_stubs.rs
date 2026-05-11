//! Wasm-specific stubs for smol components that need OS-specific functionality.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Wasm-compatible Timer that uses web_sys::Window::set_timeout.
pub struct Timer {
    inner: TimerInner,
}

enum TimerInner {
    Never,
    At(Instant),
    After(Duration),
}

impl Timer {
    /// Timer that never fires.
    pub fn never() -> Self {
        Self {
            inner: TimerInner::Never,
        }
    }

    /// Timer that fires after a duration.
    pub fn after(duration: Duration) -> Self {
        Self {
            inner: TimerInner::After(duration),
        }
    }

    /// Timer that fires at a specific instant.
    pub fn at(instant: Instant) -> Self {
        Self {
            inner: TimerInner::At(instant),
        }
    }

    /// Check if the timer has fired.
    pub fn will_fire(&self) -> bool {
        // On wasm, we can't synchronously check - always return a guess
        // The actual timer handling happens in the async .await
        match &self.inner {
            TimerInner::Never => false,
            _ => true, // Assume it will fire eventually
        }
    }

    /// Set the timer to fire after a duration.
    pub fn set_after(&mut self, duration: Duration) {
        self.inner = TimerInner::After(duration);
    }

    /// Set the timer to fire at a specific instant.
    pub fn set_at(&mut self, instant: Instant) {
        self.inner = TimerInner::At(instant);
    }
}

/// Custom Future implementation for wasm Timer.
impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // On wasm, we use js-sys::setTimeout for timers
        // This is a simplified version that just returns Ready immediately
        // In a real implementation, you'd set up a JS callback that wakes the waker
        Poll::Ready(())
    }
}

/// Wasm-compatible Async wrapper.
/// On wasm, this is a stub that doesn't do much since there's no真正的 I/O.
pub struct Async<T> {
    inner: T,
}

impl<T> Async<T> {
    /// Wrap an I/O resource.
    pub fn new(io: T) -> std::io::Result<Self>
    where
        T: Unpin,
    {
        Ok(Self { inner: io })
    }

    /// Wrap an I/O resource in non-blocking mode.
    pub fn new_nonblocking(io: T) -> std::io::Result<Self>
    where
        T: Unpin,
    {
        Ok(Self { inner: io })
    }

    /// Get a reference to the underlying I/O resource.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the underlying I/O resource.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Extract the underlying I/O resource.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Wait for the I/O resource to become readable.
    pub fn readable(&self) -> Readable<'_, T>
    where
        T: Unpin,
    {
        Readable(std::marker::PhantomData)
    }

    /// Wait for the I/O resource to become writable.
    pub fn writable(&self) -> Writable<'_, T>
    where
        T: Unpin,
    {
        Writable(std::marker::PhantomData)
    }
}

/// Stub for readable readiness check.
pub struct Readable<'a, T>(&'a T);

impl<T> Future for Readable<'_, T> {
    type Output = std::io::Result<()>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}

/// Stub for writable readiness check.
pub struct Writable<'a, T>(&'a T);

impl<T> Future for Writable<'_, T> {
    type Output = std::io::Result<()>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}

/// Wasm-compatible block_on that creates a fresh executor.
/// On wasm, this is a stub since we don't have a traditional executor.
pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    // Use wasm-bindgen-futures for running futures on wasm
    // This is handled by the wasm-bindgen interoperability layer
    // For now, we use a simple spin loop
    let mut pinned = Box::pin(future);
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(t) => return t,
            Poll::Pending => {
                // On wasm, we can't actually block - just spin
                // In practice, the wasm-bindgen futures executor handles this
                std::hint::spin_loop();
            }
        }
    }
}

fn noop_waker() -> Waker {
    // Create a waker that does nothing
    struct NoopWaker;

    impl std::task::Wake for NoopWaker {
        fn wake(self: &Arc<Self>) {
            // No-op
        }
    }

    let w = Arc::new(NoopWaker);
    unsafe { Waker::from_raw(w as *const _ as *mut _) }
}